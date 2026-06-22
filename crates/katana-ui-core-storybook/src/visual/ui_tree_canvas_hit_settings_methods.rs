use super::{
    SettingsListLayoutMetrics, UiNode, UiTreeHitRect, UiTreeHostActionHit,
    UiTreeHostActionHitCollector, duplicate_panel_label, remaining_width, tree_parts,
};

impl UiTreeHostActionHitCollector<'_> {
    pub(super) fn settings_list(&mut self, node: &UiNode, x: usize) {
        self.y = self
            .y
            .saturating_add(SettingsListLayoutMetrics::DEFAULT.title_height() as usize);
        let child_x = x.saturating_add(SettingsListLayoutMetrics::DEFAULT.child_indent() as usize);
        for child in node.children() {
            self.node(child, child_x);
        }
    }

    pub(super) fn settings_panel(&mut self, node: &UiNode, x: usize) {
        self.push_node_action_hits(
            node,
            UiTreeHitRect {
                x,
                y: self.y,
                width: remaining_width(self.area, x),
                height: SettingsListLayoutMetrics::DEFAULT.section_height() as usize,
            },
        );
        self.y = self
            .y
            .saturating_add(SettingsListLayoutMetrics::DEFAULT.section_height() as usize);
        let child_x = x.saturating_add(SettingsListLayoutMetrics::DEFAULT.child_indent() as usize);
        for child in node.children() {
            if duplicate_panel_label(node, child) {
                continue;
            }
            self.node(child, child_x);
        }
    }

    pub(super) fn settings_form_field(&mut self, node: &UiNode, x: usize) {
        let row_top = self.y;
        self.push_node_action_hits(
            node,
            UiTreeHitRect {
                x,
                y: row_top,
                width: remaining_width(self.area, x),
                height: SettingsListLayoutMetrics::DEFAULT.field_height() as usize,
            },
        );
        let mut control_y = row_top;
        let control_x =
            x.saturating_add(SettingsListLayoutMetrics::DEFAULT.field_label_width() as usize);
        let before_child_hits = self.hits.len();
        for child in node.children() {
            self.y = control_y;
            self.node(child, control_x);
            control_y = self.y;
        }
        let child_hits = self.hits.split_off(before_child_hits);
        self.hits.extend(child_hits.into_iter().filter(|hit| {
            !(hit.action.target.as_str().starts_with("settings-control:")
                && hit.action.settings_field_control_target().is_some())
        }));
        self.y = control_y.max(
            row_top.saturating_add(SettingsListLayoutMetrics::DEFAULT.field_height() as usize),
        );
    }

    pub(super) fn tree_view(&mut self, node: &UiNode, x: usize) {
        if !node.props().label.trim().is_empty() {
            self.y = self.y.saturating_add(tree_parts::ROW_HEIGHT);
        }
        let row_width = tree_parts::row_background_width(self.area.width, self.area.x, x);
        for tree_node in &node.props().tree.nodes {
            let rect = UiTreeHitRect {
                x,
                y: self.y,
                width: row_width,
                height: tree_parts::ROW_HEIGHT,
            };
            for action in self.tree_row_actions(node, &tree_node.id) {
                self.hits.push(UiTreeHostActionHit {
                    action,
                    rect,
                    cursor: node.props().tree.row_cursor,
                });
            }
            self.y = self.y.saturating_add(tree_parts::ROW_HEIGHT);
        }
        self.y = self.y.saturating_add(tree_parts::NODE_GAP);
    }
}
