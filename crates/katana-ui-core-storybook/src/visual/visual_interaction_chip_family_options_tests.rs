use super::visual_interaction_test_support::{
    assert_inspector_option_contract_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const ATTACHMENT_PAGE: &str = "attachment-chip";
const CHIP_GROUP_PAGE: &str = "chip-group";

#[test]
fn attachment_chip_inspector_options_mutate_kind_status_and_retry_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in attachment_expected_states() {
        assert_option_state(ATTACHMENT_PAGE, setting, expected_state)?;
    }
    Ok(())
}

#[test]
fn chip_group_inspector_options_mutate_overflow_reorder_and_width_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in chip_group_expected_states() {
        assert_option_state(CHIP_GROUP_PAGE, setting, expected_state)?;
    }
    Ok(())
}

fn attachment_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("attachment.kind", "attachment.kind=Image"),
        ("attachment.name", "attachment.name=proposal.pdf"),
        ("attachment.meta", "attachment.meta=size+mime"),
        ("attachment.thumbnail", "attachment.thumbnail=preview"),
        ("attachment.status", "attachment.status=Error"),
        ("attachment.progress", "attachment.progress=100"),
        ("attachment.retry", "attachment.retry=visible"),
    ]
}

fn chip_group_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("chip_group.label", "chip_group.label=Active filters"),
        ("chip_group.chip_count", "chip_group.chip_count=5"),
        ("chip_group.wrap", "chip_group.wrap=true"),
        ("chip_group.overflow", "chip_group.overflow=Menu"),
        ("chip_group.reorder", "chip_group.reorder=true"),
        ("chip_group.gap", "chip_group.gap=8"),
        (
            "chip_group.available_width",
            "chip_group.available_width=132",
        ),
        (
            "chip_group.overflow_trigger_width",
            "chip_group.overflow_trigger_width=32",
        ),
        ("chip_group.hidden_count", "chip_group.hidden_count=2"),
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
