use super::types::ContextMenuPresentationItem;
use crate::render_model::UiRect;

pub(super) fn publish_menu(ui: &egui::Ui, id: egui::Id, bounds: UiRect) {
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::Menu);
        node.set_bounds(accesskit_bounds(bounds));
    });
}

pub(super) fn publish_item(
    ui: &egui::Ui,
    id: egui::Id,
    item: &ContextMenuPresentationItem,
    bounds: UiRect,
) {
    let label = accessibility_label(item).to_owned();
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::MenuItem);
        node.set_label(label.as_str());
        node.set_bounds(accesskit_bounds(bounds));
        node.add_action(egui::accesskit::Action::Click);
        if !item.enabled {
            node.set_disabled();
        }
    });
    crate::egui::text_command_surface::accesskit_evidence::record(
        ui.ctx(),
        crate::egui::text_command_surface::accesskit_evidence::AccessKitEvidence {
            response_id: id,
            bounds,
            label,
            disabled: !item.enabled,
            target_identity: item.id.clone(),
            target_class: crate::egui::text_command_surface::accesskit_evidence::AccessKitTargetClass::ContextMenuItem,
        },
    );
}

fn accessibility_label(item: &ContextMenuPresentationItem) -> &str {
    if item.accessibility_label.is_empty() {
        item.label.as_str()
    } else {
        item.accessibility_label.as_str()
    }
}

fn accesskit_bounds(bounds: UiRect) -> egui::accesskit::Rect {
    egui::accesskit::Rect {
        x0: bounds.x.into(),
        y0: bounds.y.into(),
        x1: bounds.x.saturating_add_unsigned(bounds.width).into(),
        y1: bounds.y.saturating_add_unsigned(bounds.height).into(),
    }
}
