use super::super::{
    MeasuredWorkspaceTab, WorkspaceTabId, WorkspaceTabOverflowConfig, WorkspaceTabOverflowPlanner,
    WorkspaceTabScrollConfig, WorkspaceTabScrollPlanner,
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

#[test]
fn overflow_keeps_visible_active_tab_and_promotes_active_when_none_fit() {
    let measured = measured_tabs();
    let visible_active = WorkspaceTabOverflowPlanner::compute(
        WorkspaceTabOverflowConfig::new(230, 40),
        &measured,
        Some(&WorkspaceTabId::new("one")),
    );
    assert!(
        visible_active
            .visible_tab_ids
            .contains(&WorkspaceTabId::new("one"))
    );

    let promoted = WorkspaceTabOverflowPlanner::compute(
        WorkspaceTabOverflowConfig::new(40, 40),
        &measured,
        Some(&WorkspaceTabId::new("three")),
    );
    assert_eq!(vec![WorkspaceTabId::new("three")], promoted.visible_tab_ids);
}

#[test]
fn scroll_planner_follows_active_tab_when_external_selection_moves_right() {
    let measured = measured_tabs();

    let plan = WorkspaceTabScrollPlanner::follow_active(
        WorkspaceTabScrollConfig::new(160, 0),
        &measured,
        Some(&WorkspaceTabId::new("three")),
    );

    assert!(plan.overflow_scroll_enabled);
    assert!(plan.active_tab_visible);
    assert_eq!(80, plan.scroll_x);
    assert_eq!(140, plan.max_scroll_x);
    assert_eq!(300, plan.total_width);
}

#[test]
fn scroll_planner_follows_active_tab_when_external_selection_moves_left() {
    let measured = measured_tabs();

    let plan = WorkspaceTabScrollPlanner::follow_active(
        WorkspaceTabScrollConfig::new(160, 120),
        &measured,
        Some(&WorkspaceTabId::new("two")),
    );

    assert_eq!(80, plan.scroll_x);
    assert!(plan.active_tab_visible);
}

#[test]
fn scroll_planner_stays_at_zero_without_overflow() {
    let measured = vec![
        MeasuredWorkspaceTab::new("one", 40),
        MeasuredWorkspaceTab::new("two", 50),
    ];

    let plan = WorkspaceTabScrollPlanner::follow_active(
        WorkspaceTabScrollConfig::new(100, 30),
        &measured,
        Some(&WorkspaceTabId::new("two")),
    );

    assert_eq!(0, plan.scroll_x);
    assert_eq!(0, plan.max_scroll_x);
    assert!(!plan.overflow_scroll_enabled);
    assert!(plan.active_tab_visible);
}

fn measured_tabs() -> Vec<MeasuredWorkspaceTab> {
    vec![
        MeasuredWorkspaceTab::new("one", 80),
        MeasuredWorkspaceTab::new("two", 70),
        MeasuredWorkspaceTab::new("three", 90),
        MeasuredWorkspaceTab::new("four", 60),
    ]
}
