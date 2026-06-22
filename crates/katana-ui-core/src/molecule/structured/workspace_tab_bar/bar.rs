use super::actions::WorkspaceTabBarAction;
use super::events::WorkspaceTabBarEvent;
use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
use super::keyboard::{WorkspaceTabKeyboardController, WorkspaceTabKeyboardInput};
pub use super::model::WorkspaceTabBar;
use super::options::{WorkspaceTab, WorkspaceTabBarOptions, WorkspaceTabGroup, WorkspaceTabTone};
use super::ordering::ordered_tabs;
use super::overflow::{
    MeasuredWorkspaceTab, WorkspaceTabOverflowConfig, WorkspaceTabOverflowPlan,
    WorkspaceTabOverflowPlanner,
};
use super::state::WorkspaceTabBarState;
use crate::render_model::{
    UiCommonProps, UiDimension, UiNode, UiNodeKind, UiStateId, UiVisualRole,
};

const WORKSPACE_TAB_BAR_HEIGHT_PX: u16 = 40;
const WORKSPACE_TAB_HEIGHT_PX: u16 = 36;

impl WorkspaceTabBar {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        let options = WorkspaceTabBarOptions::default();
        Self {
            label: label.into(),
            state: WorkspaceTabBarState::new(&options.tabs),
            options,
            event_log: Vec::new(),
        }
    }

    #[must_use]
    pub fn tab(mut self, tab: WorkspaceTab) -> Self {
        self.options.tabs.push(tab);
        self.normalize_tabs();
        self
    }

    #[must_use]
    pub fn recently_closed_tab(mut self, tab: WorkspaceTab) -> Self {
        self.state.record_closed_tab(tab);
        self
    }

    #[must_use]
    pub fn group(mut self, group: WorkspaceTabGroup) -> Self {
        self.options.groups.push(group);
        self
    }

    #[must_use]
    pub fn active_tab_id(mut self, value: impl Into<WorkspaceTabId>) -> Self {
        self.state.active_tab_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn stable_state_id(mut self, value: impl Into<UiStateId>) -> Self {
        self.state.state_id = value.into();
        self.state.sync_child_states(&self.options.tabs);
        self
    }

    #[must_use]
    pub fn overflow_trigger_width(mut self, value: u16) -> Self {
        self.options.overflow_trigger_width = value;
        self
    }

    #[must_use]
    pub fn collapsed_group_auto_expand_ms(mut self, value: u16) -> Self {
        self.options.collapsed_group_auto_expand_ms = value;
        self
    }

    #[must_use]
    pub fn options(&self) -> &WorkspaceTabBarOptions {
        &self.options
    }

    #[must_use]
    pub fn state(&self) -> &WorkspaceTabBarState {
        &self.state
    }

    #[must_use]
    pub fn event_log(&self) -> &[WorkspaceTabBarEvent] {
        &self.event_log
    }

    #[must_use]
    pub fn visual_tabs(&self) -> Vec<&WorkspaceTab> {
        ordered_tabs(&self.options.tabs, &self.options.groups)
    }

    #[must_use]
    pub fn overflow_plan(
        &self,
        available_width: u16,
        measured_tabs: &[MeasuredWorkspaceTab],
    ) -> WorkspaceTabOverflowPlan {
        WorkspaceTabOverflowPlanner::compute(
            WorkspaceTabOverflowConfig::new(available_width, self.options.overflow_trigger_width),
            measured_tabs,
            self.state.active_tab_id.as_ref(),
        )
    }

    pub fn apply_keyboard_input(
        &mut self,
        input: WorkspaceTabKeyboardInput,
        visible_tab_ids: &[WorkspaceTabId],
    ) -> Vec<WorkspaceTabBarEvent> {
        if input == WorkspaceTabKeyboardInput::CancelDrag {
            return self.apply_action(WorkspaceTabBarAction::CancelDrag);
        }
        let action = WorkspaceTabKeyboardController::action_for_input(
            &input,
            self.state.active_tab_id.as_ref(),
            visible_tab_ids,
        );
        action.map_or_else(Vec::new, |it| self.apply_action(it))
    }
}

impl From<WorkspaceTabBar> for UiNode {
    fn from(value: WorkspaceTabBar) -> Self {
        let common = UiCommonProps::default()
            .width(UiDimension::Fill)
            .height(UiDimension::Px(WORKSPACE_TAB_BAR_HEIGHT_PX))
            .selectable(false)
            .accessibility_label(value.label.clone());
        let interaction = value.state.interaction(value.options.tabs.len());
        let state_id = value.state.state_id.clone();
        let label = value.label.clone();
        let node = UiNode::from_state(UiNodeKind::CloseableTabStrip, label, state_id)
            .common(common)
            .interaction(interaction)
            .visual_role(UiVisualRole::Control)
            .style_class("closeable-tab-strip");
        append_workspace_tab_children(node, &value.options, &value.state)
    }
}

fn append_workspace_tab_children(
    mut node: UiNode,
    options: &WorkspaceTabBarOptions,
    state: &WorkspaceTabBarState,
) -> UiNode {
    for tab in options.tabs.iter().filter(|tab| tab.pinned) {
        node = node.child(workspace_tab_node(tab, state.child_state_id(&tab.id)));
    }
    for group in &options.groups {
        let grouped_tabs = group_tabs(options, &group.id);
        if grouped_tabs.is_empty() {
            continue;
        }
        node = node.child(workspace_group_header_node(group));
        if group.collapsed {
            continue;
        }
        for tab in grouped_tabs {
            node = node.child(workspace_tab_node(tab, state.child_state_id(&tab.id)));
        }
    }
    for tab in unknown_group_tabs(options) {
        node = node.child(workspace_tab_node(tab, state.child_state_id(&tab.id)));
    }
    for tab in options
        .tabs
        .iter()
        .filter(|tab| !tab.pinned && tab.group_id.is_none())
    {
        node = node.child(workspace_tab_node(tab, state.child_state_id(&tab.id)));
    }
    node
}

fn group_tabs<'a>(
    options: &'a WorkspaceTabBarOptions,
    group_id: &WorkspaceTabGroupId,
) -> Vec<&'a WorkspaceTab> {
    options
        .tabs
        .iter()
        .filter(|tab| !tab.pinned && tab.group_id.as_ref() == Some(group_id))
        .collect()
}

fn unknown_group_tabs(options: &WorkspaceTabBarOptions) -> Vec<&WorkspaceTab> {
    options
        .tabs
        .iter()
        .filter(|tab| {
            !tab.pinned
                && tab.group_id.as_ref().is_some_and(|group_id| {
                    options.groups.iter().all(|group| group.id != *group_id)
                })
        })
        .collect()
}

fn workspace_tab_node(
    tab: &WorkspaceTab,
    state_id: Option<&crate::render_model::UiStateId>,
) -> UiNode {
    let mut node = state_id.map_or_else(
        || UiNode::new(UiNodeKind::CloseableTab, tab.title.clone()),
        |it| UiNode::from_state(UiNodeKind::CloseableTab, tab.title.clone(), it.clone()),
    );
    node = node
        .width(UiDimension::FitContent)
        .height(UiDimension::Px(WORKSPACE_TAB_HEIGHT_PX))
        .focusable(true)
        .selectable(true)
        .tone(tab_tone(tab.tone))
        .accessibility_label(tab.accessibility_text())
        .style_class("closeable-tab");
    if tab.pinned {
        node = node.style_class("closeable-tab-pinned");
    }
    if tab.closeable && !tab.pinned {
        node = node.style_class("closeable-tab-closeable");
    }
    if tab.dirty {
        node = node.style_class("closeable-tab-dirty");
    }
    if let Some(icon) = tab.icon.as_ref() {
        node = node.icon(icon.clone());
    }
    node
}

fn workspace_group_header_node(group: &WorkspaceTabGroup) -> UiNode {
    let mut node = UiNode::new(UiNodeKind::CloseableTabGroupHeader, group.label.clone())
        .width(UiDimension::FitContent)
        .height(UiDimension::Px(WORKSPACE_TAB_HEIGHT_PX))
        .focusable(true)
        .selectable(true)
        .accessibility_label(group_accessibility_text(group))
        .style_class("closeable-tab-group-header");
    if group.collapsed {
        node = node.style_class("closeable-tab-group-collapsed");
    }
    node
}

fn group_accessibility_text(group: &WorkspaceTabGroup) -> String {
    let expanded = if group.collapsed {
        "collapsed"
    } else {
        "expanded"
    };
    format!("{} group {expanded}", group.label)
}

fn tab_tone(tone: WorkspaceTabTone) -> crate::render_model::UiTone {
    match tone {
        WorkspaceTabTone::Default | WorkspaceTabTone::Muted => crate::render_model::UiTone::Neutral,
        WorkspaceTabTone::Accent => crate::render_model::UiTone::Accent,
        WorkspaceTabTone::Warning => crate::render_model::UiTone::Warning,
        WorkspaceTabTone::Danger => crate::render_model::UiTone::Danger,
    }
}
