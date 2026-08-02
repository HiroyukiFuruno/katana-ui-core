use super::StorybookWindowState;
use crate::visual::{
    dedicated_closeable_tab_strip, dedicated_tabs, layout_metrics::LayoutRect, preview_detail,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::visual) struct TabsDragTarget {
    pub(super) tab_id: String,
    pub(super) committed: bool,
}

pub(super) fn start_at(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !is_tab_drag_page(state.selected_page) || tab_pin_or_control_at(state, x, y) {
        return false;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    let Some((tab_id, _)) = tab_hit_at(state, origin.x, origin.y, x, y) else {
        return false;
    };
    state.screen_state.register_tabs_drag_start(&tab_id);
    state.tabs_drag_target = Some(TabsDragTarget {
        tab_id,
        committed: false,
    });
    true
}

pub(super) fn apply_drag_at(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    let Some(target) = state.tabs_drag_target.clone() else {
        return false;
    };
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    let Some((target_tab_id, rect)) = tab_hit_at(state, origin.x, origin.y, x, y) else {
        return false;
    };
    if target_tab_id == target.tab_id {
        return false;
    }
    let Some(to_visual_index) = drop_visual_index(
        &state.screen_state.tabs.core_visual_tab_ids(),
        &target.tab_id,
        &target_tab_id,
        x,
        rect.x + rect.width / 2,
    ) else {
        return false;
    };
    state
        .screen_state
        .register_tabs_drag_move(&target.tab_id, to_visual_index);
    if let Some(active) = state.tabs_drag_target.as_mut() {
        active.committed = true;
    }
    true
}

pub(super) fn release(state: &mut StorybookWindowState) -> bool {
    let Some(target) = state.tabs_drag_target.take() else {
        return false;
    };
    state
        .screen_state
        .register_tabs_drag_end(&target.tab_id, target.committed);
    true
}

fn tab_pin_or_control_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    if state.selected_page == "tabs" {
        return dedicated_tabs::pin_icon_hit_at(origin.x, origin.y, x, y, &state.screen_state.tabs)
            .is_some()
            || dedicated_tabs::control_at(origin.x, origin.y, x, y).is_some();
    }
    state.selected_page == "closeable-tab-strip"
        && dedicated_closeable_tab_strip::control_at(origin.x, origin.y, x, y).is_some()
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

fn is_tab_drag_page(page: &str) -> bool {
    page == "tabs" || page == "closeable-tab-strip"
}

fn drop_visual_index(
    visual_ids: &[String],
    dragged_tab_id: &str,
    target_tab_id: &str,
    pointer_x: usize,
    target_mid_x: usize,
) -> Option<usize> {
    let from = visual_ids
        .iter()
        .position(|tab_id| tab_id == dragged_tab_id)?;
    let target = visual_ids
        .iter()
        .position(|tab_id| tab_id == target_tab_id)?;
    let drop_after = pointer_x >= target_mid_x;
    Some(match (from < target, drop_after) {
        (true, true) => target,
        (true, false) => target.saturating_sub(1),
        (false, true) => target + 1,
        (false, false) => target,
    })
}

#[cfg(test)]
mod tests {
    use super::{StorybookWindowState, TabsDragTarget, apply_drag_at, drop_visual_index, start_at};
    use crate::test_assert::KucTestExpect;
    use crate::visual::{dedicated_tabs, preview_detail};

    #[test]
    fn tabs_drag_rejects_empty_same_and_missing_targets_and_maps_all_drop_sides() {
        let mut state = StorybookWindowState {
            selected_page: "tabs",
            ..StorybookWindowState::default()
        };
        assert!(!start_at(&mut state, 0, 0));
        assert!(!apply_drag_at(&mut state, 0, 0));

        let tab_id = state.screen_state.tabs.core_visual_tab_ids()[0].clone();
        let rect = dedicated_tabs::tab_rect_for_test(&state.screen_state.tabs, &tab_id)
            .kuc_expect("default tabs must expose their first visual tab");
        let component = preview_detail::component_action_hit_rect("tabs");
        let x = component.x + rect.x + rect.width / 2;
        let y = component.y + rect.y + rect.height / 2;
        assert!(start_at(&mut state, x, y));
        assert!(!apply_drag_at(&mut state, x, y));

        state.tabs_drag_target = Some(TabsDragTarget {
            tab_id: "missing".to_string(),
            committed: false,
        });
        assert!(!apply_drag_at(&mut state, x, y));

        let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(Some(2), drop_visual_index(&ids, "a", "c", 10, 10));
        assert_eq!(Some(1), drop_visual_index(&ids, "a", "c", 9, 10));
        assert_eq!(Some(1), drop_visual_index(&ids, "c", "a", 10, 10));
        assert_eq!(Some(0), drop_visual_index(&ids, "c", "a", 9, 10));
        assert_eq!(None, drop_visual_index(&ids, "missing", "a", 9, 10));
        assert_eq!(None, drop_visual_index(&ids, "a", "missing", 9, 10));
    }
}
