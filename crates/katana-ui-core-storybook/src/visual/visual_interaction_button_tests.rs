use super::button_options::{StorybookButtonOptionControl, StorybookButtonOptions, control_rect};
use super::button_options_draw;
use super::render_context::ScenarioContext;
use super::visual_interaction_test_support::{
    component_body_pixel_diff, pixel_at, rect_non_background_pixels, rect_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{StorybookVisual, palette, preview, preview_detail, render};

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
const SUMMARY_SETTING_INDEX: usize = 2;
const SUMMARY_TOOLTIP_DIFF_THRESHOLD: usize = 100;
const SUMMARY_TOOLTIP_SCAN_HEIGHT: usize = 40;
const SUMMARY_TOOLTIP_SCAN_WIDTH: usize = 360;

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

#[test]
fn button_preset_tab_updates_summary_setting_value() {
    let modern = preview::summary_full_samples_for_test(button_summary_scenario(DEFAULT_PRESET));
    let classic =
        preview::summary_full_samples_for_test(button_summary_scenario(INTERACTIVE_PRESET));
    let basic = preview::summary_full_samples_for_test(button_summary_scenario(EDGE_PRESET));

    assert_eq!("setting layout=modern", modern[SUMMARY_SETTING_INDEX]);
    assert_eq!("setting layout=classic", classic[SUMMARY_SETTING_INDEX]);
    assert_eq!("setting layout=basic", basic[SUMMARY_SETTING_INDEX]);
    assert_ne!(
        modern[SUMMARY_SETTING_INDEX],
        classic[SUMMARY_SETTING_INDEX]
    );
    assert_ne!(classic[SUMMARY_SETTING_INDEX], basic[SUMMARY_SETTING_INDEX]);
}

#[test]
fn button_preset_tab_updates_inspector_effective_setting_values() {
    let modern = button_options_draw::effective_setting_value_for_test(
        button_summary_scenario(DEFAULT_PRESET),
        StorybookButtonOptionControl::Height,
    );
    let classic = button_options_draw::effective_setting_value_for_test(
        button_summary_scenario(INTERACTIVE_PRESET),
        StorybookButtonOptionControl::Height,
    );
    let basic = button_options_draw::effective_setting_value_for_test(
        button_summary_scenario(EDGE_PRESET),
        StorybookButtonOptionControl::Height,
    );

    assert_eq!("auto 40px", modern);
    assert_eq!("auto 38px", classic);
    assert_eq!("auto 34px", basic);
}

#[test]
fn summary_ellipsis_exposes_full_value_with_tooltip_state() {
    let mut screen_state = super::screen_state::StorybookScreenState {
        last_setting: "label",
        last_setting_value: "保存する長いラベルを確認する",
        ..Default::default()
    };
    let scenario = button_summary_scenario_with_state(DEFAULT_PRESET, screen_state);
    let full = preview::summary_full_samples_for_test(scenario);
    let visible = preview::summary_visible_samples_for_test(scenario);

    assert_eq!(
        "setting label=保存する長いラベルを確認する",
        full[SUMMARY_SETTING_INDEX]
    );
    assert_ne!(full[SUMMARY_SETTING_INDEX], visible[SUMMARY_SETTING_INDEX]);

    let hidden = render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        BUTTON_PAGE,
        DEFAULT_PRESET,
        screen_state,
    );
    screen_state.hovered_summary_index = Some(SUMMARY_SETTING_INDEX);
    let shown = render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        BUTTON_PAGE,
        DEFAULT_PRESET,
        screen_state,
    );

    assert!(summary_tooltip_pixel_diff(&hidden, &shown) > SUMMARY_TOOLTIP_DIFF_THRESHOLD);
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

fn button_summary_scenario(preset_index: usize) -> ScenarioContext<'static> {
    button_summary_scenario_with_state(
        preset_index,
        super::screen_state::StorybookScreenState::default(),
    )
}

fn button_summary_scenario_with_state(
    preset_index: usize,
    screen_state: super::screen_state::StorybookScreenState,
) -> ScenarioContext<'static> {
    ScenarioContext {
        selected_page: BUTTON_PAGE,
        preset_index,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state,
    }
}

fn summary_tooltip_pixel_diff(before: &super::Canvas, after: &super::Canvas) -> usize {
    let rect = preview::summary_control_rect_for_test(SUMMARY_SETTING_INDEX);
    let start_y = rect.bottom();
    let end_y = start_y + SUMMARY_TOOLTIP_SCAN_HEIGHT;
    let end_x = (rect.x + SUMMARY_TOOLTIP_SCAN_WIDTH).min(before.width());
    let mut diff = 0;
    for current_y in start_y..end_y {
        for current_x in rect.x..end_x {
            let index = current_y * before.width() + current_x;
            if before.pixels()[index] != after.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}
