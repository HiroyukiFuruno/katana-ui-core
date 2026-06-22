use super::visual_interaction_test_support::{
    assert_inspector_option_contract_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "icon";

#[test]
fn icon_inspector_options_mutate_svg_source_role_paint_and_token_semantic_state()
-> Result<(), String> {
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
        ("content.value", "icon.content.value=custom"),
        ("visual.role", "icon.visual.role=icon"),
        ("a11y.label", "icon.a11y.label=changed"),
        ("theme.color", "icon.theme.color=accent"),
        ("icon.svg_source", "icon.svg_source=custom-svg"),
        ("icon.svg_icon", "icon.svg_icon=props-object"),
        ("icon.view_box", "icon.view_box=0 0 24 24"),
        ("icon.path_summary", "icon.path_summary=search-outline"),
        ("icon.paint_policy", "icon.paint_policy=currentColor"),
        ("icon.role", "icon.role=action"),
        ("icon.color_token", "icon.color_token=accent"),
        ("icon.theme_token", "icon.theme_token=muted"),
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
        .ok_or_else(|| format!("missing icon option `{setting}`"))
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}
