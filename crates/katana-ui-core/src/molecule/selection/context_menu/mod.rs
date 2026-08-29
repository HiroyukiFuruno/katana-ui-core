mod actions;
mod events;
mod item_state;
mod keyboard;
mod options;
mod placement;
mod state;
#[cfg(test)]
mod tests;
mod types;

use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiNode, UiNodeKind};
pub use actions::ContextMenuAction;
pub use events::{ContextMenuCloseReason, ContextMenuEvent};
pub use keyboard::{
    ContextMenuKeyboardInput, ContextMenuKeyboardIntent, ContextMenuKeyboardNavigator,
    ContextMenuTypeAheadBuffer,
};
pub use placement::{
    ContextMenuPlacementResolver, ContextMenuPlacementResult, ContextMenuSize, ContextMenuViewport,
};
use serde::{Deserialize, Serialize};
pub use types::{
    ContextMenuAnchor, ContextMenuDividerTone, ContextMenuItem, ContextMenuItemKind,
    ContextMenuPlacement, ContextMenuRect,
};

use item_state::path_for_command;
use state::ContextMenuState;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextMenu {
    label: String,
    state: ContextMenuState,
    props: crate::render_model::UiContextMenuProps,
    children: Vec<UiNode>,
}

impl ContextMenu {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: ContextMenuState::new(),
            props: crate::render_model::UiContextMenuProps::default(),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn item(mut self, item: ContextMenuItem) -> Self {
        self.props.items.push(item);
        self.state.item_count = self.props.items.len();
        self.state.sync_submenu_state_ids(&self.props.items);
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn apply_context_action(&mut self, action: &ContextMenuAction) -> ContextMenuEvent {
        let events = self.state.apply(action, &mut self.props);
        let event = events.first().cloned().unwrap_or(ContextMenuEvent::Closed {
            reason: ContextMenuCloseReason::OutsideClick,
        });
        self.state.callback_log.extend(events);
        event
    }

    /// Applies one action while preserving every typed event emitted by the core model.
    pub fn apply_context_action_events(
        &mut self,
        action: &ContextMenuAction,
    ) -> Vec<ContextMenuEvent> {
        let event_count = self.state.callback_log.len();
        self.apply_context_action(action);
        self.state.callback_log[event_count..].to_vec()
    }

    /// Replaces controlled presentation without resetting retained interaction state.
    pub fn synchronize_items(&mut self, items: Vec<ContextMenuItem>) {
        self.props.items = items;
        self.state.item_count = self.props.items.len();
        self.state.sync_submenu_state_ids(&self.props.items);
    }

    #[must_use]
    pub fn presentation_items(&self) -> &[ContextMenuItem] {
        &self.props.items
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.state.open
    }

    #[must_use]
    pub fn current_highlighted_path(&self) -> &[usize] {
        &self.props.highlighted_path
    }

    #[must_use]
    pub fn state_id(&self) -> &crate::render_model::UiStateId {
        &self.state.state_id
    }

    #[must_use]
    pub fn submenu_state_ids(&self) -> &[crate::render_model::UiStateId] {
        &self.state.submenu_state_ids
    }

    #[must_use]
    pub fn callback_log(&self) -> &[ContextMenuEvent] {
        &self.state.callback_log
    }
}

impl ComponentAction for ContextMenu {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.state.interaction();
        if action.target() != &self.state.state_id {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        match action {
            UiAction::Press { .. } => {
                self.apply_context_action(&ContextMenuAction::Open {
                    anchor: self.props.anchor.clone(),
                });
            }
            UiAction::SetOpen { open, .. } => {
                let context_action = if *open {
                    ContextMenuAction::Open {
                        anchor: self.props.anchor.clone(),
                    }
                } else {
                    ContextMenuAction::Close {
                        reason: ContextMenuCloseReason::OutsideClick,
                    }
                };
                self.apply_context_action(&context_action);
            }
            UiAction::SetSelectedIndex { selected_index, .. } => {
                self.apply_context_action(&ContextMenuAction::Highlight {
                    path: vec![*selected_index],
                });
            }
            UiAction::Dismiss { .. } => {
                self.apply_context_action(&ContextMenuAction::Close {
                    reason: ContextMenuCloseReason::Escape,
                });
            }
            UiAction::InvokeCallback { callback, .. } => {
                let Some(path) = path_for_command(&self.props, callback) else {
                    return UiActionResult::ignored(self.state.state_id.clone(), before);
                };
                self.apply_context_action(&ContextMenuAction::Activate { path });
            }
            _ => {}
        }
        UiActionResult::handled(
            self.state.state_id.clone(),
            action,
            before,
            self.state.interaction(),
        )
    }
}

impl From<ContextMenu> for UiNode {
    fn from(value: ContextMenu) -> Self {
        let mut node = UiNode::from_state(
            UiNodeKind::ContextMenu,
            value.label,
            value.state.state_id.clone(),
        )
        .interaction(value.state.interaction())
        .context_menu(value.props);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}
