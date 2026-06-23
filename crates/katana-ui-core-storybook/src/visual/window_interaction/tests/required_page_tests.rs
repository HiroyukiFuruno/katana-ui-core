use super::super::{StorybookWindowState, apply_click};
use crate::catalog::StoryPresetLabels;
use crate::requirements::StoryRequirements;
use crate::visual::button_options::{StorybookButtonOptionControl, control_rect, is_button_page};
use crate::visual::interaction_spec::StorybookInteractionSpec;
use crate::visual::storybook_ui_option_contract;
use crate::visual::visual_interaction_test_support::component_body_pixel_diff;
use crate::visual::{layout_metrics, preview_detail, render};

const CLICK_POINT_OFFSET: usize = 1;
const COMPONENT_BODY_REPAINT_THRESHOLD: usize = 80;
const PRESET_BODY_REPAINT_THRESHOLD: usize = 16;
const SECOND_PRESET_INDEX: usize = 1;
const PRIMARY_INSTANCE: &str = "required.primary";
const SECONDARY_INSTANCE: &str = "required.secondary";

#[test]
fn every_required_page_has_screen_action_and_settings_paths() {
    for page in StoryRequirements::required_pages() {
        assert_required_page_has_screen_action_and_setting_path(page);
    }
}

#[test]
fn every_required_page_keeps_action_state_separate_from_other_pages() {
    for &page in StoryRequirements::required_pages() {
        assert_required_page_keeps_action_state_separate_from(other_page_for(page), page);
    }
}

#[test]
fn every_required_page_keeps_window_interaction_instances_separate() {
    for &page in StoryRequirements::required_pages() {
        assert_required_page_keeps_window_interaction_instances_separate(page);
    }
}

#[test]
fn every_required_page_keeps_settings_state_separate_from_other_pages() {
    for &page in StoryRequirements::required_pages() {
        assert_required_page_keeps_settings_state_separate_from(other_page_for(page), page);
    }
}

#[test]
fn every_required_page_click_repaints_component_body() {
    for &page in StoryRequirements::required_pages() {
        assert_preview_click_repaints_component_body(page);
    }
}

#[test]
fn every_required_page_setting_repaints_component_body() {
    for &page in StoryRequirements::required_pages() {
        assert_setting_click_repaints_component_body(page);
    }
}

#[test]
fn every_required_page_preset_tab_repaints_component_body() {
    for &page in StoryRequirements::required_pages() {
        assert_preset_tabs_repaint_component_body(page);
    }
}

#[test]
fn every_required_page_keeps_action_and_settings_state_separate_between_presets() {
    for &page in StoryRequirements::required_pages() {
        if StoryPresetLabels::for_page(page).len() <= SECOND_PRESET_INDEX {
            continue;
        }
        assert_required_page_keeps_action_and_settings_state_separate_between_presets(page);
    }
}

fn assert_required_page_has_screen_action_and_setting_path(page: &'static str) {
    let mut state = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    let target = preview_detail::component_action_hit_rect(page);

    assert!(target.width > 0, "{page} lacks preview action target");
    assert!(
        click_rect(&mut state, target),
        "{page} preview action did not mutate state"
    );
    assert_eq!(1, state.screen_state.action_count, "{page} action count");
    let spec = StorybookInteractionSpec::for_page(page);
    assert_eq!(spec.action, state.screen_state.last_action, "{page} action");
    assert_eq!(spec.event, state.screen_state.last_event, "{page} event");
    assert_eq!(spec.state, state.screen_state.state_label, "{page} state");

    let setting = setting_target_for_page(page);
    assert!(
        click_rect(&mut state, setting),
        "{page} setting click did not mutate state"
    );
    assert_settings_result(page, spec, state);
}

fn assert_required_page_keeps_action_state_separate_from(
    other_page: &'static str,
    page: &'static str,
) {
    let mut state = StorybookWindowState::default();
    let spec = StorybookInteractionSpec::for_page(page);

    state.select_page(page);
    assert_preview_click(&mut state, page);
    assert_eq!(spec.action, state.screen_state.last_action, "{page} action");

    state.select_page(other_page);
    assert_eq!(
        "idle", state.screen_state.state_label,
        "{page} leaked state"
    );
    assert_eq!(
        "none", state.screen_state.last_action,
        "{page} leaked action"
    );
    assert_eq!(0, state.screen_state.action_count, "{page} leaked count");

    state.select_page(page);
    assert_eq!(
        spec.action, state.screen_state.last_action,
        "{page} lost action"
    );
    assert_eq!(
        spec.event, state.screen_state.last_event,
        "{page} lost event"
    );
    assert_eq!(1, state.screen_state.action_count, "{page} lost count");
}

fn assert_required_page_keeps_settings_state_separate_from(
    other_page: &'static str,
    page: &'static str,
) {
    let mut state = StorybookWindowState::default();
    let spec = StorybookInteractionSpec::for_page(page);

    state.select_page(page);
    assert_setting_click(&mut state, page);
    assert_settings_result(page, spec, state.clone());

    state.select_page(other_page);
    assert_eq!(
        0, state.screen_state.settings_revision,
        "{page} leaked setting revision"
    );
    assert_eq!(
        "none", state.screen_state.last_setting,
        "{page} leaked setting"
    );

    state.select_page(page);
    assert_settings_result(page, spec, state);
}

fn assert_required_page_keeps_window_interaction_instances_separate(page: &'static str) {
    let mut state = StorybookWindowState::default();
    let spec = StorybookInteractionSpec::for_page(page);

    state.select_page(page);
    state.select_instance(PRIMARY_INSTANCE);
    assert_preview_click(&mut state, page);
    let primary_state = state.screen_state.clone();
    assert_eq!(
        spec.action, primary_state.last_action,
        "{page} primary action"
    );

    state.select_instance(SECONDARY_INSTANCE);
    assert_default_instance_state(page, &state.screen_state);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(
        primary_state, state.screen_state,
        "{page} lost primary instance"
    );

    state.select_instance(SECONDARY_INSTANCE);
    assert_default_instance_state(page, &state.screen_state);
}

fn assert_required_page_keeps_action_and_settings_state_separate_between_presets(
    page: &'static str,
) {
    let mut state = StorybookWindowState::default();

    state.select_page(page);
    let original_preset_index = state.preset_index;
    assert_preview_click(&mut state, page);
    let action_screen_state = state.screen_state.clone();
    assert_setting_click(&mut state, page);
    let stored_preset_index = state.preset_index;
    let expected_action_count = if stored_preset_index == original_preset_index {
        1
    } else {
        0
    };
    assert_eq!(
        expected_action_count, state.screen_state.action_count,
        "{page} action"
    );
    assert_eq!(1, state.screen_state.settings_revision, "{page} setting");
    let other_preset_index = other_preset_for(page, original_preset_index, stored_preset_index);
    let stored_screen_state = state.screen_state.clone();

    state.select_preset(other_preset_index);
    assert_eq!(
        expected_preset_default_state_label(page, other_preset_index),
        state.screen_state.state_label,
        "{page} leaked state across presets"
    );
    assert_eq!(
        "none", state.screen_state.last_action,
        "{page} leaked action across presets"
    );
    assert_eq!(
        0, state.screen_state.settings_revision,
        "{page} leaked setting across presets"
    );

    state.select_preset(stored_preset_index);
    assert_eq!(
        stored_screen_state,
        state.screen_state.clone(),
        "{page} lost preset-local state"
    );
    if stored_preset_index != original_preset_index {
        state.select_preset(original_preset_index);
        assert_eq!(
            action_screen_state,
            state.screen_state.clone(),
            "{page} lost original preset action state"
        );
    }
}

fn other_preset_for(page: &str, original: usize, current: usize) -> usize {
    let preset_count = StoryPresetLabels::for_page(page).len();
    if preset_count <= 1 {
        return 0;
    }
    (0..preset_count)
        .find(|index| *index != original && *index != current)
        .unwrap_or(SECOND_PRESET_INDEX)
}

fn expected_preset_default_state_label(page: &str, preset_index: usize) -> &'static str {
    match (page, preset_index) {
        ("checkbox", 1) => "checked=true",
        ("checkbox", 2) => "disabled=true",
        ("checkbox", 3) => "focused=true",
        ("toggle", 1) => "checked=true",
        _ => "idle",
    }
}

fn assert_default_instance_state(
    page: &str,
    state: &crate::visual::screen_state::StorybookScreenState,
) {
    assert_eq!(0, state.action_count, "{page} leaked instance action count");
    assert_eq!("none", state.last_action, "{page} leaked instance action");
    assert_eq!("none", state.last_event, "{page} leaked instance event");
    assert_eq!("idle", state.state_label, "{page} leaked instance state");
}

fn assert_preview_click_repaints_component_body(page: &'static str) {
    let mut state = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    let before = render_state(&state);

    assert_preview_click(&mut state, page);

    let after = render_state(&state);
    assert!(
        component_body_pixel_diff(page, &before, &after) > COMPONENT_BODY_REPAINT_THRESHOLD,
        "{page} preview action did not repaint component body"
    );
}

fn assert_setting_click_repaints_component_body(page: &'static str) {
    let mut state = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    let before = render_state(&state);
    let setting = repaint_setting_target_for_page(page);

    assert!(
        click_rect(&mut state, setting),
        "{page} setting click did not mutate state"
    );

    let after = render_state(&state);
    assert!(
        component_body_pixel_diff(page, &before, &after) > COMPONENT_BODY_REPAINT_THRESHOLD,
        "{page} setting did not repaint component body"
    );
}

fn assert_preset_tabs_repaint_component_body(page: &'static str) {
    let presets = StoryPresetLabels::for_page(page);
    let base = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    let before = render_state(&base);

    let Some((index, label)) = presets
        .iter()
        .copied()
        .enumerate()
        .find(|(index, _)| *index > 0)
    else {
        return;
    };
    let mut state = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    state.select_preset(index);
    let after = render_state(&state);
    let diff = component_body_pixel_diff(page, &before, &after);
    assert!(
        diff > PRESET_BODY_REPAINT_THRESHOLD,
        "{page} preset `{label}` did not repaint component body: diff={diff}"
    );
}

fn assert_preview_click(state: &mut StorybookWindowState, page: &'static str) {
    let target = preview_detail::component_action_hit_rect(page);

    assert!(target.width > 0, "{page} lacks preview action target");
    assert!(
        click_rect(state, target),
        "{page} preview action did not mutate state"
    );
}

fn assert_setting_click(state: &mut StorybookWindowState, page: &'static str) {
    let setting = setting_target_for_page(page);

    assert!(
        click_rect(state, setting),
        "{page} setting click did not mutate state"
    );
}

fn click_rect(state: &mut StorybookWindowState, rect: layout_metrics::LayoutRect) -> bool {
    apply_click(
        state,
        rect.x + CLICK_POINT_OFFSET,
        rect.y + CLICK_POINT_OFFSET,
    )
}

fn other_page_for(page: &str) -> &'static str {
    if page == "button" {
        "text-button"
    } else {
        "button"
    }
}

fn setting_target_for_page(page: &str) -> layout_metrics::LayoutRect {
    if is_button_page(page) {
        return control_rect(StorybookButtonOptionControl::Label);
    }
    layout_metrics::button_setting_hit_rect()
}

fn repaint_setting_target_for_page(page: &str) -> layout_metrics::LayoutRect {
    if is_button_page(page) {
        return control_rect(StorybookButtonOptionControl::Border);
    }
    layout_metrics::button_setting_hit_rect()
}

fn render_state(state: &StorybookWindowState) -> crate::visual::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn assert_settings_result(
    page: &str,
    _spec: StorybookInteractionSpec,
    state: StorybookWindowState,
) {
    assert_eq!(1, state.screen_state.settings_revision, "{page} setting");
    if is_button_page(page) {
        assert_eq!(
            "label", state.screen_state.last_setting,
            "{page} setting option"
        );
        assert_eq!(
            "保存する", state.screen_state.last_setting_value,
            "{page} setting value"
        );
        return;
    }
    assert_eq!(
        first_option_for_page(page).setting,
        state.screen_state.last_setting,
        "{page} setting option"
    );
    assert_eq!(
        first_option_for_page(page).after,
        state.screen_state.last_setting_value,
        "{page} setting value"
    );
}

fn first_option_for_page(page: &str) -> storybook_ui_option_contract::StorybookUiOptionContract {
    storybook_ui_option_contract::options_for_page(page)
        .first()
        .copied()
        .unwrap_or_else(|| {
            storybook_ui_option_contract::StorybookUiOptionContract::new(
                "option",
                "unchanged",
                "changed",
            )
        })
}
