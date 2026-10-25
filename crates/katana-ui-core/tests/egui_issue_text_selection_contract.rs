#![cfg(feature = "egui")]

use std::collections::HashSet;

use katana_ui_core::atom::{TextArea, TextAreaAction, TextAreaSelection};
use katana_ui_core::egui::text_surface::{
    EguiTextSurfaceAdapter, TextSurfacePaintStyle, TextSurfaceRasterStyle,
};
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_raster::{
    PlatformTextRasterConfig, PlatformTextRasterError, PlatformTextRasterRequest,
    PlatformTextRasterizer,
};
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfacePoint, TextSurfaceProps, TextSurfaceViewport,
};
use katana_ui_core::theme::{FontFamily, FontToken};

fn raster_style() -> TextSurfaceRasterStyle {
    TextSurfaceRasterStyle::new(
        FontToken {
            name: "issue-text-selection".to_owned(),
            family: FontFamily::Monospace,
            size: 16.0,
            weight: 400,
        },
        [235, 235, 235, 255],
        24.0,
    )
}

#[test]
fn public_raster_layout_keeps_rtl_hard_break_at_logical_terminal()
-> Result<(), Box<dyn std::error::Error>> {
    let source = "אב\nx";
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let raster = match rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        source,
        FontToken {
            name: "issue-rtl-break".to_owned(),
            family: FontFamily::Monospace,
            size: 16.0,
            weight: 400,
        },
        [235, 235, 235, 255],
    )) {
        Ok(raster) => raster,
        Err(PlatformTextRasterError::ColorEmojiUnavailable { face }) => {
            assert!(!face.is_available());
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let newline = raster
        .grapheme_bounds
        .iter()
        .find(|bounds| source.get(bounds.byte_start..bounds.byte_end) == Some("\n"))
        .ok_or("hard break bound missing")?;
    let rtl = raster
        .grapheme_bounds
        .iter()
        .filter(|bounds| bounds.byte_end <= "אב".len())
        .collect::<Vec<_>>();
    let left = rtl
        .iter()
        .map(|bounds| bounds.x)
        .fold(f32::INFINITY, f32::min);
    assert_eq!(newline.x, left);
    let layout = raster.text_surface_layout("issue-public-rtl-break", TextSurfacePoint::new(0, 0));
    assert_eq!(
        newline.x as i32,
        layout.caret_rect(UiTextSelectionRange::caret(2)).x
    );
    Ok(())
}

#[test]
fn adapter_accesskit_preserves_hard_breaks_and_selection_positions()
-> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let source = "A\n日本語⭐️\nZ";
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("issue-89-accesskit").value(source),
        UiTextSpan::emoji_marked_spans(source, Default::default()),
        TextSurfaceViewport::new(0, 0, 900, 640),
    ));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 2, end: 17 },
    )));
    let raster = raster_style();
    let paint = paint_style();
    let mut rendered = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 640.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                rendered = Some(adapter.show(ui, &mut surface, &raster, &paint));
            });
        },
    );
    full_output.textures_delta.clear();
    rendered.expect("adapter output")?;
    let update = full_output
        .platform_output
        .accesskit_update
        .ok_or("missing AccessKit update")?;
    let root = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.role() == egui::accesskit::Role::MultilineTextInput)
        .ok_or("missing text input root")?;
    assert_eq!(root.value(), Some(source));
    let selection = root.text_selection().ok_or("missing text selection")?;
    assert_eq!(selection.anchor.character_index, 0);
    assert_eq!(selection.focus.character_index, 5);
    let runs = update
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::TextRun)
        .map(|(_, node)| node)
        .collect::<Vec<_>>();
    assert!(runs.iter().any(|node| node.value() == Some("A\n")));
    assert!(runs.iter().any(|node| node.value() == Some("日本語⭐️\n")));
    assert!(runs.iter().any(|node| node.value() == Some("Z")));
    Ok(())
}

#[test]
fn adapter_accesskit_crlf_run_keeps_separator_bytes_as_one_character()
-> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let source = "A\r\nB";
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("issue-89-crlf").value(source),
        vec![UiTextSpan::plain(source)],
        TextSurfaceViewport::new(0, 0, 900, 640),
    ));
    let raster = raster_style();
    let paint = paint_style();
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 640.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = adapter.show(ui, &mut surface, &raster, &paint);
            });
        },
    );
    full_output.textures_delta.clear();
    let update = full_output
        .platform_output
        .accesskit_update
        .ok_or("missing AccessKit update")?;
    let run = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.role() == egui::accesskit::Role::TextRun && node.value() == Some("A\r\n"))
        .ok_or("missing CRLF text run")?;
    assert_eq!(run.character_lengths(), &[1, 2]);
    Ok(())
}

#[test]
fn adapter_accesskit_softwrap_keeps_source_and_unique_text_run_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let source = "soft-wrap 日本語⭐️ boundary without a hard break";
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("issue-89-softwrap").value(source),
        vec![UiTextSpan::plain(source)],
        TextSurfaceViewport::new(0, 0, 96, 640),
    ));
    let mut rendered = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(96.0, 640.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                rendered = Some(adapter.show(ui, &mut surface, &raster_style(), &paint_style()));
            });
        },
    );
    full_output.textures_delta.clear();
    let output = rendered.expect("softwrap adapter output")?;
    assert!(!output.record.frame.visible_logical_rows.is_empty());
    let update = full_output
        .platform_output
        .accesskit_update
        .ok_or("missing AccessKit update")?;
    let root = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.role() == egui::accesskit::Role::MultilineTextInput)
        .ok_or("missing softwrap text input root")?;
    assert_eq!(root.value(), Some(source));
    let run_ids = update
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::TextRun)
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();
    assert!(!run_ids.is_empty(), "softwrap must expose a TextRun");
    assert_eq!(run_ids.len(), run_ids.iter().collect::<HashSet<_>>().len());
    Ok(())
}

fn paint_style() -> TextSurfacePaintStyle {
    TextSurfacePaintStyle {
        background_rgba: [24, 24, 24, 255],
        gutter_background_rgba: [32, 32, 32, 255],
        gutter_paints: Vec::new(),
        selection_rgba: [64, 96, 160, 180],
        preedit_rgba: [255, 196, 64, 255],
        caret_rgba: [255, 255, 255, 255],
        annotation_paints: Vec::new(),
    }
}

fn render(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
) -> Result<
    katana_ui_core::egui::text_surface::EguiTextSurfaceOutput,
    katana_ui_core::egui::text_surface::EguiTextSurfaceError,
> {
    let mut output = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 640.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            output = Some(adapter.show(ui, surface, &raster_style(), &paint_style()));
        },
    );
    full_output.textures_delta.clear();
    output.expect("adapter frame")
}

#[test]
fn adapter_preserves_empty_logical_rows_and_hard_breaks() -> Result<(), Box<dyn std::error::Error>>
{
    let context = egui::Context::default();
    let source = "A\n\nB";
    let mut adapter = EguiTextSurfaceAdapter::new(PlatformTextRasterConfig::default());
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("issue-89").value(source),
        vec![UiTextSpan::plain(source)],
        TextSurfaceViewport::new(0, 0, 900, 640),
    ));
    let output = render(&context, &mut adapter, &mut surface)?;
    assert_eq!(output.record.frame.visible_logical_rows, vec![0, 1, 2]);

    let source = "A\n日本語⭐️\nZ";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("issue-89-selection").value(source),
        UiTextSpan::emoji_marked_spans(source, Default::default()),
        TextSurfaceViewport::new(0, 0, 900, 640),
    ));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 2, end: 17 },
    )));
    let output = render(&context, &mut adapter, &mut surface)?;
    assert_eq!(output.record.frame.selection_start, 2);
    assert_eq!(output.record.frame.selection_end, 17);
    assert_eq!(output.record.frame.visible_logical_rows, vec![0, 1, 2]);
    Ok(())
}
