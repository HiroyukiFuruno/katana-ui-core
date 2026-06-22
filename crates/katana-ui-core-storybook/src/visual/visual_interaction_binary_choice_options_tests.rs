use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, preview_detail, render, storybook_ui_option_contract};
use katana_ui_core::state::UiComponentState;

const DARK_THEME: &str = "dark";

#[test]
fn binary_choice_inspector_options_mutate_selected_disabled_focus_and_checked_semantic_state()
-> Result<(), String> {
    for &(page, prefix) in pages() {
        assert_options(page, prefix)?;
    }
    Ok(())
}

fn pages() -> &'static [(&'static str, &'static str)] {
    &[
        ("checkbox", "checkbox"),
        ("radio", "radio"),
        ("toggle", "toggle"),
        ("segmented-toggle", "segmented_toggle"),
    ]
}

fn assert_options(page: &'static str, prefix: &'static str) -> Result<(), String> {
    for &(setting, expected_value, suffix) in expected_states() {
        let mut state = page_state(page);
        let before = render_state(&state, page);
        click_option(&mut state, page, setting)?;
        let after = render_state(&state, page);

        assert_eq!(setting, state.screen_state.last_setting);
        assert_eq!(expected_value, state.screen_state.last_setting_value);
        assert_eq!(action_for(setting), state.screen_state.last_action);
        assert_eq!("selection_settings_changed", state.screen_state.last_event);
        assert_eq!(state_label(prefix, suffix), state.screen_state.state_label);
        assert_component_state(page, setting, &state.screen_state);
        assert!(component_body_pixel_diff(page, &before, &after) > 0);
    }
    Ok(())
}

#[test]
fn binary_choice_disabled_option_blocks_preview_mutation() -> Result<(), String> {
    for &(page, _) in &[("checkbox", "checkbox"), ("radio", "radio")] {
        let mut state = page_state(page);
        click_option(&mut state, page, "disabled")?;
        let disabled_state = state.screen_state.clone();
        let rect = preview_detail::component_action_hit_rect(page);

        assert!(apply_click(&mut state, rect.x + 1, rect.y + 1));
        assert_eq!(disabled_state, state.screen_state);
    }
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("selected", "true", "selected=true"),
        ("disabled", "true", "disabled=true"),
        ("focus", "visible", "focus=visible"),
        ("checked", "true", "checked=true"),
    ]
}

fn action_for(setting: &str) -> &'static str {
    match setting {
        "selected" => "settings_selected",
        "disabled" => "settings_disabled",
        "focus" => "settings_focus",
        "checked" => "settings_checked",
        _ => "",
    }
}

fn assert_component_state(
    page: &str,
    setting: &str,
    state: &super::screen_state::StorybookScreenState,
) {
    match page {
        "checkbox" => assert_binary_component_state(setting, state.checkbox_state_snapshot()),
        "radio" => assert_binary_component_state(setting, state.radio_state_snapshot()),
        _ => {}
    }
}

fn assert_binary_component_state(setting: &str, state: &UiComponentState) {
    match setting {
        "selected" | "checked" => assert_selected(state),
        "disabled" => {
            assert!(state.disabled);
            assert!(state.common.disabled);
        }
        "focus" => {
            assert!(state.focusable);
            assert!(state.common.focusable);
            assert!(state.interaction.focused);
        }
        _ => {}
    }
}

fn assert_selected(state: &UiComponentState) {
    assert!(state.checked);
    assert!(state.interaction.has_selection);
    assert_eq!(1, state.interaction.selected_index);
}

fn state_label(prefix: &str, suffix: &str) -> &'static str {
    match (prefix, suffix) {
        ("checkbox", "selected=true") => "checkbox.selected=true",
        ("checkbox", "disabled=true") => "checkbox.disabled=true",
        ("checkbox", "focus=visible") => "checkbox.focus=visible",
        ("checkbox", "checked=true") => "checkbox.checked=true",
        ("radio", "selected=true") => "radio.selected=true",
        ("radio", "disabled=true") => "radio.disabled=true",
        ("radio", "focus=visible") => "radio.focus=visible",
        ("radio", "checked=true") => "radio.checked=true",
        ("toggle", "selected=true") => "toggle.selected=true",
        ("toggle", "disabled=true") => "toggle.disabled=true",
        ("toggle", "focus=visible") => "toggle.focus=visible",
        ("toggle", "checked=true") => "toggle.checked=true",
        ("segmented_toggle", "selected=true") => "segmented_toggle.selected=true",
        ("segmented_toggle", "disabled=true") => "segmented_toggle.disabled=true",
        ("segmented_toggle", "focus=visible") => "segmented_toggle.focus=visible",
        ("segmented_toggle", "checked=true") => "segmented_toggle.checked=true",
        _ => "",
    }
}

fn click_option(state: &mut StorybookWindowState, page: &str, setting: &str) -> Result<(), String> {
    let index = option_index(page, setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn option_index(page: &str, setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(page)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing {page} option `{setting}`"))
}

fn render_state(state: &StorybookWindowState, page: &str) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}
