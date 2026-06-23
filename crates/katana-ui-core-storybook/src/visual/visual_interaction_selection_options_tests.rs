use super::visual_interaction_test_support::{
    assert_inspector_option_contract_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn combo_box_inspector_options_mutate_choice_semantic_state() -> Result<(), String> {
    assert_page_states(
        "combo-box",
        &[
            ("combo.items", "combo.items=6"),
            ("interaction.open", "combo.open=true"),
            ("interaction.selected_index", "combo.selected_index=1"),
            ("interaction.value", "combo.value=two"),
            ("placeholder", "combo.placeholder=visible"),
            ("disabled", "combo.disabled=true"),
            ("readonly", "combo.readonly=true"),
            ("combo.input_value", "combo.input_value=tw"),
            ("combo.filter_result", "combo.filter_result=filtered"),
            ("combo.free_input", "combo.free_input=true"),
            (
                "combo.keyboard_navigation",
                "combo.keyboard_navigation=active",
            ),
            ("combo.placement", "combo.placement=above"),
            ("combo.highlighted_index", "combo.highlighted_index=1"),
            ("combo.long_list", "combo.long_list=true"),
            (
                "combo.outside_click_dismiss",
                "combo.outside_click_dismiss=true",
            ),
            ("combo.framed", "combo.framed=true"),
            ("combo.trigger_summary", "combo.trigger_summary=selected"),
            ("combo.select_action", "combo.select_action=callback"),
            ("validation", "combo.validation=invalid"),
        ],
    )
}

#[test]
fn select_box_inspector_options_mutate_choice_semantic_state() -> Result<(), String> {
    assert_page_states(
        "select-box",
        &[
            ("select.items", "select.items=6"),
            ("interaction.open", "select.open=true"),
            ("interaction.selected_index", "select.selected_index=1"),
            ("placeholder", "select.placeholder=visible"),
            ("disabled", "select.disabled=true"),
        ],
    )
}

#[test]
fn selection_list_inspector_options_mutate_list_semantic_state() -> Result<(), String> {
    assert_page_states(
        "selection-list",
        &[
            ("selection_list.items", "selection_list.items=1000"),
            (
                "interaction.selected_index",
                "selection_list.selected_index=2",
            ),
            ("selection_list.section", "selection_list.section=Recent"),
            ("selection_list.marker", "selection_list.marker=check"),
            ("selection_list.more_row", "selection_list.more_row=true"),
        ],
    )
}

#[test]
fn menu_button_inspector_options_mutate_menu_semantic_state() -> Result<(), String> {
    assert_page_states(
        "menu-button",
        &[
            ("menu.items", "menu_button.items=4"),
            ("interaction.open", "menu_button.open=true"),
            ("disabled", "menu_button.disabled=true"),
            ("menu.select_action", "menu_button.select_action=callback"),
        ],
    )
}

#[test]
fn search_box_inspector_options_mutate_search_semantic_state() -> Result<(), String> {
    assert_page_states(
        "search-box",
        &[
            ("text_entry.value", "search_box.value=typed query"),
            (
                "text_entry.submit_on_enter",
                "search_box.submit_on_enter=true",
            ),
            ("text_entry.clear_button", "search_box.clear_button=cleared"),
            ("text_entry.regex_case", "search_box.regex_case=true/true"),
        ],
    )
}

fn assert_page_states(
    page: &'static str,
    expected_states: &[(&'static str, &'static str)],
) -> Result<(), String> {
    for &(setting, expected_state) in expected_states {
        let mut state = page_state(page);
        let before = render_state(&state, page);
        click_option(&mut state, page, setting)?;
        let after = render_state(&state, page);

        assert_inspector_option_contract_state(&state, page, setting, expected_state)?;
        assert!(
            component_body_pixel_diff(page, &before, &after) > 0,
            "{page}"
        );
    }
    Ok(())
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
