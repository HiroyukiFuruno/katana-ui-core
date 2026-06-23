use super::button_options;
use super::layout_metrics;
use super::storybook_ui_option_contract;
use super::window_interaction::{StorybookWindowState, apply_click};
use crate::requirements::StoryRequirements;
use button_options::StorybookButtonOptionControl;

const P0_INSPECTOR_OPTION_PAGES: [&str; 4] = ["tabs", "text-input", "text-area", "toolbar"];
const GENERIC_INSPECTOR_FALLBACKS: [&str; 2] =
    ["settings_option_changed", "component_settings_changed"];

#[test]
fn p0_inspector_option_changes_do_not_use_generic_fallback_status() {
    for page in P0_INSPECTOR_OPTION_PAGES {
        assert_page_options_do_not_use_generic_fallback(page);
    }
}

#[test]
fn required_page_inspector_options_do_not_use_generic_fallback_status() {
    for page in StoryRequirements::required_pages() {
        assert_page_options_do_not_use_generic_fallback(page);
    }
}

fn assert_page_options_do_not_use_generic_fallback(page: &'static str) {
    for (index, option) in storybook_ui_option_contract::options_for_page(page)
        .iter()
        .enumerate()
    {
        let mut state = StorybookWindowState {
            selected_page: page,
            ..StorybookWindowState::default()
        };
        let row = option_hit_rect(page, index, *option);

        assert!(
            apply_click(&mut state, row.x + 1, row.y + 1),
            "{page} Inspector option `{}` was not clickable",
            option.setting
        );
        assert_not_generic_inspector_fallback(
            "last_action",
            state.screen_state.last_action,
            page,
            option.setting,
        );
        assert_not_generic_inspector_fallback(
            "last_event",
            state.screen_state.last_event,
            page,
            option.setting,
        );
        assert_not_generic_inspector_fallback(
            "state_label",
            state.screen_state.state_label,
            page,
            option.setting,
        );
        assert_ne!(
            option.after, state.screen_state.state_label,
            "{page} Inspector option `{}` used raw option value as state_label",
            option.setting
        );
        assert_ne!(
            option.setting, state.screen_state.state_label,
            "{page} Inspector option `{}` used raw option setting as state_label",
            option.setting
        );
    }
}

fn option_hit_rect(
    page: &str,
    index: usize,
    option: storybook_ui_option_contract::StorybookUiOptionContract,
) -> layout_metrics::LayoutRect {
    if !button_options::is_button_page(page) {
        return layout_metrics::inspector_setting_row_hit_rect(index);
    }
    let Some(control) = StorybookButtonOptionControl::all()
        .iter()
        .copied()
        .find(|control| control.setting_name() == option.setting)
    else {
        return layout_metrics::inspector_setting_row_hit_rect(index);
    };
    button_options::control_rect(control)
}

fn assert_not_generic_inspector_fallback(
    field_name: &str,
    actual: &str,
    page: &str,
    setting: &str,
) {
    assert!(
        !GENERIC_INSPECTOR_FALLBACKS.contains(&actual),
        "{page} Inspector option `{setting}` used generic fallback {field_name} `{actual}`"
    );
}
