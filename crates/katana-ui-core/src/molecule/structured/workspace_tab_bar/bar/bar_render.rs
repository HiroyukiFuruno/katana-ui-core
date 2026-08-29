use super::super::{
    identifiers::WorkspaceTabGroupId,
    options::{WorkspaceTab, WorkspaceTabBarOptions, WorkspaceTabGroup, WorkspaceTabTone},
    state::WorkspaceTabBarState,
};
use crate::render_model::{UiDimension, UiNode, UiNodeKind, UiStateId, UiTone};
use std::collections::HashSet;

const WORKSPACE_TAB_HEIGHT_PX: u16 = 36;

pub(super) fn append_workspace_tab_children(
    mut node: UiNode,
    options: &WorkspaceTabBarOptions,
    state: &WorkspaceTabBarState,
) -> UiNode {
    for tab in options.tabs.iter().filter(|tab| tab.pinned) {
        node = node.child(workspace_tab_node(
            tab,
            state.stable_child_state_id(&tab.id),
        ));
    }
    let mut rendered_group_ids = HashSet::new();
    for group in root_groups(&options.groups) {
        node = append_group_with_children(node, options, state, group, &mut rendered_group_ids);
    }
    for tab in unknown_group_tabs(options) {
        node = node.child(workspace_tab_node(
            tab,
            state.stable_child_state_id(&tab.id),
        ));
    }
    for tab in options
        .tabs
        .iter()
        .filter(|tab| !tab.pinned && tab.group_id.is_none())
    {
        node = node.child(workspace_tab_node(
            tab,
            state.stable_child_state_id(&tab.id),
        ));
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
        node = node.child(workspace_tab_node(
            tab,
            state.stable_child_state_id(&tab.id),
        ));
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

fn workspace_tab_node(tab: &WorkspaceTab, state_id: UiStateId) -> UiNode {
    let mut node = UiNode::from_state(UiNodeKind::CloseableTab, tab.title.clone(), state_id);
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

fn tab_tone(tone: WorkspaceTabTone) -> UiTone {
    match tone {
        WorkspaceTabTone::Default | WorkspaceTabTone::Muted => UiTone::Neutral,
        WorkspaceTabTone::Accent => UiTone::Accent,
        WorkspaceTabTone::Warning => UiTone::Warning,
        WorkspaceTabTone::Danger => UiTone::Danger,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_model::UiNodeKind;

    #[test]
    fn standalone_workspace_tab_node_preserves_stable_state_identity() {
        let state_id = crate::render_model::UiStateId::new("workspace:standalone");
        let node = workspace_tab_node(
            &WorkspaceTab::new("standalone", "Standalone"),
            state_id.clone(),
        );

        assert_eq!(UiNodeKind::CloseableTab, node.kind());
        assert_eq!("Standalone", node.props().label);
        assert_eq!(state_id, node.props().state_id);
    }

    #[test]
    fn duplicate_root_group_identity_is_rendered_at_most_once() {
        let options = WorkspaceTabBarOptions {
            groups: vec![
                WorkspaceTabGroup::new("duplicate", "First"),
                WorkspaceTabGroup::new("duplicate", "Second"),
            ],
            ..WorkspaceTabBarOptions::default()
        };
        let state = WorkspaceTabBarState::new(&options.tabs);
        let node = append_workspace_tab_children(
            UiNode::new(UiNodeKind::CloseableTabStrip, "tabs"),
            &options,
            &state,
        );
        assert!(node.children().is_empty());
    }

    #[test]
    fn root_and_child_group_hierarchy_renders_hierarchy_once() {
        let options = WorkspaceTabBarOptions {
            tabs: vec![
                WorkspaceTab::new("a", "A").group_id("child"),
                WorkspaceTab::new("b", "B").group_id("parent"),
            ],
            groups: vec![
                WorkspaceTabGroup::new("parent", "Parent"),
                WorkspaceTabGroup::new("child", "Child").parent_group("parent"),
            ],
            ..WorkspaceTabBarOptions::default()
        };
        let state = WorkspaceTabBarState::new(&options.tabs);
        let node = append_workspace_tab_children(
            UiNode::new(UiNodeKind::CloseableTabStrip, "tabs"),
            &options,
            &state,
        );
        let children = node.children();
        assert_eq!(4, children.len());
        assert_eq!(UiNodeKind::CloseableTabGroupHeader, children[0].kind());
        assert_eq!(UiNodeKind::CloseableTab, children[1].kind());
        assert_eq!(UiNodeKind::CloseableTabGroupHeader, children[2].kind());
        assert_eq!(UiNodeKind::CloseableTab, children[3].kind());
    }

    #[test]
    fn collapsed_group_renders_only_header_without_children_and_unknown_group_tabs_appear_last() {
        let options = WorkspaceTabBarOptions {
            tabs: vec![
                WorkspaceTab::new("normal", "Normal"),
                WorkspaceTab::new("unknown", "Unknown").group_id("missing"),
                WorkspaceTab::new("child", "Child").group_id("child"),
            ],
            groups: vec![
                WorkspaceTabGroup::new("child", "Child Group").collapsed(true),
                WorkspaceTabGroup::new("orphan", "Orphan"),
            ],
            ..WorkspaceTabBarOptions::default()
        };
        let state = WorkspaceTabBarState::new(&options.tabs);
        let node = append_workspace_tab_children(
            UiNode::new(UiNodeKind::CloseableTabStrip, "tabs"),
            &options,
            &state,
        );
        let children = node.children();
        assert_eq!(3, children.len());
        assert_eq!(UiNodeKind::CloseableTabGroupHeader, children[0].kind());
        assert_eq!(UiNodeKind::CloseableTab, children[1].kind());
        assert_eq!("Unknown", children[1].props().label);
        assert_eq!(UiNodeKind::CloseableTab, children[2].kind());
        assert_eq!("Normal", children[2].props().label);
    }
}
