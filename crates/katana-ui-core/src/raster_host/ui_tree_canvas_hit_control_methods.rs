use super::{
    INDENT, NODE_GAP, SettingsListLayoutMetrics, TEXT_HEIGHT, UI_LINK_OPEN_ACTION_ID, UiNode,
    UiTreeHitRect, UiTreeHostActionHitCollector, UiTreeTextMetrics, button_dimensions,
    checkbox_row_height, checkbox_row_width, dimension_px, image_target_size,
    logical_image_height_exact, logical_image_width_exact, remaining_width, toggle_dimensions,
};
use katana_ui_core::render_model::UiVisualRole;

impl UiTreeHostActionHitCollector<'_> {
    pub(super) fn accordion(&mut self, node: &UiNode, x: usize) {
        let document_accordion = node.props().text.role == "html-accordion";
        let header_height =
            UiTreeTextMetrics::for_node_with_typography(node, self.typography).line_height;
        self.push_node_action_hits(
            node,
            UiTreeHitRect {
                x,
                y: self.y,
                width: remaining_width(self.area, x),
                height: header_height,
            },
        );
        self.y = self.y.saturating_add(header_height);
        if !node.props().interaction.open {
            return;
        }
        let child_x = if document_accordion {
            x
        } else {
            x.saturating_add(INDENT)
        };
        for child in node.children() {
            self.node(child, child_x);
        }
        if !document_accordion {
            self.y = self.y.saturating_add(NODE_GAP);
        }
    }

    pub(super) fn text(&mut self, node: &UiNode, x: usize) {
        let text_x = x.saturating_add(dimension_px(&node.props().common.margin.left));
        let requested_height = dimension_px(&node.props().common.height);
        let height = if requested_height > 0 {
            requested_height
        } else {
            self.text_hit_height(node, text_x)
        };
        let full_rect = UiTreeHitRect {
            x: text_x,
            y: self.y,
            width: remaining_width(self.area, text_x)
                .saturating_sub(dimension_px(&node.props().common.margin.right))
                .max(1),
            height,
        };
        let actions = self.actions_for_node(node);
        self.push_node_hit(node, full_rect);
        self.push_text_link_action_hits(node, text_x, height, &actions);
        self.push_action_hits(
            node,
            actions
                .into_iter()
                .filter(|action| action.action_id != UI_LINK_OPEN_ACTION_ID),
            full_rect,
        );
        self.y = self.y.saturating_add(height);
    }

    pub(super) fn checkbox(&mut self, node: &UiNode, x: usize) {
        let height = checkbox_row_height(node);
        self.push_node_action_hits(
            node,
            UiTreeHitRect {
                x,
                y: self.y,
                width: checkbox_row_width(self.text, node),
                height,
            },
        );
        self.y = self.y.saturating_add(height);
    }

    pub(super) fn image(&mut self, node: &UiNode, x: usize) {
        let image = &node.props().image_surface;
        let width = logical_image_width_exact(image);
        let height = logical_image_height_exact(image);
        let max_width = remaining_width(self.area, x);
        let requested_height = dimension_px(&node.props().common.height);
        let image_box_height = requested_height;
        let (target_width, target_height) =
            image_target_size(width, height, max_width, image_box_height);
        let advance = if requested_height > 0 {
            requested_height
        } else {
            target_height.saturating_add(NODE_GAP)
        };
        let media_frame = matches!(
            node.props().visual_role,
            UiVisualRole::MediaFrame | UiVisualRole::ExportMediaFrame
        );
        let hit_x = if media_frame {
            x.saturating_add(max_width.saturating_sub(target_width) / 2)
        } else {
            x
        };
        let hit_y = if media_frame && requested_height > target_height {
            self.y
                .saturating_add(requested_height.saturating_sub(target_height) / 2)
        } else {
            self.y
        };
        let hit_rect = UiTreeHitRect {
            x: hit_x,
            y: hit_y,
            width: target_width,
            height: target_height,
        };
        self.push_node_hit(node, hit_rect);
        self.y = self.y.saturating_add(advance);
    }

    pub(super) fn button(&mut self, node: &UiNode, x: usize) {
        let (width, height) = button_dimensions(node);
        self.push_node_action_hits(
            node,
            UiTreeHitRect {
                x,
                y: self.y,
                width,
                height,
            },
        );
        self.y = self.y.saturating_add(TEXT_HEIGHT + NODE_GAP);
    }

    pub(super) fn toggle(&mut self, node: &UiNode, x: usize) {
        let (width, height) = toggle_dimensions();
        self.push_node_action_hits(
            node,
            UiTreeHitRect {
                x,
                y: self.y,
                width,
                height,
            },
        );
        self.y = self.y.saturating_add(height.max(TEXT_HEIGHT));
    }

    pub(super) fn text_entry_control(&mut self, node: &UiNode, x: usize) {
        let width = dimension_px(&node.props().common.width)
            .max(SettingsListLayoutMetrics::DEFAULT.text_entry_width() as usize);
        self.push_node_action_hits(
            node,
            UiTreeHitRect {
                x,
                y: self.y,
                width,
                height: TEXT_HEIGHT,
            },
        );
        self.y = self.y.saturating_add(TEXT_HEIGHT);
    }
}
