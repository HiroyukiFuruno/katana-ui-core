use super::visual_interaction_test_support::{
    assert_inspector_option_contract_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "toolbar";

#[test]
fn toolbar_inspector_options_mutate_action_split_and_group_semantic_state() -> Result<(), String> {
    for &(setting, expected_state) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_inspector_option_contract_state(&state, PAGE, setting, expected_state)?;
        assert!(component_body_pixel_diff(PAGE, &before, &after) > 0);
    }
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("toolbar.display_mode", "toolbar.display=icon_text"),
        ("toolbar.density", "toolbar.density=compact"),
        ("toolbar.overflow_strategy", "toolbar.overflow=menu"),
        ("toolbar.actions", "toolbar.actions=changed"),
        ("toolbar.groups", "toolbar.groups=changed"),
        ("toolbar.context_menu_anchor", "toolbar.anchor=pointer"),
        ("toolbar.action_priority", "toolbar.action.priority=90"),
        (
            "toolbar.action_accelerator",
            "toolbar.action.accelerator=Alt+P",
        ),
        ("toolbar.action_split", "toolbar.action.split=menu"),
        ("toolbar.action_group", "toolbar.action.group=edit"),
        ("toolbar.action_tooltip", "toolbar.action.tooltip=Save file"),
        ("toolbar.action_a11y", "toolbar.action.a11y=Save file"),
        ("toolbar.action_disabled", "toolbar.action.disabled=true"),
        ("toolbar.group_label", "toolbar.group.label=File actions"),
        ("toolbar.group_divider", "toolbar.group.divider=false"),
        ("toolbar.split_disabled", "toolbar.split.disabled=true"),
        ("toolbar.split_tooltip", "toolbar.split.tooltip=visible"),
        ("toolbar.split_a11y", "toolbar.split.a11y=Open menu"),
    ]
}

fn click_option(state: &mut StorybookWindowState, setting: &str) -> Result<(), String> {
    let index = option_index(setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn option_index(setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(PAGE)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing toolbar option `{setting}`"))
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        0,
        state.screen_state.clone(),
    )
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}
