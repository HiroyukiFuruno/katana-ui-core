use super::selection_screen_state::SelectionScreenState;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiActionResult};
use katana_ui_core::molecule::{ChoiceItem, ComboBox, MenuButton, SelectBox, SelectionList};

const SELECT_ITEM_COUNT: usize = 4;
const COMBO_ITEM_COUNT: usize = 2;
const LIST_ITEM_COUNT: usize = 4;
const SELECTION_SCROLL_DELTA_Y: i32 = 48;

pub(super) struct ComboFilterResult {
    pub(super) opened: bool,
    pub(super) filtered: bool,
}

impl SelectionScreenState {
    pub(super) fn apply_core_select_open(&self, open: bool) -> UiActionResult {
        let mut select = self.core_select_box();
        let action = UiAction::set_open(select.state_id().clone(), open);
        select.apply_action(&action)
    }

    pub(super) fn apply_core_select_selected(&self, index: usize) -> UiActionResult {
        let mut select = self.core_select_box().open(self.select_open);
        let action = UiAction::select_box_selected(select.state_id().clone(), index);
        select.apply_action(&action)
    }

    pub(super) fn apply_core_select_focus(&self) -> UiActionResult {
        let mut select = self.core_select_box().open(self.select_open);
        let action = UiAction::focus(select.state_id().clone());
        select.apply_action(&action)
    }

    pub(super) fn apply_core_select_hover(&self) -> UiActionResult {
        let mut select = self.core_select_box().open(self.select_open);
        let action = UiAction::hover(select.state_id().clone(), true);
        select.apply_action(&action)
    }

    pub(super) fn apply_core_menu_button_open(&self, open: bool) -> UiActionResult {
        let mut menu = self.core_menu_button();
        let action = UiAction::set_open(menu.state_id().clone(), open);
        menu.apply_action(&action)
    }

    pub(super) fn apply_core_menu_button_selected(&self, index: usize) -> UiActionResult {
        let mut menu = self.core_menu_button().open(self.select_open);
        let action = UiAction::select_box_selected(menu.state_id().clone(), index);
        menu.apply_action(&action)
    }

    pub(super) fn apply_core_combo_filter(&self, value: &str) -> ComboFilterResult {
        let mut combo = self.core_combo_box().open(true);
        let target = combo.state_id().clone();
        let open = combo.apply_action(&UiAction::set_open(target.clone(), true));
        let input = combo.apply_action(&UiAction::input_value(target, value));
        ComboFilterResult {
            opened: open.handled && open.after.open,
            filtered: input.handled && input.after.value == value,
        }
    }

    pub(super) fn apply_core_combo_selected(&self, index: usize) -> UiActionResult {
        let mut combo = self.core_combo_box().open(self.combo_open);
        let action = UiAction::select_box_selected(combo.state_id().clone(), index);
        combo.apply_action(&action)
    }

    pub(super) fn apply_core_combo_focus(&self) -> UiActionResult {
        let mut combo = self.core_combo_box().open(self.combo_open);
        let action = UiAction::focus(combo.state_id().clone());
        combo.apply_action(&action)
    }

    pub(super) fn apply_core_combo_hover(&self) -> UiActionResult {
        let mut combo = self.core_combo_box().open(self.combo_open);
        let action = UiAction::hover(combo.state_id().clone(), true);
        combo.apply_action(&action)
    }

    pub(super) fn apply_core_selection_list_selected(
        &self,
        index: usize,
        select_box_source: bool,
    ) -> UiActionResult {
        let mut list = self.core_selection_list();
        let target = list.state_id().clone();
        let action = if select_box_source {
            UiAction::select_box_selected(target, index)
        } else {
            UiAction::set_selected_index(target, index)
        };
        list.apply_action(&action)
    }

    pub(super) fn apply_core_selection_list_focus(&self) -> UiActionResult {
        let mut list = self.core_selection_list();
        let action = UiAction::focus(list.state_id().clone());
        list.apply_action(&action)
    }

    pub(super) fn apply_core_selection_list_hover(&self) -> UiActionResult {
        let mut list = self.core_selection_list();
        let action = UiAction::hover(list.state_id().clone(), true);
        list.apply_action(&action)
    }

    pub(super) fn apply_core_selection_list_scroll(&self) -> UiActionResult {
        let mut list = self.core_selection_list();
        let action = UiAction::scroll_by(list.state_id().clone(), 0, SELECTION_SCROLL_DELTA_Y);
        list.apply_action(&action)
    }

    fn core_select_box(&self) -> SelectBox {
        let select = select_items().into_iter().fold(
            SelectBox::new("Storybook select")
                .open(self.select_open)
                .long_list(true)
                .keyboard_navigation("arrow-down selects next option"),
            |select, item| select.item(item),
        );
        match self.select_selected_index {
            Some(index) => select.selected_index(index),
            None => select,
        }
    }

    fn core_menu_button(&self) -> MenuButton {
        let menu = menu_button_items().into_iter().fold(
            MenuButton::new("Storybook menu button").open(self.select_open),
            |menu, item| menu.item(item),
        );
        match self.select_selected_index {
            Some(index) => menu.selected_index(index),
            None => menu,
        }
    }

    fn core_combo_box(&self) -> ComboBox {
        let combo = combo_items().into_iter().fold(
            ComboBox::new("Storybook combo").open(self.combo_open),
            |combo, item| combo.item(item),
        );
        match self.combo_selected_index {
            Some(index) => combo.selected_index(index),
            None => combo,
        }
    }

    fn core_selection_list(&self) -> SelectionList {
        let list = list_items().into_iter().fold(
            SelectionList::new("Storybook selection list"),
            |list, item| list.item(item),
        );
        match self.selection_list_selected_index {
            Some(index) => list.selected_index(index),
            None => list,
        }
    }
}

fn select_items() -> [ChoiceItem; SELECT_ITEM_COUNT] {
    [
        ChoiceItem::new("none", "None"),
        ChoiceItem::new("light", "Light"),
        ChoiceItem::new("dark", "Dark"),
        ChoiceItem::new("system", "System"),
    ]
}

fn menu_button_items() -> [ChoiceItem; COMBO_ITEM_COUNT] {
    [
        ChoiceItem::new("new-file", "New file"),
        ChoiceItem::new("rename", "Rename"),
    ]
}

fn combo_items() -> [ChoiceItem; COMBO_ITEM_COUNT] {
    [ChoiceItem::new("one", "One"), ChoiceItem::new("two", "Two")]
}

fn list_items() -> [ChoiceItem; LIST_ITEM_COUNT] {
    [
        ChoiceItem::new("zero", "Zero"),
        ChoiceItem::new("one", "One"),
        ChoiceItem::new("two", "Two"),
        ChoiceItem::new("three", "Three"),
    ]
}
