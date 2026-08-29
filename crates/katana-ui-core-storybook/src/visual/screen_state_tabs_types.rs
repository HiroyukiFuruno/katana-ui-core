use katana_ui_core::widget::molecules::{
    CloseableTab, CloseableTabGroup, CloseableTabTone, ContextMenuItem,
};

pub(super) const DOCS_GROUP_COLOR: u32 = 0x4a90d9;
const DEFAULT_OVERFLOW_TRIGGER_WIDTH: u16 = 44;
const DEFAULT_COLLAPSED_GROUP_AUTO_EXPAND_MS: u16 = 500;

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
pub(super) enum TabsContextMenuCommand {
    Close,
    CloseOthers,
    CloseAll,
    CloseToRight,
    CloseToLeft,
    RestoreClosed,
    Pin,
    Unpin,
    NewGroup,
    MoveToGroup,
    MoveToExistingGroup(String),
    GroupRename,
    GroupSetColor,
    GroupCollapse,
    GroupExpand,
    GroupMove,
    GroupUngroup,
    GroupClose,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TabsScreenState {
    pub(super) tabs: Vec<TabsScreenTab>,
    pub(super) groups: Vec<TabsScreenGroup>,
    pub(super) active_tab_id: String,
    pub(super) focused_tab_id: Option<String>,
    pub(super) overflow_open: bool,
    pub(super) scroll_x: usize,
    pub(super) overflow_trigger_width: u16,
    pub(super) collapsed_group_auto_expand_ms: u16,
    pub(super) context_menu: Option<TabsContextMenuState>,
    pub(super) recently_closed_tabs: Vec<TabsScreenTab>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TabsContextMenuState {
    pub(super) tab_id: String,
    pub(super) group_id: Option<String>,
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) commands: Vec<TabsContextMenuCommand>,
    pub(super) items: Vec<ContextMenuItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TabsScreenTab {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) icon_visible: bool,
    pub(super) pinned: bool,
    pub(super) dirty: bool,
    pub(super) closeable: bool,
    pub(super) groupable: bool,
    pub(super) tone: &'static str,
    pub(super) tooltip: Option<String>,
    pub(super) group_id: Option<String>,
    pub(super) accessibility_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TabsScreenGroup {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) color: u32,
    pub(super) collapsed: bool,
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
            focused_tab_id: None,
            overflow_open: false,
            scroll_x: 0,
            overflow_trigger_width: DEFAULT_OVERFLOW_TRIGGER_WIDTH,
            collapsed_group_auto_expand_ms: DEFAULT_COLLAPSED_GROUP_AUTO_EXPAND_MS,
            context_menu: None,
            recently_closed_tabs: Vec::new(),
        }
    }
}

impl TabsScreenTab {
    pub(in crate::visual) fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon_visible: false,
            pinned: false,
            dirty: false,
            closeable: true,
            groupable: true,
            tone: "default",
            tooltip: None,
            group_id: None,
            accessibility_label: None,
        }
    }

    pub(in crate::visual) fn pinned(mut self, value: bool) -> Self {
        self.pinned = value;
        self
    }

    pub(in crate::visual) fn dirty(mut self, value: bool) -> Self {
        self.dirty = value;
        self
    }

    pub(in crate::visual) fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub(in crate::visual) fn to_core_tab(&self) -> CloseableTab {
        let mut tab = CloseableTab::new(self.id.as_str(), self.title.as_str())
            .pinned(self.pinned)
            .dirty(self.dirty)
            .closeable(self.closeable)
            .groupable(self.groupable)
            .tone(tab_tone(self.tone));
        if self.icon_visible {
            tab = tab.icon("svg");
        }
        if let Some(tooltip) = self.tooltip.as_ref() {
            tab = tab.tooltip(tooltip.as_str());
        }
        if let Some(label) = self.accessibility_label.as_ref() {
            tab = tab.accessibility_label(label.as_str());
        }
        if let Some(group_id) = self.group_id.as_ref() {
            tab = tab.group_id(group_id.as_str());
        }
        tab
    }
}

fn tab_tone(tone: &str) -> CloseableTabTone {
    match tone {
        "accent" => CloseableTabTone::Accent,
        "warning" => CloseableTabTone::Warning,
        "danger" => CloseableTabTone::Danger,
        "muted" => CloseableTabTone::Muted,
        _ => CloseableTabTone::Default,
    }
}

impl TabsScreenGroup {
    pub(in crate::visual) fn docs() -> Self {
        Self {
            id: "docs".to_string(),
            title: "Docs".to_string(),
            color: DOCS_GROUP_COLOR,
            collapsed: false,
        }
    }

    pub(in crate::visual) fn to_core_group(&self) -> CloseableTabGroup {
        CloseableTabGroup::new(self.id.as_str(), self.title.as_str())
            .color(format!("#{:06x}", self.color))
            .collapsed(self.collapsed)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_tab_conversion_preserves_optional_presentation_metadata() {
        let mut tab = TabsScreenTab::new("report", "Report");
        tab.icon_visible = true;
        tab.tooltip = Some("Quarterly report".to_string());
        tab.accessibility_label = Some("Quarterly report tab".to_string());

        let core = tab.to_core_tab();
        assert!(core.icon.is_some());
        assert_eq!(Some("Quarterly report"), core.tooltip.as_deref());
        assert_eq!(
            Some("Quarterly report tab"),
            core.accessibility_label.as_deref()
        );
    }
}
