use super::{Chip, ChipAction, ChipEvent, ChipKeyboardInput};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};

impl Chip {
    #[must_use]
    pub fn apply_action(&mut self, action: ChipAction) -> Vec<ChipEvent> {
        let event = match action {
            ChipAction::Press if self.can_press() => Some(ChipEvent::ChipPressed {
                id: self.state_id.clone(),
            }),
            ChipAction::Dismiss if self.can_dismiss() => Some(ChipEvent::ChipDismissed {
                id: self.state_id.clone(),
            }),
            ChipAction::Keyboard(input) if self.can_keyboard_dismiss(&input) => {
                Some(ChipEvent::ChipDismissed {
                    id: self.state_id.clone(),
                })
            }
            ChipAction::Focus if !self.disabled => self.focus_event(true),
            ChipAction::Blur if !self.disabled => self.focus_event(false),
            _ => None,
        };
        event.map_or_else(Vec::new, |it| self.record_event(it))
    }

    fn can_press(&self) -> bool {
        self.interactive && !self.disabled
    }

    fn can_dismiss(&self) -> bool {
        self.dismissible && !self.disabled
    }

    fn can_keyboard_dismiss(&self, input: &ChipKeyboardInput) -> bool {
        self.focused
            && self.can_dismiss()
            && matches!(
                input,
                ChipKeyboardInput::Backspace | ChipKeyboardInput::Delete
            )
    }

    fn focus_event(&mut self, focused: bool) -> Option<ChipEvent> {
        if self.focused == focused {
            return None;
        }
        self.focused = focused;
        let id = self.state_id.clone();
        Some(if focused {
            ChipEvent::Focus { id }
        } else {
            ChipEvent::Blur { id }
        })
    }

    fn record_event(&mut self, event: ChipEvent) -> Vec<ChipEvent> {
        self.callback_log.push(event.clone());
        vec![event]
    }
}

impl ComponentAction for Chip {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.interaction_state();
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        let events = match action {
            UiAction::Press { .. } => self.apply_action(ChipAction::Press),
            UiAction::Dismiss { .. } => self.apply_action(ChipAction::Dismiss),
            UiAction::SetFocus { focused, .. } if *focused => self.apply_action(ChipAction::Focus),
            UiAction::SetFocus { .. } => self.apply_action(ChipAction::Blur),
            _ => Vec::new(),
        };
        if events.is_empty() {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        UiActionResult::handled(
            self.state_id.clone(),
            action,
            before,
            self.interaction_state(),
        )
    }
}
