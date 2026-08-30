//! Frame composition for the retained diagnostics adapter.

use super::super::accessibility::DiagnosticsAccessibility;
use super::super::identity::DiagnosticsTargetIdentity;
use super::super::paint::DiagnosticsPaint;
use super::super::types::{
    DiagnosticsListPaintPlan, DiagnosticsListStyle, EguiDiagnosticsListError,
    EguiDiagnosticsListOutput,
};
use super::{DiagnosticsRenderLayout, EguiDiagnosticsListAdapter};
use katana_ui_core::molecule::{DiagnosticSeverity, DiagnosticsList, DiagnosticsListAction};

impl EguiDiagnosticsListAdapter {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        diagnostics: &mut DiagnosticsList,
    ) -> Result<EguiDiagnosticsListOutput, EguiDiagnosticsListError> {
        self.raster_evidence.clear();
        let scale = ui.ctx().pixels_per_point();
        let style = DiagnosticsListStyle::standard();
        let width = ui.available_width().max(1.0);
        let snapshot = diagnostics.render_snapshot();
        let scope_height = if snapshot.scopes.is_empty() {
            0.0
        } else {
            style.scope_row_height
        };
        let height = (style.header_height + scope_height + style.viewport_height).max(1.0);
        let (surface, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
        let mut plan = DiagnosticsListPaintPlan {
            surface_bounds: DiagnosticsPaint::ui_rect(surface),
            operations: vec![DiagnosticsPaint::fill(
                DiagnosticsPaint::ui_rect(surface),
                DiagnosticsPaint::ui_rect(surface),
                style.background,
            )],
        };
        let mut output = EguiDiagnosticsListOutput {
            events: Vec::new(),
            paint_plan: DiagnosticsListPaintPlan {
                surface_bounds: DiagnosticsPaint::ui_rect(surface),
                operations: Vec::new(),
            },
        };
        let header = egui::Rect::from_min_size(
            surface.min,
            egui::vec2(surface.width(), style.header_height + scope_height),
        );
        let viewport = egui::Rect::from_min_size(
            egui::pos2(surface.left(), header.bottom()),
            egui::vec2(surface.width(), style.viewport_height),
        );
        let row_heights = snapshot
            .visible
            .visible_ids
            .iter()
            .map(|item_id| {
                snapshot
                    .items
                    .iter()
                    .find(|item| &item.id == item_id)
                    .map_or(style.row_height, |item| {
                        self.row_height(item, &snapshot.state.expanded_ids, &style)
                    })
            })
            .collect::<Vec<_>>();
        let content_height = row_heights.iter().sum::<f32>();
        let max_scroll_y = (content_height - viewport.height()).max(0.0);
        self.scroll_y = self.scroll_y.clamp(0.0, max_scroll_y);
        let scroll_id = self.id.with("viewport");
        let has_accesskit_scroll_down = ui.input(|input| {
            input.has_accesskit_action_request(scroll_id, egui::accesskit::Action::ScrollDown)
        });
        let has_accesskit_scroll_up = ui.input(|input| {
            input.has_accesskit_action_request(scroll_id, egui::accesskit::Action::ScrollUp)
        });
        let scroll_response = ui.interact(viewport, scroll_id, egui::Sense::hover());
        let viewport_is_hovered =
            scroll_response.hovered() || ui.rect_contains_pointer(scroll_response.rect);
        DiagnosticsAccessibility::publish_scroll_accessibility(
            ui,
            scroll_response.id,
            viewport,
            max_scroll_y > 0.0,
        );
        let scroll_delta = if viewport_is_hovered {
            ui.input(|input| {
                input
                    .raw
                    .events
                    .iter()
                    .filter_map(|event| match event {
                        egui::Event::MouseWheel { delta, .. } => Some(delta.y),
                        _ => None,
                    })
                    .sum::<f32>()
            })
        } else {
            0.0
        };
        if scroll_delta != 0.0 {
            self.scroll_y = (self.scroll_y - scroll_delta).clamp(0.0, max_scroll_y);
        }
        if has_accesskit_scroll_down {
            self.scroll_y = (self.scroll_y + style.accessibility_scroll_step).min(max_scroll_y);
        }
        if has_accesskit_scroll_up {
            self.scroll_y = (self.scroll_y - style.accessibility_scroll_step).max(0.0);
        }
        let title_bounds =
            egui::Rect::from_min_size(header.min, egui::vec2(header.width(), style.header_height));
        self.paint_text(&mut plan, title_bounds, &snapshot.label, &style, scale)?;
        let scopes_result = self.render_scopes_and_severity(
            ui,
            &mut plan,
            &mut output,
            diagnostics,
            &style,
            header,
            scale,
        );
        scopes_result?;
        let items_result = self.render_items(
            ui,
            &mut plan,
            &mut output,
            DiagnosticsRenderLayout {
                style: &style,
                surface,
                header,
                viewport,
                row_heights: &row_heights,
                scale,
            },
            diagnostics,
        );
        items_result?;
        self.process_keyboard(
            ui,
            &mut output,
            &row_heights,
            viewport,
            max_scroll_y,
            diagnostics,
        );
        self.process_focus_closure(ui, &mut output, surface, diagnostics);
        self.paint_plan(ui, &plan);
        self.last_paint_plan = Some(plan);
        output.paint_plan = self
            .last_paint_plan
            .clone()
            .ok_or(EguiDiagnosticsListError::PaintPlanNotProduced)?;
        Ok(output)
    }

    fn render_scopes_and_severity(
        &mut self,
        ui: &mut egui::Ui,
        plan: &mut DiagnosticsListPaintPlan,
        output: &mut EguiDiagnosticsListOutput,
        diagnostics: &mut DiagnosticsList,
        style: &DiagnosticsListStyle,
        header: egui::Rect,
        scale: f32,
    ) -> Result<(), EguiDiagnosticsListError> {
        let snapshot = diagnostics.render_snapshot();
        let mut x = header.left() + super::super::paint::DIAGNOSTICS_LEFT_INSET;
        if !snapshot.scopes.is_empty() {
            let scope_row = egui::Rect::from_min_size(
                egui::pos2(header.left(), header.bottom() - style.scope_row_height),
                egui::vec2(header.width(), style.scope_row_height),
            );
            let scope_count = snapshot.scopes.len();
            for scope in &snapshot.scopes {
                let label_width = self.text_width(&scope.label, style, scale)?
                    + super::super::paint::DIAGNOSTICS_SCOPE_LABEL_PADDING;
                let bounds = egui::Rect::from_min_size(
                    egui::pos2(
                        x,
                        scope_row.top() + super::super::paint::DIAGNOSTICS_SMALL_INSET,
                    ),
                    egui::vec2(
                        label_width,
                        scope_row.height() - super::super::paint::DIAGNOSTICS_DOUBLE_INSET,
                    ),
                );
                let identity = DiagnosticsTargetIdentity::scope(scope.key.as_str());
                let response = ui.interact(
                    bounds,
                    self.id.with(identity.clone()),
                    if scope_count > 1 {
                        egui::Sense::click()
                    } else {
                        egui::Sense::hover()
                    },
                );
                DiagnosticsAccessibility::publish_accessibility_with_enabled(
                    ui,
                    response.id,
                    bounds,
                    &scope.accessible_label,
                    egui::accesskit::Role::RadioButton,
                    identity,
                    crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::DiagnosticsScope,
                    scope_count > 1,
                );
                if scope_count > 1
                    && (DiagnosticsAccessibility::pointer_click_requested(ui, &response)
                        || DiagnosticsAccessibility::accesskit_click_requested(ui, response.id))
                {
                    self.focused_scope = Some(scope.key.as_str().to_string());
                    response.request_focus();
                    output.events.extend(
                        diagnostics
                            .apply_action(DiagnosticsListAction::SelectScope(scope.key.clone())),
                    );
                }
                self.paint_text(plan, bounds, &scope.label, style, scale)?;
                x += label_width + super::super::paint::DIAGNOSTICS_SMALL_INSET;
            }
        }
        let mut x = header.left() + super::super::paint::DIAGNOSTICS_LEFT_INSET;
        for severity in DiagnosticSeverity::all() {
            let label = format!("{severity:?}");
            let width = self.text_width(&label, style, scale)?
                + super::super::paint::DIAGNOSTICS_FILTER_PADDING;
            let bounds = egui::Rect::from_min_size(
                egui::pos2(
                    x,
                    header.top() + super::super::paint::DIAGNOSTICS_SMALL_INSET,
                ),
                egui::vec2(
                    width,
                    style.header_height - super::super::paint::DIAGNOSTICS_DOUBLE_INSET,
                ),
            );
            let response = ui.interact(
                bounds,
                self.id.with(format!("filter-{severity:?}")),
                egui::Sense::click(),
            );
            DiagnosticsAccessibility::publish_accessibility(
                ui,
                response.id,
                bounds,
                &label,
                egui::accesskit::Role::CheckBox,
                DiagnosticsTargetIdentity::severity_filter(severity),
                crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::DiagnosticsSeverityFilter,
            );
            if DiagnosticsAccessibility::pointer_click_requested(ui, &response)
                || DiagnosticsAccessibility::accesskit_click_requested(ui, response.id)
            {
                let mut filters = snapshot.options.severity_filter.clone();
                if !filters.remove(&severity) {
                    filters.insert(severity);
                }
                output.events.extend(
                    diagnostics.apply_action(DiagnosticsListAction::SetSeverityFilter(filters)),
                );
            }
            self.paint_text(plan, bounds, &label, style, scale)?;
            x += width + super::super::paint::DIAGNOSTICS_SMALL_INSET;
        }
        Ok(())
    }
}
