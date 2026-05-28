use super::super::{StorybookWindowState, apply_click, click_content_y};
use crate::catalog::story_map::{StoryGroup, StorySection};
use crate::visual::navigation_tree::{NavigationRow, row_from_click};
use crate::visual::panel_scroll_state::PanelScrollOffsets;
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
    click_preset(&mut state, 2);
    click_preset(&mut state, 3);
    click_page(&mut state, "button");
}

#[test]
fn preset_tab_selection_is_owned_by_component() {
    let mut state = StorybookWindowState {
        selected_page: "button",
        ..StorybookWindowState::default()
    };

    click_preset(&mut state, 3);
    click_page(&mut state, "text-button");
    assert_eq!(0, state.preset_index);

    click_preset(&mut state, 1);
    click_page(&mut state, "button");
    assert_eq!(3, state.preset_index);

    click_page(&mut state, "text-button");
    assert_eq!(1, state.preset_index);
}

#[test]
fn theme_tokens_accepts_component_owned_preset_tabs() {
    let mut state = StorybookWindowState {
        selected_page: "theme-tokens",
        ..StorybookWindowState::default()
    };
    let light_palette = layout_metrics::preset_tab_rect(1);

    assert!(apply_click(
        &mut state,
        light_palette.x + 1,
        light_palette.y + 1
    ));
    assert_eq!(1, state.preset_index);
}

#[test]
fn click_mapping_toggles_tree_groups() {
    let mut state = StorybookWindowState::default();
    let target = group_click_target(StoryGroup::Atoms);

    assert!(target.is_some());
    assert!(state.tree_expansion.is_open(StoryGroup::Atoms));
    if let Some((x, y)) = target {
        assert!(apply_click(&mut state, x, y));
    }
    assert!(!state.tree_expansion.is_open(StoryGroup::Atoms));
}

#[test]
fn click_group_toggle_keeps_selected_page() {
    let mut state = StorybookWindowState::default();
    click_page(&mut state, "text");
    let target = group_click_target(StoryGroup::Atoms);
    assert!(target.is_some());
    assert_eq!("text", state.selected_page);

    if let Some((x, y)) = target {
        assert!(apply_click(&mut state, x, y));
    }

    assert_eq!("text", state.selected_page);
    assert!(!state.tree_expansion.is_open(StoryGroup::Atoms));

    if let Some((x, y)) = target {
        assert!(apply_click(&mut state, x, y));
    }
    assert!(state.tree_expansion.is_open(StoryGroup::Atoms));
    assert_eq!("text", state.selected_page);
}

#[test]
fn click_mapping_toggles_tree_sections() {
    let mut state = StorybookWindowState::default();
    let target = section_click_target(StoryGroup::Forms, StorySection::Selection);

    assert!(target.is_some());
    assert!(
        state
            .tree_expansion
            .is_section_open(StoryGroup::Forms, StorySection::Selection)
    );
    if let Some((x, y)) = target {
        assert!(apply_click(&mut state, x, y));
    }
    assert!(
        !state
            .tree_expansion
            .is_section_open(StoryGroup::Forms, StorySection::Selection)
    );

    if let Some((x, y)) = target {
        assert!(apply_click(&mut state, x, y));
    }
    assert!(
        state
            .tree_expansion
            .is_section_open(StoryGroup::Forms, StorySection::Selection)
    );
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

#[test]
fn viewport_click_mapping_keeps_navigation_rows_aligned_after_root_scroll() {
    let mut state = StorybookWindowState {
        scroll_y: layout_metrics::SCROLL_STEP,
        panel_scroll: PanelScrollOffsets {
            root_y: layout_metrics::SCROLL_STEP,
            navigation_y: layout_metrics::NAV_ROW_STEP,
            ..PanelScrollOffsets::default()
        },
        ..StorybookWindowState::default()
    };
    let target = visible_navigation_target(&state);

    assert!(target.is_some());
    if let Some((x, logical_y, target_page)) = target {
        let visible_y =
            logical_y.saturating_sub(state.panel_scroll.navigation_y + state.panel_scroll.root_y);
        let content_y = click_content_y(&state, x, visible_y);

        assert!(apply_click(&mut state, x, content_y));
        assert_eq!(target_page, state.selected_page);
    }
}

#[test]
fn click_mapping_can_select_nested_story_with_navigation_scroll() {
    let mut state = StorybookWindowState {
        selected_page: "button",
        panel_scroll: PanelScrollOffsets {
            root_y: layout_metrics::SCROLL_STEP,
            navigation_y: layout_metrics::NAV_ROW_STEP,
            ..PanelScrollOffsets::default()
        },
        ..StorybookWindowState::default()
    };
    let target = click_target_for_page_in_state("select-box", &state);

    assert!(target.is_some());
    if let Some((x, logical_y)) = target {
        let visible_y =
            logical_y.saturating_sub(state.panel_scroll.navigation_y + state.panel_scroll.root_y);
        let content_y = click_content_y(&state, x, visible_y);
        assert!(apply_click(&mut state, x, content_y));
        assert_eq!("select-box", state.selected_page);
    }
}

#[test]
fn navigation_page_row_hit_target_accepts_last_pixel_and_rejects_outside_x() {
    let mut state = StorybookWindowState::default();
    let target = click_target_for_page("tree-view");
    assert!(target.is_some());
    if let Some((_, y)) = target {
        let hit_x = layout_metrics::NAV_ROW_X + layout_metrics::NAV_ROW_WIDTH - 1;
        let outside_x = layout_metrics::NAV_ROW_X + layout_metrics::NAV_ROW_WIDTH;
        assert!(apply_click(&mut state, hit_x, y));
        assert_eq!("tree-view", state.selected_page);
        assert!(!apply_click(&mut state, outside_x, y));
    }
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
    click_target_for_page_in_state(page, &StorybookWindowState::default())
}

fn click_target_for_page_in_state(
    page: &str,
    state: &StorybookWindowState,
) -> Option<(usize, usize)> {
    for y in 0..layout_metrics::CONTENT_HEIGHT {
        let x = layout_metrics::NAV_ROW_X + 1;
        let logical_y =
            y.saturating_add(state.panel_scroll.navigation_y + state.panel_scroll.root_y);
        if let Some(row) = row_from_click(x, logical_y, state.tree_expansion) {
            let is_target = matches!(
                row,
                NavigationRow::Page { page: found, .. }
                | NavigationRow::PageWithoutSection { page: found, .. }
                    if found == page
            );
            if is_target {
                return Some((x, logical_y));
            }
        }
    }
    None
}

fn visible_navigation_target(state: &StorybookWindowState) -> Option<(usize, usize, &'static str)> {
    let panel = layout_metrics::navigation_menu_panel_rect();
    let max_visible = panel
        .bottom()
        .saturating_sub(state.panel_scroll.root_y + state.panel_scroll.navigation_y);
    for visible_y in 0..max_visible {
        let logical_y = visible_y + state.panel_scroll.root_y + state.panel_scroll.navigation_y;
        if let Some(
            NavigationRow::Page { page, .. } | NavigationRow::PageWithoutSection { page, .. },
        ) = row_from_click(
            layout_metrics::NAV_ROW_X + 1,
            logical_y,
            state.tree_expansion,
        ) {
            return Some((layout_metrics::NAV_ROW_X + 1, logical_y, page));
        }
    }
    None
}

fn group_click_target(group: StoryGroup) -> Option<(usize, usize)> {
    for y in 0..render::HEIGHT {
        let x = layout_metrics::NAV_ROW_X + 1;
        if row_from_click(x, y, Default::default()) == Some(NavigationRow::Group(group)) {
            return Some((x, y));
        }
    }
    None
}

fn section_click_target(group: StoryGroup, section: StorySection) -> Option<(usize, usize)> {
    for y in 0..render::HEIGHT {
        let x = layout_metrics::NAV_ROW_X + 1;
        if row_from_click(x, y, Default::default())
            == Some(NavigationRow::Section { group, section })
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
