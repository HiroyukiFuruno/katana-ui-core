use super::dedicated_tabs_metrics::{
    GROUP_HEADER_WIDTH, STRIP_LEADING_INSET, STRIP_WIDTH, TAB_GAP, tab_width,
};
use super::screen_state_tabs::TabsScreenState;
use katana_ui_core::widget::molecules::{
    CloseableTabId, CloseableTabScrollConfig, CloseableTabScrollPlanner, MeasuredCloseableTab,
};

pub(super) fn scroll_x(state: &TabsScreenState) -> usize {
    let measured = measured_items(state);
    let active = CloseableTabId::new(state.active_tab_id.clone());
    let plan = CloseableTabScrollPlanner::follow_active(
        CloseableTabScrollConfig::new(viewport_width(), state.scroll_x as u32),
        &measured,
        Some(&active),
    );
    plan.scroll_x as usize
}

const fn viewport_width() -> u16 {
    (STRIP_WIDTH - STRIP_LEADING_INSET * 2) as u16
}

fn measured_items(state: &TabsScreenState) -> Vec<MeasuredCloseableTab> {
    let mut items = Vec::new();
    push_pinned_items(&mut items, state);
    push_grouped_items(&mut items, state);
    push_unknown_group_items(&mut items, state);
    push_ungrouped_items(&mut items, state);
    items
}

#[cfg(test)]
pub(super) fn measured_item_ids_for_test(state: &TabsScreenState) -> Vec<String> {
    measured_items(state)
        .into_iter()
        .map(|item| item.tab_id.as_str().to_string())
        .collect()
}

fn push_grouped_items(items: &mut Vec<MeasuredCloseableTab>, state: &TabsScreenState) {
    for group in &state.groups {
        if !state
            .tabs
            .iter()
            .any(|tab| !tab.pinned && tab.group_id.as_deref() == Some(group.id.as_str()))
        {
            continue;
        }
        items.push(measured(format!("group:{}", group.id), GROUP_HEADER_WIDTH));
        if group.collapsed {
            continue;
        }
        for tab in state
            .tabs
            .iter()
            .filter(|tab| !tab.pinned && tab.group_id.as_deref() == Some(group.id.as_str()))
        {
            items.push(measured(tab.id.clone(), tab_width(tab)));
        }
    }
}

fn push_pinned_items(items: &mut Vec<MeasuredCloseableTab>, state: &TabsScreenState) {
    for tab in state.tabs.iter().filter(|tab| tab.pinned) {
        items.push(measured(tab.id.clone(), tab_width(tab)));
    }
}

fn push_ungrouped_items(items: &mut Vec<MeasuredCloseableTab>, state: &TabsScreenState) {
    for tab in state
        .tabs
        .iter()
        .filter(|tab| !tab.pinned && tab.group_id.is_none())
    {
        items.push(measured(tab.id.clone(), tab_width(tab)));
    }
}

fn push_unknown_group_items(items: &mut Vec<MeasuredCloseableTab>, state: &TabsScreenState) {
    for tab in state.tabs.iter().filter(|tab| {
        !tab.pinned
            && tab.group_id.as_ref().is_some_and(|group_id| {
                state
                    .groups
                    .iter()
                    .all(|group| group.id.as_str() != group_id.as_str())
            })
    }) {
        items.push(measured(tab.id.clone(), tab_width(tab)));
    }
}

fn measured(id: impl Into<String>, width: usize) -> MeasuredCloseableTab {
    MeasuredCloseableTab::new(id.into(), width_with_gap(width))
}

fn width_with_gap(width: usize) -> u16 {
    u16::try_from(width + TAB_GAP).unwrap_or(u16::MAX)
}
