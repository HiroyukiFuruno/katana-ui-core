use super::CommandChromeDropdownLayout;
use crate::interaction::placement::{PlacementResult, Rect};
use crate::molecule::toolbar::ToolbarActionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeOpenDropdown {
    action_id: ToolbarActionId,
    trigger_bounds: Rect,
    placement: PlacementResult,
    bounds: Rect,
    focused_item_index: Option<usize>,
}

impl CommandChromeOpenDropdown {
    #[must_use]
    pub(crate) fn new(
        action_id: ToolbarActionId,
        layout: CommandChromeDropdownLayout,
        focused_item_index: Option<usize>,
    ) -> Self {
        let placement = layout.resolve();
        let bounds = Rect::new(
            placement.position.x,
            placement.position.y,
            layout.panel_size().width,
            layout.panel_size().height,
        );
        Self {
            action_id,
            trigger_bounds: layout.trigger_bounds(),
            placement,
            bounds,
            focused_item_index,
        }
    }

    #[must_use]
    pub const fn action_id(&self) -> &ToolbarActionId {
        &self.action_id
    }

    #[must_use]
    pub const fn placement(&self) -> PlacementResult {
        self.placement
    }

    #[must_use]
    pub const fn trigger_bounds(&self) -> Rect {
        self.trigger_bounds
    }

    #[must_use]
    pub const fn bounds(&self) -> Rect {
        self.bounds
    }

    #[must_use]
    pub const fn focused_item_index(&self) -> Option<usize> {
        self.focused_item_index
    }

    pub(crate) fn set_focused_item_index(&mut self, value: Option<usize>) {
        self.focused_item_index = value;
    }

    pub(crate) fn update_layout(&mut self, layout: CommandChromeDropdownLayout) {
        self.trigger_bounds = layout.trigger_bounds();
        self.placement = layout.resolve();
        self.bounds = Rect::new(
            self.placement.position.x,
            self.placement.position.y,
            layout.panel_size().width,
            layout.panel_size().height,
        );
    }
}
