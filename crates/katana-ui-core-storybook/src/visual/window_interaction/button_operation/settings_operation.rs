use super::StorybookButtonOperation;
use crate::visual::button_options::{control_at, is_button_page};
use crate::visual::storybook_ui_option_contract::{self, StorybookUiOptionContract};

const TEXT_INPUT_VALUE_PRESET: usize = 0;
const TEXT_INPUT_IME_PRESET: usize = 1;
const TEXT_INPUT_READONLY_PRESET: usize = 2;
const TEXT_INPUT_PLACEHOLDER_PRESET: usize = 3;
const TEXT_INPUT_RESERVED_SLOT_PRESET: usize = 4;
const TEXT_INPUT_LEADING_ICON_PRESET: usize = 5;
const TEXT_INPUT_TRAILING_BUTTONS_PRESET: usize = 6;
const TEXT_INPUT_VALIDATION_PRESET: usize = 7;
const TEXT_INPUT_THEME_PRESET: usize = 8;
const TEXT_AREA_SUBMIT_PRESET: usize = 0;
const TEXT_AREA_NEWLINE_PRESET: usize = 1;
const TEXT_AREA_WRAP_PRESET: usize = 2;
const TEXT_AREA_RESIZE_PRESET: usize = 3;
const TEXT_AREA_AUTO_GROW_PRESET: usize = 4;
const TEXT_AREA_VERTICAL_SCROLL_PRESET: usize = 5;
const TEXT_AREA_HORIZONTAL_SCROLL_PRESET: usize = 6;
const TEXT_AREA_TAB_PRESET: usize = 7;
const TEXT_AREA_VERTICAL_SCROLLBAR_PRESET: usize = 8;
const TEXT_AREA_HORIZONTAL_SCROLLBAR_PRESET: usize = 9;
const TEXT_AREA_LEADING_SLOT_PRESET: usize = 10;
const TEXT_AREA_TRAILING_BUTTONS_PRESET: usize = 11;
const TEXT_AREA_CLEAR_ACTION_PRESET: usize = 12;
const TABS_ADD_CLOSE_PRESET: usize = 1;
const TABS_PIN_PRESET: usize = 2;
const TABS_MOVE_PRESET: usize = 3;
const TABS_GROUP_PRESET: usize = 4;
const TABS_OVERFLOW_PRESET: usize = 5;
const TABS_ACTIVE_FOLLOW_PRESET: usize = 6;

pub(super) fn operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if let Some(control) = control_at(page, x, y) {
        return Some(StorybookButtonOperation::ButtonOption(control));
    }
    if is_button_page(page) {
        return None;
    }
    settings_option_at(page, x, y).map(|(option, preset_index)| {
        StorybookButtonOperation::SettingsOption {
            option,
            preset_index,
        }
    })
}

fn settings_option_at(
    page: &str,
    x: usize,
    y: usize,
) -> Option<(StorybookUiOptionContract, Option<usize>)> {
    storybook_ui_option_contract::options_for_page(page)
        .iter()
        .enumerate()
        .find_map(|(index, option)| {
            if !button_setting_hit_rect_for_index(index).contains(x, y) {
                return None;
            }
            Some((
                *option,
                Some(preset_index_for_option(page, option.setting, index)),
            ))
        })
}

fn button_setting_hit_rect_for_index(index: usize) -> crate::visual::layout_metrics::LayoutRect {
    crate::visual::layout_metrics::inspector_setting_row_hit_rect(index)
}

fn preset_index_for_setting(page: &str, setting: &str) -> Option<usize> {
    match (page, setting) {
        ("text-input", "interaction.value") => Some(TEXT_INPUT_VALUE_PRESET),
        ("text-input", "ime") => Some(TEXT_INPUT_IME_PRESET),
        ("text-input", "readonly") => Some(TEXT_INPUT_READONLY_PRESET),
        ("text-input", "placeholder") => Some(TEXT_INPUT_PLACEHOLDER_PRESET),
        ("text-input", "text_entry.leading_slot_reserved") => Some(TEXT_INPUT_RESERVED_SLOT_PRESET),
        ("text-input", "text_entry.leading_slot.icon") => Some(TEXT_INPUT_LEADING_ICON_PRESET),
        ("text-input", "text_entry.trailing_icon_buttons") => {
            Some(TEXT_INPUT_TRAILING_BUTTONS_PRESET)
        }
        ("text-input", "validation") => Some(TEXT_INPUT_VALIDATION_PRESET),
        ("text-input", "theme.input_bg") => Some(TEXT_INPUT_THEME_PRESET),
        ("text-area", "text_area.wrap_policy") => Some(TEXT_AREA_WRAP_PRESET),
        ("text-area", "text_area.resize_enabled") => Some(TEXT_AREA_RESIZE_PRESET),
        ("text-area", "text_area.auto_grow") => Some(TEXT_AREA_AUTO_GROW_PRESET),
        ("text-area", "text_area.vertical_scroll_enabled") => {
            Some(TEXT_AREA_VERTICAL_SCROLL_PRESET)
        }
        ("text-area", "text_area.vertical_scrollbar_visible") => {
            Some(TEXT_AREA_VERTICAL_SCROLLBAR_PRESET)
        }
        ("text-area", "text_area.horizontal_scroll_enabled") => {
            Some(TEXT_AREA_HORIZONTAL_SCROLL_PRESET)
        }
        ("text-area", "text_area.horizontal_scrollbar_visible") => {
            Some(TEXT_AREA_HORIZONTAL_SCROLLBAR_PRESET)
        }
        ("text-area", "text_area.leading_slot.icon") => Some(TEXT_AREA_LEADING_SLOT_PRESET),
        ("text-area", "text_area.trailing_icon_buttons") => Some(TEXT_AREA_TRAILING_BUTTONS_PRESET),
        ("text-area", "text_area.clear_action") => Some(TEXT_AREA_CLEAR_ACTION_PRESET),
        ("text-area", "text_area.submit_key") => Some(TEXT_AREA_SUBMIT_PRESET),
        ("text-area", "text_area.newline_key") => Some(TEXT_AREA_NEWLINE_PRESET),
        ("text-area", "text_area.tab_behavior") => Some(TEXT_AREA_TAB_PRESET),
        ("tabs", "tabs.add") => Some(TABS_ADD_CLOSE_PRESET),
        ("tabs", "tabs.close") => Some(TABS_ADD_CLOSE_PRESET),
        ("tabs", "tabs.pin") => Some(TABS_PIN_PRESET),
        ("tabs", "tabs.move") => Some(TABS_MOVE_PRESET),
        ("tabs", "tabs.group") => Some(TABS_GROUP_PRESET),
        ("tabs", "tabs.overflow") => Some(TABS_OVERFLOW_PRESET),
        ("tabs", "tabs.active_scroll") => Some(TABS_ACTIVE_FOLLOW_PRESET),
        _ => None,
    }
}

fn preset_index_for_option(page: &str, setting: &str, option_index: usize) -> usize {
    match preset_index_for_setting(page, setting) {
        Some(index) => index,
        None => option_index,
    }
}
