use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiCommonProps, UiDimension, UiInteractionState, UiNode, UiNodeKind, UiStateId, UiZIndex,
};
use serde::{Deserialize, Serialize};

const FLOATING_OVERLAY_Z_INDEX: i32 = 80;
const FLOATING_OVERLAY_INDEX: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidebarMode {
    Expanded,
    IconOnly,
    Collapsed,
    FloatingOverlay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResizableWidth {
    pub min: u16,
    pub max: u16,
    pub current: u16,
    pub persist_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidebarEvent {
    None,
    ModeChanged(SidebarMode),
    WidthChanged(u16, String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollapsibleSidebar {
    label: String,
    state_id: UiStateId,
    mode: SidebarMode,
    width: ResizableWidth,
    pinned: bool,
    expand_on_hover: bool,
    last_event: SidebarEvent,
}

impl CollapsibleSidebar {
    #[must_use]
    pub fn new(label: impl Into<String>, width: ResizableWidth) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::CollapsibleSidebar),
            mode: SidebarMode::Expanded,
            width,
            pinned: true,
            expand_on_hover: false,
            last_event: SidebarEvent::None,
        }
    }

    #[must_use]
    pub fn mode(mut self, mode: SidebarMode) -> Self {
        self.mode = mode;
        self
    }

    #[must_use]
    pub fn hover_expand(mut self, pinned: bool, expand_on_hover: bool) -> Self {
        self.pinned = pinned;
        self.expand_on_hover = expand_on_hover;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn mode_state(&self) -> SidebarMode {
        self.mode
    }

    #[must_use]
    pub fn width(&self) -> u16 {
        self.width.current
    }

    #[must_use]
    pub fn last_event(&self) -> &SidebarEvent {
        &self.last_event
    }
}

impl ComponentAction for CollapsibleSidebar {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = state(self);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        match action {
            UiAction::SetSelectedIndex { selected_index, .. } => {
                self.mode = mode_for_index(*selected_index);
                self.last_event = SidebarEvent::ModeChanged(self.mode);
            }
            UiAction::SetValue { value, .. } => {
                let width = value.parse::<u16>().unwrap_or(self.width.current);
                self.width.current = width.clamp(self.width.min, self.width.max);
                self.last_event =
                    SidebarEvent::WidthChanged(self.width.current, self.width.persist_id.clone());
            }
            UiAction::SetHover { hovered, .. } if !self.pinned && self.expand_on_hover => {
                self.mode = if *hovered {
                    SidebarMode::Expanded
                } else {
                    SidebarMode::Collapsed
                };
                self.last_event = SidebarEvent::ModeChanged(self.mode);
            }
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        }
        UiActionResult::handled(self.state_id.clone(), action, before, state(self))
    }
}

impl From<CollapsibleSidebar> for UiNode {
    fn from(value: CollapsibleSidebar) -> Self {
        let state = state(&value);
        let common = UiCommonProps::default().width(UiDimension::Px(value.width.current));
        let node = UiNode::from_state(UiNodeKind::CollapsibleSidebar, value.label, value.state_id)
            .common(common)
            .interaction(state);
        if value.mode == SidebarMode::FloatingOverlay {
            node.common(UiCommonProps::default().z_index(UiZIndex::Value(FLOATING_OVERLAY_Z_INDEX)))
        } else {
            node
        }
    }
}

fn mode_for_index(index: usize) -> SidebarMode {
    match index {
        0 => SidebarMode::Expanded,
        1 => SidebarMode::IconOnly,
        2 => SidebarMode::Collapsed,
        _ => SidebarMode::FloatingOverlay,
    }
}

fn state(value: &CollapsibleSidebar) -> UiInteractionState {
    UiInteractionState {
        open: !matches!(value.mode, SidebarMode::Collapsed),
        selected_index: match value.mode {
            SidebarMode::Expanded => 0,
            SidebarMode::IconOnly => 1,
            SidebarMode::Collapsed => 2,
            SidebarMode::FloatingOverlay => FLOATING_OVERLAY_INDEX,
        },
        value: value.width.current.to_string(),
        ..UiInteractionState::default()
    }
}
