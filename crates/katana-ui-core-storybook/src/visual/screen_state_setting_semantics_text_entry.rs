pub(in crate::visual) fn text_input_state(setting: &'static str) -> &'static str {
    match setting {
        "interaction.value" => "text_input.value=typed 日本語",
        "readonly" => "text_input.readonly=true",
        "placeholder" => "text_input.placeholder=hidden",
        "text_entry.leading_slot_reserved" => "text_input.leading_slot.reserved=true",
        "text_entry.leading_slot.icon" => "text_input.leading_slot.icon=search-svg",
        "text_entry.trailing_icon_buttons" => "text_input.trailing_icon_buttons=callbacks",
        "validation" => "text_input.validation=invalid",
        "ime" => "text_input.ime=composition",
        "theme.input_bg" => "text_input.theme.input_bg=light",
        "disabled" => "text_input.disabled=true",
        "font_role" => "text_input.font_role=monospace",
        "text_entry.trailing_slot_reserved" => "text_input.trailing_slot.reserved=true",
        "text_entry.clear_action" => "text_input.clear_action=visible",
        "text_entry.submit_on_enter" => "text_input.submit_on_enter=true",
        "text_entry.emoji_enabled" => "text_input.emoji_enabled=false",
        _ => setting,
    }
}

pub(in crate::visual) fn text_area_state(setting: &'static str) -> &'static str {
    match setting {
        "text_area.submit_key" => "text_area.submit_key=ModEnter",
        "text_area.newline_key" => "text_area.newline_key=Enter",
        "text_area.tab_behavior" => "text_area.tab_behavior=InsertTab",
        "text_area.auto_grow" => "text_area.auto_grow=false",
        "text_area.wrap_policy" => "text_area.wrap_policy=None",
        "text_area.resize_enabled" => "text_area.resize_enabled=true",
        "text_area.vertical_scroll_enabled" => "text_area.vertical_scroll_enabled=true",
        "text_area.horizontal_scroll_enabled" => "text_area.horizontal_scroll_enabled=true",
        "text_area.vertical_scrollbar_visible" => "text_area.vertical_scrollbar_visible=true",
        "text_area.horizontal_scrollbar_visible" => "text_area.horizontal_scrollbar_visible=true",
        "text_area.leading_slot.icon" => "text_area.leading_slot.icon=search-svg",
        "text_area.trailing_icon_buttons" => "text_area.trailing_icon_buttons=callbacks",
        "text_area.clear_action" => "text_area.clear_action=visible",
        "text_area.value" => "text_area.value=typed",
        "text_area.placeholder" => "text_area.placeholder=visible",
        "text_area.font_role" => "text_area.font_role=monospace",
        "text_area.disabled" => "text_area.disabled=true",
        "text_area.readonly" => "text_area.readonly=true",
        "text_area.invalid" => "text_area.invalid=true",
        "text_area.min_rows" => "text_area.min_rows=3",
        "text_area.max_rows" => "text_area.max_rows=8",
        "text_area.ime_enabled" => "text_area.ime_enabled=false",
        "text_area.leading_slot_reserved" => "text_area.leading_slot.reserved=true",
        "text_area.trailing_slot_reserved" => "text_area.trailing_slot.reserved=true",
        _ => setting,
    }
}

#[cfg(test)]
mod tests {
    use super::{text_area_state, text_input_state};

    #[test]
    fn unknown_text_entry_settings_are_preserved() {
        assert_eq!("custom.input", text_input_state("custom.input"));
        assert_eq!("custom.area", text_area_state("custom.area"));
    }
}
