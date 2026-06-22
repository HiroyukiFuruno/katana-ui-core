use super::dedicated_dod_molecule_tree_parts as tree_parts;
use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{self, StorybookWindowState, apply_click};
use super::{StorybookVisual, preview_detail, render};

const PAGE: &str = "tree-view";
const DARK_THEME: &str = "dark";
const SCROLLED_TREE_OFFSET: u32 = 96;
const TREE_ROW_LABEL_CLICK_OFFSET: usize = 8;
const SCROLLED_FILE_ID: &str = "katana/nested/b.md";

#[test]
fn tree_view_click_after_scroll_uses_core_hit_target_and_keeps_viewport_rendered() {
    let top = StorybookVisual.render_preset(DARK_THEME, PAGE, 0, 0);
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    state.screen_state.tree_view_scroll_offset = SCROLLED_TREE_OFFSET;
    let before_click = render_state(&state);

    click_visible_tree_row(&mut state);
    let after_click = render_state(&state);

    assert_eq!(
        SCROLLED_TREE_OFFSET, state.screen_state.tree_view_scroll_offset,
        "TreeView click must keep the scrolled viewport instead of jumping to the top"
    );
    assert_eq!("tree_select_file", state.screen_state.last_action);
    assert_eq!("tree_selected", state.screen_state.last_event);
    assert_eq!(SCROLLED_FILE_ID, state.screen_state.last_setting_value);
    assert!(
        component_body_pixel_diff(PAGE, &top, &after_click) > 0,
        "click after scroll must not redraw the top viewport"
    );
    assert!(
        component_body_pixel_diff(PAGE, &before_click, &after_click) > 0,
        "selected row must repaint through TreeView public state"
    );
}

#[test]
fn tree_view_audit_scroll_uses_window_wheel_route_before_clicking_visible_row() {
    let top = StorybookVisual.render_preset(DARK_THEME, PAGE, 0, 0);
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let (x, y) = tree_panel_scroll_point();

    while state.screen_state.tree_view_scroll_offset < SCROLLED_TREE_OFFSET {
        assert!(window_interaction::apply_tree_view_scroll_for_audit(
            &mut state, x, y
        ));
    }
    assert_eq!(
        "wheel", state.screen_state.last_setting_value,
        "TreeView audit scroll must go through the same wheel route as window interaction"
    );
    let before_click = render_state(&state);

    click_visible_tree_row(&mut state);
    let after_click = render_state(&state);

    assert_eq!(
        SCROLLED_TREE_OFFSET, state.screen_state.tree_view_scroll_offset,
        "TreeView audit scroll must use the same retained wheel offset as the live window"
    );
    assert_eq!("tree_select_file", state.screen_state.last_action);
    assert_eq!("tree_selected", state.screen_state.last_event);
    assert_eq!(SCROLLED_FILE_ID, state.screen_state.last_setting_value);
    assert!(
        component_body_pixel_diff(PAGE, &top, &after_click) > 0,
        "audit wheel route must keep the scrolled viewport visible after click"
    );
    assert!(
        component_body_pixel_diff(PAGE, &before_click, &after_click) > 0,
        "visible row selection must repaint after audit scroll"
    );
}

fn click_visible_tree_row(state: &mut StorybookWindowState) {
    let x =
        preview_detail::HERO_PREVIEW_X_FOR_TEST + tree_parts::LABEL_X + TREE_ROW_LABEL_CLICK_OFFSET;
    let y = preview_detail::HERO_PREVIEW_Y_FOR_TEST
        + tree_parts::TREE_PANEL_Y
        + tree_parts::ROW_HEIGHT / 2;

    assert!(apply_click(state, x, y));
}

fn tree_panel_scroll_point() -> (usize, usize) {
    let component = preview_detail::component_action_hit_rect(PAGE);

    (
        component.x + tree_parts::TREE_PANEL_X + 1,
        component.y + tree_parts::TREE_PANEL_Y + 1,
    )
}

fn render_state(state: &StorybookWindowState) -> super::canvas::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
