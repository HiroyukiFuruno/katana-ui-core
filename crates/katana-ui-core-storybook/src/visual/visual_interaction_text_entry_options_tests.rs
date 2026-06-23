use super::visual_interaction_test_support::{
    assert_inspector_option_contract_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const TEXT_INPUT_PAGE: &str = "text-input";
const TEXT_AREA_PAGE: &str = "text-area";

#[test]
fn text_input_inspector_options_mutate_value_slot_icon_and_blocking_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in text_input_expected_states() {
        assert_option_state(TEXT_INPUT_PAGE, setting, expected_state)?;
    }
    Ok(())
}

#[test]
fn text_area_inspector_options_mutate_multiline_scroll_slot_and_blocking_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in text_area_expected_states() {
        assert_option_state(TEXT_AREA_PAGE, setting, expected_state)?;
    }
    Ok(())
}

fn text_input_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("interaction.value", "text_input.value=typed 日本語"),
        ("readonly", "text_input.readonly=true"),
        ("placeholder", "text_input.placeholder=hidden"),
        (
            "text_entry.leading_slot_reserved",
            "text_input.leading_slot.reserved=true",
        ),
        (
            "text_entry.leading_slot.icon",
            "text_input.leading_slot.icon=search-svg",
        ),
        (
            "text_entry.trailing_icon_buttons",
            "text_input.trailing_icon_buttons=callbacks",
        ),
        ("validation", "text_input.validation=invalid"),
        ("ime", "text_input.ime=composition"),
        ("theme.input_bg", "text_input.theme.input_bg=light"),
        ("disabled", "text_input.disabled=true"),
        ("font_role", "text_input.font_role=monospace"),
        (
            "text_entry.trailing_slot_reserved",
            "text_input.trailing_slot.reserved=true",
        ),
        ("text_entry.clear_action", "text_input.clear_action=visible"),
        (
            "text_entry.submit_on_enter",
            "text_input.submit_on_enter=true",
        ),
        ("text_entry.emoji_enabled", "text_input.emoji_enabled=false"),
    ]
}

fn text_area_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("text_area.submit_key", "text_area.submit_key=ModEnter"),
        ("text_area.newline_key", "text_area.newline_key=Enter"),
        ("text_area.tab_behavior", "text_area.tab_behavior=InsertTab"),
        ("text_area.auto_grow", "text_area.auto_grow=false"),
        ("text_area.wrap_policy", "text_area.wrap_policy=None"),
        ("text_area.resize_enabled", "text_area.resize_enabled=true"),
        (
            "text_area.vertical_scroll_enabled",
            "text_area.vertical_scroll_enabled=true",
        ),
        (
            "text_area.horizontal_scroll_enabled",
            "text_area.horizontal_scroll_enabled=true",
        ),
        (
            "text_area.vertical_scrollbar_visible",
            "text_area.vertical_scrollbar_visible=true",
        ),
        (
            "text_area.horizontal_scrollbar_visible",
            "text_area.horizontal_scrollbar_visible=true",
        ),
        (
            "text_area.leading_slot.icon",
            "text_area.leading_slot.icon=search-svg",
        ),
        (
            "text_area.trailing_icon_buttons",
            "text_area.trailing_icon_buttons=callbacks",
        ),
        ("text_area.clear_action", "text_area.clear_action=visible"),
        ("text_area.value", "text_area.value=typed"),
        ("text_area.placeholder", "text_area.placeholder=visible"),
        ("text_area.font_role", "text_area.font_role=monospace"),
        ("text_area.disabled", "text_area.disabled=true"),
        ("text_area.readonly", "text_area.readonly=true"),
        ("text_area.invalid", "text_area.invalid=true"),
        ("text_area.min_rows", "text_area.min_rows=3"),
        ("text_area.max_rows", "text_area.max_rows=8"),
        ("text_area.ime_enabled", "text_area.ime_enabled=false"),
        (
            "text_area.leading_slot_reserved",
            "text_area.leading_slot.reserved=true",
        ),
        (
            "text_area.trailing_slot_reserved",
            "text_area.trailing_slot.reserved=true",
        ),
    ]
}

fn assert_option_state(
    page: &'static str,
    setting: &str,
    expected_state: &'static str,
) -> Result<(), String> {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    click_option(&mut state, page, setting)?;
    let after = render_state(page, &state);

    assert_inspector_option_contract_state(&state, page, setting, expected_state)?;
    assert!(component_body_pixel_diff(page, &before, &after) > 0);
    Ok(())
}

fn click_option(
    state: &mut StorybookWindowState,
    page: &'static str,
    setting: &str,
) -> Result<(), String> {
    let index = option_index(page, setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn option_index(page: &'static str, setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(page)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing {page} option `{setting}`"))
}

fn render_state(page: &'static str, state: &StorybookWindowState) -> super::Canvas {
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
