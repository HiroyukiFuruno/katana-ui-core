use super::visual_interaction_test_support::{
    assert_inspector_option_state, assert_inspector_option_state_with_event,
    component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn navigation_inspector_options_mutate_menu_form_breadcrumb_side_and_tree_semantic_state()
-> Result<(), String> {
    assert_options("menu", menu_states())?;
    assert_options("form-field", form_field_states())?;
    assert_options("breadcrumb", breadcrumb_states())?;
    assert_options("side-menu", side_menu_states())?;
    assert_options("tree-view", tree_states())
}

fn menu_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("menu.common_props", "dense", "menu.common_props=dense"),
        ("children", "changed", "menu.children=changed"),
        ("interaction.selected_index", "1", "menu.selected_index=1"),
        (
            "menu.panel_placement",
            "resolved",
            "menu.panel_placement=resolved",
        ),
    ]
}

fn form_field_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        (
            "form_field.common_props",
            "dense",
            "form_field.common_props=dense",
        ),
        ("children", "changed", "form_field.children=changed"),
        ("form_field.invalid", "true", "form_field.invalid=true"),
        (
            "form_field.helper_text",
            "long",
            "form_field.helper_text=long",
        ),
    ]
}

fn breadcrumb_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("breadcrumb.items", "4", "breadcrumb.items=4"),
        ("children", "changed", "breadcrumb.children=changed"),
        ("interaction.selected_index", "2", "route=2"),
        (
            "breadcrumb.crumb_action",
            "callback",
            "breadcrumb.crumb_action=callback",
        ),
    ]
}

fn side_menu_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("side_menu.items", "5", "side_menu.items=5"),
        ("children", "changed", "side_menu.children=changed"),
        (
            "interaction.selected_index",
            "1",
            "side_menu.selected_index=1",
        ),
        (
            "side_menu.hover_expansion",
            "true",
            "side_menu.hover_expansion=true",
        ),
    ]
}

fn tree_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("line", "hidden", "tree.line=hidden"),
        ("node_marker", "leaf", "tree.node_marker=leaf"),
        ("trigger", "text", "tree.trigger=text"),
        ("context_menu", "enabled", "tree.context_menu=enabled"),
    ]
}

fn assert_options(
    page: &'static str,
    expected_states: &'static [(&'static str, &'static str, &'static str)],
) -> Result<(), String> {
    for &(setting, expected_value, expected_state) in expected_states {
        let mut state = page_state(page);
        let before = render_state(&state, page);
        click_option(&mut state, page, setting)?;
        let after = render_state(&state, page);

        assert_navigation_option_state(&state, page, setting, expected_value, expected_state);
        assert!(component_body_pixel_diff(page, &before, &after) > 0);
    }
    Ok(())
}

fn assert_navigation_option_state(
    state: &StorybookWindowState,
    page: &str,
    setting: &str,
    expected_value: &str,
    expected_state: &str,
) {
    if page == "breadcrumb" && setting == "interaction.selected_index" {
        assert_inspector_option_state_with_event(
            state,
            setting,
            expected_value,
            expected_state,
            "breadcrumb_click",
            "route_changed",
        );
        return;
    }
    if page == "form-field" && setting == "form_field.invalid" {
        assert_inspector_option_state_with_event(
            state,
            setting,
            expected_value,
            expected_state,
            "field_validate",
            "validation_changed",
        );
        return;
    }
    if page == "form-field" && setting == "form_field.helper_text" {
        assert_inspector_option_state_with_event(
            state,
            setting,
            expected_value,
            expected_state,
            "form_field_helper_text",
            "helper_text_changed",
        );
        return;
    }
    assert_inspector_option_state(state, page, setting, expected_value, expected_state);
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
