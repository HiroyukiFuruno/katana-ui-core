use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiSize, UiStateId, UiVariant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleBarStyle {
    Native,
    Unified,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowControlsPosition {
    Leading,
    Trailing,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowControlKind {
    Close,
    Minimize,
    Maximize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleBarEvent {
    None,
    ControlPressed(WindowControlKind),
    FullscreenVisibilityChanged(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TitleBar {
    label: String,
    state_id: UiStateId,
    style: TitleBarStyle,
    controls_position: WindowControlsPosition,
    height: UiSize,
    fullscreen: bool,
    last_event: TitleBarEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowChrome {
    label: String,
    state_id: UiStateId,
    title_bar: TitleBar,
}

impl TitleBar {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::TitleBar),
            style: TitleBarStyle::Native,
            controls_position: WindowControlsPosition::Leading,
            height: UiSize::Medium,
            fullscreen: false,
            last_event: TitleBarEvent::None,
        }
    }

    #[must_use]
    pub fn options(
        mut self,
        style: TitleBarStyle,
        controls_position: WindowControlsPosition,
        height: UiSize,
    ) -> Self {
        self.style = style;
        self.controls_position = controls_position;
        self.height = height;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn last_event(&self) -> &TitleBarEvent {
        &self.last_event
    }
}

impl ComponentAction for TitleBar {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = state(self);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        match action {
            UiAction::SetSelectedIndex { selected_index, .. } => {
                self.last_event = TitleBarEvent::ControlPressed(match selected_index {
                    0 => WindowControlKind::Close,
                    1 => WindowControlKind::Minimize,
                    _ => WindowControlKind::Maximize,
                });
            }
            UiAction::SetHover { hovered, .. } if self.fullscreen => {
                self.last_event = TitleBarEvent::FullscreenVisibilityChanged(*hovered);
            }
            UiAction::SetOpen { open, .. } => self.fullscreen = *open,
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        }
        UiActionResult::handled(self.state_id.clone(), action, before, state(self))
    }
}

impl From<TitleBar> for UiNode {
    fn from(value: TitleBar) -> Self {
        let state = state(&value);
        UiNode::from_state(UiNodeKind::TitleBar, value.label, value.state_id)
            .size(value.height)
            .variant(match value.style {
                TitleBarStyle::Native => UiVariant::Plain,
                TitleBarStyle::Unified => UiVariant::Filled,
                TitleBarStyle::Custom => UiVariant::Outline,
            })
            .style_class(format!("{:?}", value.controls_position))
            .interaction(state)
    }
}

impl WindowChrome {
    #[must_use]
    pub fn new(label: impl Into<String>, title_bar: TitleBar) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::WindowChrome),
            title_bar,
        }
    }
}

impl From<WindowChrome> for UiNode {
    fn from(value: WindowChrome) -> Self {
        UiNode::from_state(UiNodeKind::WindowChrome, value.label, value.state_id)
            .child(value.title_bar)
    }
}

fn state(value: &TitleBar) -> UiInteractionState {
    UiInteractionState {
        open: !value.fullscreen,
        selected_index: match value.controls_position {
            WindowControlsPosition::Leading => 0,
            WindowControlsPosition::Trailing => 1,
            WindowControlsPosition::Hidden => 2,
        },
        ..UiInteractionState::default()
    }
}
