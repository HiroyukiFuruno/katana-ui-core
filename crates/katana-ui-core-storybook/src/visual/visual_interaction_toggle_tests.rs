use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at, rect_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_hover_at};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "toggle";
const OFF_PRESET: usize = 0;
const ON_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const ROW_FILL_SAMPLE_X_OFFSET: usize = 4;
const ROW_FILL_SAMPLE_Y_OFFSET: usize = 4;
const ROW_BORDER_SAMPLE_X_OFFSET: usize = 8;
const SWITCH_TRACK_SAMPLE_X_OFFSET: usize = 6;
const ROUNDED_CORNER_SAMPLE_OFFSET: usize = 0;

#[test]
fn toggle_exposes_leaf_presets_options_and_checked_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("toggle_change", spec.action);
    assert_eq!("toggle_changed", spec.event);
    assert_eq!("checked", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("checked=true", spec.state);
}

#[test]
fn toggle_presets_render_distinct_switch_and_row_bodies() {
    let off = StorybookVisual.render_preset(DARK_THEME, PAGE, OFF_PRESET, 0);
    let on = StorybookVisual.render_preset(DARK_THEME, PAGE, ON_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);
    let row = super::dedicated_dod_atom_buttons::toggle_row_rect_for_test();

    assert!(component_body_pixel_diff(PAGE, &off, &on) > BODY_DIFF_THRESHOLD);
    assert!(rect_pixel_diff(row, &off, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(rect_pixel_diff(row, &disabled, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn toggle_setting_option_updates_switch_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn toggle_preview_action_updates_switch_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn toggle_on_preset_click_returns_to_off_on_first_click() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    state.select_preset(ON_PRESET);
    let row = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(ON_PRESET, state.preset_index);
    assert!(state.screen_state.toggle_checked);
    assert_eq!("checked=true", state.screen_state.state_label);

    assert!(super::window_interaction::apply_click(
        &mut state,
        row.x + row.width / 2,
        row.y + row.height / 2
    ));

    assert!(!state.screen_state.toggle_checked);
    assert_eq!("toggle_change", state.screen_state.last_action);
    assert_eq!("toggle_changed", state.screen_state.last_event);
    assert_eq!("checked=false", state.screen_state.state_label);
}

#[test]
fn toggle_preview_action_rect_matches_rendered_row_body() {
    assert_eq!(
        super::dedicated_dod_atom_buttons::toggle_row_rect_for_test(),
        preview_detail::component_action_hit_rect(PAGE)
    );
}

#[test]
fn toggle_runtime_label_does_not_overlap_switch() {
    let mut state = super::window_interaction::StorybookWindowState {
        selected_page: PAGE,
        ..super::window_interaction::StorybookWindowState::default()
    };
    let rect = preview_detail::component_action_hit_rect(PAGE);

    assert!(super::window_interaction::apply_click(
        &mut state,
        rect.x + 1,
        rect.y + 1
    ));
    let rendered = super::render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state,
    );

    assert!(
        rendered
            .text_runs()
            .iter()
            .all(|run| !run.text().starts_with("clicked ")),
        "toggle owns its inline state surface; Storybook runtime clicked label must not overlap the switch"
    );
}

#[test]
fn toggle_hover_uses_checkbox_style_row_border() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let row = super::dedicated_dod_atom_buttons::toggle_row_rect_for_test();
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());

    assert!(apply_hover_at(&mut state, row.x + 1, row.y + 1));
    let rendered = super::render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state,
    );

    assert!(
        count_color_in_rect(&rendered, row, colors.hover_border) > 0,
        "toggle hover must use the same hover_border token as checkbox rows"
    );
    assert_ne!(
        Some(colors.hover_border),
        pixel_at(
            &rendered,
            row.x + ROUNDED_CORNER_SAMPLE_OFFSET,
            row.y + ROUNDED_CORNER_SAMPLE_OFFSET,
        ),
        "toggle row hover chrome must keep the rounded checkbox-style corner"
    );
}

#[test]
fn toggle_preview_does_not_render_storybook_status_boxes_inside_component() {
    let rendered = StorybookVisual.render_preset(DARK_THEME, PAGE, OFF_PRESET, 0);

    for label in ["toggle_change", "toggle_changed", "checked=false"] {
        assert!(
            rendered.text_runs().iter().all(|run| run.text() != label),
            "toggle component preview must not render Storybook action/status box label {label}"
        );
    }
}

fn count_color_in_rect(
    canvas: &super::Canvas,
    rect: super::layout_metrics::LayoutRect,
    color: u32,
) -> usize {
    let mut count = 0;
    for y in rect.y..rect.bottom() {
        for x in rect.x..rect.right() {
            if pixel_at(canvas, x, y) == Some(color) {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn toggle_light_and_dark_rows_use_theme_tokens() {
    assert_row_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_row_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_row_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let off = StorybookVisual.render_preset(theme_id, PAGE, OFF_PRESET, 0);
    let on = StorybookVisual.render_preset(theme_id, PAGE, ON_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let row = super::dedicated_dod_atom_buttons::toggle_row_rect_for_test();
    let switch = super::dedicated_dod_atom_buttons::toggle_switch_rect_for_test();

    assert_eq!(
        Some(colors.border),
        pixel_at(&off, row.x + ROW_BORDER_SAMPLE_X_OFFSET, row.y)
    );
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &off,
            row.x + ROW_FILL_SAMPLE_X_OFFSET,
            row.y + ROW_FILL_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &on,
            switch.x + SWITCH_TRACK_SAMPLE_X_OFFSET,
            switch.y + switch.height / 2
        )
    );
}
