use crate::visual::dedicated_dod_form_input_live as input_live;
use crate::visual::layout_metrics::MAX_SCROLL_Y;
use crate::visual::panel_scroll_state::{self, PanelScrollRegion};
use crate::visual::panel_scrollbars;

use super::StorybookWindowState;
use super::panel_scroll_drag::PanelScrollDragTarget;

#[path = "scroll_operation/scroll_hit.rs"]
mod scroll_hit;
#[path = "scroll_operation/scroll_limits.rs"]
mod scroll_limits;

pub(super) fn apply_scroll_delta(state: &mut StorybookWindowState, delta_y: f32) -> bool {
    apply_scroll_delta_at_root(state, delta_y)
}

pub(super) fn apply_scroll_delta_at(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
    delta_y: f32,
) -> bool {
    if delta_y == 0.0 {
        return false;
    }
    if scroll_hit::preset_tab_at(state, x, y) {
        return state.scroll_preset_tabs(delta_y);
    }
    let region =
        panel_scroll_state::PanelScrollRegionModel::region_at(x, y + state.panel_scroll.root_y);
    let mut changed = scroll_limits::clamp_vertical_offset(state, region);
    if scroll_hit::text_area_at(state, region, x, y) {
        let instance =
            super::component_instance_id_for_page(state.selected_page, state.selected_instance_id);
        let vertical_enabled = input_live::text_area_vertical_scroll_enabled_for_instance(
            state.preset_index,
            &state.screen_state,
            instance,
        );
        let max_y = input_live::text_area_vertical_scroll_max_offset_for_instance(
            state.preset_index,
            &state.screen_state,
            instance,
        );
        let vertical_changed = state.screen_state.scroll_text_area_vertical_for(
            instance,
            delta_y,
            vertical_enabled,
            max_y,
        );
        if vertical_changed || changed {
            return true;
        }
        let horizontal_enabled = input_live::text_area_horizontal_scroll_enabled_for_instance(
            state.preset_index,
            &state.screen_state,
            instance,
        );
        let max_x = input_live::text_area_horizontal_scroll_max_offset_for_instance(
            state.preset_index,
            &state.screen_state,
            instance,
        );
        return state.screen_state.scroll_text_area_horizontal_for(
            instance,
            delta_y,
            horizontal_enabled,
            max_x,
        );
    }
    if scroll_hit::list_at(state, x, y) {
        state.screen_state.register_list_scroll();
        return true;
    }
    if scroll_hit::select_box_at(state, x, y) {
        state.screen_state.register_selection_action(
            crate::visual::selection_screen_state::SelectionScreenAction::SelectScroll,
        );
        return true;
    }
    if scroll_hit::scroll_area_at(state, x, y) {
        state.screen_state.register_scroll_area_action(
            crate::visual::window_interaction::scroll_area_operation::ScrollAreaStoryAction::Scroll,
        );
        let _preview_scroll_changed = state.panel_scroll.scroll_delta_with_max(
            PanelScrollRegion::Preview,
            scroll_limits::max_scroll_y(state, PanelScrollRegion::Preview),
            delta_y,
        );
        return true;
    }
    if scroll_hit::tree_view_at(state, x, y) {
        return state.screen_state.scroll_tree_view(delta_y);
    }
    if component_at(state, "code-diff", x, y) {
        state.screen_state.register_code_diff_scroll_sync();
        return true;
    }
    if component_at(state, "selection-list", x, y) {
        state.screen_state.register_selection_action(
            crate::visual::selection_screen_state::SelectionScreenAction::SelectionListScroll,
        );
        return true;
    }
    if component_at(state, "side-menu", x, y) {
        state.screen_state.register_side_menu_action(
            crate::visual::screen_state_side_menu::SideMenuScreenAction::Scroll,
        );
        return true;
    }
    if component_at(state, "shortcut-cheatsheet", x, y) {
        state.screen_state.register_shortcut_cheatsheet_scroll();
        return true;
    }
    if component_at(state, "settings-list", x, y) {
        state.screen_state.register_settings_list_action(
            crate::visual::window_interaction::settings_list_operation::SettingsListStoryAction::Scroll,
        );
        return true;
    }
    if component_at(state, "diagnostics-list", x, y) {
        state.screen_state.register_diagnostics_list_action(
            crate::visual::window_interaction::diagnostics_list_operation::DiagnosticsListStoryAction::ScrollRetention,
        );
        return true;
    }
    if component_at(state, "virtualization", x, y) {
        state.screen_state.register_virtualization_action(
            crate::visual::window_interaction::virtualization_state::VirtualizationStoryAction::Scroll,
        );
        return true;
    }
    if let Some(panel) = scroll_hit::panel_child_at(state, region, x, y) {
        return state.screen_state.scroll_panel_vertical(panel, delta_y) || changed;
    }
    if !panel_scrollbars::vertical_region_scrollable_for(
        region,
        state.selected_page,
        state.tree_expansion,
    ) {
        return changed;
    }
    changed |= state.panel_scroll.scroll_delta_with_max(
        region,
        scroll_limits::max_scroll_y(state, region),
        delta_y,
    );
    if region == PanelScrollRegion::Root {
        state.scroll_y = state.panel_scroll.root_y;
    }
    changed
}

fn component_at(state: &StorybookWindowState, page: &str, x: usize, y: usize) -> bool {
    state.selected_page == page
        && crate::visual::preview_detail::component_action_hit_rect(page).contains(x, y)
}

pub(super) fn apply_scroll_delta_x_at(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
    delta_x: f32,
) -> bool {
    if delta_x == 0.0 {
        return false;
    }
    if scroll_hit::preset_tab_at(state, x, y) {
        return state.scroll_preset_tabs(delta_x);
    }
    if scroll_hit::tab_strip_at(state, x, y) {
        state.screen_state.register_tabs_horizontal_scroll(delta_x);
        return true;
    }
    let region =
        panel_scroll_state::PanelScrollRegionModel::region_at(x, y + state.panel_scroll.root_y);
    let changed = scroll_limits::clamp_horizontal_offset(state, region);
    if scroll_hit::text_area_at(state, region, x, y) {
        let instance =
            super::component_instance_id_for_page(state.selected_page, state.selected_instance_id);
        let enabled = input_live::text_area_horizontal_scroll_enabled_for_instance(
            state.preset_index,
            &state.screen_state,
            instance,
        );
        let max_x = input_live::text_area_horizontal_scroll_max_offset_for_instance(
            state.preset_index,
            &state.screen_state,
            instance,
        );
        return state
            .screen_state
            .scroll_text_area_horizontal_for(instance, delta_x, enabled, max_x)
            || changed;
    }
    if let Some(panel) = scroll_hit::panel_child_at(state, region, x, y) {
        return state.screen_state.scroll_panel_horizontal(panel, delta_x) || changed;
    }
    changed
}

fn apply_scroll_delta_at_root(state: &mut StorybookWindowState, delta_y: f32) -> bool {
    let changed =
        state
            .panel_scroll
            .scroll_delta_with_max(PanelScrollRegion::Root, MAX_SCROLL_Y, delta_y);
    state.scroll_y = state.panel_scroll.root_y.min(MAX_SCROLL_Y);
    changed
}

pub(super) fn apply_scrollbar_drag(
    state: &mut StorybookWindowState,
    region: PanelScrollRegion,
    y: usize,
) -> bool {
    let mut changed = scroll_limits::clamp_vertical_offset(state, region);
    let next = panel_scrollbars::offset_from_drag_for(
        region,
        y,
        state.selected_page,
        state.tree_expansion,
    );
    changed |= state.panel_scroll.set_drag_offset_with_max(
        region,
        next,
        scroll_limits::max_scroll_y(state, region),
    );
    if region == PanelScrollRegion::Root {
        state.scroll_y = state.panel_scroll.root_y.min(MAX_SCROLL_Y);
    }
    changed
}

pub(super) fn apply_scrollbar_drag_target(
    state: &mut StorybookWindowState,
    target: PanelScrollDragTarget,
    _x: usize,
    y: usize,
) -> bool {
    match target {
        PanelScrollDragTarget::Vertical(region) => apply_scrollbar_drag(state, region, y),
    }
}

#[cfg(test)]
pub(super) fn apply_horizontal_scrollbar_drag(
    state: &mut StorybookWindowState,
    region: PanelScrollRegion,
    x: usize,
) -> bool {
    let mut changed = scroll_limits::clamp_horizontal_offset(state, region);
    let next = panel_scrollbars::horizontal_offset_from_drag_for(
        region,
        x,
        state.selected_page,
        state.tree_expansion,
    );
    changed |= state.panel_scroll.set_drag_offset_x_with_max(
        region,
        next,
        scroll_limits::max_scroll_x(state, region),
    );
    changed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::preview_detail;

    #[test]
    fn component_wheel_routes_cover_specialized_scroll_actions() {
        for page in [
            "list",
            "select-box",
            "code-diff",
            "selection-list",
            "side-menu",
            "shortcut-cheatsheet",
            "settings-list",
            "diagnostics-list",
            "virtualization",
        ] {
            let rect = preview_detail::component_action_hit_rect(page);
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };

            assert!(
                apply_scroll_delta_at(&mut state, rect.x + 1, rect.y + 1, -1.0),
                "{page} must consume wheel input inside its component surface"
            );
            assert_ne!("none", state.screen_state.last_action, "{page}");
        }
    }

    #[test]
    fn zero_deltas_and_root_drag_boundaries_are_explicit() {
        let mut state = StorybookWindowState::default();
        assert!(!apply_scroll_delta_at(&mut state, 0, 0, 0.0));
        assert!(!apply_scroll_delta_x_at(&mut state, 0, 0, 0.0));

        let changed = apply_scrollbar_drag(&mut state, PanelScrollRegion::Root, usize::MAX);
        assert!(changed);
        assert_eq!(state.panel_scroll.root_y, state.scroll_y);
    }

    #[test]
    fn horizontal_wheel_over_overflowing_preset_tabs_changes_only_tab_offset() {
        let mut state = StorybookWindowState {
            selected_page: "text-input",
            ..StorybookWindowState::default()
        };
        let viewport = crate::visual::preset_tab_scroll::viewport_rect();

        assert!(apply_scroll_delta_x_at(
            &mut state,
            viewport.x + 1,
            viewport.y + 1,
            -1.0,
        ));
        assert!(state.preset_tab_scroll_x > 0);
        assert_eq!(0, state.scroll_y);
    }
}
