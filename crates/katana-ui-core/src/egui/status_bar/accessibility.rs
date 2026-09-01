use super::paint::StatusBarPaint;

pub(super) fn publish_accessibility(
    ui: &egui::Ui,
    id: egui::Id,
    rect: egui::Rect,
    label: &str,
    identity: &str,
) {
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::Button);
        node.set_label(label);
        node.set_bounds(egui::accesskit::Rect {
            x0: f64::from(rect.min.x),
            y0: f64::from(rect.min.y),
            x1: f64::from(rect.max.x),
            y1: f64::from(rect.max.y),
        });
        node.add_action(egui::accesskit::Action::Click);
    });
    crate::egui::text_command_surface::accesskit_evidence::record_custom(
        ui.ctx(),
        id,
        StatusBarPaint::ui_rect(rect),
        identity,
        false,
        label,
        crate::egui::text_command_surface::accesskit_evidence::AccessKitTargetClass::StatusBarSegment,
    );
}
