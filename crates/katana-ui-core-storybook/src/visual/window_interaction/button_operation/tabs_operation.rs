use super::{StorybookButtonOperation, StorybookWindowState};
use crate::visual::{dedicated_closeable_tab_strip, dedicated_tabs, preview_detail};

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if state.selected_page != "tabs" {
        return closeable_tab_strip_operation_at(state, x, y);
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    if let Some(tab_id) =
        dedicated_tabs::pin_icon_hit_at(origin.x, origin.y, x, y, &state.screen_state.tabs)
    {
        return Some(StorybookButtonOperation::TabsPinIcon { tab_id });
    }
    dedicated_tabs::control_at(origin.x, origin.y, x, y).map(StorybookButtonOperation::TabsControl)
}

fn closeable_tab_strip_operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if state.selected_page != "closeable-tab-strip" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    if let Some(action) = dedicated_closeable_tab_strip::control_at(origin.x, origin.y, x, y) {
        return Some(StorybookButtonOperation::TabsControl(action));
    }
    dedicated_closeable_tab_strip::tab_hit_at(origin.x, origin.y, x, y, &state.screen_state.tabs)
        .map(|(tab_id, _)| StorybookButtonOperation::CloseableTabStripSelect { tab_id })
}
