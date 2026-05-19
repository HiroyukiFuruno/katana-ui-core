use super::options::WorkspaceTab;

#[must_use]
pub(super) fn ordered_tabs(tabs: &[WorkspaceTab]) -> Vec<&WorkspaceTab> {
    tabs.iter()
        .filter(|tab| tab.pinned)
        .chain(tabs.iter().filter(|tab| !tab.pinned))
        .collect()
}
