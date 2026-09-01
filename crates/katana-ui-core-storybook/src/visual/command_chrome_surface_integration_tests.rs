use super::super::command_chrome_fixture::{FRAME_HEIGHT, FRAME_WIDTH};
use super::super::text_surface_fixture::{
    paint_style as text_paint_style, raster_style as text_raster_style, text_surface_fixture,
};
use super::{command_chrome_surface_fixture, show_command_chrome};
use katana_ui_core::egui::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core::egui::text_surface::{EguiTextSurfaceAdapter, TextSurfacePaintOperationKind};
use katana_ui_core::interaction::placement::Rect;
use katana_ui_core::molecule::command_chrome::{
    FloatingCommandToolbarPresentation, FloatingCommandToolbarVisibility,
};
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::TextSurfacePresentation;
use std::io;

const STAR: &str = "⭐️";
const TEXT_SELECTION_END: usize = "一行目: 日本語 ⭐️".len();
const RGBA_CHANNELS: usize = 4;
const ALPHA_CHANNEL: usize = 3;

#[derive(Debug, PartialEq, Eq)]
struct IntegratedArtifactFacts {
    canvas: UiRect,
    selection_anchor: UiRect,
    floating_panel_bounds: UiRect,
    text_record_hash: String,
    text_plan_hash: String,
    toolbar_record_hash: String,
    toolbar_plan_hash: String,
    floating_record_hash: String,
    floating_plan_hash: String,
    search_record_hash: String,
    search_plan_hash: String,
    composite_plan_hash: String,
    rgba_hash: String,
    colored_star_texture: bool,
    accesskit_labels: Vec<String>,
}

#[test]
fn actual_egui_integrated_artifact_composition_is_repeatable()
-> Result<(), Box<dyn std::error::Error>> {
    let first = run_actual_integrated_frame()?;
    let second = run_actual_integrated_frame()?;
    assert_eq!(first, second);
    assert_eq!(
        first.canvas,
        UiRect::new(0, 0, FRAME_WIDTH as u32, FRAME_HEIGHT as u32)
    );
    assert!(first.selection_anchor.width > 0);
    assert!(first.selection_anchor.height > 0);
    assert!(first.floating_panel_bounds.x >= first.selection_anchor.x);
    assert!(first.floating_panel_bounds.y >= first.selection_anchor.y);
    assert!(first.colored_star_texture);
    assert!(
        first
            .accesskit_labels
            .iter()
            .any(|label| label.contains("太字"))
    );
    assert!(
        first
            .accesskit_labels
            .iter()
            .any(|label| label.contains("検索"))
    );
    assert!(
        first
            .accesskit_labels
            .iter()
            .any(|label| label.contains(STAR))
    );
    Ok(())
}

fn run_actual_integrated_frame() -> Result<IntegratedArtifactFacts, Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let raw_screen_rect =
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(FRAME_WIDTH, FRAME_HEIGHT));
    let actual_canvas = ui_rect(raw_screen_rect);
    let raw_input = egui::RawInput {
        screen_rect: Some(raw_screen_rect),
        ..egui::RawInput::default()
    };

    let mut text_adapter = EguiTextSurfaceAdapter::default();
    let mut text_surface = text_surface_fixture();
    let mut text_presentation = TextSurfacePresentation::from_props(text_surface.props());
    text_presentation.selection_start = 0;
    text_presentation.selection_end = TEXT_SELECTION_END;
    assert!(text_surface.synchronize_presentation(text_presentation));

    let mut chrome = command_chrome_surface_fixture(false);
    let mut result = None;
    let mut full_output = context.run_ui(raw_input, |ui| {
        result = Some((|| -> Result<IntegratedArtifactFacts, io::Error> {
            let text = text_adapter
                .show(
                    ui,
                    &mut text_surface,
                    &text_raster_style(),
                    &text_paint_style(),
                )
                .map_err(adapter_error)?;
            let selection_anchor = text
                .record
                .frame
                .selection
                .rects
                .first()
                .copied()
                .ok_or_else(|| {
                    io::Error::other("TextSurface selection did not produce an anchor rect")
                })?;
            assert!(chrome.floating.synchronize_presentation(
                FloatingCommandToolbarPresentation::new(
                    placement_rect(selection_anchor),
                    placement_rect(actual_canvas),
                    FloatingCommandToolbarVisibility::Visible,
                )
            ));
            let command_chrome = show_command_chrome(
                ui,
                &mut chrome.adapter,
                &mut chrome.toolbar,
                &mut chrome.floating,
                &mut chrome.search,
            )
            .map_err(adapter_error)?;
            let floating = command_chrome.floating.artifact.as_ref().ok_or_else(|| {
                io::Error::other("selection-anchored floating toolbar was absent")
            })?;
            let floating_panel_bounds = command_chrome
                .floating
                .record
                .as_ref()
                .ok_or_else(|| io::Error::other("selection-anchored floating record was absent"))?
                .panel_bounds;
            let plans = [
                ArtifactPaintPlanRef::TextSurface(&text.artifact.paint_plan),
                ArtifactPaintPlanRef::CommandChrome(&command_chrome.toolbar.artifact.paint_plan),
                ArtifactPaintPlanRef::CommandChrome(&floating.paint_plan),
                ArtifactPaintPlanRef::CommandChrome(&command_chrome.search.artifact.paint_plan),
            ];
            let composite = ArtifactCompositor::compose(ArtifactCompositeRequest {
                canvas: ArtifactCanvasBounds::new(actual_canvas),
                plans: &plans,
            })
            .map_err(adapter_error)?;
            let floating_record_hash = floating.frame_record_hash.clone();
            let floating_plan_hash = floating.paint_plan_hash.clone();
            Ok(IntegratedArtifactFacts {
                canvas: composite.canvas.ui_rect(),
                selection_anchor,
                floating_panel_bounds,
                text_record_hash: text.artifact.frame_record_hash,
                text_plan_hash: text.artifact.paint_plan_hash,
                toolbar_record_hash: command_chrome.toolbar.artifact.frame_record_hash,
                toolbar_plan_hash: command_chrome.toolbar.artifact.paint_plan_hash,
                floating_record_hash,
                floating_plan_hash,
                search_record_hash: command_chrome.search.artifact.frame_record_hash,
                search_plan_hash: command_chrome.search.artifact.paint_plan_hash,
                composite_plan_hash: composite.paint_plan_hash,
                rgba_hash: composite.pixel_hash,
                colored_star_texture: text_plan_has_colored_star(&text.artifact.paint_plan),
                accesskit_labels: Vec::new(),
            })
        })());
    });
    full_output.textures_delta.clear();
    let mut facts =
        result.ok_or_else(|| io::Error::other("actual egui frame was not produced"))??;
    facts.accesskit_labels = accesskit_labels(full_output);
    Ok(facts)
}

fn ui_rect(rect: egui::Rect) -> UiRect {
    UiRect::new(
        rect.min.x as i32,
        rect.min.y as i32,
        rect.width() as u32,
        rect.height() as u32,
    )
}

fn placement_rect(rect: UiRect) -> Rect {
    Rect::new(rect.x, rect.y, rect.width, rect.height)
}

fn adapter_error(error: impl std::fmt::Display) -> io::Error {
    io::Error::other(error.to_string())
}

fn accesskit_labels(output: egui::FullOutput) -> Vec<String> {
    let mut labels = output
        .platform_output
        .accesskit_update
        .into_iter()
        .flat_map(|update| update.nodes)
        .flat_map(|(_, node)| {
            [node.label(), node.placeholder()]
                .into_iter()
                .flatten()
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    labels.sort();
    labels.dedup();
    labels
}

fn text_plan_has_colored_star(
    plan: &katana_ui_core::egui::text_surface::TextSurfacePaintPlan,
) -> bool {
    plan.operations.iter().any(|operation| {
        let TextSurfacePaintOperationKind::Texture { texture, .. } = &operation.kind else {
            return false;
        };
        is_colored_star_texture(&texture.identity, &texture.rgba_pixels)
    })
}

fn is_colored_star_texture(identity: &str, pixels: &[u8]) -> bool {
    identity.contains(STAR)
        && pixels
            .as_chunks::<RGBA_CHANNELS>()
            .0
            .iter()
            .any(|rgba| rgba[ALPHA_CHANNEL] > 0 && (rgba[0] != rgba[1] || rgba[1] != rgba[2]))
}
