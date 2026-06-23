use super::button_options;
use super::layout_metrics;
use super::storybook_ui_option_contract;
use super::window_interaction::{StorybookWindowState, apply_click};
use crate::catalog::StoryPresetLabels;
use crate::requirements::StoryRequirements;

const VALUE_PRESET_INDEX: usize = 0;
const IME_PRESET_INDEX: usize = 1;
const READONLY_PRESET_INDEX: usize = 2;
const PLACEHOLDER_PRESET_INDEX: usize = 3;
const RESERVED_SLOT_PRESET_INDEX: usize = 4;
const LEADING_ICON_PRESET_INDEX: usize = 5;
const TRAILING_BUTTONS_PRESET_INDEX: usize = 6;
const VALIDATION_PRESET_INDEX: usize = 7;
const THEME_PRESET_INDEX: usize = 8;
const SUBMIT_PRESET_INDEX: usize = 0;
const NEWLINE_PRESET_INDEX: usize = 1;
const WRAP_PRESET_INDEX: usize = 2;
const RESIZE_PRESET_INDEX: usize = 3;
const AUTO_GROW_PRESET_INDEX: usize = 4;
const VERTICAL_SCROLL_PRESET_INDEX: usize = 5;
const HORIZONTAL_SCROLL_PRESET_INDEX: usize = 6;
const TAB_PRESET_INDEX: usize = 7;
const VERTICAL_SCROLLBAR_PRESET_INDEX: usize = 8;
const HORIZONTAL_SCROLLBAR_PRESET_INDEX: usize = 9;
const TEXT_AREA_LEADING_SLOT_PRESET_INDEX: usize = 10;
const TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX: usize = 11;
const TEXT_AREA_CLEAR_ACTION_PRESET_INDEX: usize = 12;
const TABS_ADD_CLOSE_PRESET_INDEX: usize = 1;
const TABS_PIN_PRESET_INDEX: usize = 2;
const TABS_MOVE_PRESET_INDEX: usize = 3;
const TABS_GROUP_PRESET_INDEX: usize = 4;
const TABS_OVERFLOW_PRESET_INDEX: usize = 5;
const TABS_ACTIVE_SCROLL_PRESET_INDEX: usize = 6;

#[test]
fn inspector_rows_select_preset_tabs_for_every_non_button_option() {
    for page in non_button_pages() {
        let options = storybook_ui_option_contract::options_for_page(page);
        let labels = StoryPresetLabels::for_page(page);
        assert!(
            labels.len() >= options.len(),
            "{page} must expose at least one preset tab for every Inspector option"
        );
        for (option_index, option) in options.iter().enumerate() {
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };
            let row = layout_metrics::inspector_setting_row_hit_rect(option_index);

            assert!(
                apply_click(&mut state, row.x + 1, row.y + 1),
                "{page} Inspector option `{}` was not clickable",
                option.setting
            );
            assert_eq!(
                expected_preset_index(page, option.setting, option_index),
                state.preset_index,
                "{page} Inspector option `{}` did not select the matching preset tab",
                option.setting
            );
        }
    }
}

fn non_button_pages() -> impl Iterator<Item = &'static str> {
    StoryRequirements::required_pages()
        .iter()
        .copied()
        .filter(|page| !button_options::is_button_page(page))
}

fn expected_preset_index(page: &str, setting: &str, option_index: usize) -> usize {
    match (page, setting) {
        ("text-input", "interaction.value") => VALUE_PRESET_INDEX,
        ("text-input", "ime") => IME_PRESET_INDEX,
        ("text-input", "readonly") => READONLY_PRESET_INDEX,
        ("text-input", "placeholder") => PLACEHOLDER_PRESET_INDEX,
        ("text-input", "text_entry.leading_slot_reserved") => RESERVED_SLOT_PRESET_INDEX,
        ("text-input", "text_entry.leading_slot.icon") => LEADING_ICON_PRESET_INDEX,
        ("text-input", "text_entry.trailing_icon_buttons") => TRAILING_BUTTONS_PRESET_INDEX,
        ("text-input", "validation") => VALIDATION_PRESET_INDEX,
        ("text-input", "theme.input_bg") => THEME_PRESET_INDEX,
        ("text-area", "text_area.submit_key") => SUBMIT_PRESET_INDEX,
        ("text-area", "text_area.newline_key") => NEWLINE_PRESET_INDEX,
        ("text-area", "text_area.wrap_policy") => WRAP_PRESET_INDEX,
        ("text-area", "text_area.resize_enabled") => RESIZE_PRESET_INDEX,
        ("text-area", "text_area.auto_grow") => AUTO_GROW_PRESET_INDEX,
        ("text-area", "text_area.vertical_scroll_enabled") => VERTICAL_SCROLL_PRESET_INDEX,
        ("text-area", "text_area.horizontal_scroll_enabled") => HORIZONTAL_SCROLL_PRESET_INDEX,
        ("text-area", "text_area.tab_behavior") => TAB_PRESET_INDEX,
        ("text-area", "text_area.vertical_scrollbar_visible") => VERTICAL_SCROLLBAR_PRESET_INDEX,
        ("text-area", "text_area.horizontal_scrollbar_visible") => {
            HORIZONTAL_SCROLLBAR_PRESET_INDEX
        }
        ("text-area", "text_area.leading_slot.icon") => TEXT_AREA_LEADING_SLOT_PRESET_INDEX,
        ("text-area", "text_area.trailing_icon_buttons") => TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX,
        ("text-area", "text_area.clear_action") => TEXT_AREA_CLEAR_ACTION_PRESET_INDEX,
        ("tabs", "tabs.add") => TABS_ADD_CLOSE_PRESET_INDEX,
        ("tabs", "tabs.close") => TABS_ADD_CLOSE_PRESET_INDEX,
        ("tabs", "tabs.pin") => TABS_PIN_PRESET_INDEX,
        ("tabs", "tabs.move") => TABS_MOVE_PRESET_INDEX,
        ("tabs", "tabs.group") => TABS_GROUP_PRESET_INDEX,
        ("tabs", "tabs.overflow") => TABS_OVERFLOW_PRESET_INDEX,
        ("tabs", "tabs.active_scroll") => TABS_ACTIVE_SCROLL_PRESET_INDEX,
        _ => option_index,
    }
}
