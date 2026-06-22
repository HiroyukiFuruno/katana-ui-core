use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "shortcut-cheatsheet";

#[test]
fn shortcut_cheatsheet_inspector_options_mutate_filter_selection_and_count_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state, expected_value) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_eq!(setting, state.screen_state.last_setting);
        assert_eq!(
            "settings_shortcut_cheatsheet_option",
            state.screen_state.last_action
        );
        assert_eq!("runtime_settings_changed", state.screen_state.last_event);
        assert_eq!(expected_value, state.screen_state.last_setting_value);
        assert_eq!(expected_state, state.screen_state.state_label);
        assert_shortcut_cheatsheet_runtime(setting, &state);
        assert!(component_body_pixel_diff(PAGE, &before, &after) > 0);
    }
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        (
            "shortcut_cheatsheet.label",
            "shortcut_cheatsheet.label=Editor keys",
            "Editor keys",
        ),
        (
            "shortcut_cheatsheet.groups",
            "shortcut_cheatsheet.groups=3",
            "3",
        ),
        (
            "shortcut_cheatsheet.group_title",
            "shortcut_cheatsheet.group_title=Navigation",
            "Navigation",
        ),
        (
            "shortcut_cheatsheet.items",
            "shortcut_cheatsheet.items=4",
            "4",
        ),
        (
            "shortcut_cheatsheet.item_combo",
            "shortcut_cheatsheet.item_combo=Cmd+Shift+P",
            "Cmd+Shift+P",
        ),
        (
            "shortcut_cheatsheet.group_layout",
            "shortcut_cheatsheet.group_layout=OneColumn",
            "OneColumn",
        ),
        (
            "shortcut_cheatsheet.query",
            "shortcut_cheatsheet.query=カテゴリ",
            "カテゴリ",
        ),
        (
            "shortcut_cheatsheet.selected",
            "shortcut_cheatsheet.selected=format",
            "format",
        ),
        (
            "shortcut_cheatsheet.result_count",
            "shortcut_cheatsheet.result_count=1",
            "1",
        ),
    ]
}

fn assert_shortcut_cheatsheet_runtime(setting: &str, state: &StorybookWindowState) {
    let cheatsheet = &state.screen_state.shortcut_cheatsheet;
    let options = cheatsheet.option_state();
    match setting {
        "shortcut_cheatsheet.label" => assert!(options.label_editor_keys),
        "shortcut_cheatsheet.groups" => assert_eq!(3, options.group_count),
        "shortcut_cheatsheet.group_title" => assert!(options.group_title_navigation),
        "shortcut_cheatsheet.items" => {
            assert_eq!(4, options.item_count);
            assert_eq!(4, cheatsheet.visible_item_count());
        }
        "shortcut_cheatsheet.item_combo" => assert!(options.item_combo_command_shift_p),
        "shortcut_cheatsheet.group_layout" => assert!(options.group_layout_one_column),
        "shortcut_cheatsheet.query" => {
            assert!(options.query_category);
            assert_eq!(1, cheatsheet.visible_item_count());
            assert_eq!("shortcut_cheatsheet_query", cheatsheet.callback_action());
        }
        "shortcut_cheatsheet.selected" => {
            assert!(options.selected_format);
            assert_eq!("shortcut_cheatsheet_selected", cheatsheet.callback_action());
        }
        "shortcut_cheatsheet.result_count" => {
            assert_eq!(1, options.result_count);
            assert_eq!(1, cheatsheet.visible_item_count());
        }
        _ => {}
    }
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
        .ok_or_else(|| format!("missing shortcut-cheatsheet option `{setting}`"))
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
