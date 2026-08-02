pub(in crate::visual) fn search_box_state(setting: &'static str) -> &'static str {
    match setting {
        "text_entry.value" => "search_box.value=typed query",
        "text_entry.submit_on_enter" => "search_box.submit_on_enter=true",
        "text_entry.clear_button" => "search_box.clear_button=cleared",
        "text_entry.regex_case" => "search_box.regex_case=true/true",
        _ => setting,
    }
}

pub(in crate::visual) fn combo_box_state(setting: &'static str) -> &'static str {
    match setting {
        "combo.items" => "combo.items=6",
        "interaction.open" => "combo.open=true",
        "interaction.selected_index" => "combo.selected_index=1",
        "interaction.value" => "combo.value=two",
        "placeholder" => "combo.placeholder=visible",
        "disabled" => "combo.disabled=true",
        "readonly" => "combo.readonly=true",
        "combo.input_value" => "combo.input_value=tw",
        "combo.filter_result" => "combo.filter_result=filtered",
        "combo.free_input" => "combo.free_input=true",
        "combo.keyboard_navigation" => "combo.keyboard_navigation=active",
        "combo.placement" => "combo.placement=above",
        "combo.highlighted_index" => "combo.highlighted_index=1",
        "combo.long_list" => "combo.long_list=true",
        "combo.outside_click_dismiss" => "combo.outside_click_dismiss=true",
        "combo.framed" => "combo.framed=true",
        "combo.trigger_summary" => "combo.trigger_summary=selected",
        "combo.select_action" => "combo.select_action=callback",
        "validation" => "combo.validation=invalid",
        _ => setting,
    }
}

pub(in crate::visual) fn select_box_state(setting: &'static str) -> &'static str {
    match setting {
        "select.items" => "select.items=6",
        "interaction.open" => "select.open=true",
        "interaction.selected_index" => "select.selected_index=1",
        "placeholder" => "select.placeholder=visible",
        "disabled" => "select.disabled=true",
        _ => setting,
    }
}

pub(in crate::visual) fn selection_list_state(setting: &'static str) -> &'static str {
    match setting {
        "selection_list.items" => "selection_list.items=1000",
        "interaction.selected_index" => "selection_list.selected_index=2",
        "selection_list.section" => "selection_list.section=Recent",
        "selection_list.marker" => "selection_list.marker=check",
        "selection_list.more_row" => "selection_list.more_row=true",
        _ => setting,
    }
}

pub(in crate::visual) fn menu_button_state(setting: &'static str) -> &'static str {
    match setting {
        "menu.items" => "menu_button.items=4",
        "interaction.open" => "menu_button.open=true",
        "disabled" => "menu_button.disabled=true",
        "menu.select_action" => "menu_button.select_action=callback",
        _ => setting,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        combo_box_state, menu_button_state, search_box_state, select_box_state,
        selection_list_state,
    };

    #[test]
    fn selection_setting_semantics_preserve_unknown_setting_keys() {
        const UNKNOWN: &str = "unknown.setting";
        for mapper in [
            search_box_state,
            combo_box_state,
            select_box_state,
            selection_list_state,
            menu_button_state,
        ] {
            assert_eq!(UNKNOWN, mapper(UNKNOWN));
        }
    }
}
