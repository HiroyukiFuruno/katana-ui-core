use super::super::{StorybookWindowState, apply_click};
use crate::visual::navigation_tree::{NavigationGroup, NavigationRow, row_from_click};
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
    click_scrollbar_off(&mut state);
    click_preset(&mut state, 2);
    click_preset(&mut state, 3);
    click_page(&mut state, "button");
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
    click_page(&mut state, "card");
    let after = render::render_storybook_canvas_for(
        state.theme_id,
        state.selected_page,
        state.preset_index > 0,
    );

    assert!(pixel_diff(&before, &after) > DIFF_THRESHOLD);
}

fn click_scrollbar_off(state: &mut StorybookWindowState) {
    let scrollbar_off = layout_metrics::scrollbar_off_rect();
    assert!(apply_click(state, scrollbar_off.x + 1, scrollbar_off.y + 1));
    assert!(!state.scrollbar_visible);
}

fn click_preset(state: &mut StorybookWindowState, index: usize) {
    let rect = layout_metrics::preset_tab_rect(index);
    assert!(apply_click(state, rect.x + 1, rect.y + 1));
    assert_eq!(index, state.preset_index);
}

fn click_page(state: &mut StorybookWindowState, page: &'static str) {
    let target = click_target_for_page(page);

    assert!(target.is_some());
    if let Some((x, y)) = target {
        assert!(apply_click(state, x, y));
        assert_eq!(page, state.selected_page);
    }
}

fn click_target_for_page(page: &str) -> Option<(usize, usize)> {
    for y in 0..render::HEIGHT {
        let x = layout_metrics::NAV_ROW_X + 1;
        if matches!(
            row_from_click(x, y, Default::default()),
            Some(NavigationRow::Page { page: found, .. }) if found == page
        ) {
            return Some((x, y));
        }
    }
    None
}

fn group_click_target(group: NavigationGroup) -> Option<(usize, usize)> {
    for y in 0..render::HEIGHT {
        let x = layout_metrics::NAV_ROW_X + 1;
        if row_from_click(x, y, Default::default()) == Some(NavigationRow::Group(group)) {
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
