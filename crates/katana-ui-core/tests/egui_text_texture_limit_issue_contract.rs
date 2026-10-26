#![cfg(feature = "egui")]

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_surface::{
    EguiTextSurfaceAdapter, TextSurfacePaintStyle, TextSurfaceRasterStyle,
};
#[cfg(feature = "native-ime-evidence")]
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_raster::PlatformTextRasterConfig;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceFrameRecord, TextSurfacePresentation, TextSurfaceProps,
    TextSurfaceScrollAlignment, TextSurfaceScrollRequest, TextSurfaceScrollRequestToken,
    TextSurfaceScrollTarget, TextSurfaceViewport,
};
use katana_ui_core::theme::{FontFamily, FontToken};

#[cfg(feature = "native-ime-evidence")]
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceHostRootFrame,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootFactory,
    KucRootEventBatchContext, TextCommandSurfaceStyle,
};
#[cfg(feature = "native-ime-evidence")]
use katana_ui_core::text_surface::TextSurfaceEvent;
#[cfg(feature = "native-ime-evidence")]
use std::{cell::Cell, rc::Rc};

fn render(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    max_texture_side: usize,
    events: Vec<egui::Event>,
) -> Result<
    (Vec<[usize; 2]>, bool, TextSurfaceFrameRecord),
    katana_ui_core::egui::text_surface::EguiTextSurfaceError,
> {
    let raster = TextSurfaceRasterStyle::new(
        FontToken {
            name: "texture-limit".into(),
            family: FontFamily::Monospace,
            size: 16.0,
            weight: 400,
        },
        [235, 235, 235, 255],
        24.0,
    );
    let paint = TextSurfacePaintStyle {
        background_rgba: [24, 24, 24, 255],
        gutter_background_rgba: [32, 32, 32, 255],
        gutter_paints: Vec::new(),
        selection_rgba: [64, 96, 160, 180],
        preedit_rgba: [255, 196, 64, 255],
        caret_rgba: [255, 255, 255, 255],
        annotation_paints: Vec::new(),
    };
    let mut result = None;
    let full_output = context.run_ui(
        egui::RawInput {
            max_texture_side: Some(max_texture_side),
            events,
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(960.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                result = Some(adapter.show(ui, surface, &raster, &paint));
            });
        },
    );
    let output = result.expect("adapter output")?;
    let sizes = full_output
        .textures_delta
        .set
        .values()
        .flat_map(|deltas| deltas.iter().map(|delta| delta.image.size()))
        .collect();
    let has_visible_pixel = full_output
        .textures_delta
        .set
        .values()
        .flatten()
        .any(|delta| match &delta.image {
            egui::ImageData::Color(image) => image.pixels.iter().any(|pixel| pixel.a() > 0),
        });
    let mut full_output = full_output;
    full_output.textures_delta.clear();
    Ok((sizes, has_visible_pixel, output.record.frame))
}

fn assert_texture_limit(sizes: &[[usize; 2]], limit: usize) {
    for [width, height] in sizes {
        assert!(*width <= limit, "texture width {width} exceeds {limit}");
        assert!(*height <= limit, "texture height {height} exceeds {limit}");
    }
}

#[test]
fn long_text_is_tiled_to_each_backend_texture_limit_and_cache_transition()
-> Result<(), Box<dyn std::error::Error>> {
    let source = (1..=240)
        .map(|line| format!("KUC texture limit row {line:03}"))
        .collect::<Vec<_>>()
        .join("\n");
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("texture-limit").value(source.clone()),
        vec![UiTextSpan::plain(&source)],
        TextSurfaceViewport::new(0, 0, 960, 720),
    ));
    let (output_4096, visible_4096, _) =
        render(&context, &mut adapter, &mut surface, 4096, Vec::new())?;
    assert!(
        !output_4096.is_empty(),
        "4096 render must deliver texture tiles"
    );
    assert!(visible_4096, "4096 tiles must contain rendered pixels");
    assert_texture_limit(&output_4096, 4096);
    let (output_2048, visible_2048, _) =
        render(&context, &mut adapter, &mut surface, 2048, Vec::new())?;
    assert!(
        output_2048.len() > 1,
        "2048 render must deliver multiple tiles for 240 lines"
    );
    assert!(visible_2048, "2048 tiles must contain rendered pixels");
    assert_texture_limit(&output_2048, 2048);
    assert_eq!(surface.state().text_area.value, source);
    Ok(())
}

#[test]
fn short_japanese_emoji_text_remains_renderable_under_texture_limit()
-> Result<(), Box<dyn std::error::Error>> {
    let source = "日本語⭐️を編集してください。";
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("texture-limit-short").value(source),
        UiTextSpan::emoji_marked_spans(source, Default::default()),
        TextSurfaceViewport::new(0, 0, 960, 720),
    ));
    let (output, visible, _) = render(&context, &mut adapter, &mut surface, 2048, Vec::new())?;
    assert!(!output.is_empty(), "short text must deliver a texture tile");
    assert!(visible, "short text tile must contain rendered pixels");
    assert_texture_limit(&output, 2048);
    assert_eq!(surface.state().text_area.value, source);
    Ok(())
}

#[test]
fn long_text_scrolls_to_last_line_and_preserves_geometry_at_both_limits()
-> Result<(), Box<dyn std::error::Error>> {
    let source = (1..=240)
        .map(|line| format!("KUC texture limit row {line:03}"))
        .collect::<Vec<_>>()
        .join("\n");
    for limit in [4096, 2048] {
        let context = egui::Context::default();
        let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
        let mut surface = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("texture-limit-scroll").value(source.clone()),
            vec![UiTextSpan::plain(&source)],
            TextSurfaceViewport::new(0, 0, 960, 720),
        ));
        let (initial_sizes, initial_visible, _) =
            render(&context, &mut adapter, &mut surface, limit, Vec::new())?;
        assert!(!initial_sizes.is_empty());
        assert!(initial_visible, "initial tile must contain rendered pixels");
        assert_texture_limit(&initial_sizes, limit);
        let mut presentation = TextSurfacePresentation::from_props(surface.props());
        presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
            TextSurfaceScrollRequestToken::new(format!("last-line-{limit}")),
            TextSurfaceScrollTarget::ByteOffset {
                byte_offset: source.len().saturating_sub(1),
            },
            TextSurfaceScrollAlignment::End,
        ));
        assert!(surface.synchronize_presentation(presentation));
        let (scrolled_sizes, _scrolled_visible, scrolled_frame) =
            render(&context, &mut adapter, &mut surface, limit, Vec::new())?;
        assert!(!scrolled_frame.visible_logical_rows.is_empty());
        if !scrolled_sizes.is_empty() {
            assert_texture_limit(&scrolled_sizes, limit);
        }
        let max_y = surface.state().scroll_bounds.expect("scroll bounds").max_y;
        assert!(surface.state().scroll_y >= max_y.saturating_sub(24));
        assert_eq!(surface.state().text_area.value, source);
        assert!(scrolled_frame.visible_logical_rows.contains(&239));
    }
    Ok(())
}

#[cfg(feature = "native-ime-evidence")]
struct HostForwarder;

#[cfg(feature = "native-ime-evidence")]
impl katana_ui_core::egui::text_command_surface::KucRootEventBatchForwarder for HostForwarder {
    type Error = ();

    fn forward_root_event_batch(
        &mut self,
        _: katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(feature = "native-ime-evidence")]
fn host_lease(
    revision: u64,
    presentation: &EguiTextCommandSurfacePresentation,
    selection: Rc<Cell<(usize, usize)>>,
) -> EguiTextCommandSurfaceHostProjectionLease {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"texture-limit-host-root".to_vec(),
        presentation.clone(),
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("host token");
    EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        move |context: KucRootEventBatchContext| {
            for event in context.text_events() {
                if let TextSurfaceEvent::SelectionChanged {
                    selection_start,
                    selection_end,
                } = event
                {
                    selection.set((*selection_start, *selection_end));
                }
            }
            Ok(None)
        },
    )
}

#[cfg(feature = "native-ime-evidence")]
fn host_frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    limit: usize,
    events: Vec<egui::Event>,
) -> EguiTextCommandSurfaceHostRootFrame {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            max_texture_side: Some(limit),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                frame = Some(root.show(ui).expect("host root frame"));
            });
        },
    );
    output.textures_delta.clear();
    frame.expect("host frame")
}

#[cfg(feature = "native-ime-evidence")]
#[test]
fn host_root_observation_drives_last_line_pointer_selection_at_both_limits()
-> Result<(), Box<dyn std::error::Error>> {
    let source = (1..=240)
        .map(|line| format!("KUC texture limit row {line:03}"))
        .collect::<Vec<_>>()
        .join("\n");
    let end = source.len().saturating_sub(1);
    let start = source[..end]
        .rfind('\n')
        .map_or(0, |offset| offset.saturating_add(1));
    let selection = Rc::new(Cell::new((end, end)));

    for limit in [4096, 2048] {
        let context = egui::Context::default();
        let surface = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("texture-limit-host").value(source.clone()),
            vec![UiTextSpan::plain(&source)],
            TextSurfaceViewport::new(0, 0, 1280, 720),
        ));
        let mut presentation = TextSurfacePresentation::from_props(surface.props());
        presentation.selection_start = start;
        presentation.selection_end = end;
        let initial_presentation = EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("texture-limit-host")),
            text: presentation.clone(),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        };
        let lease = host_lease(1, &initial_presentation, Rc::clone(&selection));
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(lease)
            .expect("host root");
        let first_frame = host_frame(&context, &mut root, limit, Vec::new());
        first_frame
            .forward_events_once(&mut HostForwarder)
            .expect("initial frame");
        presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
            TextSurfaceScrollRequestToken::new(format!("host-last-line-{limit}")),
            TextSurfaceScrollTarget::LogicalRow { logical_row: 239 },
            TextSurfaceScrollAlignment::Center,
        ));
        root.synchronize_with_lease(host_lease(
            2,
            &EguiTextCommandSurfacePresentation {
                text_state_id: Some(UiStateId::new("texture-limit-host")),
                text: presentation,
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            Rc::clone(&selection),
        ))?;
        let applied = host_frame(&context, &mut root, limit, Vec::new());
        applied
            .forward_events_once(&mut HostForwarder)
            .expect("scroll frame");
        let frame = host_frame(&context, &mut root, limit, Vec::new());
        let first = frame
            .observe_unicode((start, start + 1))
            .map_err(|error| format!("first observation failed: {error:?}"))?;
        let last = frame
            .observe_unicode((end - 1, end))
            .map_err(|error| format!("last observation failed: {error:?}"))?;
        assert_eq!(first.hit_test().byte_start, start);
        assert_eq!(last.hit_test().byte_end, end);
        let first_point = egui::pos2(
            first.hit_test().query_x as f32,
            first.hit_test().query_y as f32,
        );
        let last_point = egui::pos2(
            last.hit_test().query_x as f32,
            last.hit_test().query_y as f32,
        );
        let midpoint = egui::pos2((first_point.x + last_point.x) / 2.0, first_point.y);
        let hover = host_frame(
            &context,
            &mut root,
            limit,
            vec![egui::Event::PointerMoved(first_point)],
        );
        hover
            .forward_events_once(&mut HostForwarder)
            .expect("hover");
        let pressed = host_frame(
            &context,
            &mut root,
            limit,
            vec![egui::Event::PointerButton {
                pos: first_point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        pressed
            .forward_events_once(&mut HostForwarder)
            .expect("press");
        let dragged = host_frame(
            &context,
            &mut root,
            limit,
            vec![egui::Event::PointerMoved(midpoint)],
        );
        dragged
            .forward_events_once(&mut HostForwarder)
            .expect("drag");
        let ended = host_frame(
            &context,
            &mut root,
            limit,
            vec![egui::Event::PointerMoved(last_point)],
        );
        ended.forward_events_once(&mut HostForwarder).expect("end");
        let released = host_frame(
            &context,
            &mut root,
            limit,
            vec![
                egui::Event::PointerMoved(last_point),
                egui::Event::PointerButton {
                    pos: last_point,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        released
            .forward_events_once(&mut HostForwarder)
            .expect("release");
        let (selection_start, selection_end) = selection.get();
        assert!(selection_start < selection_end);
        assert!(selection_start >= start);
        assert!(selection_end <= end);
    }
    Ok(())
}
