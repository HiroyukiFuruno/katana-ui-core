use super::accessibility::publish_accesskit;
use super::artifact_model::{
    EguiTextSurfaceError, EguiTextSurfaceFrameRecord, EguiTextSurfaceOutput,
    TextSurfaceArtifactFrame, context_target_from_actual_input, publish_ime_output,
    raster_identity, ui_rect,
};
use super::controlled_focus::{focus_request_event, synchronize_focus_request};
use super::controlled_scroll::{
    scroll_request_event, synchronize_scroll_bounds, synchronize_scroll_request,
};
use super::input::TextSurfaceInteraction;
use super::measurement::{
    controlled_gutter_width, placeholder_bounds, placeholder_raster_identity, surface_extent_for_ui,
};
use super::model::{
    EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy, TextSurfacePaintStyle,
    TextSurfaceRasterStyle,
};
use super::paint::build_paint_plan;
use super::raster::{RasterFrame, layout_for_surface, rasterize_placeholder, rasterize_surface};
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceFocusRequestResult, TextSurfacePoint, TextSurfaceViewportSizing,
};

pub(super) fn show_with_input_policy_unpainted(
    ui: &mut egui::Ui,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    raster_style: &TextSurfaceRasterStyle,
    paint_style: &TextSurfacePaintStyle,
    input_policy: &EguiTextSurfaceInputPolicy,
) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
    let (surface_width, surface_height) = surface_extent_for_ui(ui, surface);
    let scale_factor = ui.ctx().pixels_per_point();
    let configured_gutter_width = surface
        .props()
        .gutter
        .as_ref()
        .map_or(0, |gutter| gutter.width);
    let mut gutter_width = configured_gutter_width;
    let mut content_width = (surface_width - gutter_width as f32).max(1.0);
    let initial_raster_result = rasterize_surface(
        &mut adapter.rasterizer,
        surface,
        raster_style,
        content_width,
        scale_factor,
        raster_identity(surface, raster_style),
    );
    let initial_gutter = ControlledGutterState {
        surface_width,
        gutter_width,
        content_width,
        raster_frame: initial_raster_result?,
    };
    let stabilized =
        stabilize_gutter(adapter, surface, raster_style, scale_factor, initial_gutter)?;
    let mut raster_frame = stabilized.raster_frame;
    gutter_width = stabilized.gutter_width;
    content_width = stabilized.content_width;
    let (bounds, response) = ui.allocate_exact_size(
        egui::vec2(surface_width, surface_height),
        egui::Sense::click_and_drag(),
    );
    let surface_bounds = ui_rect(bounds);
    let mut viewport_bounds = UiRect::new(
        surface_bounds.x.saturating_add(gutter_width as i32),
        surface_bounds.y,
        surface_bounds.width.saturating_sub(gutter_width),
        surface_bounds.height,
    );
    synchronize_scroll_bounds(surface, &raster_frame, viewport_bounds);
    let initial_layout = layout_for_surface(
        &raster_frame,
        surface,
        TextSurfacePoint::new(
            viewport_bounds.x.saturating_sub(surface.state().scroll_x),
            viewport_bounds.y.saturating_sub(surface.state().scroll_y),
        ),
    );
    let initial_frame = surface.frame_with_bounds(&initial_layout, surface_bounds, viewport_bounds);
    let mut events = TextSurfaceInteraction::apply_interactions(
        ui,
        &response,
        surface,
        &initial_layout,
        &initial_frame,
        input_policy,
        adapter.pending_focus_request.take(),
    );
    let focus_request = synchronize_focus_request(&response, surface);
    if let Some(TextSurfaceFocusRequestResult::Acknowledged(value)) = focus_request.as_ref() {
        adapter.pending_focus_request = Some(value.focused);
    }
    let post_input_raster_result = rasterize_surface(
        &mut adapter.rasterizer,
        surface,
        raster_style,
        content_width,
        scale_factor,
        raster_identity(surface, raster_style),
    );
    let post_input_gutter = ControlledGutterState {
        surface_width,
        gutter_width,
        content_width,
        raster_frame: post_input_raster_result?,
    };
    let style = raster_style;
    let stabilized = stabilize_gutter(adapter, surface, style, scale_factor, post_input_gutter)?;
    raster_frame = stabilized.raster_frame;
    gutter_width = stabilized.gutter_width;
    viewport_bounds = UiRect::new(
        surface_bounds.x.saturating_add(gutter_width as i32),
        surface_bounds.y,
        surface_bounds.width.saturating_sub(gutter_width),
        surface_bounds.height,
    );
    if surface.props().viewport_sizing == TextSurfaceViewportSizing::AdapterMeasured {
        let _ = surface
            .synchronize_measured_viewport_size(viewport_bounds.width, viewport_bounds.height);
    }
    let scroll_request =
        synchronize_scroll_request(surface, &raster_frame, viewport_bounds, scale_factor);
    let layout = layout_for_surface(
        &raster_frame,
        surface,
        TextSurfacePoint::new(
            viewport_bounds.x.saturating_sub(surface.state().scroll_x),
            viewport_bounds.y.saturating_sub(surface.state().scroll_y),
        ),
    );
    if let Some(result) = &scroll_request {
        events.push(scroll_request_event(result));
    }
    if let Some(result) = &focus_request {
        events.push(focus_request_event(result));
    }
    let frame = surface.frame_with_bounds(&layout, surface_bounds, viewport_bounds);
    let placeholder_result = rasterize_placeholder(
        &mut adapter.rasterizer,
        surface,
        raster_style,
        content_width,
        scale_factor,
        placeholder_raster_identity(surface, raster_style),
    );
    let placeholder = placeholder_result?;
    let record = EguiTextSurfaceFrameRecord {
        texture_bounds: frame.content_bounds,
        hit_target: surface.state().text_area.state_id.as_str().to_string(),
        raster_identity: raster_frame.identity.clone(),
        placeholder_raster_identity: placeholder.as_ref().map(|value| value.identity.clone()),
        placeholder_texture_bounds: placeholder_bounds(
            &frame.content_bounds,
            placeholder.as_ref(),
            scale_factor,
        ),
        layers: super::paint_layers::layers(placeholder.is_some()),
        scroll_request,
        focus_request,
        frame,
    };
    let paint_plan_result = build_paint_plan(
        &mut adapter.rasterizer,
        &mut adapter.svg_rasterizer,
        &raster_frame,
        placeholder.as_ref(),
        &record,
        paint_style,
        raster_style,
        scale_factor,
    );
    let paint_plan = paint_plan_result?;
    publish_ime_output(ui, surface, &record);
    let root_id = response.id;
    publish_accesskit(
        ui,
        root_id,
        &record,
        &layout,
        surface.props().text_area.options().placeholder.as_str(),
        surface.props().text_area.options().max_rows == 1,
    );
    if input_policy.publish_context_target {
        crate::text_command_surface::accesskit_evidence::AccessKitEvidenceLedger::record(
            ui.ctx(),
            crate::text_command_surface::accesskit_evidence::AccessKitEvidence {
                response_id: root_id,
                bounds: ui_rect(response.rect),
                label: surface.props().accessibility_label.clone(),
                disabled: record.frame.accessibility.root.disabled,
                target_identity: "kuc.text-surface.context-target".to_owned(),
                target_class:
                    crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::TextSurfaceContextTarget,
            },
        );
    }
    let (context_target, pointer_context_invoked) =
        context_target_from_actual_input(ui, &response, &record);
    if context_target.is_some() && !pointer_context_invoked {
        events.extend(
            surface
                .apply_action(
                    katana_ui_core::text_surface::TextSurfaceAction::RequestContextTarget {
                        selection: record.frame.selection.range,
                    },
                )
                .events,
        );
    }
    let artifact = TextSurfaceArtifactFrame::new(record.clone(), paint_plan, events.clone())?;
    Ok(EguiTextSurfaceOutput {
        record,
        events,
        artifact,
        context_target,
        raster: raster_frame.raster,
    })
}

struct ControlledGutterState {
    surface_width: f32,
    gutter_width: u32,
    content_width: f32,
    raster_frame: RasterFrame,
}

fn stabilize_gutter(
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &TextSurface,
    raster_style: &TextSurfaceRasterStyle,
    scale_factor: f32,
    mut state: ControlledGutterState,
) -> Result<ControlledGutterState, EguiTextSurfaceError> {
    if surface.has_controlled_automatic_gutter() {
        for _ in 0..2 {
            let measured_width = controlled_gutter_width(
                &mut adapter.rasterizer,
                &layout_for_surface(&state.raster_frame, surface, TextSurfacePoint::new(0, 0)),
                raster_style,
                scale_factor,
            )?;
            if measured_width == state.gutter_width {
                break;
            }
            state.gutter_width = measured_width;
            state.content_width = (state.surface_width - state.gutter_width as f32).max(1.0);
            state.raster_frame = rasterize_surface(
                &mut adapter.rasterizer,
                surface,
                raster_style,
                state.content_width,
                scale_factor,
                raster_identity(surface, raster_style),
            )?;
        }
    }
    Ok(state)
}

#[cfg(test)]
mod controlled_gutter_error_tests {
    use super::*;
    use katana_ui_core::atom::TextArea;
    use katana_ui_core::text_surface::{
        TextSurfaceAutomaticGutterPresentation, TextSurfacePresentation, TextSurfaceProps,
        TextSurfaceViewport,
    };
    use katana_ui_core::theme::{FontFamily, FontToken};
    use katana_ui_core_text_raster::{
        PlatformColorEmojiAvailability, PlatformColorEmojiFaceRecord,
        PlatformFontCatalogFingerprint, PlatformFontProfile, PlatformTextGraphemeBounds,
        PlatformTextRaster, PlatformTextRasterConfig, PlatformTextRasterReport,
        PlatformTextRasterStats,
    };

    fn surface(text: &str) -> TextSurface {
        let mut surface = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("controlled").value(text),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 100, 40),
        ));
        let mut presentation = TextSurfacePresentation::from_props(surface.props());
        presentation.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
        assert!(surface.synchronize_presentation(presentation));
        surface
    }

    fn style() -> TextSurfaceRasterStyle {
        TextSurfaceRasterStyle::new(
            FontToken {
                name: "test".into(),
                family: FontFamily::Proportional,
                size: 12.0,
                weight: 400,
            },
            [255; 4],
            16.0,
        )
    }

    fn frame(text: &str) -> RasterFrame {
        RasterFrame {
            identity: "synthetic".into(),
            raster: PlatformTextRaster {
                text: text.into(),
                width: 8,
                height: 16,
                rgba_pixels: vec![[0, 0, 0, 0]; 128],
                grapheme_bounds: vec![PlatformTextGraphemeBounds {
                    byte_start: 0,
                    byte_end: text.len(),
                    x: 0.0,
                    y: 0.0,
                    width: 8.0,
                    height: 16.0,
                }],
                report: PlatformTextRasterReport {
                    resolved_emoji_font_family: None,
                    color_emoji_font_available: false,
                    emoji_face: PlatformColorEmojiFaceRecord {
                        platform_profile: PlatformFontProfile::Unsupported,
                        family_identity: String::new(),
                        source_file_path: None,
                        raw_file_sha256: None,
                        catalog_fingerprint: PlatformFontCatalogFingerprint::from_bytes([0; 32]),
                        availability: PlatformColorEmojiAvailability::Unavailable(
                            katana_ui_core_text_raster::PlatformColorEmojiUnavailableReason::NoCandidates,
                        ),
                    },
                    cache_hit: false,
                    stats: PlatformTextRasterStats::default(),
                },
            },
        }
    }

    fn state(text: &str) -> ControlledGutterState {
        ControlledGutterState {
            surface_width: 100.0,
            gutter_width: 0,
            content_width: 100.0,
            raster_frame: frame(text),
        }
    }

    #[test]
    fn controlled_gutter_propagates_label_and_surface_raster_failures() {
        let mut adapter = EguiTextSurfaceAdapter::default();
        assert!(
            stabilize_gutter(&mut adapter, &surface("a"), &style(), f32::NAN, state("a"),).is_err()
        );

        let mut no_emoji = PlatformTextRasterConfig::default();
        no_emoji.emoji_candidates.clear();
        no_emoji.emoji_candidate_sha256.clear();
        let mut adapter = EguiTextSurfaceAdapter::new(no_emoji);
        assert!(
            stabilize_gutter(&mut adapter, &surface("⭐️"), &style(), 1.0, state("⭐️"),).is_err()
        );
    }
}
