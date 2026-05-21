use super::super::{StorybookWindowState, apply_click, apply_context_click};
use crate::requirements::StoryRequirements;
use crate::visual::button_options::{StorybookButtonOptionControl, control_rect};
use crate::visual::dedicated_dod_molecule_tree_parts as tree_parts;
use crate::visual::interaction_spec::StorybookInteractionSpec;
use crate::visual::visual_interaction_test_support::rect_non_background_pixels;
use crate::visual::{layout_metrics, palette, preview_detail, render};

const UI_INTERACTION_DIFF_THRESHOLD: usize = 500;
const MIN_HIT_TARGET_PIXELS: usize = 64;
const REPRESENTATIVE_PREVIEW_PAGES: &[&str] = &[
    "toggle",
    "select-box",
    "color-swatch",
    "tooltip",
    "popover",
    "accordion",
    "split-pane",
    "modal-overlay",
    "color-picker-rgba",
    "code-diff",
    "badge",
    "card",
];

#[test]
fn clicking_preview_button_emits_action_event_and_changes_rendering() {
    let mut state = StorybookWindowState::default();
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    let button = preview_detail::button_action_hit_rect("button");

    assert!(apply_click(&mut state, button.x + 1, button.y + 1));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("button_press", state.screen_state.last_action);
    assert_eq!("button_clicked", state.screen_state.last_event);

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    assert!(pixel_diff(&before, &after) > UI_INTERACTION_DIFF_THRESHOLD);
}

#[test]
fn clicking_button_setting_updates_props_and_rendering() {
    let mut state = StorybookWindowState::default();
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    let setting = control_rect(StorybookButtonOptionControl::Border);

    assert!(apply_click(&mut state, setting.x + 1, setting.y + 1));
    assert_eq!(1, state.screen_state.settings_revision);
    assert!(state.screen_state.has_settings_override());
    assert_eq!("button_option_apply", state.screen_state.last_action);
    assert_eq!("button_option_changed", state.screen_state.last_event);
    assert_eq!("border", state.screen_state.last_setting);

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    assert!(pixel_diff(&before, &after) > UI_INTERACTION_DIFF_THRESHOLD);
}

#[test]
fn clicking_selected_preview_emits_component_event_for_non_button_pages() {
    assert_preview_click_updates_state_and_pixels("card");
}

#[test]
fn representative_preview_clicks_emit_action_event_state_and_repaint_canvas() {
    for &page in REPRESENTATIVE_PREVIEW_PAGES {
        assert_preview_click_updates_state_and_pixels(page);
    }
}

#[test]
fn every_required_page_preview_hit_target_contains_drawn_component_pixels() {
    for &page in StoryRequirements::required_pages() {
        let canvas = render::render_storybook_canvas_for_preset("dark", page, 0, 0);
        let target = preview_detail::component_action_hit_rect(page);

        assert!(target.width > 0, "{page} lacks preview hit target");
        assert!(
            rect_non_background_pixels(target, &canvas, palette::DEFAULT_BACKGROUND)
                > MIN_HIT_TARGET_PIXELS,
            "{page} hit target does not contain rendered component pixels"
        );
    }
}

#[test]
fn clicking_settings_row_mutates_selected_component_options() {
    let mut state = StorybookWindowState {
        selected_page: "card",
        ..StorybookWindowState::default()
    };
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    let setting = layout_metrics::button_setting_hit_rect();

    assert!(apply_click(&mut state, setting.x + 1, setting.y + 1));
    assert_eq!(1, state.screen_state.settings_revision);
    assert_eq!("settings_option_changed", state.screen_state.last_action);
    assert_eq!("interaction.active", state.screen_state.last_setting);

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    assert!(pixel_diff(&before, &after) > UI_INTERACTION_DIFF_THRESHOLD);
}

#[test]
fn right_clicking_tree_view_preview_opens_context_menu_state() {
    let mut state = StorybookWindowState {
        selected_page: "tree-view",
        ..StorybookWindowState::default()
    };
    let target = preview_detail::component_action_hit_rect(state.selected_page);

    assert!(apply_context_click(&mut state, target.x + 1, target.y + 1));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("tree_context_menu", state.screen_state.last_action);
    assert_eq!("tree_context_opened", state.screen_state.last_event);
    assert_eq!("context_menu=open", state.screen_state.state_label);
    assert_eq!("empty_area_context_menu", state.screen_state.last_setting);
}

#[test]
fn clicking_visible_tree_view_row_uses_the_drawn_row_hit_target() {
    let mut state = StorybookWindowState {
        selected_page: "tree-view",
        ..StorybookWindowState::default()
    };
    let x = preview_detail::HERO_PREVIEW_X_FOR_TEST + tree_parts::LABEL_X + 8;
    let y = preview_detail::HERO_PREVIEW_Y_FOR_TEST
        + tree_parts::TREE_PANEL_Y
        + tree_parts::ROW_HEIGHT / 2;

    assert!(apply_click(&mut state, x, y));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("tree_click_toggle", state.screen_state.last_action);
    assert_eq!("tree_toggled", state.screen_state.last_event);
    assert_eq!("open=false", state.screen_state.state_label);
}

#[test]
fn clicking_outside_controls_does_not_mutate_state() {
    let mut state = StorybookWindowState::default();
    let original = state.clone();

    assert!(!apply_click(&mut state, 0, 0));
    assert_eq!(original, state);
}

fn assert_preview_click_updates_state_and_pixels(page: &'static str) {
    let mut state = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    let target = preview_detail::component_action_hit_rect(state.selected_page);

    assert!(target.width > 0, "{page} lacks preview action target");
    assert!(
        apply_click(&mut state, target.x + 1, target.y + 1),
        "{page} preview action did not mutate state"
    );
    let spec = StorybookInteractionSpec::for_page(page);
    assert_eq!(1, state.screen_state.action_count, "{page} action count");
    assert_eq!(spec.action, state.screen_state.last_action, "{page} action");
    assert_eq!(spec.event, state.screen_state.last_event, "{page} event");
    assert_eq!(spec.state, state.screen_state.state_label, "{page} state");

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    assert!(
        pixel_diff(&before, &after) > UI_INTERACTION_DIFF_THRESHOLD,
        "{page} preview action did not repaint canvas"
    );
}

fn pixel_diff(before: &crate::visual::Canvas, after: &crate::visual::Canvas) -> usize {
    before
        .pixels()
        .iter()
        .zip(after.pixels().iter())
        .filter(|(left, right)| left != right)
        .count()
}
