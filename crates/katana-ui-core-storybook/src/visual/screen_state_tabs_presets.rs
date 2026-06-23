use super::screen_state_tabs::{TabsScreenAction, TabsScreenState, TabsScreenTab};

const ADD_CLOSE_PRESET_INDEX: usize = 1;
const PIN_PRESET_INDEX: usize = 2;
const MOVE_PRESET_INDEX: usize = 3;
const GROUP_PRESET_INDEX: usize = 4;
const OVERFLOW_PRESET_INDEX: usize = 5;
const ACTIVE_FOLLOW_PRESET_INDEX: usize = 6;
const ICON_PRESET_INDEX: usize = 7;
const DIRTY_PRESET_INDEX: usize = 8;
const CLOSEABLE_PRESET_INDEX: usize = 9;
const TONE_PRESET_INDEX: usize = 10;
const TOOLTIP_PRESET_INDEX: usize = 11;
const ACCESSIBILITY_PRESET_INDEX: usize = 12;
const GROUP_COLOR_PRESET_INDEX: usize = 13;
const GROUP_COLLAPSE_PRESET_INDEX: usize = 14;
const OVERFLOW_WIDTH_PRESET_INDEX: usize = 15;
const GROUP_AUTO_EXPAND_PRESET_INDEX: usize = 16;
const TERMINAL_TAB_INDEX: usize = 4;
const SCRATCH_TAB_INDEX: usize = 3;
const EDITOR_TAB_INDEX: usize = 1;
const DOCS_GROUP_INDEX: usize = 0;
const GROUP_COLOR_PRESET_COLOR: u32 = 0x5aa65a;
const EXPANDED_OVERFLOW_TRIGGER_WIDTH: u16 = 72;
const EXPANDED_GROUP_AUTO_EXPAND_MS: u16 = 1000;

impl TabsScreenState {
    pub(in crate::visual) fn for_preset(preset_index: usize) -> Self {
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
            OVERFLOW_PRESET_INDEX => state.open_overflow_preset(),
            ACTIVE_FOLLOW_PRESET_INDEX => state.follow_overflow_preset(),
            ICON_PRESET_INDEX => state.tabs[EDITOR_TAB_INDEX].icon_visible = true,
            DIRTY_PRESET_INDEX => state.dirty_terminal_preset(),
            CLOSEABLE_PRESET_INDEX => state.closeable_preset(),
            TONE_PRESET_INDEX => state.tabs[SCRATCH_TAB_INDEX].tone = "warning",
            TOOLTIP_PRESET_INDEX => {
                state.tabs[SCRATCH_TAB_INDEX].tooltip = Some("Open scratch buffer".to_string());
            }
            ACCESSIBILITY_PRESET_INDEX => {
                state.tabs[SCRATCH_TAB_INDEX].accessibility_label =
                    Some("Scratch tab with unsaved draft".to_string());
            }
            GROUP_COLOR_PRESET_INDEX => {
                state.groups[DOCS_GROUP_INDEX].color = GROUP_COLOR_PRESET_COLOR;
            }
            GROUP_COLLAPSE_PRESET_INDEX => state.group_collapse_preset(),
            OVERFLOW_WIDTH_PRESET_INDEX => state.overflow_width_preset(),
            GROUP_AUTO_EXPAND_PRESET_INDEX => state.group_auto_expand_preset(),
            _ => {}
        }
        state
    }

    fn open_overflow_preset(&mut self) {
        self.add_many_for_overflow();
        self.overflow_open = true;
    }

    fn follow_overflow_preset(&mut self) {
        self.add_many_for_overflow();
        self.active_tab_id = "theme.rs".to_string();
    }

    fn dirty_terminal_preset(&mut self) {
        self.tabs[TERMINAL_TAB_INDEX].dirty = true;
        self.active_tab_id = "terminal".to_string();
    }

    fn closeable_preset(&mut self) {
        self.tabs.push(TabsScreenTab::new("locked.md", "locked"));
        self.active_tab_id = "locked.md".to_string();
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == "locked.md") {
            tab.closeable = false;
        }
    }

    fn overflow_width_preset(&mut self) {
        self.open_overflow_preset();
        self.overflow_trigger_width = EXPANDED_OVERFLOW_TRIGGER_WIDTH;
    }

    fn group_collapse_preset(&mut self) {
        self.groups[DOCS_GROUP_INDEX].collapsed = true;
        self.groups[DOCS_GROUP_INDEX].title = "Docs collapsed".to_string();
    }

    fn group_auto_expand_preset(&mut self) {
        self.ensure_docs_group();
        self.active_tab_id = "editor.rs".to_string();
        self.collapsed_group_auto_expand_ms = EXPANDED_GROUP_AUTO_EXPAND_MS;
    }
}
