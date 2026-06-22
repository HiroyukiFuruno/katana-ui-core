use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiActionResult};
use katana_ui_core::molecule::{ChoiceItem, SideMenu};

const SIDE_MENU_ITEM_COUNT: usize = 5;
const EXTENSIONS_SIDE_MENU_INDEX: usize = 3;
const LAST_SIDE_MENU_INDEX: usize = 4;
const MAX_SIDE_MENU_SCROLL_OFFSET: usize = 3;
const SIDE_MENU_SCROLL_DELTA_Y: i32 = 48;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct SideMenuScreenState {
    pub(super) selected_index: Option<usize>,
    pub(super) focus_index: Option<usize>,
    pub(super) hovered: bool,
    pub(super) scroll_offset: usize,
    pub(super) hover_expansion: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SideMenuScreenAction {
    Select(usize),
    Focus,
    Hover,
    KeyboardNext,
    Scroll,
    HoverExpansion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SideMenuScreenUpdate {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) state: &'static str,
}

impl SideMenuScreenUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

impl SideMenuScreenState {
    pub(in crate::visual) fn apply(
        &mut self,
        action: SideMenuScreenAction,
    ) -> SideMenuScreenUpdate {
        match action {
            SideMenuScreenAction::Select(index) => self.select(index),
            SideMenuScreenAction::Focus => self.focus(),
            SideMenuScreenAction::Hover => self.hover(),
            SideMenuScreenAction::KeyboardNext => self.keyboard_next(),
            SideMenuScreenAction::Scroll => self.scroll(),
            SideMenuScreenAction::HoverExpansion => self.apply_hover_expansion(),
        }
    }

    fn select(&mut self, index: usize) -> SideMenuScreenUpdate {
        let bounded = index.min(LAST_SIDE_MENU_INDEX);
        let result = self.apply_core_selected(bounded, true);
        if result.handled {
            self.selected_index = Some(result.after.selected_index);
            self.focus_index = Some(result.after.selected_index);
        }
        SideMenuScreenUpdate::new(
            "side_menu_select",
            "select_box_selected",
            self.state_label(),
        )
    }

    fn focus(&mut self) -> SideMenuScreenUpdate {
        let result = self.apply_core_focus();
        if result.handled && result.after.focused {
            self.focus_index = Some(0);
        }
        SideMenuScreenUpdate::new("side_menu_focus", "focus", self.state_label())
    }

    fn hover(&mut self) -> SideMenuScreenUpdate {
        let result = self.apply_core_hover();
        self.hovered = result.handled && result.after.hovered;
        SideMenuScreenUpdate::new(
            "side_menu_hover",
            "hover_start",
            if self.hovered {
                "hover=true"
            } else {
                "hover=false"
            },
        )
    }

    fn keyboard_next(&mut self) -> SideMenuScreenUpdate {
        let next = match self.focus_index {
            Some(index) if index < LAST_SIDE_MENU_INDEX => index + 1,
            _ => 0,
        };
        let result = self.apply_core_selected(next, false);
        if result.handled {
            self.selected_index = Some(result.after.selected_index);
            self.focus_index = Some(result.after.selected_index);
        }
        SideMenuScreenUpdate::new(
            "side_menu_keyboard_next",
            "set_selected_index",
            self.state_label(),
        )
    }

    fn scroll(&mut self) -> SideMenuScreenUpdate {
        let result = self.apply_core_scroll();
        if result.handled {
            self.scroll_offset = (self.scroll_offset + 1).min(MAX_SIDE_MENU_SCROLL_OFFSET);
        }
        SideMenuScreenUpdate::new("side_menu_scroll", "scroll_by", self.scroll_label())
    }

    fn apply_hover_expansion(&mut self) -> SideMenuScreenUpdate {
        self.hover_expansion = self.core_side_menu().hover_expansion_model();
        SideMenuScreenUpdate::new(
            "side_menu_hover_expansion",
            "side_menu_option_applied",
            if self.hover_expansion {
                "hover_expansion=true"
            } else {
                "hover_expansion=false"
            },
        )
    }

    fn apply_core_selected(&self, index: usize, select_box_source: bool) -> UiActionResult {
        let mut side_menu = self.core_side_menu();
        let target = side_menu.state_id().clone();
        let action = if select_box_source {
            UiAction::select_box_selected(target, index)
        } else {
            UiAction::set_selected_index(target, index)
        };
        side_menu.apply_action(&action)
    }

    fn apply_core_focus(&self) -> UiActionResult {
        let mut side_menu = self.core_side_menu();
        let action = UiAction::focus(side_menu.state_id().clone());
        side_menu.apply_action(&action)
    }

    fn apply_core_hover(&self) -> UiActionResult {
        let mut side_menu = self.core_side_menu();
        let action = UiAction::hover(side_menu.state_id().clone(), true);
        side_menu.apply_action(&action)
    }

    fn apply_core_scroll(&self) -> UiActionResult {
        let mut side_menu = self.core_side_menu();
        let action = UiAction::scroll_by(side_menu.state_id().clone(), 0, SIDE_MENU_SCROLL_DELTA_Y);
        side_menu.apply_action(&action)
    }

    fn core_side_menu(&self) -> SideMenu {
        let side_menu = side_menu_items().into_iter().fold(
            SideMenu::new("Storybook side menu")
                .hover_expansion(true)
                .keyboard_navigation("arrow-down selects next route"),
            |side_menu, item| side_menu.item(item),
        );
        match self.selected_index {
            Some(index) => side_menu.selected_index(index),
            None => side_menu,
        }
    }

    fn state_label(&self) -> &'static str {
        match (self.selected_index, self.focus_index) {
            (Some(0), Some(0)) => "route=0 focus=0",
            (Some(1), Some(1)) => "route=1 focus=1",
            (Some(2), Some(2)) => "route=2 focus=2",
            (Some(EXTENSIONS_SIDE_MENU_INDEX), Some(EXTENSIONS_SIDE_MENU_INDEX)) => {
                "route=3 focus=3"
            }
            (Some(_), Some(_)) => "route=4 focus=4",
            (None, Some(0)) => "route=none focus=0",
            (None, Some(1)) => "route=none focus=1",
            (None, Some(2)) => "route=none focus=2",
            (None, Some(EXTENSIONS_SIDE_MENU_INDEX)) => "route=none focus=3",
            (None, Some(_)) => "route=none focus=4",
            (Some(0), None) => "route=0 focus=none",
            (Some(1), None) => "route=1 focus=none",
            (Some(2), None) => "route=2 focus=none",
            (Some(EXTENSIONS_SIDE_MENU_INDEX), None) => "route=3 focus=none",
            (Some(_), None) => "route=4 focus=none",
            (None, None) => "route=none focus=none",
        }
    }

    fn scroll_label(&self) -> &'static str {
        match self.scroll_offset {
            0 => "scroll=0",
            1 => "scroll=1",
            2 => "scroll=2",
            _ => "scroll=3",
        }
    }
}

fn side_menu_items() -> [ChoiceItem; SIDE_MENU_ITEM_COUNT] {
    [
        ChoiceItem::new("files", "Files"),
        ChoiceItem::new("settings", "Settings"),
        ChoiceItem::new("search", "Search"),
        ChoiceItem::new("extensions", "Extensions"),
        ChoiceItem::new("accounts", "Accounts"),
    ]
}
