use super::ContextMenu;
use super::types::{ContextMenuAnchor, ContextMenuItem, ContextMenuPlacement};

impl ContextMenu {
    #[must_use]
    pub fn anchor(mut self, value: ContextMenuAnchor) -> Self {
        self.props.anchor = value;
        self
    }

    #[must_use]
    pub fn placement_priority(mut self, value: Vec<ContextMenuPlacement>) -> Self {
        self.props.placement_priority = value;
        self
    }

    #[must_use]
    pub fn placement_used(mut self, value: ContextMenuPlacement) -> Self {
        self.props.placement_used = value;
        self
    }

    #[must_use]
    pub fn min_width(mut self, value: u32) -> Self {
        self.props.min_width = value;
        self
    }

    #[must_use]
    pub fn max_height(mut self, value: u32) -> Self {
        self.props.max_height = value;
        self
    }

    #[must_use]
    pub fn submenu_open_delay_ms(mut self, value: u16) -> Self {
        self.props.submenu_open_delay_ms = value;
        self
    }

    #[must_use]
    pub fn focus_return_target(mut self, value: impl Into<String>) -> Self {
        self.props.focus_return_target = value.into();
        self
    }

    #[must_use]
    pub fn highlighted_path(mut self, value: Vec<usize>) -> Self {
        self.props.highlighted_path = value;
        self
    }

    #[must_use]
    pub fn items(mut self, value: Vec<ContextMenuItem>) -> Self {
        self.state.item_count = value.len();
        self.props.items = value;
        self.state.sync_submenu_state_ids(&self.props.items);
        self
    }
}
