use super::visual_interaction_test_support::{
    assert_inspector_option_contract_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const BADGE_PAGE: &str = "badge";
const CARD_PAGE: &str = "card";
const EMPTY_STATE_PAGE: &str = "empty-state";

#[test]
fn badge_inspector_options_mutate_status_size_icon_and_variant_semantic_state() -> Result<(), String>
{
    for &(setting, expected_state) in badge_expected_states() {
        assert_option_state(BADGE_PAGE, setting, expected_state)?;
    }
    Ok(())
}

#[test]
fn card_inspector_options_mutate_slot_click_and_child_semantic_state() -> Result<(), String> {
    for &(setting, expected_state) in card_expected_states() {
        assert_option_state(CARD_PAGE, setting, expected_state)?;
    }
    Ok(())
}

#[test]
fn empty_state_inspector_options_mutate_content_alignment_and_action_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in empty_state_expected_states() {
        assert_option_state(EMPTY_STATE_PAGE, setting, expected_state)?;
    }
    Ok(())
}

fn badge_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("status.severity", "badge.status.severity=Danger"),
        ("badge.passive", "badge.passive=use-chip"),
        ("size", "badge.size=small"),
        ("tone", "badge.tone=accent"),
        ("badge.leading_icon", "badge.leading_icon=dot"),
        ("variant", "badge.variant=filled"),
    ]
}

fn card_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("card.label", "card.label=Project summary"),
        ("card.header", "card.header=custom"),
        ("card.footer", "card.footer=visible"),
        ("card.variant", "card.variant=theme_border"),
        ("card.padding", "card.padding=Large"),
        ("card.clickable", "card.clickable=true"),
        ("card.nested_controls", "card.nested_controls=interactive"),
        ("card.child_state", "card.child_state=changed"),
    ]
}

fn empty_state_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("empty_state.heading", "empty_state.heading=Empty project"),
        ("empty_state.body", "empty_state.body=create a file"),
        ("empty_state.icon", "empty_state.icon=search"),
        (
            "empty_state.illustration",
            "empty_state.illustration=folder",
        ),
        ("empty_state.tone", "empty_state.tone=Danger"),
        ("empty_state.size", "empty_state.size=Large"),
        ("empty_state.alignment", "empty_state.alignment=Leading"),
        (
            "empty_state.actions",
            "empty_state.actions=Primary+Secondary",
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
