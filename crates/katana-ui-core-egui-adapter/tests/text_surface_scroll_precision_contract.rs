use katana_ui_core::atom::TextArea;
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfaceGutter, TextSurfaceLogicalPixels,
    TextSurfacePresentation, TextSurfaceProps, TextSurfaceScrollAlignment,
    TextSurfaceScrollRequest, TextSurfaceScrollRequestRejection, TextSurfaceScrollRequestToken,
    TextSurfaceScrollTarget, TextSurfaceViewport,
};
use katana_ui_core::theme::{FontFamily, FontToken};
use katana_ui_core_egui_adapter::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceError, EguiTextSurfaceOutput, TextSurfacePaintStyle,
    TextSurfaceRasterStyle,
};

const SCREEN_WIDTH: f32 = 640.0;
const VIEWPORT_HEIGHT: f32 = 32.0;
const FONT_SIZE: f32 = 16.0;
const LINE_HEIGHT: f32 = 24.0;
const FONT_WEIGHT: u16 = 400;
const TEXT_COLOR: [u8; 4] = [235, 235, 235, 255];
const BACKGROUND_COLOR: [u8; 4] = [24, 24, 24, 255];
const GUTTER_COLOR: [u8; 4] = [32, 32, 32, 255];
const SELECTION_COLOR: [u8; 4] = [64, 96, 160, 180];
const PREEDIT_COLOR: [u8; 4] = [255, 196, 64, 255];
const CARET_COLOR: [u8; 4] = [255, 255, 255, 255];

#[test]
fn actual_raw_input_relative_pixel_precision_nonfinite_and_visible_rows_are_artifacted()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let source = source();
    let screen =
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(SCREEN_WIDTH, VIEWPORT_HEIGHT));
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("scroll-precision-raw-input").value(&source),
            UiTextSpan::emoji_marked_spans(&source, Default::default()),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, VIEWPORT_HEIGHT as u32),
        )
        .gutter(TextSurfaceGutter::new(32).automatic_numbered()),
    );

    let (initial_accesskit, initial) =
        run_frame(&context, &mut adapter, &mut surface, Vec::new(), screen)?;
    assert!(!initial.record.frame.visible_logical_rows.is_empty());
    assert!(!initial.record.frame.gutter.is_empty());
    assert!(initial.record.frame.gutter.len() >= initial.record.frame.visible_logical_rows.len());
    assert_eq!(initial.record, initial.artifact.record);
    assert_eq!(64, initial.artifact.frame_record_hash.len());
    assert_eq!(64, initial.artifact.paint_plan_hash.len());
    assert_accesskit_text_input(&initial_accesskit);

    let point = center(initial.record.frame.viewport_bounds);
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, true), pointer_button(point, false)],
        screen,
    )?;
    let (_, preedit) = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit(
            "入力中 ⭐️".to_string(),
        ))],
        screen,
    )?;
    let before_first = surface.state().clone();
    assert!(before_first.text_area.focused);
    assert_eq!(
        Some("入力中 ⭐️"),
        preedit
            .record
            .frame
            .preedit
            .as_ref()
            .map(|value| value.text.as_str())
    );
    assert!(
        surface
            .state()
            .scroll_bounds
            .expect("adapter must measure bounds")
            .max_y
            >= 42
    );

    synchronize_relative_request(&mut surface, "fractional-1", 20.6);
    let (first_accesskit, first) =
        run_frame(&context, &mut adapter, &mut surface, Vec::new(), screen)?;
    assert_acknowledgement(&first, "fractional-1", 21);
    assert_eq!(before_first.text_area, surface.state().text_area);
    assert_eq!(before_first.pointer_anchor, surface.state().pointer_anchor);
    assert_ne!(
        preedit.record.frame.visible_logical_rows,
        first.record.frame.visible_logical_rows
    );
    assert_eq!(64, first.artifact.frame_record_hash.len());
    assert_eq!(64, first.artifact.paint_plan_hash.len());
    assert_accesskit_text_input(&first_accesskit);

    let first_scroll_y = surface.state().scroll_y;
    let (_, repeated) = run_frame(&context, &mut adapter, &mut surface, Vec::new(), screen)?;
    assert_eq!(first_scroll_y, surface.state().scroll_y);
    assert!(!repeated.events.iter().any(is_scroll_request_event));

    synchronize_relative_request(&mut surface, "fractional-2", 20.6);
    let (_, fresh) = run_frame(&context, &mut adapter, &mut surface, Vec::new(), screen)?;
    assert_acknowledgement(&fresh, "fractional-2", first_scroll_y + 21);
    assert_ne!(
        first.record.frame.visible_logical_rows,
        fresh.record.frame.visible_logical_rows
    );

    let before_nonfinite = surface.state().clone();
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("nonfinite-raw-input"),
        TextSurfaceScrollTarget::relative_pixels(
            TextSurfaceLogicalPixels::new(f32::NAN),
            TextSurfaceLogicalPixels::new(f32::INFINITY),
        ),
        TextSurfaceScrollAlignment::Nearest,
    ));
    assert!(surface.synchronize_presentation(presentation));
    let (nonfinite_accesskit, nonfinite) =
        run_frame(&context, &mut adapter, &mut surface, Vec::new(), screen)?;
    let rejection = TextSurfaceEvent::ScrollRequestRejected {
        token: TextSurfaceScrollRequestToken::new("nonfinite-raw-input"),
        reason: TextSurfaceScrollRequestRejection::NonFiniteRelativePixels,
    };
    assert_eq!(
        std::slice::from_ref(&rejection),
        nonfinite.events.as_slice()
    );
    assert_eq!(
        std::slice::from_ref(&rejection),
        nonfinite.artifact.events.as_slice()
    );
    assert_eq!(before_nonfinite.text_area, surface.state().text_area);
    assert_eq!(
        before_nonfinite.pointer_anchor,
        surface.state().pointer_anchor
    );
    assert_eq!(before_nonfinite.scroll_x, surface.state().scroll_x);
    assert_eq!(before_nonfinite.scroll_y, surface.state().scroll_y);
    assert_eq!(
        before_nonfinite.scroll_bounds,
        surface.state().scroll_bounds
    );
    assert_eq!(fresh.record.frame, nonfinite.record.frame);
    assert_eq!(
        fresh.record.raster_identity,
        nonfinite.record.raster_identity
    );
    assert_eq!(fresh.artifact.paint_plan, nonfinite.artifact.paint_plan);
    assert_ne!(
        fresh.artifact.frame_record_hash,
        nonfinite.artifact.frame_record_hash
    );
    assert_eq!(
        fresh.artifact.paint_plan_hash,
        nonfinite.artifact.paint_plan_hash
    );
    assert_accesskit_text_input(&nonfinite_accesskit);
    Ok(())
}

fn source() -> String {
    [
        "日本語 ⭐️",
        "二行目",
        "三行目",
        "四行目",
        "五行目",
        "六行目",
        "七行目",
        "八行目",
        "九行目",
        "十行目",
        "十一行目",
        "十二行目",
    ]
    .join("\n")
}

fn synchronize_relative_request(surface: &mut TextSurface, token: &str, delta_y: f32) {
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new(token),
        TextSurfaceScrollTarget::relative_pixels(0.0, delta_y),
        TextSurfaceScrollAlignment::Nearest,
    ));
    assert!(surface.synchronize_presentation(presentation));
}

fn assert_acknowledgement(output: &EguiTextSurfaceOutput, token: &str, scroll_y: i32) {
    let acknowledgements = output
        .events
        .iter()
        .filter(|event| matches!(
            event,
            TextSurfaceEvent::ScrollRequestAcknowledged(value)
                if value.token.as_str() == token && value.target_bounds.is_none() && value.scroll_y == scroll_y
        ))
        .count();
    assert_eq!(1, acknowledgements);
    assert_eq!(output.events, output.artifact.events);
    assert_eq!(scroll_y, output.record.frame.viewport.scroll_y);
}

fn is_scroll_request_event(event: &TextSurfaceEvent) -> bool {
    matches!(
        event,
        TextSurfaceEvent::ScrollRequestAcknowledged(_)
            | TextSurfaceEvent::ScrollRequestRejected { .. }
    )
}

fn assert_accesskit_text_input(full_output: &egui::FullOutput) {
    let update = full_output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("actual RawInput frame must publish AccessKit");
    assert!(
        update
            .nodes
            .iter()
            .any(|(_, node)| node.role() == egui::accesskit::Role::MultilineTextInput)
    );
}

fn center(bounds: katana_ui_core::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

fn run_frame(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
    screen_rect: egui::Rect,
) -> Result<(egui::FullOutput, EguiTextSurfaceOutput), EguiTextSurfaceError> {
    let mut result = None;
    let full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(screen_rect),
            events,
            ..egui::RawInput::default()
        },
        |ui| result = Some(adapter.show(ui, surface, &raster_style(), &paint_style())),
    );
    Ok((
        full_output,
        result.ok_or(EguiTextSurfaceError::FrameNotProduced)??,
    ))
}

fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

fn raster_style() -> TextSurfaceRasterStyle {
    TextSurfaceRasterStyle::new(
        FontToken {
            name: "scroll-precision".to_string(),
            family: FontFamily::Monospace,
            size: FONT_SIZE,
            weight: FONT_WEIGHT,
        },
        TEXT_COLOR,
        LINE_HEIGHT,
    )
}

fn paint_style() -> TextSurfacePaintStyle {
    TextSurfacePaintStyle {
        background_rgba: BACKGROUND_COLOR,
        gutter_background_rgba: GUTTER_COLOR,
        gutter_paints: Vec::new(),
        selection_rgba: SELECTION_COLOR,
        preedit_rgba: PREEDIT_COLOR,
        caret_rgba: CARET_COLOR,
        annotation_paints: Vec::new(),
    }
}
