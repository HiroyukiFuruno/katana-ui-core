use minifb::{MouseButton, MouseMode, Window};

use super::layout_metrics::{
    MAX_SCROLL_Y, PRESET_TAB_COUNT, SCROLL_STEP, dark_theme_rect, light_theme_rect,
    preset_tab_rect, scrollbar_off_rect, scrollbar_on_rect,
};
use super::navigation_tree::{NavigationRow, TreeExpansionState, row_from_click};

const DEFAULT_SELECTED_PAGE: &str = "button";
const DEFAULT_THEME_ID: &str = "dark";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StorybookWindowState {
    pub(super) selected_page: &'static str,
    pub(super) theme_id: &'static str,
    pub(super) preset_index: usize,
    pub(super) scroll_y: usize,
    pub(super) scrollbar_visible: bool,
    pub(super) tree_expansion: TreeExpansionState,
}

impl Default for StorybookWindowState {
    fn default() -> Self {
        Self {
            selected_page: DEFAULT_SELECTED_PAGE,
            theme_id: DEFAULT_THEME_ID,
            preset_index: 0,
            scroll_y: 0,
            scrollbar_visible: true,
            tree_expansion: TreeExpansionState::default(),
        }
    }
}

pub(super) fn apply_scroll(window: &Window, state: &mut StorybookWindowState) -> bool {
    let Some((_, delta_y)) = window.get_scroll_wheel() else {
        return false;
    };
    if delta_y == 0.0 {
        return false;
    }
    apply_scroll_delta(state, delta_y)
}

pub(super) fn apply_mouse_click(
    window: &Window,
    state: &mut StorybookWindowState,
    mouse_was_down: &mut bool,
) -> bool {
    let mouse_down = window.get_mouse_down(MouseButton::Left);
    let click_started = mouse_down && !*mouse_was_down;
    *mouse_was_down = mouse_down;
    if !click_started {
        return false;
    }
    let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) else {
        return false;
    };
    apply_click(state, x as usize, y as usize + state.scroll_y)
}

fn apply_click(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if light_theme_rect().contains(x, y) {
        state.theme_id = "light";
        return true;
    }
    if dark_theme_rect().contains(x, y) {
        state.theme_id = "dark";
        return true;
    }
    if scrollbar_on_rect().contains(x, y) {
        state.scrollbar_visible = true;
        return true;
    }
    if scrollbar_off_rect().contains(x, y) {
        state.scrollbar_visible = false;
        return true;
    }
    if let Some(preset_index) = preset_index_from_click(x, y) {
        state.preset_index = preset_index;
        return true;
    }
    if let Some(row) = row_from_click(x, y, state.tree_expansion) {
        match row {
            NavigationRow::Group(group) => state.tree_expansion.toggle(group),
            NavigationRow::Page { page, .. } => state.selected_page = page,
        }
        return true;
    }
    false
}

fn preset_index_from_click(x: usize, y: usize) -> Option<usize> {
    (0..PRESET_TAB_COUNT).find(|index| preset_tab_rect(*index).contains(x, y))
}

fn apply_scroll_delta(state: &mut StorybookWindowState, delta_y: f32) -> bool {
    let before = state.scroll_y;
    if delta_y < 0.0 {
        state.scroll_y = (state.scroll_y + SCROLL_STEP).min(MAX_SCROLL_Y);
    } else {
        state.scroll_y = state.scroll_y.saturating_sub(SCROLL_STEP);
    }
    before != state.scroll_y
}

#[cfg(test)]
mod tests {
    use super::{StorybookWindowState, apply_click, apply_scroll_delta};
    use crate::visual::navigation_tree::{NavigationGroup, row_from_click};
    use crate::visual::{layout_metrics, render};

    const DIFF_THRESHOLD: usize = 8_000;

    #[test]
    fn click_mapping_updates_theme_preset_and_story_selection() {
        let mut state = StorybookWindowState::default();
        let light = layout_metrics::light_theme_rect();
        let interactive = layout_metrics::preset_tab_rect(layout_metrics::PRESET_INTERACTIVE_INDEX);

        assert!(apply_click(&mut state, light.x + 1, light.y + 1));
        assert_eq!("light", state.theme_id);
        assert!(apply_click(
            &mut state,
            interactive.x + 1,
            interactive.y + 1
        ));
        assert_eq!(1, state.preset_index);
        let scrollbar_off = layout_metrics::scrollbar_off_rect();
        assert!(apply_click(
            &mut state,
            scrollbar_off.x + 1,
            scrollbar_off.y + 1
        ));
        assert!(!state.scrollbar_visible);
        let edge = layout_metrics::preset_tab_rect(2);
        assert!(apply_click(&mut state, edge.x + 1, edge.y + 1));
        assert_eq!(2, state.preset_index);
        let theme = layout_metrics::preset_tab_rect(3);
        assert!(apply_click(&mut state, theme.x + 1, theme.y + 1));
        assert_eq!(3, state.preset_index);
        let button_target = click_target_for_page("button");
        assert!(button_target.is_some());
        if let Some((x, y)) = button_target {
            assert!(apply_click(&mut state, x, y));
            assert_eq!("button", state.selected_page);
        }
    }

    #[test]
    fn click_mapping_toggles_tree_groups() {
        let mut state = StorybookWindowState::default();
        let target = group_click_target(NavigationGroup::Atoms);

        assert!(target.is_some());
        assert!(state.tree_expansion.is_open(NavigationGroup::Atoms));
        if let Some((x, y)) = target {
            assert!(apply_click(&mut state, x, y));
        }
        assert!(!state.tree_expansion.is_open(NavigationGroup::Atoms));
    }

    #[test]
    fn click_mapping_can_select_visible_story_and_change_rendered_scene() {
        let mut state = StorybookWindowState::default();
        let before = render::render_storybook_canvas_for(
            state.theme_id,
            state.selected_page,
            state.preset_index > 0,
        );
        let card_target = click_target_for_page("card");

        assert!(card_target.is_some());
        if let Some((x, y)) = card_target {
            assert!(apply_click(&mut state, x, y));
            assert_eq!("card", state.selected_page);
            let after = render::render_storybook_canvas_for(
                state.theme_id,
                state.selected_page,
                state.preset_index > 0,
            );
            assert!(pixel_diff(&before, &after) > DIFF_THRESHOLD);
        }
    }

    #[test]
    fn scroll_delta_updates_vertical_viewport() {
        let mut state = StorybookWindowState::default();

        assert!(apply_scroll_delta(&mut state, -1.0));
        assert_eq!(layout_metrics::SCROLL_STEP, state.scroll_y);
        assert!(apply_scroll_delta(&mut state, 1.0));
        assert_eq!(0, state.scroll_y);
    }

    #[test]
    fn clicking_outside_controls_does_not_mutate_state() {
        let mut state = StorybookWindowState::default();
        let original = state.clone();

        assert!(!apply_click(&mut state, 0, 0));
        assert_eq!(original, state);
    }

    fn click_target_for_page(page: &str) -> Option<(usize, usize)> {
        for y in 0..render::HEIGHT {
            let x = layout_metrics::NAV_ROW_X + 1;
            if matches!(
                row_from_click(x, y, Default::default()),
                Some(crate::visual::navigation_tree::NavigationRow::Page { page: found, .. })
                    if found == page
            ) {
                return Some((x, y));
            }
        }
        None
    }

    fn group_click_target(group: NavigationGroup) -> Option<(usize, usize)> {
        for y in 0..render::HEIGHT {
            let x = layout_metrics::NAV_ROW_X + 1;
            if row_from_click(x, y, Default::default())
                == Some(crate::visual::navigation_tree::NavigationRow::Group(group))
            {
                return Some((x, y));
            }
        }
        None
    }

    fn pixel_diff(before: &crate::visual::Canvas, after: &crate::visual::Canvas) -> usize {
        before
            .pixels()
            .iter()
            .zip(after.pixels().iter())
            .filter(|(left, right)| left != right)
            .count()
    }
}
