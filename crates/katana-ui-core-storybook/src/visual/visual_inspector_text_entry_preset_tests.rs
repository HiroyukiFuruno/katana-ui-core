use super::layout_metrics;
use super::storybook_ui_option_contract;
use super::window_interaction::{StorybookWindowState, apply_click};

#[test]
fn inspector_rows_select_matching_preset_tabs_for_option_focused_pages() {
    for (page, expected) in [
        (
            "text-input",
            &[
                ("interaction.value", 0),
                ("ime", 1),
                ("readonly", 2),
                ("placeholder", 3),
                ("text_entry.leading_slot_reserved", 4),
                ("text_entry.leading_slot.icon", 5),
                ("text_entry.trailing_icon_buttons", 6),
                ("validation", 7),
                ("theme.input_bg", 8),
            ][..],
        ),
        (
            "text-area",
            &[
                ("text_area.submit_key", 0),
                ("text_area.newline_key", 1),
                ("text_area.wrap_policy", 2),
                ("text_area.resize_enabled", 3),
                ("text_area.auto_grow", 4),
                ("text_area.vertical_scroll_enabled", 5),
                ("text_area.horizontal_scroll_enabled", 6),
                ("text_area.tab_behavior", 7),
                ("text_area.vertical_scrollbar_visible", 8),
                ("text_area.horizontal_scrollbar_visible", 9),
                ("text_area.leading_slot.icon", 10),
                ("text_area.trailing_icon_buttons", 11),
                ("text_area.clear_action", 12),
            ][..],
        ),
        (
            "tabs",
            &[
                ("tabs.add", 1),
                ("tabs.close", 1),
                ("tabs.pin", 2),
                ("tabs.move", 3),
                ("tabs.group", 4),
                ("tabs.overflow", 5),
                ("tabs.active_scroll", 6),
            ][..],
        ),
    ] {
        for &(setting, preset_index) in expected {
            let index = option_index(page, setting);
            assert!(index.is_some(), "{page} option `{setting}` is missing");
            let Some(index) = index else {
                continue;
            };
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };
            let row = layout_metrics::inspector_setting_row_hit_rect(index);

            assert!(
                apply_click(&mut state, row.x + 1, row.y + 1),
                "{page} Inspector option `{setting}` was not clickable"
            );
            assert_eq!(
                preset_index, state.preset_index,
                "{page} Inspector option `{setting}` did not select the matching preset tab"
            );
        }
    }
}

fn option_index(page: &str, setting: &str) -> Option<usize> {
    storybook_ui_option_contract::options_for_page(page)
        .iter()
        .position(|option| option.setting == setting)
}
