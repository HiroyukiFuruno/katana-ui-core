use katana_ui_core::atom::TextArea;
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfaceFocusRequest, TextSurfaceFocusRequestToken,
    TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use katana_ui_core::theme::{FontFamily, FontToken};
use katana_ui_core_egui_adapter::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceError, EguiTextSurfaceOutput, TextSurfacePaintStyle,
    TextSurfaceRasterStyle,
};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 240.0;
const SURFACE_HEIGHT: f32 = 72.0;
const LINE_HEIGHT: f32 = 24.0;
const FONT_SIZE: f32 = 16.0;
const TEXT_COLOR: [u8; 4] = [235, 235, 235, 255];
const BACKGROUND_COLOR: [u8; 4] = [24, 24, 24, 255];
const SELECTION_COLOR: [u8; 4] = [64, 96, 160, 180];
const PREEDIT_COLOR: [u8; 4] = [255, 196, 64, 255];
const CARET_COLOR: [u8; 4] = [255, 255, 255, 255];

#[test]
fn actual_raw_input_controlled_focus_requests_are_idempotent_and_artifacted()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let source = "一行目 日本語 ⭐️\n二行目\n三行目\n四行目\n五行目";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("controlled-focus").value(source),
        UiTextSpan::emoji_marked_spans(source, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SURFACE_HEIGHT as u32)
            .scroll_offset(0, LINE_HEIGHT as i32),
    ));

    let _ = run_frame(&context, &mut adapter, &mut surface, Vec::new(), false)?;
    let mut focus = TextSurfacePresentation::from_props(surface.props());
    focus.focus_request = Some(TextSurfaceFocusRequest::new(
        TextSurfaceFocusRequestToken::new("focus-1"),
        true,
    ));
    assert!(surface.synchronize_presentation(focus));
    let (full_output, issued) = run_frame(&context, &mut adapter, &mut surface, Vec::new(), false)?;
    assert!(issued.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::FocusRequestAcknowledged(value)
            if value.token.as_str() == "focus-1" && value.focused
    )));
    assert!(issued.artifact.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::FocusRequestAcknowledged(value)
            if value.token.as_str() == "focus-1" && value.focused
    )));
    assert!(issued.record.focus_request.is_some());
    assert!(!issued.record.frame.accessibility.root.focused);
    assert_eq!(issued.record, issued.artifact.record);
    assert_eq!(64, issued.artifact.frame_record_hash.len());
    assert_eq!(64, issued.artifact.paint_plan_hash.len());
    let accesskit = full_output
        .platform_output
        .accesskit_update
        .expect("focus request frame must publish AccessKit");
    assert!(
        accesskit
            .nodes
            .iter()
            .any(|(_, node)| { node.role() == egui::accesskit::Role::MultilineTextInput })
    );

    let focused = run_frame(&context, &mut adapter, &mut surface, Vec::new(), false)?.1;
    assert!(focused.record.frame.accessibility.root.focused);
    assert!(
        focused
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::FocusChanged(true)))
    );
    let before = surface.state().clone();
    let preedit = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit(
            "入力中⭐️".to_string(),
        ))],
        false,
    )?
    .1;
    assert_eq!(source, surface.state().text_area.value);
    assert_eq!(
        before.text_area.selection,
        surface.state().text_area.selection
    );
    assert_eq!(
        Some("入力中⭐️"),
        preedit
            .record
            .frame
            .preedit
            .as_ref()
            .map(|value| value.text.as_str())
    );
    let preedit_raster_identity = preedit.record.raster_identity.clone();
    let preedit_texture_bounds = preedit.record.texture_bounds;

    let outside = egui::pos2(12.0, SURFACE_HEIGHT + 16.0);
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(outside, true)],
        true,
    )?;
    let user_blur = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(outside, false)],
        true,
    )?
    .1;
    assert!(
        user_blur
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::FocusChanged(false)))
    );
    assert!(!surface.state().text_area.focused);
    let after_user_blur = surface.state().clone();

    let same = TextSurfacePresentation::from_props(surface.props());
    let _ = surface.synchronize_presentation(same);
    let same_token = run_frame(&context, &mut adapter, &mut surface, Vec::new(), true)?.1;
    assert!(
        !same_token
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::FocusRequestAcknowledged(_)))
    );
    assert!(!same_token.record.frame.accessibility.root.focused);
    assert_eq!(after_user_blur.text_area, surface.state().text_area);
    assert_eq!(after_user_blur.scroll_x, surface.state().scroll_x);
    assert_eq!(after_user_blur.scroll_y, surface.state().scroll_y);

    let mut replacement = TextSurfacePresentation::from_props(surface.props());
    replacement.focus_request = Some(TextSurfaceFocusRequest::new(
        TextSurfaceFocusRequestToken::new("focus-2"),
        true,
    ));
    assert!(surface.synchronize_presentation(replacement));
    let focus_again = run_frame(&context, &mut adapter, &mut surface, Vec::new(), true)?.1;
    assert!(focus_again.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::FocusRequestAcknowledged(value)
            if value.token.as_str() == "focus-2" && value.focused
    )));
    let focused_again = run_frame(&context, &mut adapter, &mut surface, Vec::new(), true)?.1;
    assert!(focused_again.record.frame.accessibility.root.focused);

    let mut blur = TextSurfacePresentation::from_props(surface.props());
    blur.focus_request = Some(TextSurfaceFocusRequest::new(
        TextSurfaceFocusRequestToken::new("blur-1"),
        false,
    ));
    assert!(surface.synchronize_presentation(blur));
    let blur_issued = run_frame(&context, &mut adapter, &mut surface, Vec::new(), true)?.1;
    assert!(blur_issued.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::FocusRequestAcknowledged(value)
            if value.token.as_str() == "blur-1" && !value.focused
    )));
    let blurred = run_frame(&context, &mut adapter, &mut surface, Vec::new(), true)?.1;
    assert!(!blurred.record.frame.accessibility.root.focused);
    assert_eq!(source, surface.state().text_area.value);
    assert_eq!(
        after_user_blur.text_area.selection,
        surface.state().text_area.selection
    );
    assert_eq!(after_user_blur.scroll_x, surface.state().scroll_x);
    assert_eq!(after_user_blur.scroll_y, surface.state().scroll_y);
    assert_eq!(
        Some("入力中⭐️"),
        blurred
            .record
            .frame
            .preedit
            .as_ref()
            .map(|value| value.text.as_str())
    );
    assert_eq!(preedit_raster_identity, blurred.record.raster_identity);
    assert_eq!(preedit_texture_bounds, blurred.record.texture_bounds);
    Ok(())
}

fn run_frame(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
    external_focus_target: bool,
) -> Result<(egui::FullOutput, EguiTextSurfaceOutput), EguiTextSurfaceError> {
    let mut rendered = None;
    let full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            rendered = Some(adapter.show(ui, surface, &raster_style(), &paint_style()));
            if external_focus_target {
                let (_, response) =
                    ui.allocate_exact_size(egui::vec2(SCREEN_WIDTH, 40.0), egui::Sense::click());
                if response.clicked() {
                    response.request_focus();
                }
            }
        },
    );
    Ok((
        full_output,
        rendered.ok_or(EguiTextSurfaceError::FrameNotProduced)??,
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
            name: "editor".to_string(),
            family: FontFamily::Monospace,
            size: FONT_SIZE,
            weight: 400,
        },
        TEXT_COLOR,
        LINE_HEIGHT,
    )
}

fn paint_style() -> TextSurfacePaintStyle {
    TextSurfacePaintStyle {
        background_rgba: BACKGROUND_COLOR,
        gutter_background_rgba: BACKGROUND_COLOR,
        gutter_paints: Vec::new(),
        selection_rgba: SELECTION_COLOR,
        preedit_rgba: PREEDIT_COLOR,
        caret_rgba: CARET_COLOR,
        annotation_paints: Vec::new(),
    }
}
