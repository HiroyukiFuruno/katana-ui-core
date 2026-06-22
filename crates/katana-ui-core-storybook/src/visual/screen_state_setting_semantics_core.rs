pub(in crate::visual) fn toolbar_state(setting: &'static str) -> &'static str {
    match setting {
        "toolbar.display_mode" => "toolbar.display=icon_text",
        "toolbar.density" => "toolbar.density=compact",
        "toolbar.overflow_strategy" => "toolbar.overflow=menu",
        "toolbar.actions" => "toolbar.actions=changed",
        "toolbar.groups" => "toolbar.groups=changed",
        "toolbar.context_menu_anchor" => "toolbar.anchor=pointer",
        "toolbar.action_priority" => "toolbar.action.priority=90",
        "toolbar.action_accelerator" => "toolbar.action.accelerator=Alt+P",
        "toolbar.action_split" => "toolbar.action.split=menu",
        "toolbar.action_group" => "toolbar.action.group=edit",
        "toolbar.action_tooltip" => "toolbar.action.tooltip=Save file",
        "toolbar.action_a11y" => "toolbar.action.a11y=Save file",
        "toolbar.action_disabled" => "toolbar.action.disabled=true",
        "toolbar.group_label" => "toolbar.group.label=File actions",
        "toolbar.group_divider" => "toolbar.group.divider=false",
        "toolbar.split_disabled" => "toolbar.split.disabled=true",
        "toolbar.split_tooltip" => "toolbar.split.tooltip=visible",
        "toolbar.split_a11y" => "toolbar.split.a11y=Open menu",
        _ => setting,
    }
}

pub(in crate::visual) fn settings_list_state(setting: &'static str) -> &'static str {
    match setting {
        "settings_list.label" => "settings_list.label=Workspace settings",
        "settings_list.density" => "settings_list.density=Compact",
        "settings_list.dirty_visualization" => "settings_list.dirty=Highlight",
        "settings_list.query" => "settings_list.query=format",
        "settings_list.sections" => "settings_list.sections=app+lint",
        "settings_list.section_label" => "settings_list.section.label=Editor",
        "settings_list.section_description" => "settings_list.section.description=visible",
        "settings_list.section_icon" => "settings_list.section.icon=gear",
        "settings_list.field_count" => "settings_list.field.count=5",
        "settings_list.section_footer" => "settings_list.section.footer=policy",
        "settings_list.control_options" => "settings_list.control.options=4",
        "settings_list.custom_control" => "settings_list.control.custom=button",
        "settings_list.set_value" => "settings_list.value=changed",
        "settings_list.reset" => "settings_list.reset=default",
        "settings_list.section_collapsible" => "settings_list.section.collapsible=true",
        "settings_list.default_collapsed" => "settings_list.section.collapsed=true",
        "settings_list.field_label" => "settings_list.field.label=Font size",
        "settings_list.field_description" => "settings_list.field.description=visible",
        "settings_list.control_kind" => "settings_list.control.kind=Number",
        _ => setting,
    }
}

pub(in crate::visual) fn color_picker_state(setting: &'static str) -> &'static str {
    match setting {
        "color_picker.rgba" => "color_picker.rgba=rgba(64,128,255,.8)",
        "color_picker.value" => "color_picker.value=rgba(72,136,240,.74)",
        "color_picker.open" => "color_picker.open=true",
        "color_picker.hue" => "color_picker.hue=214",
        "color_picker.alpha" => "color_picker.alpha=204",
        "color_picker.blending" => "color_picker.blending=Multiply",
        "color_picker.color_area" => "color_picker.color_area=saturation/value",
        "color_picker.trigger_size" => "color_picker.trigger.size=Large",
        "color_picker.title" => "color_picker.title=Brand accent",
        "color_picker.rgba_mode" => "color_picker.rgba_mode=false",
        "color_picker.panel_scale_percent" => "color_picker.panel.scale=100",
        "color_picker.trigger_border" => "color_picker.trigger.border=false",
        "color_picker.eyedropper_callback" => "color_picker.eyedropper=storybook-eyedropper",
        "color_picker.readonly" => "color_picker.readonly.blocks_writes",
        "color_picker.disabled" => "color_picker.disabled.blocks_focus",
        _ => setting,
    }
}

pub(in crate::visual) fn virtualization_state(setting: &'static str) -> &'static str {
    match setting {
        "viewport.offset" => "virtualization.viewport.offset=1260",
        "virtualization.overscan" => "virtualization.overscan=4",
        "virtualization.focused_index" => "virtualization.focused_index=42",
        "virtualization.measured_correction" => "virtualization.measured_correction=+8",
        "virtualization.row_height_provider" => "virtualization.row_height=variable",
        _ => setting,
    }
}

pub(in crate::visual) fn search_control_state(setting: &'static str) -> &'static str {
    match setting {
        "search_control.query" => "search_control.query=heading",
        "search_control.match_case" => "search_control.match_case=true",
        "search_control.whole_word" => "search_control.whole_word=true",
        "search_control.use_regex" => "search_control.regex=true",
        "search_control.replace_mode" => "search_control.replace=disabled",
        "search_control.result_count" => "search_control.result_count=0",
        "search_control.active_index" => "search_control.active_index=none",
        _ => setting,
    }
}

pub(in crate::visual) fn status_bar_state(setting: &'static str) -> &'static str {
    match setting {
        "status_bar.mode" => "status_bar.mode=MultiSegment",
        "status_bar.segments" => "status_bar.segments=4",
        "status_bar.density" => "status_bar.density=Compact",
        "status_bar.progress_popover" => "status_bar.progress_popover=true",
        "status_bar.message" => "status_bar.message=Ready",
        "status_bar.severity" => "status_bar.severity=Warning",
        "status_bar.dismiss" => "status_bar.dismiss=available",
        "status_bar.segment_a11y" => "status_bar.segment_a11y=custom",
        _ => setting,
    }
}
