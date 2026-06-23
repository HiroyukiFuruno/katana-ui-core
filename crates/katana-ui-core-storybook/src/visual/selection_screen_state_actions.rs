use super::selection_screen_state::{
    SelectionScreenAction, SelectionScreenState, SelectionScreenUpdate,
};
use super::selection_screen_state_labels::{
    select_read_state, select_scroll_state, select_state, selection_list_state,
};

#[path = "selection_screen_state_combo_actions.rs"]
mod combo_actions;

const FOURTH_LIST_INDEX: usize = 3;
const MAX_SELECTION_SCROLL_OFFSET: usize = 3;

impl SelectionScreenState {
    pub(super) fn apply(&mut self, action: SelectionScreenAction) -> SelectionScreenUpdate {
        match action {
            SelectionScreenAction::SelectStateRead => self.read_select_state(),
            SelectionScreenAction::SelectOpen => self.open_select(),
            SelectionScreenAction::SelectClose => self.close_select(),
            SelectionScreenAction::SelectOption(index) => self.select_option(index),
            SelectionScreenAction::SelectFocus => self.focus_select(),
            SelectionScreenAction::SelectHover => self.hover_select(),
            SelectionScreenAction::SelectKeyboardSelect => self.keyboard_select(),
            SelectionScreenAction::SelectScroll => self.scroll_select(),
            SelectionScreenAction::SelectReset => self.reset_select(),
            SelectionScreenAction::ComboStateRead => combo_actions::read_combo_state(self),
            SelectionScreenAction::ComboFilter => combo_actions::filter_combo(self),
            SelectionScreenAction::ComboOption(index) => {
                combo_actions::select_combo_option(self, index)
            }
            SelectionScreenAction::ComboFocus => combo_actions::focus_combo(self),
            SelectionScreenAction::ComboHover => combo_actions::hover_combo(self),
            SelectionScreenAction::ComboKeyboardSelect => {
                combo_actions::keyboard_select_combo(self)
            }
            SelectionScreenAction::ComboReset => combo_actions::reset_combo(self),
            SelectionScreenAction::SelectionListStateRead => self.read_selection_list_state(),
            SelectionScreenAction::SelectionListSelectRow(index) => {
                self.select_selection_list_row(index)
            }
            SelectionScreenAction::SelectionListMultiToggle(index) => {
                self.toggle_selection_list_multi(index)
            }
            SelectionScreenAction::SelectionListFocus => self.focus_selection_list(),
            SelectionScreenAction::SelectionListHover => self.hover_selection_list(),
            SelectionScreenAction::SelectionListKeyboardNext => self.selection_list_keyboard_next(),
            SelectionScreenAction::SelectionListScroll => self.scroll_selection_list(),
            SelectionScreenAction::SelectionListReset => self.reset_selection_list(),
            SelectionScreenAction::SelectionListToggle(index) => {
                self.select_selection_list_row(index)
            }
        }
    }

    fn read_select_state(&mut self) -> SelectionScreenUpdate {
        SelectionScreenUpdate::new(
            "select_state_read",
            "select_state_read",
            select_read_state(self.select_open, self.select_selected_index),
        )
    }

    fn open_select(&mut self) -> SelectionScreenUpdate {
        let result = self.apply_core_select_open(true);
        self.select_open = result.handled && result.after.open;
        SelectionScreenUpdate::new("select_open", "open", "open=true")
    }

    fn close_select(&mut self) -> SelectionScreenUpdate {
        let result = self.apply_core_select_open(false);
        self.select_open = result.handled && result.after.open;
        SelectionScreenUpdate::new("select_close", "close", "open=false")
    }

    fn select_option(&mut self, index: usize) -> SelectionScreenUpdate {
        let result = self.apply_core_select_selected(index);
        if result.handled {
            self.select_open = result.after.open;
            self.select_selected_index = Some(result.after.selected_index);
        }
        SelectionScreenUpdate::new("select_option", "select_box_selected", select_state(index))
    }

    fn focus_select(&mut self) -> SelectionScreenUpdate {
        let result = self.apply_core_select_focus();
        self.select_focused = result.handled && result.after.focused;
        SelectionScreenUpdate::new(
            "select_focus",
            "focus",
            if self.select_focused {
                "focus=true"
            } else {
                "focus=false"
            },
        )
    }

    fn hover_select(&mut self) -> SelectionScreenUpdate {
        let result = self.apply_core_select_hover();
        self.select_hovered = result.handled && result.after.hovered;
        SelectionScreenUpdate::new(
            "select_hover",
            "hover_start",
            if self.select_hovered {
                "hover=true"
            } else {
                "hover=false"
            },
        )
    }

    fn keyboard_select(&mut self) -> SelectionScreenUpdate {
        let result = self.apply_core_select_selected(1);
        if result.handled {
            self.select_open = result.after.open;
            self.select_selected_index = Some(result.after.selected_index);
        }
        SelectionScreenUpdate::new(
            "select_keyboard_select",
            "select_box_selected",
            select_state(1),
        )
    }

    fn scroll_select(&mut self) -> SelectionScreenUpdate {
        self.select_open = true;
        self.select_scroll_offset =
            (self.select_scroll_offset + 1).min(MAX_SELECTION_SCROLL_OFFSET);
        SelectionScreenUpdate::new(
            "select_option_scroll",
            "select_options_scrolled",
            select_scroll_state(self.select_scroll_offset),
        )
    }

    fn reset_select(&mut self) -> SelectionScreenUpdate {
        self.select_open = false;
        self.select_selected_index = None;
        SelectionScreenUpdate::new("select_reset", "select_reset", "selected=none")
    }

    fn read_selection_list_state(&mut self) -> SelectionScreenUpdate {
        SelectionScreenUpdate::new(
            "selection_list_state_read",
            "selection_list_state_read",
            self.selection_list_state_label(),
        )
    }

    fn select_selection_list_row(&mut self, index: usize) -> SelectionScreenUpdate {
        self.apply_selection_list_single(index, true);
        SelectionScreenUpdate::new(
            "selection_list_select_row",
            "select_box_selected",
            self.selection_list_state_label(),
        )
    }

    fn toggle_selection_list_multi(&mut self, index: usize) -> SelectionScreenUpdate {
        let bounded = index.min(FOURTH_LIST_INDEX);
        self.apply_selection_list_single(bounded, true);
        self.selection_list_multi_mask ^= 1u8 << bounded;
        SelectionScreenUpdate::new(
            "selection_list_multi_toggle",
            "select_box_selected",
            self.selection_list_state_label(),
        )
    }

    fn focus_selection_list(&mut self) -> SelectionScreenUpdate {
        let result = self.apply_core_selection_list_focus();
        if result.handled && result.after.focused {
            self.selection_list_focus_index = Some(0);
        }
        SelectionScreenUpdate::new(
            "selection_list_focus",
            "focus",
            self.selection_list_state_label(),
        )
    }

    fn hover_selection_list(&mut self) -> SelectionScreenUpdate {
        let result = self.apply_core_selection_list_hover();
        self.selection_list_hovered = result.handled && result.after.hovered;
        SelectionScreenUpdate::new(
            "selection_list_hover",
            "hover_start",
            if self.selection_list_hovered {
                "hover=true"
            } else {
                "hover=false"
            },
        )
    }

    fn selection_list_keyboard_next(&mut self) -> SelectionScreenUpdate {
        let next = match self.selection_list_focus_index {
            Some(index) if index < FOURTH_LIST_INDEX => index + 1,
            _ => 0,
        };
        self.apply_selection_list_single(next, false);
        SelectionScreenUpdate::new(
            "selection_list_keyboard_next",
            "set_selected_index",
            self.selection_list_state_label(),
        )
    }

    fn scroll_selection_list(&mut self) -> SelectionScreenUpdate {
        let result = self.apply_core_selection_list_scroll();
        if result.handled {
            self.selection_list_scroll_offset =
                (self.selection_list_scroll_offset + 1).min(MAX_SELECTION_SCROLL_OFFSET);
        }
        SelectionScreenUpdate::new(
            "selection_list_scroll",
            "scroll_by",
            match self.selection_list_scroll_offset {
                0 => "scroll=0",
                1 => "scroll=1",
                2 => "scroll=2",
                _ => "scroll=3",
            },
        )
    }

    fn reset_selection_list(&mut self) -> SelectionScreenUpdate {
        self.selection_list_selected_index = None;
        self.selection_list_multi_mask = 0;
        self.selection_list_focus_index = None;
        SelectionScreenUpdate::new(
            "selection_list_reset",
            "selection_list_reset",
            "single=none multi=none focus=none",
        )
    }

    fn apply_selection_list_single(&mut self, index: usize, select_box_source: bool) {
        let result = self.apply_core_selection_list_selected(index, select_box_source);
        if result.handled {
            self.selection_list_selected_index = Some(result.after.selected_index);
            self.selection_list_focus_index = Some(result.after.selected_index);
        }
    }

    fn selection_list_state_label(&self) -> &'static str {
        selection_list_state(
            self.selection_list_selected_index,
            self.selection_list_multi_mask,
            self.selection_list_focus_index,
        )
    }
}
