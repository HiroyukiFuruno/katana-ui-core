use super::WorkspaceTabBar;
use super::identifiers::WorkspaceTabGroupId;
use super::options::{WorkspaceTab, WorkspaceTabBarOptions, WorkspaceTabGroup, WorkspaceTabTone};
use super::state::WorkspaceTabBarState;
use crate::render_model::{UiCommonProps, UiDimension, UiNode, UiNodeKind, UiVisualRole};
use std::collections::HashSet;

const WORKSPACE_TAB_BAR_HEIGHT_PX: u16 = 40;
const WORKSPACE_TAB_HEIGHT_PX: u16 = 36;

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
    let mut rendered_group_ids = HashSet::new();
    for group in root_groups(&options.groups) {
        node = append_group_with_children(node, options, state, group, &mut rendered_group_ids);
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

fn append_group_with_children(
    mut node: UiNode,
    options: &WorkspaceTabBarOptions,
    state: &WorkspaceTabBarState,
    group: &WorkspaceTabGroup,
    rendered_group_ids: &mut HashSet<WorkspaceTabGroupId>,
) -> UiNode {
    if !rendered_group_ids.insert(group.id.clone()) {
        return node;
    }

    let direct_tabs = group_tabs(options, &group.id);
    let child_groups = child_groups(&options.groups, &group.id);
    if direct_tabs.is_empty() && child_groups.is_empty() {
        return node;
    }

    node = node.child(workspace_group_header_node(group));
    if group.collapsed {
        return node;
    }

    for tab in direct_tabs {
        node = node.child(workspace_tab_node(tab, state.child_state_id(&tab.id)));
    }
    for child_group in child_groups {
        node = append_group_with_children(node, options, state, child_group, rendered_group_ids);
    }
    node
}

fn root_groups(groups: &[WorkspaceTabGroup]) -> Vec<&WorkspaceTabGroup> {
    groups
        .iter()
        .filter(|group| is_root_group(group, groups))
        .collect()
}

fn is_root_group(group: &WorkspaceTabGroup, groups: &[WorkspaceTabGroup]) -> bool {
    if let Some(parent_group_id) = group.parent_group_id.as_ref() {
        groups
            .iter()
            .all(|candidate| candidate.id != *parent_group_id)
    } else {
        true
    }
}

fn child_groups<'a>(
    groups: &'a [WorkspaceTabGroup],
    parent_group_id: &WorkspaceTabGroupId,
) -> Vec<&'a WorkspaceTabGroup> {
    groups
        .iter()
        .filter(|group| group.parent_group_id.as_ref() == Some(parent_group_id))
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
