use super::{
    ContainerPadding, INDENT, MeasuredNodeHeightCache, NODE_GAP, ScrollContainerPadding,
    ScrollHitClip, SettingsListLayoutMetrics, TEXT_HEIGHT, TextRenderer, ThemeSnapshot,
    UI_LINK_OPEN_ACTION_ID, UiHostActionPlan, UiNode, UiNodeKind, UiTextSpan, UiTreeCanvasPalette,
    UiTreeHitRect, UiTreeHostActionHit, UiTreeHostActionHitCollector, UiTreeNodeHit,
    UiTreeRenderArea, UiTreeRowLayout, UiTreeTextContext, UiTreeTextMetrics, UiTreeTextRenderer,
    UiTreeTextRoleRenderer, absolute_child_rect, button_dimensions,
    can_render_children_incrementally, checkbox_row_height, checkbox_row_width, child_container_x,
    child_render_area, clip_scroll_hit, dimension_px, duplicate_panel_label, frame_height,
    has_absolute_child, image_target_size, is_absolute, logical_image_height_exact,
    logical_image_width_exact, node_cursor, remaining_width, scroll_child_render_area,
    scroll_container_gap, scroll_source_y, semantic_node_id, should_draw_container_label,
    toggle_dimensions, tree_parts, whitespace_width,
};
#[path = "ui_tree_canvas_hit_action_methods.rs"]
mod ui_tree_canvas_hit_action_methods;
#[path = "ui_tree_canvas_hit_container_methods.rs"]
mod ui_tree_canvas_hit_container_methods;
#[path = "ui_tree_canvas_hit_control_methods.rs"]
mod ui_tree_canvas_hit_control_methods;
#[path = "ui_tree_canvas_hit_scroll_methods.rs"]
mod ui_tree_canvas_hit_scroll_methods;
#[path = "ui_tree_canvas_hit_settings_methods.rs"]
mod ui_tree_canvas_hit_settings_methods;
#[path = "ui_tree_canvas_hit_text_methods.rs"]
mod ui_tree_canvas_hit_text_methods;

impl UiTreeHostActionHitCollector<'_> {
    pub(super) fn node(&mut self, node: &UiNode, x: usize) {
        let logical_start_y = self.text_logical_y.max(self.y as f32);
        self.text_logical_y = logical_start_y;
        let previous_semantic_node_id = self.semantic_node_id.clone();
        if let Some(semantic_node_id) = semantic_node_id(node) {
            self.semantic_node_id = Some(semantic_node_id);
        }
        let start_y = self.y;
        match node.kind() {
            UiNodeKind::Row => self.row(node, x),
            UiNodeKind::Accordion => self.accordion(node, x),
            UiNodeKind::ImageSurface => self.image(node, x),
            UiNodeKind::Button | UiNodeKind::TextButton | UiNodeKind::IconTextButton => {
                self.button(node, x)
            }
            UiNodeKind::Toggle => self.toggle(node, x),
            UiNodeKind::Input
            | UiNodeKind::TextArea
            | UiNodeKind::SearchBox
            | UiNodeKind::SelectBox
            | UiNodeKind::ComboBox => self.text_entry_control(node, x),
            UiNodeKind::SettingsList => self.settings_list(node, x),
            UiNodeKind::Panel => self.settings_panel(node, x),
            UiNodeKind::FormField => self.settings_form_field(node, x),
            UiNodeKind::TreeView => self.tree_view(node, x),
            UiNodeKind::ScrollArea => self.scroll_area(node, x),
            UiNodeKind::Checkbox => self.checkbox(node, x),
            UiNodeKind::Divider
            | UiNodeKind::Spinner
            | UiNodeKind::LoadingDots
            | UiNodeKind::Text => self.text(node, x),
            _ => self.container(node, x),
        }
        let requested_height = dimension_px(&node.props().common.height);
        if node.kind() != UiNodeKind::Text && requested_height > 0 {
            self.text_logical_y = logical_start_y + requested_height as f32;
            self.y = self.text_logical_y.floor().max(0.0) as usize;
        } else if node.kind() == UiNodeKind::Accordion {
            self.y = self.text_logical_y.floor().max(0.0) as usize;
        } else if node.kind() != UiNodeKind::Text {
            let advance = self.y.saturating_sub(start_y);
            self.text_logical_y = self.text_logical_y.max(logical_start_y + advance as f32);
            self.y = self.text_logical_y.floor().max(0.0) as usize;
        }
        self.semantic_node_id = previous_semantic_node_id;
    }

    pub(super) fn advance_y(&mut self, amount: usize) {
        self.y = self.y.saturating_add(amount);
        self.text_logical_y += amount as f32;
    }

    pub(super) fn row(&mut self, node: &UiNode, x: usize) {
        let row_x = UiTreeRowLayout::row_x(node, x);
        let row_top = self.y;
        let row_logical_top = self.text_logical_y;
        let mut row_bottom = self.y;
        let mut row_logical_bottom = self.text_logical_y;
        for child_layout in UiTreeRowLayout::children(node, x, self.area) {
            self.y = row_top;
            self.text_logical_y = row_logical_top;
            self.node(child_layout.child, child_layout.x);
            row_bottom = row_bottom.max(self.y);
            row_logical_bottom = row_logical_bottom.max(self.text_logical_y);
        }
        if row_bottom > row_top {
            self.push_node_action_hits(
                node,
                UiTreeHitRect {
                    x: row_x,
                    y: row_top,
                    width: remaining_width(self.area, row_x),
                    height: row_bottom.saturating_sub(row_top),
                },
            );
        }
        self.y = row_bottom;
        self.text_logical_y = row_logical_bottom;
    }
}
