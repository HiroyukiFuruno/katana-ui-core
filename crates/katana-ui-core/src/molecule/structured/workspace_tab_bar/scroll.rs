use super::identifiers::WorkspaceTabId;
use super::overflow::MeasuredWorkspaceTab;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabScrollConfig {
    pub viewport_width: u16,
    pub current_scroll_x: u32,
}

impl WorkspaceTabScrollConfig {
    #[must_use]
    pub const fn new(viewport_width: u16, current_scroll_x: u32) -> Self {
        Self {
            viewport_width,
            current_scroll_x,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabScrollPlan {
    pub scroll_x: u32,
    pub max_scroll_x: u32,
    pub total_width: u32,
    pub overflow_scroll_enabled: bool,
    pub active_tab_visible: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabScrollPlanner;

impl WorkspaceTabScrollPlanner {
    #[must_use]
    pub fn follow_active(
        config: WorkspaceTabScrollConfig,
        measured_tabs: &[MeasuredWorkspaceTab],
        active_tab_id: Option<&WorkspaceTabId>,
    ) -> WorkspaceTabScrollPlan {
        let total_width = total_width(measured_tabs);
        let viewport_width = u32::from(config.viewport_width);
        let max_scroll_x = total_width.saturating_sub(viewport_width);
        let Some((active_start, active_end)) = active_span(measured_tabs, active_tab_id) else {
            return scroll_plan(
                config.current_scroll_x.min(max_scroll_x),
                max_scroll_x,
                total_width,
            );
        };
        let scroll_x =
            followed_scroll_x(config, viewport_width, active_start, active_end).min(max_scroll_x);
        let active_tab_visible =
            active_start >= scroll_x && active_end <= scroll_x + viewport_width;

        WorkspaceTabScrollPlan {
            scroll_x,
            max_scroll_x,
            total_width,
            overflow_scroll_enabled: max_scroll_x > 0,
            active_tab_visible,
        }
    }
}

fn total_width(measured_tabs: &[MeasuredWorkspaceTab]) -> u32 {
    measured_tabs.iter().map(|tab| u32::from(tab.width)).sum()
}

fn active_span(
    measured_tabs: &[MeasuredWorkspaceTab],
    active_tab_id: Option<&WorkspaceTabId>,
) -> Option<(u32, u32)> {
    let active_tab_id = active_tab_id?;
    let mut start = 0;
    for tab in measured_tabs {
        let end = start + u32::from(tab.width);
        if &tab.tab_id == active_tab_id {
            return Some((start, end));
        }
        start = end;
    }
    None
}

fn followed_scroll_x(
    config: WorkspaceTabScrollConfig,
    viewport_width: u32,
    active_start: u32,
    active_end: u32,
) -> u32 {
    if active_start < config.current_scroll_x {
        return active_start;
    }
    if active_end > config.current_scroll_x + viewport_width {
        return active_end.saturating_sub(viewport_width);
    }
    config.current_scroll_x
}

fn scroll_plan(scroll_x: u32, max_scroll_x: u32, total_width: u32) -> WorkspaceTabScrollPlan {
    WorkspaceTabScrollPlan {
        scroll_x,
        max_scroll_x,
        total_width,
        overflow_scroll_enabled: max_scroll_x > 0,
        active_tab_visible: false,
    }
}
