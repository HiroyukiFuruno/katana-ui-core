use super::paint::StatusBarPaint;

pub(super) fn publish_accessibility(
    ui: &egui::Ui,
    id: egui::Id,
    rect: egui::Rect,
    label: &str,
    identity: &str,
    interactive: bool,
) {
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(if interactive {
            egui::accesskit::Role::Button
        } else {
            egui::accesskit::Role::Label
        });
        node.set_label(label);
        node.set_bounds(egui::accesskit::Rect {
            x0: f64::from(rect.min.x),
            y0: f64::from(rect.min.y),
            x1: f64::from(rect.max.x),
            y1: f64::from(rect.max.y),
        });
        if interactive {
            node.add_action(egui::accesskit::Action::Click);
        }
    });
    crate::egui::text_command_surface::accesskit_evidence::record_custom(
        ui.ctx(),
        id,
        StatusBarPaint::ui_rect(rect),
        label,
        !interactive,
        identity,
        crate::egui::text_command_surface::accesskit_evidence::AccessKitTargetClass::StatusBarSegment,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egui::text_command_surface::accesskit_evidence;

    #[test]
    fn passive_label_changes_bind_the_actual_ax_text_without_exposing_an_action() {
        let context = egui::Context::default();
        context.enable_accesskit();
        let mut hashes = Vec::new();
        for label in ["Status A", "Status B"] {
            let mut evidence = None;
            let mut output = context.run_ui(egui::RawInput::default(), |ui| {
                accesskit_evidence::begin_frame(ui.ctx());
                let id = egui::Id::new("status-message-contract");
                let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 20.0));
                let _ = ui.interact(rect, id, egui::Sense::hover());
                publish_accessibility(ui, id, rect, label, "message", false);
                evidence = Some(accesskit_evidence::finish_frame(ui.ctx()));
            });
            output.textures_delta.clear();
            let evidence = evidence.expect("frame evidence");
            assert_eq!(evidence.entries.len(), 1);
            assert_eq!(evidence.entries[0].label, label);
            assert_eq!(evidence.entries[0].target_identity, "message");
            assert!(
                evidence.entries[0].disabled,
                "passive text cannot issue a locator action"
            );
            hashes.push(accesskit_evidence::snapshot_hash(&evidence.entries).expect("hash"));
            let update = output.platform_output.accesskit_update.expect("AX update");
            let node = update
                .nodes
                .iter()
                .find_map(|(_, node)| (node.label() == Some(label)).then_some(node))
                .expect("passive label");
            assert_eq!(node.role(), egui::accesskit::Role::Label);
            assert!(!node.supports_action(egui::accesskit::Action::Click));
        }
        assert_ne!(
            hashes[0], hashes[1],
            "identical bounds with different AX text must change the record"
        );
    }
}
