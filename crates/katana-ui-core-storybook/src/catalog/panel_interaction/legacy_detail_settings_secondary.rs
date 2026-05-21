use super::legacy_detail_core::{callback_actions, virtualization_log_after};
use crate::catalog::StoryExample;

pub(super) fn settings_line(
    example: &StoryExample,
    marker: &str,
    option: &str,
    value_type: &str,
    before: &str,
    after: &str,
) -> String {
    match example.page {
        "banner" => banner_settings_line(example, marker),
        "toast-stack-manager" => toast_stack_settings_line(example, marker),
        "notification-toast" => notification_toast_settings_line(marker),
        "status-bar" => status_bar_settings_line(example, marker),
        "shortcut-combo" => shortcut_combo_settings_line(example, marker),
        "shortcut-cheatsheet" => shortcut_cheatsheet_settings_line(example, marker),
        "settings-list" => settings_list_settings_line(example, marker),
        "collapsible-panel" => collapsible_panel_settings_line(example, marker),
        "key-cap" => key_cap_settings_line(marker),
        "chip" => chip_settings_line(marker),
        "attachment-chip" => attachment_chip_settings_line(marker),
        "chip-group" => chip_group_settings_line(marker),
        "diagnostics-list" => diagnostics_settings_line(example, marker),
        "empty-state" => empty_state_settings_line(example, marker),
        "closeable-tab-strip" => closeable_tab_strip_settings_line(example, marker),
        "window-control-button-group" => window_control_button_group_settings_line(example, marker),
        "startup-state-panel" => startup_state_panel_settings_line(example, marker),
        _ => format!("{marker} settings: {option} ({value_type}) {before} -> {after}"),
    }
}

fn banner_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: severity=Warning density=Compact actions=2 details=Closed dismissible=true callback_log={} actions={actions} -> severity=Danger density=Default actions=1 details=Open dismissible=false",
        example.callback_logs.len()
    )
}

fn toast_stack_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: position=BottomEnd max_visible=2 dedup=ById duration=8000 pause_on_hover=true stack_gap=10 callback_log={} actions={actions} -> position=TopCenter max_visible=4 dedup=ByIdAndSeverity duration=3000 pause_on_hover=false stack_gap=16",
        example.callback_logs.len()
    )
}

fn notification_toast_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: single transient toast -> use ToastStackManager for queue/dedup/position"
    )
}

fn status_bar_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: mode=SingleMessage segments=1 density=Default callback_log={} actions={actions} -> mode=MultiSegment segments=4 density=Compact",
        example.callback_logs.len()
    )
}

fn shortcut_combo_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: platform_display=Auto separator=Plus size=Medium tone=Neutral callback_log={} actions={actions} -> platform_display=MacOS separator=None size=Large tone=Accent",
        example.callback_logs.len()
    )
}

fn shortcut_cheatsheet_settings_line(example: &StoryExample, marker: &str) -> String {
    format!(
        "{marker} settings: group_layout=TwoColumn query=format callback_log={} -> group_layout=OneColumn query=カテゴリ",
        example.callback_logs.len()
    )
}

fn settings_list_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: density=Default dirty_visualization=Marker query=None sections=3 control_kind=Toggle+Select+Combo+Input+TextArea+Number+Chips+Radio+ColorPicker+Custom callback_log={} actions={actions} -> density=Compact dirty_visualization=Highlight query=format sections=app+chat+lint control_kind=Number reset=true",
        example.callback_logs.len()
    )
}

fn key_cap_settings_line(marker: &str) -> String {
    format!("{marker} settings: single key only -> use ShortcutCombo for multi-key combinations")
}

fn chip_settings_line(marker: &str) -> String {
    format!("{marker} settings: variant/tone/size Outline,Accent,Medium -> Filled,Danger,Large")
}

fn attachment_chip_settings_line(marker: &str) -> String {
    format!("{marker} settings: status/progress Uploading,42 -> Error,100")
}

fn chip_group_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: overflow/wrap/reorder Menu,false,true -> ScrollHorizontal,true,false"
    )
}

fn diagnostics_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: group_by=Severity sort_by=Severity severity_filter=Error+Warning bulk_action=Preview fix_preview=Expanded actions={actions} virtualization enabled=true->false overscan=2->4 row_height_provider=Fixed->Variable visible_range={} -> group_by=Source sort_by=Location severity_filter=Error bulk_action=Apply fix_preview=Collapsed",
        virtualization_log_after(example)
    )
}

fn empty_state_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: tone=Accent size=Default alignment=Center actions=Primary callback_log={} actions={actions} -> tone=Danger size=Large alignment=Leading actions=Primary+Secondary",
        example.callback_logs.len()
    )
}

fn closeable_tab_strip_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: tab.count=6 pinned=false dirty=false group=docs actions={actions} callback_log={} -> tab.count=7 pinned=true dirty=true group=preview",
        example.callback_logs.len()
    )
}

fn collapsible_panel_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: mode=Expanded width=240 pinned=true expand_on_hover=true resize_handle=true callback_log={} actions={actions} -> mode=IconOnly width=320 pinned=false expand_on_hover=true resize_handle=true",
        example.callback_logs.len()
    )
}

fn window_control_button_group_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: position=Leading/Trailing/Auto size=Compact/Default/Tall controls=Close+Minimize+Maximize+Restore visibility=Always/Hover/FullscreenHover callback_log={} actions={actions} state=visible event=ControlPressed+VisibilityChanged+FullscreenChanged action=window_control_press -> position=Trailing size=Tall controls=Close visibility=Hover",
        example.callback_logs.len()
    )
}

fn startup_state_panel_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: state=Idle/Loading/Error progress=None/64/100 label=Loading workspace retry=true/false cancel=true/false version_label=none/v0.1.0 callback_log={} actions={actions} event=StartupStateChanged+StartupRetried+StartupCanceled -> state=Error progress=100 label=Workspace failed retry=true cancel=true",
        example.callback_logs.len()
    )
}
