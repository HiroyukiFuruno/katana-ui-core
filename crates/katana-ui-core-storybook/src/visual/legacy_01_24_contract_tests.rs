use super::button_options::{StorybookButtonOptionControl, control_rect, is_button_page};
use super::interaction_spec::StorybookInteractionSpec;
use super::legacy_01_24_contract::{LegacyPageContract, legacy_01_24_contracts};
use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{Canvas, layout_metrics, preview_detail, render};
use crate::catalog::StoryPresetLabels;
use std::collections::BTreeSet;

const FIRST_LEGACY_NUMBER: u8 = 1;
const LAST_LEGACY_NUMBER: u8 = 24;
const CLICK_POINT_OFFSET: usize = 1;
const BODY_REPAINT_THRESHOLD: usize = 80;
const SECOND_PRESET_INDEX: usize = 1;

#[test]
fn legacy_01_24_contract_cases_cover_every_legacy_number() {
    let numbers: BTreeSet<u8> = legacy_01_24_contracts().map(|case| case.number).collect();

    for number in FIRST_LEGACY_NUMBER..=LAST_LEGACY_NUMBER {
        assert!(numbers.contains(&number), "legacy {number:02} is missing");
    }
}

#[test]
fn legacy_01_24_specs_and_presets_are_explicit_per_widget() {
    for case in legacy_01_24_contracts() {
        let spec = StorybookInteractionSpec::for_page(case.page);
        let presets = StoryPresetLabels::for_page(case.page);

        assert_eq!(case.action, spec.action, "{} action", case.label);
        assert_eq!(case.event, spec.event, "{} event", case.label);
        assert_eq!(case.option, spec.option, "{} option", case.label);
        assert_eq!(case.after, spec.after, "{} after", case.label);
        assert_eq!(case.state, spec.state, "{} state", case.label);
        assert!(
            presets.contains(&case.preset),
            "{} preset `{}` missing in {presets:?}",
            case.label,
            case.preset
        );
    }
}

#[test]
fn legacy_01_24_clicks_emit_expected_action_event_state_and_repaint_body() {
    for case in legacy_01_24_contracts() {
        let mut state = new_state(case.page);
        let before = render_state(&state);

        assert!(click_preview(&mut state, case.page), "{} click", case.label);
        assert_eq!(1, state.screen_state.action_count, "{} count", case.label);
        assert_eq!(
            case.action, state.screen_state.last_action,
            "{} action",
            case.label
        );
        assert_eq!(
            case.event, state.screen_state.last_event,
            "{} event",
            case.label
        );
        assert_eq!(
            case.state, state.screen_state.state_label,
            "{} state",
            case.label
        );
        assert_body_repainted(case, &before, &render_state(&state), "click");
    }
}

#[test]
fn legacy_01_24_settings_mutate_option_and_repaint_body() {
    for case in legacy_01_24_contracts() {
        let mut state = new_state(case.page);
        let before = render_state(&state);

        assert!(
            click_settings(&mut state, case.page),
            "{} settings",
            case.label
        );
        assert_eq!(
            1, state.screen_state.settings_revision,
            "{} revision",
            case.label
        );
        assert_setting(case, &state);
        assert_body_repainted(case, &before, &render_state(&state), "settings");
    }
}

#[test]
fn legacy_01_24_state_is_isolated_by_page_and_preset() {
    for case in legacy_01_24_contracts() {
        let mut state = StorybookWindowState::default();

        state.select_page(case.page);
        assert!(click_preview(&mut state, case.page), "{} click", case.label);
        assert!(
            click_settings(&mut state, case.page),
            "{} settings",
            case.label
        );
        let stored = state.screen_state;

        state.select_page(other_page(case.page));
        assert_eq!(
            "idle", state.screen_state.state_label,
            "{} page leak",
            case.label
        );
        assert_eq!(
            "none", state.screen_state.last_action,
            "{} action leak",
            case.label
        );
        assert_eq!(
            0, state.screen_state.settings_revision,
            "{} setting leak",
            case.label
        );

        state.select_page(case.page);
        assert_eq!(stored, state.screen_state, "{} page restore", case.label);
        if StoryPresetLabels::for_page(case.page).len() > SECOND_PRESET_INDEX {
            state.select_preset(SECOND_PRESET_INDEX);
            assert_eq!(
                "idle", state.screen_state.state_label,
                "{} preset leak",
                case.label
            );
            state.select_preset(0);
            assert_eq!(stored, state.screen_state, "{} preset restore", case.label);
        }
    }
}

fn new_state(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    )
}

fn click_preview(state: &mut StorybookWindowState, page: &str) -> bool {
    click_rect(state, preview_detail::component_action_hit_rect(page))
}

fn click_settings(state: &mut StorybookWindowState, page: &str) -> bool {
    click_rect(state, setting_rect(page))
}

fn setting_rect(page: &str) -> layout_metrics::LayoutRect {
    if is_button_page(page) {
        return control_rect(StorybookButtonOptionControl::Border);
    }
    layout_metrics::button_setting_hit_rect()
}

fn click_rect(state: &mut StorybookWindowState, rect: layout_metrics::LayoutRect) -> bool {
    assert!(rect.width > 0, "empty click rect");
    apply_click(
        state,
        rect.x + CLICK_POINT_OFFSET,
        rect.y + CLICK_POINT_OFFSET,
    )
}

fn assert_setting(case: &LegacyPageContract, state: &StorybookWindowState) {
    if is_button_page(case.page) {
        assert_eq!(
            "button_option_apply", state.screen_state.last_action,
            "{} button action",
            case.label
        );
        assert_eq!(
            "border", state.screen_state.last_setting,
            "{} button option",
            case.label
        );
        return;
    }
    assert_eq!(
        case.option, state.screen_state.last_setting,
        "{} setting",
        case.label
    );
    assert_eq!(
        case.after, state.screen_state.last_setting_value,
        "{} value",
        case.label
    );
}

fn assert_body_repainted(case: &LegacyPageContract, before: &Canvas, after: &Canvas, phase: &str) {
    assert!(
        component_body_pixel_diff(case.page, before, after) > BODY_REPAINT_THRESHOLD,
        "{} {phase} did not repaint component body",
        case.label
    );
}

fn other_page(page: &str) -> &'static str {
    if page == "text" { "icon" } else { "text" }
}
