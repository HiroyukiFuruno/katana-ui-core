#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct ListScreenState {
    pub(super) selected_index: Option<usize>,
    pub(super) focused_index: Option<usize>,
    pub(super) scrolled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ListScreenUpdate {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) state: &'static str,
}

impl ListScreenUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

impl ListScreenState {
    pub(super) fn select_row(&mut self, index: usize) -> ListScreenUpdate {
        self.selected_index = Some(index);
        self.focused_index = Some(index);
        ListScreenUpdate::new("list_select", "selection_changed", "selected=1")
    }

    pub(super) fn focus_row(&mut self, index: usize) -> ListScreenUpdate {
        self.focused_index = Some(index);
        ListScreenUpdate::new("list_focus", "list_focused", "focused=1")
    }

    pub(super) fn keyboard_next(&mut self) -> ListScreenUpdate {
        let next = match self.focused_index {
            Some(index) if index < 2 => index + 1,
            _ => 1,
        };
        self.selected_index = Some(next);
        self.focused_index = Some(next);
        ListScreenUpdate::new("list_keyboard_next", "selection_changed", "selected=2")
    }

    pub(super) fn scroll_virtual_range(&mut self) -> ListScreenUpdate {
        self.scrolled = true;
        ListScreenUpdate::new(
            "list_scroll",
            "list_virtual_range_changed",
            "virtual=48/200",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ListScreenState;

    #[test]
    fn keyboard_next_starts_and_wraps_at_the_second_row() {
        let mut state = ListScreenState::default();
        state.keyboard_next();
        assert_eq!(Some(1), state.focused_index);

        state.focused_index = Some(2);
        state.keyboard_next();
        assert_eq!(Some(1), state.focused_index);
    }
}
