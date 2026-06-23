pub(in crate::visual) fn command_palette_state(setting: &'static str) -> &'static str {
    match setting {
        "command_palette.query" => "command_palette.query=theme",
        "command_palette.highlight" => "command_palette.highlight=2",
        "command_palette.row_count" => "command_palette.row_count=50",
        "command_palette.provider_group" => "command_palette.provider_group=workspace/editor/app",
        "command_palette.shortcut_display" => "command_palette.shortcut_display=false",
        _ => setting,
    }
}

pub(in crate::visual) fn diagnostics_list_state(setting: &'static str) -> &'static str {
    match setting {
        "diagnostics.group_by" => "diagnostics.group_by=Source",
        "diagnostics.sort_by" => "diagnostics.sort_by=Location",
        "diagnostics.severity_filter" => "diagnostics.severity_filter=Error",
        "diagnostics.wrap_error_navigation" => "diagnostics.wrap_error_navigation=false",
        "diagnostics.virtualization" => "diagnostics.virtualization=Windowed",
        "diagnostics.bulk_action" => "diagnostics.bulk_action=Apply",
        "diagnostics.fix_preview" => "diagnostics.fix_preview=Collapsed",
        _ => setting,
    }
}

pub(in crate::visual) fn shortcut_cheatsheet_state(setting: &'static str) -> &'static str {
    match setting {
        "shortcut_cheatsheet.label" => "shortcut_cheatsheet.label=Editor keys",
        "shortcut_cheatsheet.groups" => "shortcut_cheatsheet.groups=3",
        "shortcut_cheatsheet.group_title" => "shortcut_cheatsheet.group_title=Navigation",
        "shortcut_cheatsheet.items" => "shortcut_cheatsheet.items=4",
        "shortcut_cheatsheet.item_combo" => "shortcut_cheatsheet.item_combo=Cmd+Shift+P",
        "shortcut_cheatsheet.group_layout" => "shortcut_cheatsheet.group_layout=OneColumn",
        "shortcut_cheatsheet.query" => "shortcut_cheatsheet.query=カテゴリ",
        "shortcut_cheatsheet.selected" => "shortcut_cheatsheet.selected=format",
        "shortcut_cheatsheet.result_count" => "shortcut_cheatsheet.result_count=1",
        _ => setting,
    }
}

pub(in crate::visual) fn shortcut_combo_state(setting: &'static str) -> &'static str {
    match setting {
        "shortcut_combo.platform_display" => "shortcut_combo.platform_display=MacOS",
        "shortcut_combo.separator" => "shortcut_combo.separator=None",
        "shortcut_combo.size" => "shortcut_combo.size=Large",
        "shortcut_combo.tone" => "shortcut_combo.tone=Accent",
        "shortcut_combo.a11y_label" => "shortcut_combo.a11y_label=custom",
        _ => setting,
    }
}

pub(in crate::visual) fn skeleton_cluster_state(setting: &'static str) -> &'static str {
    match setting {
        "skeleton_cluster.preset" => "skeleton_cluster.preset=Card",
        "skeleton_cluster.children" => "skeleton_cluster.children=3",
        "skeleton_cluster.live_region" => "skeleton_cluster.live_region=card",
        "skeleton_cluster.reduced_motion" => "skeleton_cluster.reduced_motion=true",
        _ => setting,
    }
}

pub(in crate::visual) fn window_control_state(setting: &'static str) -> &'static str {
    match setting {
        "window_control.position" => "window_control.position=Trailing",
        "window_control.size" => "window_control.size=Tall",
        "window_control.controls" => "window_control.controls=Close",
        "window_control.visibility" => "window_control.visibility=Hover",
        _ => setting,
    }
}

pub(in crate::visual) fn accordion_state(setting: &'static str) -> &'static str {
    match setting {
        "accordion.expanded" => "accordion.expanded=true",
        "accordion.disabled" => "accordion.disabled=true",
        "accordion.controlled" => "accordion.controlled=true",
        "accordion.trigger_area" => "accordion.trigger_area=full-row",
        "accordion.reduced_motion" => "accordion.reduced_motion=true",
        _ => setting,
    }
}

pub(in crate::visual) fn context_menu_state(setting: &'static str) -> &'static str {
    match setting {
        "context_menu.anchor" => "context_menu.anchor=Pointer(0,0)",
        "context_menu.placement_priority" => "context_menu.placement_priority=AboveEnd>BelowStart",
        "context_menu.placement_used" => "context_menu.placement_used=AboveEnd",
        "context_menu.min_width" => "context_menu.min_width=280",
        "context_menu.max_height" => "context_menu.max_height=320",
        _ => setting,
    }
}

pub(in crate::visual) fn startup_state(setting: &'static str) -> &'static str {
    match setting {
        "startup_state.state" => "startup_state.state=Error",
        "startup_state.progress" => "startup_state.progress=64",
        "startup_state.label" => "startup_state.label=Loading workspace",
        "startup_state.retry" => "startup_state.retry=true",
        "startup_state.cancel" => "startup_state.cancel=true",
        _ => setting,
    }
}

pub(in crate::visual) fn code_diff_state(setting: &'static str) -> &'static str {
    match setting {
        "code_diff.mode" => "code_diff.mode=Split",
        "code_diff.whitespace" => "code_diff.whitespace=Visible",
        "code_diff.direction" => "code_diff.direction=Vertical",
        "code_diff.context_lines" => "code_diff.context_lines=0",
        "code_diff.item_count" => "code_diff.item_count=3",
        "code_diff.scroll_sync" => "code_diff.scroll_sync=false",
        "code_diff.language" => "code_diff.language=markdown",
        _ => setting,
    }
}
