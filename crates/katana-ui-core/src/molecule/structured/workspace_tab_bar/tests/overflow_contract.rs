use super::super::{
    MeasuredWorkspaceTab, WorkspaceTabId, WorkspaceTabOverflowConfig, WorkspaceTabOverflowPlanner,
};

#[test]
fn overflow_uses_measured_width_and_preserves_hidden_order() {
    let measured = vec![
        MeasuredWorkspaceTab::new("one", 80),
        MeasuredWorkspaceTab::new("two", 70),
        MeasuredWorkspaceTab::new("three", 90),
        MeasuredWorkspaceTab::new("four", 60),
    ];

    let plan = WorkspaceTabOverflowPlanner::compute(
        WorkspaceTabOverflowConfig::new(230, 40),
        &measured,
        Some(&WorkspaceTabId::new("three")),
    );

    assert!(plan.overflow_visible);
    assert_eq!(
        vec![WorkspaceTabId::new("one"), WorkspaceTabId::new("three")],
        plan.visible_tab_ids
    );
    assert_eq!(
        vec![WorkspaceTabId::new("two"), WorkspaceTabId::new("four")],
        plan.hidden_tab_ids
    );
}

#[test]
fn overflow_stays_hidden_when_all_measured_tabs_fit() {
    let measured = vec![
        MeasuredWorkspaceTab::new("one", 40),
        MeasuredWorkspaceTab::new("two", 50),
    ];

    let plan = WorkspaceTabOverflowPlanner::compute(
        WorkspaceTabOverflowConfig::new(100, 40),
        &measured,
        None,
    );

    assert!(!plan.overflow_visible);
    assert_eq!(
        vec![WorkspaceTabId::new("one"), WorkspaceTabId::new("two")],
        plan.visible_tab_ids
    );
    assert!(plan.hidden_tab_ids.is_empty());
}
