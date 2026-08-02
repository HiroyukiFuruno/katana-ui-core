pub(in crate::visual) fn hover_card_state(setting: &'static str) -> &'static str {
    match setting {
        "hover_card.open_delay_ms" => "hover_card.open_delay_ms=180",
        "hover_card.close_delay_ms" => "hover_card.close_delay_ms=220",
        "hover_card.pointer_follow" => "hover_card.pointer_follow=true",
        "hover_card.slot_action" => "hover_card.slot_action=visible",
        _ => setting,
    }
}

pub(in crate::visual) fn menu_state(setting: &'static str) -> &'static str {
    match setting {
        "menu.common_props" => "menu.common_props=dense",
        "children" => "menu.children=changed",
        "interaction.selected_index" => "menu.selected_index=1",
        "menu.panel_placement" => "menu.panel_placement=resolved",
        _ => setting,
    }
}

pub(in crate::visual) fn form_field_state(setting: &'static str) -> &'static str {
    match setting {
        "form_field.common_props" => "form_field.common_props=dense",
        "children" => "form_field.children=changed",
        "form_field.invalid" => "form_field.invalid=true",
        "form_field.helper_text" => "form_field.helper_text=long",
        "form_field.required" => "form_field.required=true",
        _ => setting,
    }
}

pub(in crate::visual) fn breadcrumb_state(setting: &'static str) -> &'static str {
    match setting {
        "breadcrumb.items" => "breadcrumb.items=4",
        "children" => "breadcrumb.children=changed",
        "breadcrumb.crumb_action" => "breadcrumb.crumb_action=callback",
        _ => setting,
    }
}

pub(in crate::visual) fn side_menu_state(setting: &'static str) -> &'static str {
    match setting {
        "side_menu.items" => "side_menu.items=5",
        "children" => "side_menu.children=changed",
        "interaction.selected_index" => "side_menu.selected_index=1",
        "side_menu.hover_expansion" => "side_menu.hover_expansion=true",
        _ => setting,
    }
}

pub(in crate::visual) fn list_state(setting: &'static str) -> &'static str {
    match setting {
        "list.rows" => "list.rows=200",
        "list.selection" => "list.selection=row-2",
        "list.empty_state" => "list.empty_state=true",
        "list.virtualization" => "list.virtualization=visible_range",
        "list.theme_row" => "list.theme_row=accent",
        _ => setting,
    }
}

pub(in crate::visual) fn collapsible_panel_state(setting: &'static str) -> &'static str {
    match setting {
        "collapsible_panel.mode" => "collapsible_panel.mode=floating_overlay",
        "collapsible_panel.width" => "collapsible_panel.width=320",
        "collapsible_panel.pinned" => "collapsible_panel.pinned=false",
        "collapsible_panel.expand_on_hover" => "collapsible_panel.expand_on_hover=true",
        "collapsible_panel.resize_handle" => "collapsible_panel.resize_handle=true",
        _ => setting,
    }
}

pub(in crate::visual) fn tree_state(setting: &'static str) -> &'static str {
    match setting {
        "line" => "tree.line=hidden",
        "node_marker" => "tree.node_marker=leaf",
        "trigger" => "tree.trigger=text",
        "context_menu" => "tree.context_menu=enabled",
        _ => setting,
    }
}

pub(in crate::visual) fn panel_state(setting: &'static str) -> &'static str {
    match setting {
        "vertical_scroll" => "panel.vertical_scroll=changed",
        "horizontal_scroll" => "panel.horizontal_scroll=changed",
        "nested_state" => "panel.nested_state=independent",
        _ => setting,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collection_setting_mappers_preserve_unknown_keys_and_form_field_semantics() {
        for actual in [
            hover_card_state("unknown"),
            menu_state("unknown"),
            form_field_state("unknown"),
            breadcrumb_state("unknown"),
            side_menu_state("unknown"),
            list_state("unknown"),
            collapsible_panel_state("unknown"),
            tree_state("unknown"),
            panel_state("unknown"),
        ] {
            assert_eq!("unknown", actual);
        }

        for (setting, expected) in [
            ("form_field.invalid", "form_field.invalid=true"),
            ("form_field.helper_text", "form_field.helper_text=long"),
            ("form_field.required", "form_field.required=true"),
        ] {
            assert_eq!(expected, form_field_state(setting));
        }
    }
}
