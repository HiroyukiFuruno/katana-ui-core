use super::options::{WorkspaceTab, WorkspaceTabGroup};

#[must_use]
pub(super) fn ordered_tabs<'a>(
    tabs: &'a [WorkspaceTab],
    groups: &[WorkspaceTabGroup],
) -> Vec<&'a WorkspaceTab> {
    let mut ordered = Vec::new();
    push_pinned_tabs(&mut ordered, tabs);
    push_group_tabs(&mut ordered, tabs, groups);
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

fn push_group_tabs<'a>(
    ordered: &mut Vec<&'a WorkspaceTab>,
    tabs: &'a [WorkspaceTab],
    groups: &[WorkspaceTabGroup],
) {
    for group in groups {
        ordered.extend(
            tabs.iter()
                .filter(|tab| !tab.pinned && tab.group_id.as_ref() == Some(&group.id)),
        );
    }
}

fn push_unknown_group_tabs<'a>(
    ordered: &mut Vec<&'a WorkspaceTab>,
    tabs: &'a [WorkspaceTab],
    groups: &[WorkspaceTabGroup],
) {
    ordered.extend(tabs.iter().filter(|tab| {
        !tab.pinned
            && tab
                .group_id
                .as_ref()
                .is_some_and(|group_id| groups.iter().all(|group| group.id != *group_id))
    }));
}
