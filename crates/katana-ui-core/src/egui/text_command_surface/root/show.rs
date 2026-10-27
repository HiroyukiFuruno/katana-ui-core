use super::root_event::build_event_batch;
use super::root_frame::build_frame;
use super::{
    EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootError, EguiTextCommandSurfaceRootOutput,
    TextCommandSurfaceStyle, global_action_render, interaction_locator, modal_input,
};
use crate::egui::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor,
};

const GLOBAL_MENU_HEIGHT_PX: f32 = 28.0;

impl EguiTextCommandSurfaceRoot {
    /// Renders one actual root frame and returns only its closed frame/event contracts.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        style: &TextCommandSurfaceStyle,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        ui.ctx().enable_accesskit();
        self.frame_serial = self.frame_serial.saturating_add(1);
        self.global_generation
            .set(self.global_generation.get().saturating_add(1));
        if let Some(dialog) = self.selection_dialog.as_mut() {
            dialog.prepare_input(ui);
        }
        let dialog_open = self
            .selection_dialog
            .as_ref()
            .is_some_and(|dialog| dialog.is_open());
        if dialog_open {
            if let Some(dialog) = self.selection_dialog.as_mut() {
                dialog.capture_restore_focus(ui.ctx());
            }
            modal_input::suspend(self)?;
        }
        let root_canvas = ui.available_rect_before_wrap();
        let global_bounds = self.global_actions.as_ref().map(|actions| {
            let bounds = ui
                .allocate_exact_size(
                    egui::vec2(ui.available_width(), GLOBAL_MENU_HEIGHT_PX),
                    egui::Sense::hover(),
                )
                .0;
            (bounds, actions)
        });
        let was_composing = self.global_ime_composing;
        ui.input(|input| {
            for event in &input.events {
                match event {
                    egui::Event::Ime(egui::ImeEvent::Preedit { text, .. }) => {
                        self.global_ime_composing = !text.is_empty()
                    }
                    egui::Event::Ime(egui::ImeEvent::Commit(_))
                    | egui::Event::WindowFocused(false) => self.global_ime_composing = false,
                    _ => {}
                }
            }
        });
        let global_menu_open = self
            .global_renderer
            .as_ref()
            .is_some_and(global_action_render::KucGlobalActionsRenderer::is_open);
        let mut global_events = self
            .global_actions
            .as_ref()
            .filter(|_| ui.is_enabled())
            .map_or_else(Vec::new, |actions| {
                actions.consume_shortcut(
                    ui,
                    was_composing
                        || self.global_ime_composing
                        || self.surface.text.state().text_area.composition.is_some(),
                    self.global_document_focus,
                )
            });
        let modal_events = (dialog_open || global_menu_open)
            .then(|| ui.input_mut(|input| std::mem::take(&mut input.events)));
        let output_result = ui
            .add_enabled_ui(!dialog_open && !global_menu_open, |ui| {
                self.adapter.show_with_tab_strip(
                    ui,
                    &mut self.surface,
                    style,
                    self.tab_strip.as_mut(),
                    self.status_bar.as_mut(),
                    self.diagnostics_list.as_mut(),
                    self.editor_viewport.as_mut(),
                )
            })
            .inner;
        if let Some(events) = modal_events {
            ui.input_mut(|input| input.events = events);
        }
        let output = output_result?;
        let dialog = if let Some(dialog) = self.selection_dialog.as_mut() {
            dialog
                .show(ui.ctx(), style)
                .map_err(EguiTextCommandSurfaceRootError::Serialization)?
        } else {
            None
        };
        let mut output = output.with_selection_dialog(dialog);
        let mut global_plan = None;
        if let Some((bounds, actions)) = global_bounds {
            output.root_bounds = global_action_render::ui_rect(root_canvas);
            if dialog_open {
                self.global_renderer = None;
            } else {
                let mut renderer = match self.global_renderer.take() {
                    Some(renderer) => renderer,
                    None => global_action_render::KucGlobalActionsRenderer::new(
                        std::sync::Arc::clone(&self.adapter.catalog),
                        self.adapter.text_raster_config.clone(),
                    )
                    .map_err(|error| {
                        EguiTextCommandSurfaceRootError::GlobalActionRaster(error.to_string())
                    })?,
                };
                let rendered = renderer.show(
                    ui,
                    &self.identity,
                    bounds,
                    root_canvas,
                    actions,
                    !dialog_open,
                    &style.chrome_raster,
                    &style.chrome_paint,
                );
                self.global_renderer = Some(renderer);
                let (events, plan) = rendered.map_err(|error| {
                    EguiTextCommandSurfaceRootError::GlobalActionRaster(error.to_string())
                })?;
                global_events.extend(events);
                global_plan = Some(plan);
            }
        }
        let accesskit = super::super::accesskit_evidence::finish_frame(ui.ctx());
        self.global_document_focus = accesskit
            .entries
            .iter()
            .find(|entry| {
                entry.target_class
                    == super::super::accesskit_evidence::AccessKitTargetClass::TextInput
                    && entry.target_identity
                        == self.surface.text.state().text_area.state_id.as_str()
            })
            .map(|entry| entry.response_id);
        output.accesskit_evidence = accesskit.entries;
        output.accesskit_text_input_nodes = accesskit.text_input_nodes;
        let mut events = build_event_batch(
            &mut output,
            self.source_address_submission_port.clone(),
            global_events,
        )
        .map_err(EguiTextCommandSurfaceRootError::Serialization)?;
        if events.has_events() {
            self.state_revision = self.state_revision.saturating_add(1);
        }
        events.set_root_metadata(&self.identity, self.state_revision);
        events.set_root_frame_serial(self.frame_serial);
        let context = events.current_context();
        let bound_evidence = super::super::accesskit_evidence::bind_frame(
            output.accesskit_evidence.clone(),
            &self.identity,
            &context,
        );
        let mut plans = output.artifact_paint_plans()?;
        if let Some(plan) = global_plan.as_ref() {
            plans.push(crate::egui::artifact_compositor::ArtifactPaintPlanRef::TabStrip(plan));
        }
        let composite = ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(output.root_bounds),
            plans: &plans,
        })?;
        let frame = build_frame(&self.identity, self.state_revision, &output, &composite)
            .map_err(EguiTextCommandSurfaceRootError::Serialization)?;
        let locator = interaction_locator::KucInteractionLocator::from_output(
            &self.identity,
            &context,
            self.frame_serial,
            &output,
            &bound_evidence,
        );
        let artifact_order = output.artifact_order().to_vec();
        #[cfg(any(test, feature = "storybook-artifacts"))]
        let context_menu_visible = output
            .context_menu
            .as_ref()
            .is_some_and(|value| value.record.is_some() && value.artifact.is_some());
        Ok(EguiTextCommandSurfaceRootOutput {
            evidence_text: output.text,
            evidence_composite: composite,
            accesskit_evidence: output.accesskit_evidence,
            accesskit_text_input_nodes: output.accesskit_text_input_nodes,
            locator,
            artifact_order,
            #[cfg(any(test, feature = "storybook-artifacts"))]
            context_menu_visible,
            frame,
            events,
            #[cfg(test)]
            toolbar_record: output.toolbar.map(|value| value.record),
            #[cfg(test)]
            floating: output.floating,
            #[cfg(test)]
            context_menu_record: output.context_menu.and_then(|value| value.record),
            #[cfg(test)]
            search_record: output.search.map(|value| value.record),
        })
    }
}

#[cfg(test)]
#[path = "show_error_tests.rs"]
mod tests;
