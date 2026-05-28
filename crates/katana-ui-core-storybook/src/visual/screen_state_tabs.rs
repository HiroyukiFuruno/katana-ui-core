pub(super) use super::screen_state_tabs_types::{
    TabsScreenAction, TabsScreenState, TabsScreenTab, TabsScreenUpdate,
};
use super::screen_state_tabs_types::{TabsScreenGroup, tabs_update};

const ADD_CLOSE_PRESET_INDEX: usize = 1;
const PIN_PRESET_INDEX: usize = 2;
const MOVE_PRESET_INDEX: usize = 3;
const GROUP_PRESET_INDEX: usize = 4;
const OVERFLOW_PRESET_INDEX: usize = 5;

impl TabsScreenState {
    pub(super) fn apply(&mut self, action: TabsScreenAction) -> TabsScreenUpdate {
        match action {
            TabsScreenAction::AddTab => self.add_tab(),
            TabsScreenAction::CloseActive => self.close_active(),
            TabsScreenAction::TogglePinActive => self.toggle_pin_active(),
            TabsScreenAction::MoveActiveRight => self.move_active_right(),
            TabsScreenAction::GroupActive => self.group_active(),
            TabsScreenAction::ToggleOverflow => self.toggle_overflow(),
        }
    }

    pub(super) fn for_preset(preset_index: usize) -> Self {
        let mut state = Self::default();
        match preset_index {
            ADD_CLOSE_PRESET_INDEX => {
                let _ = state.apply(TabsScreenAction::AddTab);
            }
            PIN_PRESET_INDEX => state.active_tab_id = "readme.md".to_string(),
            MOVE_PRESET_INDEX => {
                let _ = state.apply(TabsScreenAction::MoveActiveRight);
            }
            GROUP_PRESET_INDEX => {
                let _ = state.apply(TabsScreenAction::GroupActive);
            }
            OVERFLOW_PRESET_INDEX => {
                state.add_many_for_overflow();
                state.overflow_open = true;
            }
            _ => {}
        }
        state
    }

    pub(super) fn state_label(&self) -> &'static str {
        if self.overflow_open {
            return "overflow=menu";
        }
        if self.active_tab().is_some_and(|tab| tab.pinned) {
            return "pinned=true";
        }
        if self
            .active_tab()
            .and_then(|tab| tab.group_id.as_ref())
            .is_some()
        {
            return "group=Docs";
        }
        if self.tabs.iter().any(|tab| tab.id == "notes.md") {
            return "tabs=6 active=notes.md";
        }
        if self.is_scratch_after_terminal() {
            return "order=changed";
        }
        "active=scratch.md tabs=5"
    }

    pub(super) fn active_tab(&self) -> Option<&TabsScreenTab> {
        self.tabs.iter().find(|tab| tab.id == self.active_tab_id)
    }

    fn add_tab(&mut self) -> TabsScreenUpdate {
        if !self.tabs.iter().any(|tab| tab.id == "notes.md") {
            self.tabs.push(TabsScreenTab::new("notes.md", "notes"));
        }
        self.active_tab_id = "notes.md".to_string();
        tabs_update(
            "add_tab",
            "closeable_tab_added",
            "tabs.add",
            "notes.md",
            "tabs=6 active=notes.md",
        )
    }

    fn close_active(&mut self) -> TabsScreenUpdate {
        if self.active_tab().is_some_and(|tab| tab.pinned) {
            return tabs_update(
                "close_tab",
                "closeable_tab_close_blocked",
                "tabs.close",
                "blocked",
                "pinned=true close=blocked",
            );
        }
        let closing = self.active_tab_id.clone();
        self.tabs.retain(|tab| tab.id != closing);
        self.remove_empty_groups();
        self.active_tab_id = self.next_active_after_close();
        tabs_update(
            "close_tab",
            "closeable_tab_closed",
            "tabs.close",
            "removed",
            "tabs=5 active=scratch.md",
        )
    }

    fn toggle_pin_active(&mut self) -> TabsScreenUpdate {
        let Some(tab) = self
            .tabs
            .iter_mut()
            .find(|tab| tab.id == self.active_tab_id)
        else {
            return tabs_update(
                "pin_tab",
                "closeable_tab_pin_missing",
                "tabs.pin",
                "none",
                "tab=missing",
            );
        };
        tab.pinned = !tab.pinned;
        let state = if tab.pinned {
            tab.group_id = None;
            "pinned=true left-fixed"
        } else {
            "pinned=false closeable=true"
        };
        tabs_update(
            "toggle_pin_tab",
            "closeable_tab_pin_changed",
            "tabs.pin",
            "toggle",
            state,
        )
    }

    fn move_active_right(&mut self) -> TabsScreenUpdate {
        let Some(from) = self
            .tabs
            .iter()
            .position(|tab| tab.id == self.active_tab_id)
        else {
            return tabs_update(
                "move_tab",
                "closeable_tab_move_missing",
                "tabs.move",
                "none",
                "tab=missing",
            );
        };
        if self.tabs[from].pinned || from + 1 >= self.tabs.len() {
            return tabs_update(
                "move_tab",
                "closeable_tab_move_blocked",
                "tabs.move",
                "blocked",
                "move=blocked",
            );
        }
        self.tabs.swap(from, from + 1);
        tabs_update(
            "move_tab",
            "closeable_tab_reordered",
            "tabs.move",
            "right",
            "order=changed",
        )
    }

    fn group_active(&mut self) -> TabsScreenUpdate {
        self.ensure_docs_group();
        if let Some(tab) = self
            .tabs
            .iter_mut()
            .find(|tab| tab.id == self.active_tab_id)
        {
            tab.pinned = false;
            tab.group_id = Some("docs".to_string());
        }
        tabs_update(
            "move_to_group",
            "closeable_tab_grouped",
            "tabs.group",
            "Docs",
            "group=Docs",
        )
    }

    fn toggle_overflow(&mut self) -> TabsScreenUpdate {
        self.add_many_for_overflow();
        self.overflow_open = !self.overflow_open;
        tabs_update(
            "open_overflow",
            "closeable_tab_overflow_opened",
            "tabs.overflow",
            "menu",
            "overflow=menu",
        )
    }

    fn ensure_docs_group(&mut self) {
        if !self.groups.iter().any(|group| group.id == "docs") {
            self.groups.push(TabsScreenGroup::docs());
        }
    }

    fn add_many_for_overflow(&mut self) {
        for (id, title) in [("lint.md", "lint"), ("theme.rs", "theme")] {
            if !self.tabs.iter().any(|tab| tab.id == id) {
                self.tabs.push(TabsScreenTab::new(id, title));
            }
        }
    }

    fn remove_empty_groups(&mut self) {
        self.groups.retain(|group| {
            self.tabs
                .iter()
                .any(|tab| tab.group_id.as_deref() == Some(group.id.as_str()))
        });
    }

    fn next_active_after_close(&self) -> String {
        if self.tabs.iter().any(|tab| tab.id == "scratch.md") {
            return "scratch.md".to_string();
        }
        self.tabs
            .first()
            .map_or_else(String::new, |tab| tab.id.clone())
    }

    fn is_scratch_after_terminal(&self) -> bool {
        self.tabs.iter().position(|tab| tab.id == "scratch.md")
            > self.tabs.iter().position(|tab| tab.id == "terminal")
    }
}
