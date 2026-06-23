use super::visual_interaction_test_support::{
    assert_inspector_option_state, assert_inspector_option_state_with_event,
    component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn collection_inspector_options_mutate_list_collapsible_hover_and_panel_semantic_state()
-> Result<(), String> {
    assert_options("list", list_states())?;
    assert_options("collapsible-panel", collapsible_panel_states())?;
    assert_options("hover-card", hover_card_states())?;
    assert_options("panel", panel_states())
}

fn list_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("list.rows", "200", "list.rows=200"),
        ("list.selection", "row-2", "list.selection=row-2"),
        ("list.empty_state", "true", "list.empty_state=true"),
        (
            "list.virtualization",
            "visible_range",
            "list.virtualization=visible_range",
        ),
        ("list.theme_row", "accent", "list.theme_row=accent"),
    ]
}

fn collapsible_panel_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        (
            "collapsible_panel.mode",
            "floating_overlay",
            "collapsible_panel.mode=floating_overlay",
        ),
        (
            "collapsible_panel.width",
            "320",
            "collapsible_panel.width=320",
        ),
        (
            "collapsible_panel.pinned",
            "false",
            "collapsible_panel.pinned=false",
        ),
        (
            "collapsible_panel.expand_on_hover",
            "true",
            "collapsible_panel.expand_on_hover=true",
        ),
        (
            "collapsible_panel.resize_handle",
            "true",
            "collapsible_panel.resize_handle=true",
        ),
    ]
}

fn hover_card_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        (
            "hover_card.open_delay_ms",
            "180",
            "hover_card.open_delay_ms=180",
        ),
        (
            "hover_card.close_delay_ms",
            "220",
            "hover_card.close_delay_ms=220",
        ),
        (
            "hover_card.pointer_follow",
            "true",
            "hover_card.pointer_follow=true",
        ),
        (
            "hover_card.slot_action",
            "visible",
            "hover_card.slot_action=visible",
        ),
    ]
}

fn panel_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("active_panel", "details", "active_panel=changed"),
        (
            "vertical_scroll",
            "changed",
            "panel.vertical_scroll=changed",
        ),
        (
            "horizontal_scroll",
            "changed",
            "panel.horizontal_scroll=changed",
        ),
        ("scrollbar_visibility", "off", "panel_scrollbar=hidden"),
        (
            "nested_state",
            "independent",
            "panel.nested_state=independent",
        ),
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

        assert_collection_option_state(&state, page, setting, expected_value, expected_state);
        assert!(component_body_pixel_diff(page, &before, &after) > 0);
    }
    Ok(())
}

fn assert_collection_option_state(
    state: &StorybookWindowState,
    page: &str,
    setting: &str,
    expected_value: &str,
    expected_state: &str,
) {
    if page == "panel" && setting == "active_panel" {
        assert_inspector_option_state_with_event(
            state,
            setting,
            expected_value,
            expected_state,
            "panel_active_select",
            "panel_active_changed",
        );
        return;
    }
    if page == "panel" && setting == "scrollbar_visibility" {
        assert_inspector_option_state_with_event(
            state,
            setting,
            expected_value,
            expected_state,
            "panel_scrollbar_hide",
            "panel_scrollbar_visibility_changed",
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
