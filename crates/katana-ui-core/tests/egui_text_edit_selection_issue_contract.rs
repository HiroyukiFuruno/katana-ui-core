#![cfg(feature = "egui")]

use katana_ui_core::atom::{TextArea, TextAreaAction, TextAreaSelection};
use katana_ui_core::egui::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceError, EguiTextSurfaceOutput, TextSurfacePaintStyle,
    TextSurfaceRasterStyle,
};
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_raster::PlatformTextRasterConfig;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceClipboardOperation, TextSurfaceEvent,
    TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use katana_ui_core::theme::{FontFamily, FontToken};

fn styles() -> (TextSurfaceRasterStyle, TextSurfacePaintStyle) {
    (
        TextSurfaceRasterStyle::new(
            FontToken {
                name: "issue-selection".into(),
                family: FontFamily::Monospace,
                size: 16.0,
                weight: 400,
            },
            [235, 235, 235, 255],
            24.0,
        ),
        TextSurfacePaintStyle {
            background_rgba: [24, 24, 24, 255],
            gutter_background_rgba: [32, 32, 32, 255],
            gutter_paints: Vec::new(),
            selection_rgba: [64, 96, 160, 180],
            preedit_rgba: [255, 196, 64, 255],
            caret_rgba: [255, 255, 255, 255],
            annotation_paints: Vec::new(),
        },
    )
}

fn frame(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
    let (raster, paint) = styles();
    let mut output = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 640.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                output = Some(adapter.show(ui, surface, &raster, &paint));
            });
        },
    );
    full_output.textures_delta.clear();
    output.expect("adapter output")
}

fn surface(value: &str) -> TextSurface {
    TextSurface::new(TextSurfaceProps::new(
        TextArea::new("issue-selection").value(value),
        UiTextSpan::emoji_marked_spans(value, Default::default()),
        TextSurfaceViewport::new(0, 0, 900, 640),
    ))
}

fn last_selection_event(output: &EguiTextSurfaceOutput) -> Option<(usize, usize)> {
    output.events.iter().rev().find_map(|event| match event {
        TextSurfaceEvent::SelectionChanged {
            selection_start,
            selection_end,
        } => Some((*selection_start, *selection_end)),
        _ => None,
    })
}

fn echo(surface: &mut TextSurface) {
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.value = surface.state().text_area.value.clone();
    presentation.selection_start = surface.state().text_area.selection.start;
    presentation.selection_end = surface.state().text_area.selection.end;
    let _ = surface.synchronize_presentation(presentation);
}

#[test]
fn raw_input_emoji_edit_publishes_final_selection_and_controlled_echo()
-> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut surface = surface("日本語⭐️");
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection::collapsed(3),
    )));
    let _ = frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let edited = frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Text("⭐️".into())],
    )?;
    assert_eq!(surface.state().text_area.value, "日⭐️本語⭐️");
    assert_eq!(surface.state().text_area.selection.start, 9);
    assert!(edited.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::SelectionChanged {
            selection_start: 9,
            selection_end: 9
        }
    )));
    let mut echo = TextSurfacePresentation::from_props(surface.props());
    echo.value = "日⭐️本語⭐️".into();
    echo.selection_start = 9;
    echo.selection_end = 9;
    let _ = surface.synchronize_presentation(echo);
    for _ in 0..2 {
        let _ = frame(&context, &mut adapter, &mut surface, Vec::new())?;
        assert_eq!(surface.state().text_area.selection.start, 9);
        assert_eq!(surface.state().text_area.selection.end, 9);
    }
    Ok(())
}

#[test]
fn readonly_and_disabled_reject_raw_text_input() -> Result<(), Box<dyn std::error::Error>> {
    for area in [
        TextArea::new("readonly").value("日本語⭐️").readonly(true),
        TextArea::new("disabled").value("日本語⭐️").disabled(true),
    ] {
        let context = egui::Context::default();
        let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
        let mut surface = TextSurface::new(TextSurfaceProps::new(
            area,
            vec![UiTextSpan::plain("日本語⭐️")],
            TextSurfaceViewport::new(0, 0, 900, 640),
        ));
        let _ = frame(&context, &mut adapter, &mut surface, Vec::new())?;
        let _ = frame(
            &context,
            &mut adapter,
            &mut surface,
            vec![egui::Event::Text("⭐️".into())],
        )?;
        assert_eq!(surface.state().text_area.value, "日本語⭐️");
    }
    Ok(())
}

#[test]
fn raw_input_selection_edges_follow_grapheme_boundaries() -> Result<(), Box<dyn std::error::Error>>
{
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut surface = surface("日本語⭐️");
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 9, end: 3 },
    )));
    let _ = frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let key = |key| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    let _ = frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![key(egui::Key::ArrowLeft)],
    )?;
    assert_eq!(surface.state().text_area.selection.start, 3);
    assert_eq!(surface.state().text_area.selection.end, 3);
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 3, end: 9 },
    )));
    let _ = frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let _ = frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![key(egui::Key::ArrowRight)],
    )?;
    assert_eq!(surface.state().text_area.selection.start, 9);
    assert_eq!(surface.state().text_area.selection.end, 9);
    Ok(())
}

#[test]
fn shift_horizontal_selection_extends_and_shrinks_from_the_caret_in_both_directions()
-> Result<(), Box<dyn std::error::Error>> {
    let key = |key, shift| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers {
            shift,
            ..egui::Modifiers::NONE
        },
    };

    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut forward = surface("日本語⭐️");
    let _ = forward.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = forward.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 3, end: 9 },
    )));
    let _ = frame(&context, &mut adapter, &mut forward, Vec::new())?;
    let _ = frame(
        &context,
        &mut adapter,
        &mut forward,
        vec![key(egui::Key::ArrowRight, true)],
    )?;
    assert_eq!(
        (
            forward.state().text_area.selection.start,
            forward.state().text_area.selection.end
        ),
        (3, 15)
    );
    let _ = frame(
        &context,
        &mut adapter,
        &mut forward,
        vec![key(egui::Key::ArrowLeft, true)],
    )?;
    assert_eq!(
        (
            forward.state().text_area.selection.start,
            forward.state().text_area.selection.end
        ),
        (3, 9)
    );

    let mut reverse = surface("日本語⭐️");
    let _ = reverse.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = reverse.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 9, end: 3 },
    )));
    let _ = frame(&context, &mut adapter, &mut reverse, Vec::new())?;
    let _ = frame(
        &context,
        &mut adapter,
        &mut reverse,
        vec![key(egui::Key::ArrowLeft, true)],
    )?;
    assert_eq!(
        (
            reverse.state().text_area.selection.start,
            reverse.state().text_area.selection.end
        ),
        (9, 0)
    );
    let _ = frame(
        &context,
        &mut adapter,
        &mut reverse,
        vec![key(egui::Key::ArrowRight, true)],
    )?;
    assert_eq!(
        (
            reverse.state().text_area.selection.start,
            reverse.state().text_area.selection.end
        ),
        (9, 3)
    );
    Ok(())
}

#[test]
fn clipboard_request_then_replacement_and_editing_keep_controlled_echo()
-> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut surface = surface("日本語⭐️");
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 3, end: 9 },
    )));
    let _ = frame(&context, &mut adapter, &mut surface, Vec::new())?;

    let paste_request = frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Paste("☆".to_owned())],
    )?;
    assert_eq!(surface.state().text_area.value, "日本語⭐️");
    assert_eq!(
        (
            surface.state().text_area.selection.start,
            surface.state().text_area.selection.end
        ),
        (3, 9)
    );
    assert_eq!(last_selection_event(&paste_request), None);
    assert!(paste_request.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::ClipboardRequested {
            operation: TextSurfaceClipboardOperation::Paste,
            selection_start: 3,
            selection_end: 9,
        }
    )));

    let pasted = frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Text("☆".to_owned())],
    )?;
    assert_eq!(surface.state().text_area.value, "日☆⭐️");
    assert_eq!(
        (
            surface.state().text_area.selection.start,
            surface.state().text_area.selection.end
        ),
        (6, 6)
    );
    assert_eq!(last_selection_event(&pasted), Some((6, 6)));
    echo(&mut surface);
    for _ in 0..2 {
        let idle = frame(&context, &mut adapter, &mut surface, Vec::new())?;
        assert_eq!(surface.state().text_area.value, "日☆⭐️");
        assert_eq!(last_selection_event(&idle), None);
        assert_eq!(
            (
                surface.state().text_area.selection.start,
                surface.state().text_area.selection.end
            ),
            (6, 6)
        );
    }

    let backspace = frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Key {
            key: egui::Key::Backspace,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert_eq!(surface.state().text_area.value, "日⭐️");
    assert_eq!(
        (
            surface.state().text_area.selection.start,
            surface.state().text_area.selection.end
        ),
        (3, 3)
    );
    assert_eq!(last_selection_event(&backspace), Some((3, 3)));
    echo(&mut surface);

    let text = frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![
            egui::Event::Text("本".to_owned()),
            egui::Event::Text("語".to_owned()),
        ],
    )?;
    assert_eq!(surface.state().text_area.value, "日本語⭐️");
    assert_eq!(
        (
            surface.state().text_area.selection.start,
            surface.state().text_area.selection.end
        ),
        (9, 9)
    );
    assert_eq!(last_selection_event(&text), Some((9, 9)));
    echo(&mut surface);
    for _ in 0..2 {
        let idle = frame(&context, &mut adapter, &mut surface, Vec::new())?;
        assert_eq!(surface.state().text_area.value, "日本語⭐️");
        assert_eq!(last_selection_event(&idle), None);
        assert_eq!(
            (
                surface.state().text_area.selection.start,
                surface.state().text_area.selection.end
            ),
            (9, 9)
        );
    }
    Ok(())
}
