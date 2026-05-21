mod actions;
mod render;
mod types;

use crate::render_model::{UiNode, UiStateId};
pub use types::{
    ScrollArea, ScrollAreaAction, ScrollAreaEvent, ScrollAxis, ScrollEdge, ScrollRejectionReason,
    ScrollbarPlacement, ScrollbarVisibility,
};

impl ScrollArea {
    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn axis(mut self, value: ScrollAxis) -> Self {
        self.axis = value;
        self.clamp_offsets();
        self
    }

    #[must_use]
    pub fn viewport(mut self, width: u32, height: u32) -> Self {
        self.viewport_width = width;
        self.viewport_height = height;
        self.clamp_offsets();
        self
    }

    #[must_use]
    pub fn content_extent(mut self, width: u32, height: u32) -> Self {
        self.content_width = width;
        self.content_height = height;
        self.clamp_offsets();
        self
    }

    #[must_use]
    pub fn offset(mut self, x: u32, y: u32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self.clamp_offsets();
        self
    }

    #[must_use]
    pub fn scrollbar_visibility(mut self, value: ScrollbarVisibility) -> Self {
        self.scrollbar_visibility = value;
        self
    }

    #[must_use]
    pub fn scrollbar_placement(mut self, value: ScrollbarPlacement) -> Self {
        self.scrollbar_placement = value;
        self
    }

    #[must_use]
    pub fn edge_threshold(mut self, value: u32) -> Self {
        self.edge_threshold = value;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub const fn offset_x(&self) -> u32 {
        self.offset_x
    }

    #[must_use]
    pub const fn offset_y(&self) -> u32 {
        self.offset_y
    }
}

impl Default for ScrollArea {
    fn default() -> Self {
        Self::new()
    }
}
