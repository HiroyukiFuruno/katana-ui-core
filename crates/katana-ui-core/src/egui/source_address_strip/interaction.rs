pub(crate) struct Interaction;

impl Interaction {
    pub(crate) fn activated(ui: &egui::Ui, response: &egui::Response) -> bool {
        let focused = response.has_focus();
        response.clicked()
            || ui.input(|input| {
                input.has_accesskit_action_request(response.id, egui::accesskit::Action::Click)
                    || focused
                        && (input.key_pressed(egui::Key::Enter)
                            || input.key_pressed(egui::Key::Space))
            })
    }

    pub(crate) fn publish_button_accessibility(
        ui: &egui::Ui,
        id: egui::Id,
        rect: egui::Rect,
        label: &str,
        enabled: bool,
    ) {
        ui.ctx().accesskit_node_builder(id, |node| {
            node.set_role(egui::accesskit::Role::Button);
            node.set_label(label);
            node.set_bounds(Self::accesskit_bounds(rect));
            node.add_action(egui::accesskit::Action::Click);
            if !enabled {
                node.set_disabled();
            }
        });
    }

    fn accesskit_bounds(rect: egui::Rect) -> egui::accesskit::Rect {
        egui::accesskit::Rect {
            x0: f64::from(rect.min.x),
            y0: f64::from(rect.min.y),
            x1: f64::from(rect.max.x),
            y1: f64::from(rect.max.y),
        }
    }
}
