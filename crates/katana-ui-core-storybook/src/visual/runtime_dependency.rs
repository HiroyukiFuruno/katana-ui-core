use super::runtime::StorybookDependencyRuntimeReport;
use katana_ui_core::molecule::{
    CloseableTab, CloseableTabGroup, CloseableTabGroupTarget, CloseableTabStrip,
    CloseableTabStripAction,
};

pub(super) fn runtime_report() -> StorybookDependencyRuntimeReport {
    let mut tabs = CloseableTabStrip::new("Workspace")
        .group(CloseableTabGroup::new("docs", "Docs"))
        .tab(CloseableTab::new("doc", "Document").group_id("docs"));
    let missing_tab_group_close_ignored = tabs
        .apply_action(CloseableTabStripAction::CloseGroup {
            group_id: "missing".into(),
        })
        .is_empty();
    let same_tab_group_move_ignored = tabs
        .apply_action(CloseableTabStripAction::MoveToGroup {
            tab_id: "doc".into(),
            target: CloseableTabGroupTarget::Existing("docs".into()),
        })
        .is_empty();
    let tab_group_removal_emitted = !tabs
        .apply_action(CloseableTabStripAction::MoveToGroup {
            tab_id: "doc".into(),
            target: CloseableTabGroupTarget::Ungrouped,
        })
        .is_empty();

    StorybookDependencyRuntimeReport {
        missing_tab_group_close_ignored,
        same_tab_group_move_ignored,
        tab_group_removal_emitted,
    }
}

#[cfg(test)]
mod tests {
    use super::runtime_report;

    #[test]
    fn dependency_runtime_report_passes() {
        assert!(runtime_report().passed());
    }
}
