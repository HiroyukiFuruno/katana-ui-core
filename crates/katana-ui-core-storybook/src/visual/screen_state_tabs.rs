use super::screen_state_tabs_core::core_event_name;
use super::screen_state_tabs_types::tabs_update;
pub(super) use super::screen_state_tabs_types::{
    TabsContextMenuCommand, TabsScreenAction, TabsScreenGroup, TabsScreenState, TabsScreenTab,
    TabsScreenUpdate,
};
use katana_ui_core::widget::molecules::{
    CloseableTab, CloseableTabGroupTarget, CloseableTabId, CloseableTabStripAction,
};

const VISIBLE_TAB_COUNT_BEFORE_OVERFLOW: usize = 4;
const TAB_SCROLL_STEP_PX: usize = 64;

impl TabsScreenState {
    pub(in crate::visual) fn apply(&mut self, action: TabsScreenAction) -> TabsScreenUpdate {
        self.context_menu = None;
        match action {
            TabsScreenAction::AddTab => self.add_tab(),
            TabsScreenAction::CloseActive => self.close_active(),
            TabsScreenAction::TogglePinActive => self.toggle_pin_active(),
            TabsScreenAction::MoveActiveRight => self.move_active_right(),
            TabsScreenAction::GroupActive => self.group_active(),
            TabsScreenAction::ToggleOverflow => self.toggle_overflow(),
        }
    }

    pub(in crate::visual) fn state_label(&self) -> &'static str {
        if self.context_menu.is_some() {
            return "tabs.context=tab-menu";
        }
        if self.overflow_open {
            return "tabs.overflow=menu";
        }
        if self.active_tab().is_some_and(|tab| tab.pinned) {
            return "tabs.pinned=true";
        }
        if self
            .active_tab()
            .and_then(|tab| tab.group_id.as_ref())
            .is_some()
        {
            return "tabs.group=Docs";
        }
        if self.tabs.iter().any(|tab| tab.id == "notes.md") {
            return "tabs.count=6 active=notes.md";
        }
        if self.is_scratch_after_terminal() {
            return "tabs.order=changed";
        }
        if self.active_tab_id == "theme.rs" {
            return "tabs.active=theme.rs follow";
        }
        "tabs.active=scratch.md count=5"
    }

    pub(in crate::visual) fn active_tab(&self) -> Option<&TabsScreenTab> {
        self.tabs.iter().find(|tab| tab.id == self.active_tab_id)
    }

    pub(in crate::visual) fn unpin_tab_by_icon(&mut self, tab_id: &str) -> TabsScreenUpdate {
        let Some(tab) = self.tabs.iter().find(|tab| tab.id == tab_id) else {
            return tabs_update(
                "tab_pin_icon_unpin",
                "closeable_tab_pin_missing",
                "tabs.pin",
                "none",
                "tabs.tab=missing",
            );
        };
        if !tab.pinned {
            return tabs_update(
                "tab_pin_icon_unpin",
                "closeable_tab_pin_missing",
                "tabs.pin",
                "none",
                "tabs.pinned=false",
            );
        }
        let events = self.apply_core_tab_action(CloseableTabStripAction::UnpinTab {
            tab_id: CloseableTabId::new(tab_id),
        });
        tabs_update(
            "tab_pin_icon_unpin",
            core_event_name(&events, "closeable_tab_pin_missing"),
            "tabs.pin",
            "direct-icon",
            "tabs.pinned=false closeable=true",
        )
    }

    fn add_tab(&mut self) -> TabsScreenUpdate {
        let events = self.apply_core_tab_action(CloseableTabStripAction::AddTab {
            tab: CloseableTab::new("notes.md", "notes"),
            activate: true,
        });
        tabs_update(
            "add_tab",
            core_event_name(&events, "closeable_tab_add_skipped"),
            "tabs.add",
            "notes.md",
            "tabs.count=6 active=notes.md",
        )
    }

    fn close_active(&mut self) -> TabsScreenUpdate {
        if self.active_tab().is_some_and(|tab| tab.pinned) {
            return tabs_update(
                "close_tab",
                "closeable_tab_close_blocked",
                "tabs.close",
                "blocked",
                "tabs.pinned=true close=blocked",
            );
        }
        let events =
            self.apply_core_tab_action_confirming_dirty(CloseableTabStripAction::CloseTab {
                tab_id: CloseableTabId::new(self.active_tab_id.clone()),
            });
        if self.tabs.iter().any(|tab| tab.id == "scratch.md") {
            let _ = self.apply_core_tab_action(CloseableTabStripAction::SelectTab {
                tab_id: CloseableTabId::new("scratch.md"),
            });
        }
        tabs_update(
            "close_tab",
            core_event_name(&events, "closeable_tab_close_missing"),
            "tabs.close",
            "removed",
            "tabs.count=5 active=scratch.md",
        )
    }

    fn toggle_pin_active(&mut self) -> TabsScreenUpdate {
        let Some(tab) = self.active_tab() else {
            return tabs_update(
                "pin_tab",
                "closeable_tab_pin_missing",
                "tabs.pin",
                "none",
                "tabs.tab=missing",
            );
        };
        let pinned = !tab.pinned;
        let action = if pinned {
            CloseableTabStripAction::PinTab {
                tab_id: CloseableTabId::new(self.active_tab_id.clone()),
            }
        } else {
            CloseableTabStripAction::UnpinTab {
                tab_id: CloseableTabId::new(self.active_tab_id.clone()),
            }
        };
        let events = self.apply_core_tab_action(action);
        let state = if pinned {
            "tabs.pinned=true left-fixed"
        } else {
            "tabs.pinned=false closeable=true"
        };
        tabs_update(
            "toggle_pin_tab",
            core_event_name(&events, "closeable_tab_pin_missing"),
            "tabs.pin",
            "toggle",
            state,
        )
    }

    fn move_active_right(&mut self) -> TabsScreenUpdate {
        let visual_ids = self.core_visual_tab_ids();
        let Some(from) = visual_ids
            .iter()
            .position(|tab_id| tab_id == &self.active_tab_id)
        else {
            return tabs_update(
                "move_tab",
                "closeable_tab_move_missing",
                "tabs.move",
                "none",
                "tabs.tab=missing",
            );
        };
        if from + 1 >= visual_ids.len() {
            return tabs_update(
                "move_tab",
                "closeable_tab_move_blocked",
                "tabs.move",
                "blocked",
                "tabs.move=blocked",
            );
        }
        let events = self.apply_core_tab_action(CloseableTabStripAction::MoveTab {
            tab_id: CloseableTabId::new(self.active_tab_id.clone()),
            to_visual_index: from + 1,
        });
        tabs_update(
            "move_tab",
            core_event_name(&events, "closeable_tab_move_blocked"),
            "tabs.move",
            "right",
            "tabs.order=changed",
        )
    }

    fn group_active(&mut self) -> TabsScreenUpdate {
        self.ensure_docs_group();
        let events = self.apply_core_tab_action(CloseableTabStripAction::MoveToGroup {
            tab_id: CloseableTabId::new(self.active_tab_id.clone()),
            target: CloseableTabGroupTarget::Existing("docs".into()),
        });
        tabs_update(
            "move_to_group",
            core_event_name(&events, "closeable_tab_group_missing"),
            "tabs.group",
            "Docs",
            "tabs.group=Docs",
        )
    }

    fn toggle_overflow(&mut self) -> TabsScreenUpdate {
        self.add_many_for_overflow();
        let next_open = !self.overflow_open;
        let hidden_tab_ids = self
            .tabs
            .iter()
            .skip(VISIBLE_TAB_COUNT_BEFORE_OVERFLOW)
            .map(|tab| CloseableTabId::new(tab.id.clone()))
            .collect();
        let events =
            self.apply_core_tab_action(CloseableTabStripAction::OpenOverflow { hidden_tab_ids });
        self.overflow_open = next_open;
        tabs_update(
            "open_overflow",
            core_event_name(&events, "closeable_tab_overflow_missing"),
            "tabs.overflow",
            "menu",
            "tabs.overflow=menu",
        )
    }

    pub(in crate::visual) fn scroll_horizontal(&mut self, delta_x: f32) -> TabsScreenUpdate {
        self.add_many_for_overflow();
        self.context_menu = None;
        if delta_x > 0.0 {
            self.scroll_x = self.scroll_x.saturating_add(TAB_SCROLL_STEP_PX);
        } else {
            self.scroll_x = self.scroll_x.saturating_sub(TAB_SCROLL_STEP_PX);
        }
        tabs_update(
            "tab_strip_scroll",
            "closeable_tab_overflow_scrolled",
            "tabs.overflow",
            "scroll_x",
            "tabs.overflow=scroll",
        )
    }

    pub(in crate::visual) fn focus_tab(&mut self, tab_id: &str) -> TabsScreenUpdate {
        self.context_menu = None;
        if !self.tabs.iter().any(|tab| tab.id == tab_id) {
            return tabs_update(
                "tab_focus",
                "closeable_tab_focus_missing",
                "tabs.focus",
                "missing",
                "tabs.focus=missing",
            );
        }
        self.focused_tab_id = Some(tab_id.to_string());
        tabs_update(
            "tab_focus",
            "closeable_tab_focused",
            "tabs.focus",
            "tab",
            "tabs.focus=tab",
        )
    }

    pub(in crate::visual) fn ensure_docs_group(&mut self) {
        if !self.groups.iter().any(|group| group.id == "docs") {
            self.groups.push(TabsScreenGroup::docs());
        }
    }

    pub(in crate::visual) fn add_many_for_overflow(&mut self) {
        for (id, title) in [("lint.md", "lint"), ("theme.rs", "theme")] {
            if !self.tabs.iter().any(|tab| tab.id == id) {
                self.tabs.push(TabsScreenTab::new(id, title));
            }
        }
    }

    fn is_scratch_after_terminal(&self) -> bool {
        self.tabs.iter().position(|tab| tab.id == "scratch.md")
            > self.tabs.iter().position(|tab| tab.id == "terminal")
    }
}
