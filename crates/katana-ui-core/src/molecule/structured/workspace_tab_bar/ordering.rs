use super::options::{WorkspaceTab, WorkspaceTabGroup};
use std::collections::HashSet;

#[must_use]
pub(super) fn ordered_tabs<'a>(
    tabs: &'a [WorkspaceTab],
    groups: &'a [WorkspaceTabGroup],
) -> Vec<&'a WorkspaceTab> {
    let mut ordered = Vec::new();
    push_pinned_tabs(&mut ordered, tabs);
    append_grouped_tabs(&mut ordered, tabs, groups, false);
    push_unknown_group_tabs(&mut ordered, tabs, groups);
    ordered.extend(
        tabs.iter()
            .filter(|tab| !tab.pinned && tab.group_id.is_none()),
    );
    ordered
}

#[must_use]
pub(super) fn ordered_visible_tabs<'a>(
    tabs: &'a [WorkspaceTab],
    groups: &'a [WorkspaceTabGroup],
) -> Vec<&'a WorkspaceTab> {
    let mut ordered = Vec::new();
    push_pinned_tabs(&mut ordered, tabs);
    append_grouped_tabs(&mut ordered, tabs, groups, true);
    push_unknown_group_tabs(&mut ordered, tabs, groups);
    ordered.extend(
        tabs.iter()
            .filter(|tab| !tab.pinned && tab.group_id.is_none()),
    );
    ordered
}

fn push_pinned_tabs<'a>(ordered: &mut Vec<&'a WorkspaceTab>, tabs: &'a [WorkspaceTab]) {
    ordered.extend(tabs.iter().filter(|tab| tab.pinned));
}

fn append_grouped_tabs<'a>(
    ordered: &mut Vec<&'a WorkspaceTab>,
    tabs: &'a [WorkspaceTab],
    groups: &'a [WorkspaceTabGroup],
    skip_collapsed_children: bool,
) {
    let mut visited_groups = HashSet::new();
    let root_groups = root_groups(groups);
    for group in root_groups {
        append_group_tabs(
            ordered,
            tabs,
            groups,
            group,
            &mut visited_groups,
            skip_collapsed_children,
        );
    }
}

fn append_group_tabs<'a>(
    ordered: &mut Vec<&'a WorkspaceTab>,
    tabs: &'a [WorkspaceTab],
    groups: &'a [WorkspaceTabGroup],
    group: &'a WorkspaceTabGroup,
    visited_groups: &mut HashSet<super::identifiers::WorkspaceTabGroupId>,
    skip_collapsed_children: bool,
) {
    if !visited_groups.insert(group.id.clone()) {
        return;
    }
    if group.collapsed && skip_collapsed_children {
        return;
    }
    ordered.extend(
        tabs.iter()
            .filter(|tab| !tab.pinned && tab.group_id.as_ref() == Some(&group.id)),
    );
    for child in child_groups(groups, &group.id) {
        append_group_tabs(
            ordered,
            tabs,
            groups,
            child,
            visited_groups,
            skip_collapsed_children,
        );
    }
}

fn push_unknown_group_tabs<'a>(
    ordered: &mut Vec<&'a WorkspaceTab>,
    tabs: &'a [WorkspaceTab],
    groups: &'a [WorkspaceTabGroup],
) {
    ordered.extend(tabs.iter().filter(|tab| {
        !tab.pinned
            && tab
                .group_id
                .as_ref()
                .is_some_and(|group_id| groups.iter().all(|group| group.id != *group_id))
    }));
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
    parent_group_id: &super::identifiers::WorkspaceTabGroupId,
) -> Vec<&'a WorkspaceTabGroup> {
    groups
        .iter()
        .filter(|group| group.parent_group_id.as_ref() == Some(parent_group_id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_group_identity_is_visited_only_once() {
        let groups = vec![
            WorkspaceTabGroup::new("duplicate", "First"),
            WorkspaceTabGroup::new("duplicate", "Second"),
        ];
        assert!(ordered_tabs(&[], &groups).is_empty());
    }
}
