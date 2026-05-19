use super::identifiers::WorkspaceTabId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasuredWorkspaceTab {
    pub tab_id: WorkspaceTabId,
    pub width: u16,
}

impl MeasuredWorkspaceTab {
    #[must_use]
    pub fn new(tab_id: impl Into<WorkspaceTabId>, width: u16) -> Self {
        Self {
            tab_id: tab_id.into(),
            width,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabOverflowConfig {
    pub available_width: u16,
    pub overflow_trigger_width: u16,
}

impl WorkspaceTabOverflowConfig {
    #[must_use]
    pub const fn new(available_width: u16, overflow_trigger_width: u16) -> Self {
        Self {
            available_width,
            overflow_trigger_width,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabOverflowPlan {
    pub visible_tab_ids: Vec<WorkspaceTabId>,
    pub hidden_tab_ids: Vec<WorkspaceTabId>,
    pub overflow_visible: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabOverflowPlanner;

impl WorkspaceTabOverflowPlanner {
    #[must_use]
    pub fn compute(
        config: WorkspaceTabOverflowConfig,
        measured_tabs: &[MeasuredWorkspaceTab],
        active_tab_id: Option<&WorkspaceTabId>,
    ) -> WorkspaceTabOverflowPlan {
        if fits_without_overflow(config.available_width, measured_tabs) {
            return plan_from_all_visible(measured_tabs);
        }

        let visible_ids = visible_tab_set(config, measured_tabs, active_tab_id);
        let visible_tab_ids = ordered_by_measurement(measured_tabs, &visible_ids, true);
        let hidden_tab_ids = ordered_by_measurement(measured_tabs, &visible_ids, false);

        WorkspaceTabOverflowPlan {
            visible_tab_ids,
            hidden_tab_ids,
            overflow_visible: true,
        }
    }
}

fn fits_without_overflow(available_width: u16, measured_tabs: &[MeasuredWorkspaceTab]) -> bool {
    let total_width: u16 = measured_tabs.iter().map(|tab| tab.width).sum();
    total_width <= available_width
}

fn plan_from_all_visible(measured_tabs: &[MeasuredWorkspaceTab]) -> WorkspaceTabOverflowPlan {
    WorkspaceTabOverflowPlan {
        visible_tab_ids: measured_tabs.iter().map(|tab| tab.tab_id.clone()).collect(),
        hidden_tab_ids: Vec::new(),
        overflow_visible: false,
    }
}

fn visible_tab_set(
    config: WorkspaceTabOverflowConfig,
    measured_tabs: &[MeasuredWorkspaceTab],
    active_tab_id: Option<&WorkspaceTabId>,
) -> HashSet<WorkspaceTabId> {
    let mut remaining = config
        .available_width
        .saturating_sub(config.overflow_trigger_width);
    let mut visible_ids = HashSet::new();

    for tab in measured_tabs {
        if tab.width <= remaining {
            remaining -= tab.width;
            visible_ids.insert(tab.tab_id.clone());
        }
    }
    promote_active_tab(measured_tabs, active_tab_id, &mut visible_ids);
    visible_ids
}

fn promote_active_tab(
    measured_tabs: &[MeasuredWorkspaceTab],
    active_tab_id: Option<&WorkspaceTabId>,
    visible_ids: &mut HashSet<WorkspaceTabId>,
) {
    let Some(active_id) = active_tab_id else {
        return;
    };
    if visible_ids.contains(active_id) || !measured_tabs.iter().any(|tab| &tab.tab_id == active_id)
    {
        return;
    }
    if let Some(last_visible) = last_visible_id(measured_tabs, visible_ids) {
        visible_ids.remove(&last_visible);
    }
    visible_ids.insert(active_id.clone());
}

fn last_visible_id(
    measured_tabs: &[MeasuredWorkspaceTab],
    visible_ids: &HashSet<WorkspaceTabId>,
) -> Option<WorkspaceTabId> {
    measured_tabs
        .iter()
        .rev()
        .find(|tab| visible_ids.contains(&tab.tab_id))
        .map(|tab| tab.tab_id.clone())
}

fn ordered_by_measurement(
    measured_tabs: &[MeasuredWorkspaceTab],
    visible_ids: &HashSet<WorkspaceTabId>,
    visible: bool,
) -> Vec<WorkspaceTabId> {
    measured_tabs
        .iter()
        .filter(|tab| visible_ids.contains(&tab.tab_id) == visible)
        .map(|tab| tab.tab_id.clone())
        .collect()
}
