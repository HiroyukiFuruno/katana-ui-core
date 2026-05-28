const DOCS_GROUP_COLOR: u32 = 0x4a90d9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TabsScreenAction {
    AddTab,
    CloseActive,
    TogglePinActive,
    MoveActiveRight,
    GroupActive,
    ToggleOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TabsScreenState {
    pub(super) tabs: Vec<TabsScreenTab>,
    pub(super) groups: Vec<TabsScreenGroup>,
    pub(super) active_tab_id: String,
    pub(super) overflow_open: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TabsScreenTab {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) pinned: bool,
    pub(super) dirty: bool,
    pub(super) group_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TabsScreenGroup {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) color: u32,
}

pub(super) struct TabsScreenUpdate {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) setting: &'static str,
    pub(super) value: &'static str,
    pub(super) state: &'static str,
}

impl Default for TabsScreenState {
    fn default() -> Self {
        Self {
            tabs: vec![
                TabsScreenTab::new("readme.md", "readme").pinned(true),
                TabsScreenTab::new("editor.rs", "editor").group_id("docs"),
                TabsScreenTab::new("preview.rs", "preview").group_id("docs"),
                TabsScreenTab::new("scratch.md", "scratch").dirty(true),
                TabsScreenTab::new("terminal", "terminal"),
            ],
            groups: vec![TabsScreenGroup::docs()],
            active_tab_id: "scratch.md".to_string(),
            overflow_open: false,
        }
    }
}

impl TabsScreenTab {
    pub(super) fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            pinned: false,
            dirty: false,
            group_id: None,
        }
    }

    fn pinned(mut self, value: bool) -> Self {
        self.pinned = value;
        self
    }

    fn dirty(mut self, value: bool) -> Self {
        self.dirty = value;
        self
    }

    fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }
}

impl TabsScreenGroup {
    pub(super) fn docs() -> Self {
        Self {
            id: "docs".to_string(),
            title: "Docs".to_string(),
            color: DOCS_GROUP_COLOR,
        }
    }
}

pub(super) const fn tabs_update(
    action: &'static str,
    event: &'static str,
    setting: &'static str,
    value: &'static str,
    state: &'static str,
) -> TabsScreenUpdate {
    TabsScreenUpdate {
        action,
        event,
        setting,
        value,
        state,
    }
}
