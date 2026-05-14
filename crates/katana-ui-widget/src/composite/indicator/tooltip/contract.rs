#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipTransition {
    Opened,
    Closed,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooltipInteractionState {
    visible: bool,
    hover_ready: bool,
    focus_ready: bool,
}

impl TooltipInteractionState {
    #[must_use]
    pub const fn hidden() -> Self {
        Self {
            visible: false,
            hover_ready: false,
            focus_ready: false,
        }
    }

    #[must_use]
    pub const fn visible() -> Self {
        Self {
            visible: true,
            hover_ready: true,
            focus_ready: false,
        }
    }

    #[must_use]
    pub const fn is_visible(self) -> bool {
        self.visible
    }

    pub fn hover_ready(&mut self) -> TooltipTransition {
        self.hover_ready = true;
        self.open()
    }

    pub fn focus_gained(&mut self) -> TooltipTransition {
        self.focus_ready = true;
        self.open()
    }

    pub fn pointer_left(&mut self, dismiss_enabled: bool) -> TooltipTransition {
        self.hover_ready = false;
        if dismiss_enabled && !self.focus_ready {
            self.close()
        } else {
            TooltipTransition::Unchanged
        }
    }

    pub fn focus_lost(&mut self, dismiss_enabled: bool) -> TooltipTransition {
        self.focus_ready = false;
        if dismiss_enabled && !self.hover_ready {
            self.close()
        } else {
            TooltipTransition::Unchanged
        }
    }

    pub fn escape_key(&mut self) -> TooltipTransition {
        self.close()
    }

    fn open(&mut self) -> TooltipTransition {
        if self.visible {
            return TooltipTransition::Unchanged;
        }

        self.visible = true;
        TooltipTransition::Opened
    }

    fn close(&mut self) -> TooltipTransition {
        if !self.visible {
            return TooltipTransition::Unchanged;
        }

        self.visible = false;
        TooltipTransition::Closed
    }
}

#[cfg(test)]
mod tests {
    use super::{TooltipInteractionState, TooltipTransition};

    #[test]
    fn hover_and_focus_open_tooltip() {
        let mut hover = TooltipInteractionState::hidden();
        let mut focus = TooltipInteractionState::hidden();

        assert_eq!(hover.hover_ready(), TooltipTransition::Opened);
        assert!(hover.is_visible());
        assert_eq!(focus.focus_gained(), TooltipTransition::Opened);
        assert!(focus.is_visible());
    }

    #[test]
    fn pointer_leave_and_focus_loss_close_when_enabled() {
        let mut pointer = TooltipInteractionState::visible();
        let mut focus = TooltipInteractionState::hidden();
        let _ = focus.focus_gained();

        assert_eq!(pointer.pointer_left(true), TooltipTransition::Closed);
        assert!(!pointer.is_visible());
        assert_eq!(focus.focus_lost(true), TooltipTransition::Closed);
        assert!(!focus.is_visible());
    }

    #[test]
    fn escape_closes_visible_tooltip() {
        let mut state = TooltipInteractionState::visible();

        assert_eq!(state.escape_key(), TooltipTransition::Closed);
        assert!(!state.is_visible());
    }
}
