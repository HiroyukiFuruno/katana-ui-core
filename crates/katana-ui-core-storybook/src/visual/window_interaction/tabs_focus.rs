use super::StorybookWindowState;
use crate::visual::{
    dedicated_closeable_tab_strip, dedicated_tabs, layout_metrics::LayoutRect, preview_detail,
};

pub(super) fn focus_at(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !is_tabs_focus_page(state.selected_page) {
        return false;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    let Some((tab_id, _)) = tab_hit_at(state, origin.x, origin.y, x, y) else {
        return false;
    };
    state.screen_state.register_tabs_focus(&tab_id);
    true
}

fn tab_hit_at(
    state: &StorybookWindowState,
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
) -> Option<(String, LayoutRect)> {
    if state.selected_page == "closeable-tab-strip" {
        return dedicated_closeable_tab_strip::tab_hit_at(
            origin_x,
            origin_y,
            x,
            y,
            &state.screen_state.tabs,
        );
    }
    dedicated_tabs::tab_hit_at(origin_x, origin_y, x, y, &state.screen_state.tabs)
}

fn is_tabs_focus_page(page: &str) -> bool {
    page == "tabs" || page == "closeable-tab-strip"
}
