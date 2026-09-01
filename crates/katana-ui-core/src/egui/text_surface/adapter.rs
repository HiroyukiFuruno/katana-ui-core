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
use super::events::TextSurfaceInteraction;
use super::measurement::{
    controlled_gutter_width, placeholder_bounds, placeholder_raster_identity, surface_extent_for_ui,
};
use super::model::{
    EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy, TextSurfacePaintStyle,
    TextSurfaceRasterStyle,
};
use super::paint::{PaintLayers, build_paint_plan, paint_surface};
use super::raster::{RasterFrame, layout_for_surface, rasterize_placeholder, rasterize_surface};
use crate::render_model::UiRect;
use crate::text_surface::{
    TextSurface, TextSurfaceFocusRequestResult, TextSurfacePoint, TextSurfaceViewportSizing,
};

mod constructors;

impl EguiTextSurfaceAdapter {
    fn raster(
        &mut self,
        surface: &TextSurface,
        style: &TextSurfaceRasterStyle,
        content_width: f32,
        scale_factor: f32,
    ) -> Result<RasterFrame, EguiTextSurfaceError> {
        rasterize_surface(
            &mut self.rasterizer,
            surface,
            style,
            content_width,
            scale_factor,
            raster_identity(surface, style),
            &self.metrics,
        )
    }

    fn gutter(
        &mut self,
        frame: &RasterFrame,
        surface: &TextSurface,
        style: &TextSurfaceRasterStyle,
        scale_factor: f32,
    ) -> Result<u32, EguiTextSurfaceError> {
        let layout = layout_for_surface(frame, surface, TextSurfacePoint::new(0, 0));
        controlled_gutter_width(
            &mut self.rasterizer,
            &layout,
            style,
            scale_factor,
            &self.metrics,
        )
    }

    pub(crate) fn request_focus_for_next_frame(&mut self, focused: bool) {
        self.pending_focus_request = Some(focused);
    }

    pub(crate) fn set_pointer_exclusion_bounds(&mut self, bounds: Vec<UiRect>) {
        self.pointer_exclusion_bounds = bounds;
    }

    pub fn show_with_input_policy(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut TextSurface,
        raster_style: &TextSurfaceRasterStyle,
        paint_style: &TextSurfacePaintStyle,
        input_policy: &EguiTextSurfaceInputPolicy,
    ) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
        let output = self.show_with_input_policy_unpainted(
            ui,
            surface,
            raster_style,
            paint_style,
            input_policy,
        )?;
        paint_surface(ui, &mut self.textures, &output.artifact.paint_plan);
        Ok(output)
    }

    pub(crate) fn show_with_input_policy_unpainted(
        &mut self,
        ui: &mut egui::Ui,
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
        let mut raster_frame = self.raster(surface, raster_style, content_width, scale_factor)?;
        if surface.has_controlled_automatic_gutter() {
            for _ in 0..2 {
                let measured_width =
                    self.gutter(&raster_frame, surface, raster_style, scale_factor)?;
                if measured_width == gutter_width {
                    break;
                }
                gutter_width = measured_width;
                content_width = (surface_width - gutter_width as f32).max(1.0);
                raster_frame = self.raster(surface, raster_style, content_width, scale_factor)?;
            }
        }
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
        let initial_frame =
            surface.frame_with_bounds(&initial_layout, surface_bounds, viewport_bounds);
        let pointer_exclusion_bounds = std::mem::take(&mut self.pointer_exclusion_bounds);
        let mut events = TextSurfaceInteraction::apply_interactions(
            ui,
            &response,
            surface,
            &initial_layout,
            &initial_frame,
            input_policy,
            self.pending_focus_request.take(),
            &pointer_exclusion_bounds,
        );
        let focus_request = synchronize_focus_request(&response, surface);
        if let Some(TextSurfaceFocusRequestResult::Acknowledged(value)) = focus_request.as_ref() {
            self.pending_focus_request = Some(value.focused);
        }
        let mut raster_frame = self.raster(surface, raster_style, content_width, scale_factor)?;
        if surface.has_controlled_automatic_gutter() {
            for _ in 0..2 {
                let measured_width =
                    self.gutter(&raster_frame, surface, raster_style, scale_factor)?;
                if measured_width == gutter_width {
                    break;
                }
                gutter_width = measured_width;
                content_width = (surface_width - gutter_width as f32).max(1.0);
                viewport_bounds = UiRect::new(
                    surface_bounds.x.saturating_add(gutter_width as i32),
                    surface_bounds.y,
                    surface_bounds.width.saturating_sub(gutter_width),
                    surface_bounds.height,
                );
                raster_frame = self.raster(surface, raster_style, content_width, scale_factor)?;
            }
        }
        synchronize_scroll_bounds(surface, &raster_frame, viewport_bounds);
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
        let placeholder = rasterize_placeholder(
            &mut self.rasterizer,
            surface,
            raster_style,
            content_width,
            scale_factor,
            placeholder_raster_identity(surface, raster_style),
            &self.metrics,
        )?;
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
            layers: PaintLayers::build(placeholder.is_some()),
            scroll_request,
            focus_request,
            frame,
        };
        let paint_plan = build_paint_plan(
            &mut self.rasterizer,
            &mut self.svg_rasterizer,
            &raster_frame,
            placeholder.as_ref(),
            &record,
            paint_style,
            raster_style,
            scale_factor,
            &self.metrics,
        )?;
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
        if input_policy.publish_text_input_target {
            crate::egui::text_command_surface::accesskit_evidence::record(
                ui.ctx(),
                crate::egui::text_command_surface::accesskit_evidence::AccessKitEvidence {
                    response_id: root_id,
                    bounds: ui_rect(response.rect),
                    label: surface.props().accessibility_label.clone(),
                    disabled: record.frame.accessibility.root.disabled,
                    target_identity: record.hit_target.clone(),
                    target_class: crate::egui::text_command_surface::accesskit_evidence::AccessKitTargetClass::TextInput,
                },
            );
        }
        if input_policy.publish_context_target {
            crate::egui::text_command_surface::accesskit_evidence::record(
                ui.ctx(),
                crate::egui::text_command_surface::accesskit_evidence::AccessKitEvidence {
                    response_id: root_id,
                    bounds: ui_rect(response.rect),
                    label: surface.props().accessibility_label.clone(),
                    disabled: record.frame.accessibility.root.disabled,
                    target_identity: "kuc.text-surface.context-target".to_owned(),
                    target_class: crate::egui::text_command_surface::accesskit_evidence::AccessKitTargetClass::TextSurfaceContextTarget,
                },
            );
        }
        let (context_target, pointer_context_invoked) =
            context_target_from_actual_input(ui, &response, &record);
        if context_target.is_some() && !pointer_context_invoked {
            events.extend(
                surface
                    .apply_action(
                        crate::text_surface::TextSurfaceAction::RequestContextTarget {
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
}
