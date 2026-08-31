//! Item rows and quickfix controls for the diagnostics surface.

use super::super::accessibility::DiagnosticsAccessibility;
use super::super::identity::DiagnosticsTargetIdentity;
use super::super::paint::DiagnosticsPaint;
use super::super::types::{
    DiagnosticsListPaintPlan, EguiDiagnosticsListError, EguiDiagnosticsListOutput,
};
use super::{DiagnosticsRenderLayout, EguiDiagnosticsListAdapter};
use katana_ui_core::molecule::{DiagnosticsList, DiagnosticsListAction};

struct DisclosureRenderContext<'a> {
    plan: &'a mut DiagnosticsListPaintPlan,
    output: &'a mut EguiDiagnosticsListOutput,
    diagnostics: &'a mut DiagnosticsList,
    item: &'a katana_ui_core::molecule::DiagnosticItem,
    expanded: bool,
    bounds: egui::Rect,
    response: egui::Response,
    style: &'a super::super::types::DiagnosticsListStyle,
    scale: f32,
}

impl EguiDiagnosticsListAdapter {
    pub(super) fn render_items(
        &mut self,
        ui: &mut egui::Ui,
        plan: &mut DiagnosticsListPaintPlan,
        output: &mut EguiDiagnosticsListOutput,
        layout: DiagnosticsRenderLayout<'_>,
        diagnostics: &mut DiagnosticsList,
    ) -> Result<(), EguiDiagnosticsListError> {
        let DiagnosticsRenderLayout {
            style,
            surface,
            header,
            viewport,
            row_heights,
            scale,
        } = layout;
        let snapshot = diagnostics.render_snapshot();
        let mut row_top = 0.0;
        let visible_items =
            snapshot
                .visible
                .visible_ids
                .iter()
                .enumerate()
                .filter_map(|(index, item_id)| {
                    snapshot
                        .items
                        .iter()
                        .find(|item| &item.id == item_id)
                        .map(|item| (index, item))
                });
        for (index, item) in visible_items {
            let item_height = row_heights.get(index).copied().unwrap_or(style.row_height);
            let top = header.bottom() + row_top - self.scroll_y;
            row_top += item_height;
            let bounds = egui::Rect::from_min_size(
                egui::pos2(surface.left(), top),
                egui::vec2(surface.width(), item_height),
            );
            if !bounds.intersects(viewport) {
                continue;
            }
            let clipped_bounds = bounds.intersect(viewport);
            if snapshot.state.selected_id.as_ref() == Some(&item.id) {
                plan.operations.push(DiagnosticsPaint::fill(
                    DiagnosticsPaint::ui_rect(viewport),
                    DiagnosticsPaint::ui_rect(clipped_bounds),
                    style.selected,
                ));
            }
            let response = ui.interact(
                clipped_bounds,
                self.id.with(item.id.as_str()),
                egui::Sense::click(),
            );
            ui.memory_mut(|memory| {
                memory.set_focus_lock_filter(
                    response.id,
                    egui::EventFilter {
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        escape: true,
                        ..egui::EventFilter::default()
                    },
                );
            });
            DiagnosticsAccessibility::publish_accessibility(
                ui,
                response.id,
                bounds,
                &format!("{:?}: {}", item.severity, item.message),
                egui::accesskit::Role::ListItem,
                DiagnosticsTargetIdentity::item(item.id.as_str()),
                crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::DiagnosticsItem,
            );
            let item_clicked = DiagnosticsAccessibility::pointer_click_requested(ui, &response)
                || DiagnosticsAccessibility::accesskit_click_requested(ui, response.id);
            if item_clicked {
                self.focused_scope = None;
                self.focused_item = Some(item.id.as_str().to_string());
                response.request_focus();
            }
            let disclosure_bounds = egui::Rect::from_min_size(
                egui::pos2(
                    clipped_bounds.left() + super::super::paint::DIAGNOSTICS_SMALL_INSET,
                    clipped_bounds.top() + super::super::paint::DIAGNOSTICS_DISCLOSURE_TOP_INSET,
                ),
                egui::vec2(
                    super::super::paint::DIAGNOSTICS_DISCLOSURE_WIDTH,
                    (clipped_bounds.height()
                        - super::super::paint::DIAGNOSTICS_DISCLOSURE_HEIGHT_INSET)
                        .max(0.0),
                ),
            )
            .intersect(clipped_bounds)
            .intersect(viewport);
            let expanded = snapshot.state.expanded_ids.contains(&item.id);
            let disclosure_response = ui.interact(
                disclosure_bounds,
                self.id.with((item.id.as_str(), "disclosure")),
                if item.fix_preview.is_some() {
                    egui::Sense::click()
                } else {
                    egui::Sense::hover()
                },
            );
            let disclosure_clicked = if item.fix_preview.is_some() {
                let disclosure_result = self.render_disclosure(
                    ui,
                    DisclosureRenderContext {
                        plan,
                        output,
                        diagnostics,
                        item,
                        expanded,
                        bounds: disclosure_bounds,
                        response: disclosure_response,
                        style,
                        scale,
                    },
                );
                disclosure_result?
            } else {
                false
            };
            if item_clicked && !disclosure_clicked {
                output.events.extend(
                    diagnostics.apply_action(DiagnosticsListAction::Select(item.id.clone())),
                );
            }
            let location = format!(
                "{}:{}:{}",
                item.location.file, item.location.line, item.location.column
            );
            let text = format!("{:?}: {}  {}", item.severity, item.message, location);
            self.paint_text(plan, clipped_bounds, &text, style, scale)?;
            if let Some(preview) = item.fix_preview.as_ref().filter(|_| expanded) {
                let preview_bounds = egui::Rect::from_min_size(
                    egui::pos2(
                        clipped_bounds.left() + super::super::paint::DIAGNOSTICS_PREVIEW_LEFT_INSET,
                        clipped_bounds.top() + style.row_height,
                    ),
                    egui::vec2(
                        clipped_bounds.width()
                            - super::super::paint::DIAGNOSTICS_PREVIEW_RIGHT_INSET,
                        (Self::item_preview_height(item, style) - style.preview_padding).max(1.0),
                    ),
                );
                self.paint_preview(plan, viewport, preview_bounds, preview, style, scale)?;
            }
            if let Some(quickfix) = &item.quickfix {
                self.render_quickfix(
                    ui,
                    output,
                    diagnostics,
                    item,
                    quickfix,
                    clipped_bounds,
                    viewport,
                );
            }
        }
        Ok(())
    }

    fn render_disclosure(
        &mut self,
        ui: &egui::Ui,
        context: DisclosureRenderContext<'_>,
    ) -> Result<bool, EguiDiagnosticsListError> {
        let DisclosureRenderContext {
            plan,
            output,
            diagnostics,
            item,
            expanded,
            bounds,
            response,
            style,
            scale,
        } = context;
        let label = if expanded {
            "折りたたむ"
        } else {
            "展開"
        };
        DiagnosticsAccessibility::publish_accessibility(
            ui,
            response.id,
            bounds,
            label,
            egui::accesskit::Role::Button,
            DiagnosticsTargetIdentity::disclosure(item.id.as_str()),
            crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::DiagnosticsItem,
        );
        let clicked = DiagnosticsAccessibility::pointer_click_requested(ui, &response)
            || DiagnosticsAccessibility::accesskit_click_requested(ui, response.id);
        if clicked {
            self.focused_scope = None;
            self.focused_item = Some(item.id.as_str().to_string());
            ui.memory_mut(|memory| memory.request_focus(self.id.with(item.id.as_str())));
            output.events.extend(
                diagnostics.apply_action(DiagnosticsListAction::ToggleFixPreview(item.id.clone())),
            );
        }
        self.paint_text(plan, bounds, if expanded { "-" } else { "+" }, style, scale)?;
        Ok(clicked)
    }

    fn render_quickfix(
        &self,
        ui: &egui::Ui,
        output: &mut EguiDiagnosticsListOutput,
        diagnostics: &mut DiagnosticsList,
        item: &katana_ui_core::molecule::DiagnosticItem,
        quickfix: &katana_ui_core::molecule::DiagnosticAction,
        clipped_bounds: egui::Rect,
        viewport: egui::Rect,
    ) {
        let fix_bounds = egui::Rect::from_min_size(
            egui::pos2(
                clipped_bounds.right() - super::super::paint::DIAGNOSTICS_QUICKFIX_RIGHT_INSET,
                clipped_bounds.top() + super::super::paint::DIAGNOSTICS_DISCLOSURE_TOP_INSET,
            ),
            egui::vec2(
                super::super::paint::DIAGNOSTICS_QUICKFIX_WIDTH,
                (clipped_bounds.height()
                    - super::super::paint::DIAGNOSTICS_DISCLOSURE_HEIGHT_INSET)
                    .max(1.0),
            ),
        );
        let fix_bounds = fix_bounds.intersect(viewport);
        let response = ui.interact(
            fix_bounds,
            self.id.with((item.id.as_str(), "quickfix")),
            egui::Sense::click(),
        );
        DiagnosticsAccessibility::publish_accessibility(
            ui,
            response.id,
            fix_bounds,
            &quickfix.label,
            egui::accesskit::Role::Button,
            DiagnosticsTargetIdentity::fix(item.id.as_str()),
            crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::DiagnosticsFix,
        );
        let keyboard_activation = response.has_focus().then(|| {
            ui.input(|input| {
                [egui::Key::Enter, egui::Key::Space]
                    .into_iter()
                    .find(|key| input.key_pressed(*key))
            })
        });
        if let Some(key) = keyboard_activation.flatten() {
            ui.input_mut(|input| {
                let modifiers = input.modifiers;
                input.consume_key(modifiers, key);
            });
        }
        if DiagnosticsAccessibility::pointer_click_requested(ui, &response)
            || DiagnosticsAccessibility::accesskit_click_requested(ui, response.id)
            || keyboard_activation.flatten().is_some()
        {
            output
                .events
                .extend(diagnostics.apply_action(DiagnosticsListAction::ApplyFix(item.id.clone())));
        }
    }
}
