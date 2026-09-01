//! Physical input and AccessKit publication for diagnostics controls.

use super::paint::DiagnosticsPaint;
use crate::egui::text_command_surface::accesskit_evidence::AccessKitTargetClass;

pub(crate) struct DiagnosticsAccessibility;

impl DiagnosticsAccessibility {
    pub(crate) fn accesskit_click_requested(ui: &egui::Ui, id: egui::Id) -> bool {
        ui.input(|input| {
            input.raw.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::AccessKitActionRequest(request)
                        if request.target_tree == egui::accesskit::TreeId::ROOT
                            && request.target_node == id.accesskit_id()
                            && request.action == egui::accesskit::Action::Click
                )
            })
        })
    }

    pub(crate) fn pointer_pressed_outside(ui: &egui::Ui, surface: egui::Rect) -> bool {
        ui.input(|input| {
            input.raw.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::PointerButton {
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        pos,
                        ..
                    } if !surface.contains(*pos)
                )
            })
        })
    }

    pub(crate) fn pointer_click_requested(ui: &egui::Ui, response: &egui::Response) -> bool {
        response.clicked()
            && ui.input(|input| {
                input.raw.events.iter().any(|event| {
                    matches!(
                        event,
                        egui::Event::PointerButton {
                            button: egui::PointerButton::Primary,
                            ..
                        }
                    )
                })
            })
    }

    pub(crate) fn publish_accessibility(
        ui: &egui::Ui,
        id: egui::Id,
        rect: egui::Rect,
        label: &str,
        role: egui::accesskit::Role,
        target_identity: String,
        target_class: AccessKitTargetClass,
    ) {
        Self::publish_accessibility_with_enabled(
            ui,
            id,
            rect,
            label,
            role,
            target_identity,
            target_class,
            true,
        );
    }

    pub(crate) fn publish_accessibility_with_enabled(
        ui: &egui::Ui,
        id: egui::Id,
        rect: egui::Rect,
        label: &str,
        role: egui::accesskit::Role,
        target_identity: String,
        target_class: AccessKitTargetClass,
        enabled: bool,
    ) {
        ui.ctx().accesskit_node_builder(id, |node| {
            node.set_role(role);
            node.set_label(label);
            node.set_bounds(egui::accesskit::Rect {
                x0: f64::from(rect.min.x),
                y0: f64::from(rect.min.y),
                x1: f64::from(rect.max.x),
                y1: f64::from(rect.max.y),
            });
            if enabled {
                node.add_action(egui::accesskit::Action::Click);
            }
        });
        crate::egui::text_command_surface::accesskit_evidence::record_custom(
            ui.ctx(),
            id,
            DiagnosticsPaint::ui_rect(rect),
            label,
            !enabled,
            &target_identity,
            target_class,
        );
    }

    pub(crate) fn publish_scroll_accessibility(
        ui: &egui::Ui,
        id: egui::Id,
        rect: egui::Rect,
        scrollable: bool,
    ) {
        ui.ctx().accesskit_node_builder(id, |node| {
            node.set_role(egui::accesskit::Role::List);
            node.set_bounds(egui::accesskit::Rect {
                x0: f64::from(rect.min.x),
                y0: f64::from(rect.min.y),
                x1: f64::from(rect.max.x),
                y1: f64::from(rect.max.y),
            });
            if scrollable {
                node.add_action(egui::accesskit::Action::ScrollUp);
                node.add_action(egui::accesskit::Action::ScrollDown);
            }
        });
    }
}
