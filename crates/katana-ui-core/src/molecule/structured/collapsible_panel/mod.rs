mod actions;
mod render;
mod state;
mod types;

use crate::render_model::{UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

pub use actions::{CollapsiblePanelAction, CollapsiblePanelEvent};
pub use state::CollapsiblePanelState;
pub use types::{CollapsiblePanelOptions, PanelMode, PanelSide, ResizableWidth};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollapsiblePanel {
    pub(super) label: String,
    pub(super) state_id: UiStateId,
    pub(super) options: CollapsiblePanelOptions,
    pub(super) state: CollapsiblePanelState,
    pub(super) children: Vec<UiNode>,
}

impl CollapsiblePanel {
    pub const OVERLAY_Z_INDEX: i32 = 80;
    const ICON_ONLY_WIDTH: u16 = 56;

    #[must_use]
    pub fn new(label: impl Into<String>, width: ResizableWidth) -> Self {
        let options = CollapsiblePanelOptions::default();
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::CollapsiblePanel),
            state: CollapsiblePanelState::new(PanelMode::Expanded, width, options.pinned),
            options,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn mode(mut self, mode: PanelMode) -> Self {
        self.state.mode = mode;
        self
    }

    #[must_use]
    pub fn side(mut self, side: PanelSide) -> Self {
        self.options.side = side;
        self
    }

    #[must_use]
    pub fn pinned(mut self, pinned: bool) -> Self {
        self.options.pinned = pinned;
        self.state.pinned = pinned;
        self
    }

    #[must_use]
    pub fn expand_on_hover(mut self, value: bool) -> Self {
        self.options.expand_on_hover = value;
        self
    }

    #[must_use]
    pub fn resize_handle(mut self, value: bool) -> Self {
        self.options.resize_handle = value;
        self
    }

    #[must_use]
    pub fn content(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn state(&self) -> &CollapsiblePanelState {
        &self.state
    }

    #[must_use]
    pub fn rendered_mode(&self) -> PanelMode {
        self.state.rendered_mode()
    }

    #[must_use]
    pub fn layout_contribution_width(&self) -> u16 {
        match self.rendered_mode() {
            PanelMode::Expanded => self.state.width.current,
            PanelMode::IconOnly => Self::ICON_ONLY_WIDTH,
            PanelMode::Collapsed | PanelMode::FloatingOverlay => 0,
        }
    }

    #[must_use]
    pub fn main_available_width(&self, container_width: u16) -> u16 {
        container_width.saturating_sub(self.layout_contribution_width())
    }

    pub fn apply_action(&mut self, action: CollapsiblePanelAction) -> Vec<CollapsiblePanelEvent> {
        self.state.apply_action(action, &self.options)
    }
}
