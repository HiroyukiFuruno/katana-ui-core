#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuButtonTransition {
    Opened,
    Closed,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuButtonInteractionState {
    open: bool,
}

impl MenuButtonInteractionState {
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

    pub fn trigger_press(&mut self) -> MenuButtonTransition {
        if self.open {
            self.open = false;
            MenuButtonTransition::Closed
        } else {
            self.open = true;
            MenuButtonTransition::Opened
        }
    }

    pub fn outside_pointer(&mut self) -> MenuButtonTransition {
        self.close_if_open()
    }

    pub fn escape_key(&mut self) -> MenuButtonTransition {
        self.close_if_open()
    }

    pub fn select_item(&mut self) -> MenuButtonTransition {
        self.close_if_open()
    }

    fn close_if_open(&mut self) -> MenuButtonTransition {
        if self.open {
            self.open = false;
            MenuButtonTransition::Closed
        } else {
            MenuButtonTransition::Unchanged
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MenuButtonInteractionState, MenuButtonTransition};

    #[test]
    fn trigger_press_toggles_open_state() {
        let mut state = MenuButtonInteractionState::closed();

        assert_eq!(state.trigger_press(), MenuButtonTransition::Opened);
        assert!(state.is_open());
        assert_eq!(state.trigger_press(), MenuButtonTransition::Closed);
        assert!(!state.is_open());
    }
}
