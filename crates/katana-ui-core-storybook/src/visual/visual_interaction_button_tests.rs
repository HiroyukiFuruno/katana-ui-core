use super::button_options::{StorybookButtonOptionControl, StorybookButtonOptions, control_rect};
use super::render_context::ScenarioContext;
use super::visual_interaction_test_support::{
    component_body_pixel_diff, pixel_at, rect_non_background_pixels, rect_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{StorybookVisual, palette, preview_detail, render};

const DARK_THEME: &str = "dark";
const BUTTON_PAGE: &str = "button";
const TEXT_BUTTON_PAGE: &str = "text-button";
const SVG_BUTTON_PAGE: &str = "svg-button";
const DEFAULT_PRESET: usize = 0;
const INTERACTIVE_PRESET: usize = 1;
const EDGE_PRESET: usize = 2;
const COMPONENT_BODY_DIFF_THRESHOLD: usize = 80;
const MIN_BUTTON_WIDTH: usize = 96;
const MIN_BUTTON_HEIGHT: usize = 36;
const BUTTON_PRESSED_FILL: u32 = 0x557846;
const BUTTON_FILL_PROBE_OFFSET: usize = 8;
const BUTTON_HOVER_DIFF_THRESHOLD: usize = 24;
const BUTTON_MAX_CONFIGURED_RIGHT: usize = 320;
const BUTTON_STATUS_GAP: usize = 24;

#[test]
fn preset_tab_updates_selected_preview_body() {
    let before = StorybookVisual.render_preset(DARK_THEME, BUTTON_PAGE, DEFAULT_PRESET, 0);
    let after = StorybookVisual.render_preset(DARK_THEME, BUTTON_PAGE, INTERACTIVE_PRESET, 0);

    assert!(
        component_body_pixel_diff(BUTTON_PAGE, &before, &after) > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn settings_update_selected_preview_body() {
    let mut state = StorybookWindowState::default();
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    let setting = control_rect(StorybookButtonOptionControl::Disabled);

    assert!(apply_click(&mut state, setting.x + 1, setting.y + 1));
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );

    assert!(
        component_body_pixel_diff(BUTTON_PAGE, &before, &after) > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn clicked_button_updates_visible_button_body() {
    let before = StorybookVisual.render_preset(DARK_THEME, BUTTON_PAGE, DEFAULT_PRESET, 0);
    let after = StorybookVisual.render_clicked_preset_with_scrollbar(
        DARK_THEME,
        BUTTON_PAGE,
        DEFAULT_PRESET,
        0,
        true,
    );
    let rect = preview_detail::button_action_hit_rect(BUTTON_PAGE);

    assert!(rect.width >= MIN_BUTTON_WIDTH);
    assert!(rect.height >= MIN_BUTTON_HEIGHT);
    assert_eq!(
        Some(BUTTON_PRESSED_FILL),
        pixel_at(
            &after,
            rect.x + BUTTON_FILL_PROBE_OFFSET,
            rect.y + BUTTON_FILL_PROBE_OFFSET
        )
    );
    assert!(
        component_body_pixel_diff(BUTTON_PAGE, &before, &after) > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

#[test]
fn hovering_button_preview_updates_surface_without_click_action() {
    let mut state = StorybookWindowState::default();
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    let rect = preview_detail::button_action_hit_rect(BUTTON_PAGE);

    assert!(super::window_interaction::apply_hover_at(
        &mut state,
        rect.x + rect.width / 2,
        rect.y + rect.height / 2
    ));
    assert_eq!(0, state.screen_state.action_count);
    assert!(state.screen_state.preview_hovered);

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );

    assert!(component_body_pixel_diff(BUTTON_PAGE, &before, &after) > BUTTON_HOVER_DIFF_THRESHOLD);
}

#[test]
fn button_text_button_and_svg_button_have_distinct_material_shapes() {
    let button = StorybookVisual.render_preset(DARK_THEME, BUTTON_PAGE, DEFAULT_PRESET, 0);
    let text_button =
        StorybookVisual.render_preset(DARK_THEME, TEXT_BUTTON_PAGE, DEFAULT_PRESET, 0);
    let svg_button = StorybookVisual.render_preset(DARK_THEME, SVG_BUTTON_PAGE, DEFAULT_PRESET, 0);
    let button_rect = preview_detail::button_action_hit_rect(BUTTON_PAGE);
    let text_rect = preview_detail::button_action_hit_rect(TEXT_BUTTON_PAGE);
    let svg_rect = preview_detail::button_action_hit_rect(SVG_BUTTON_PAGE);

    assert!(button_rect.width > svg_rect.width);
    assert!(text_rect.width > svg_rect.width);
    assert!(
        rect_pixel_diff(button_rect, &button, &text_button) > COMPONENT_BODY_DIFF_THRESHOLD,
        "contained button and text button must not share the same surface"
    );
    assert!(
        rect_non_background_pixels(svg_rect, &svg_button, palette::DEFAULT_BACKGROUND)
            < rect_non_background_pixels(button_rect, &button, palette::DEFAULT_BACKGROUND),
        "svg button must stay icon-only instead of rendering a text-button sized body"
    );
}

#[test]
fn button_status_rows_use_compact_labels_that_fit() {
    let pressed = button_status_scenario("button_press", "button_clicked", "pressed=true");
    let option_changed =
        button_status_scenario("button_option_apply", "button_option_changed", "label=ja");

    assert!(super::dedicated_dod_atom_button_live_status::status_rows_fit_for_test(pressed));
    assert!(super::dedicated_dod_atom_button_live_status::status_rows_fit_for_test(option_changed));
    assert!(
        super::dedicated_dod_atom_button_live_status::status_rows_have_frame_padding_for_test()
    );
    assert!(
        super::dedicated_dod_atom_button_live_status::status_rows_start_x_for_test()
            >= BUTTON_MAX_CONFIGURED_RIGHT + BUTTON_STATUS_GAP
    );
}

#[test]
fn button_layout_presets_change_button_body_size() {
    let modern = preview_detail::button_action_hit_rect(BUTTON_PAGE);
    let classic = StorybookVisual.render_preset(DARK_THEME, BUTTON_PAGE, INTERACTIVE_PRESET, 0);
    let basic = StorybookVisual.render_preset(DARK_THEME, BUTTON_PAGE, EDGE_PRESET, 0);

    assert!(modern.width >= MIN_BUTTON_WIDTH);
    assert!(
        component_body_pixel_diff(BUTTON_PAGE, &classic, &basic) > COMPONENT_BODY_DIFF_THRESHOLD
    );
}

fn button_status_scenario(
    last_action: &'static str,
    last_event: &'static str,
    state_label: &'static str,
) -> ScenarioContext<'static> {
    ScenarioContext {
        selected_page: BUTTON_PAGE,
        preset_index: DEFAULT_PRESET,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state: super::screen_state::StorybookScreenState {
            last_action,
            last_event,
            state_label,
            button_options: StorybookButtonOptions::default(),
            ..Default::default()
        },
    }
}
