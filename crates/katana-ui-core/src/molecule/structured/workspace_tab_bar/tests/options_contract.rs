use super::super::{
    MeasuredWorkspaceTab, WorkspaceTabBar, WorkspaceTabBarAction, WorkspaceTabBarEvent,
    WorkspaceTabGroup, WorkspaceTabId,
};

const AVAILABLE_WIDTH: u16 = 170;
const DEFAULT_TRIGGER_WIDTH: u16 = 44;
const WIDE_TRIGGER_WIDTH: u16 = 90;
const CUSTOM_GROUP_EXPAND_DELAY_MS: u16 = 1000;

#[test]
fn overflow_trigger_width_builder_controls_bar_overflow_plan() {
    let measured_tabs = [
        MeasuredWorkspaceTab::new("one", 80),
        MeasuredWorkspaceTab::new("two", 60),
        MeasuredWorkspaceTab::new("three", 40),
    ];
    let default_plan = WorkspaceTabBar::new("Workspace")
        .overflow_trigger_width(DEFAULT_TRIGGER_WIDTH)
        .overflow_plan(AVAILABLE_WIDTH, &measured_tabs);
    let wide_trigger_plan = WorkspaceTabBar::new("Workspace")
        .overflow_trigger_width(WIDE_TRIGGER_WIDTH)
        .overflow_plan(AVAILABLE_WIDTH, &measured_tabs);

    assert_eq!(
        vec![WorkspaceTabId::new("one"), WorkspaceTabId::new("three")],
        default_plan.visible_tab_ids
    );
    assert_eq!(
        vec![WorkspaceTabId::new("one")],
        wide_trigger_plan.visible_tab_ids
    );
    assert!(wide_trigger_plan.overflow_visible);
}

#[test]
fn collapsed_group_auto_expand_builder_controls_hover_delay() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .collapsed_group_auto_expand_ms(CUSTOM_GROUP_EXPAND_DELAY_MS)
        .group(WorkspaceTabGroup::new("docs", "Docs").collapsed(true));

    let early = bar.apply_action(WorkspaceTabBarAction::HoverCollapsedGroupForDrop {
        group_id: "docs".into(),
        elapsed_ms: CUSTOM_GROUP_EXPAND_DELAY_MS - 1,
    });
    let expanded = bar.apply_action(WorkspaceTabBarAction::HoverCollapsedGroupForDrop {
        group_id: "docs".into(),
        elapsed_ms: CUSTOM_GROUP_EXPAND_DELAY_MS,
    });

    assert!(early.is_empty());
    assert_eq!(
        vec![WorkspaceTabBarEvent::GroupCollapseChanged {
            group_id: "docs".into(),
            collapsed: false
        }],
        expanded
    );
}
