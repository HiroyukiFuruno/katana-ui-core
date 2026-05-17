#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverTransition {
    Opened,
    Closed,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PopoverInteractionState {
    open: bool,
}

impl PopoverInteractionState {
    #[must_use]
    pub const fn closed() -> Self {
        Self { open: false }
    }

    #[must_use]
    pub const fn opened() -> Self {
        Self { open: true }
    }

    #[must_use]
    pub const fn is_open(self) -> bool {
        self.open
    }

    pub fn trigger_press(&mut self) -> PopoverTransition {
        self.open = !self.open;
        if self.open {
            PopoverTransition::Opened
        } else {
            PopoverTransition::Closed
        }
    }

    pub fn outside_pointer(&mut self, dismiss_enabled: bool) -> PopoverTransition {
        self.close_when(dismiss_enabled)
    }

    pub fn escape_key(&mut self, dismiss_enabled: bool) -> PopoverTransition {
        self.close_when(dismiss_enabled)
    }

    fn close_when(&mut self, dismiss_enabled: bool) -> PopoverTransition {
        if !self.open || !dismiss_enabled {
            return PopoverTransition::Unchanged;
        }

        self.open = false;
        PopoverTransition::Closed
    }
}

#[cfg(test)]
mod tests {
    use super::{PopoverInteractionState, PopoverTransition};

    #[test]
    fn trigger_press_toggles_open_state() {
        let mut state = PopoverInteractionState::closed();

        assert_eq!(state.trigger_press(), PopoverTransition::Opened);
        assert!(state.is_open());
        assert_eq!(state.trigger_press(), PopoverTransition::Closed);
        assert!(!state.is_open());
    }

    #[test]
    fn outside_pointer_closes_only_when_enabled() {
        let mut state = PopoverInteractionState::opened();

        assert_eq!(state.outside_pointer(false), PopoverTransition::Unchanged);
        assert!(state.is_open());
        assert_eq!(state.outside_pointer(true), PopoverTransition::Closed);
        assert!(!state.is_open());
    }

    #[test]
    fn escape_key_closes_only_when_enabled() {
        let mut state = PopoverInteractionState::opened();

        assert_eq!(state.escape_key(false), PopoverTransition::Unchanged);
        assert!(state.is_open());
        assert_eq!(state.escape_key(true), PopoverTransition::Closed);
        assert!(!state.is_open());
    }
}
