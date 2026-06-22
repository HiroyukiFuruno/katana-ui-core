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
        "{marker} settings: severity=Warning density=Compact actions=2 details=Closed dismissible=true title=Format result leading_icon=alert-triangle placement=Sticky callback_log={} actions={actions} -> severity=Danger density=Default actions=1 details=Open dismissible=false title=none leading_icon=info placement=Inline",
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
        "{marker} settings: mode=SingleMessage segments=1 density=Default message=None severity=Neutral dismiss=None segment_a11y=default callback_log={} actions={actions} -> mode=MultiSegment segments=4 density=Compact message=Ready severity=Warning dismiss=Available segment_a11y=custom",
        example.callback_logs.len()
    )
}

fn shortcut_combo_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: platform_display=Auto separator=Plus size=Medium tone=Neutral a11y_label=generated callback_log={} actions={actions} -> platform_display=MacOS separator=None size=Large tone=Accent a11y_label=custom",
        example.callback_logs.len()
    )
}

fn shortcut_cheatsheet_settings_line(example: &StoryExample, marker: &str) -> String {
    format!(
        "{marker} settings: label=Shortcuts groups=2 group_title=Editing items=2 item_combo=Cmd+F group_layout=TwoColumn query=format selected=None result_count=2 callback_log={} -> label=Editor keys groups=3 group_title=Navigation items=4 item_combo=Cmd+Shift+P group_layout=OneColumn query=カテゴリ selected=format result_count=1",
        example.callback_logs.len()
    )
}

fn settings_list_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: settings_list.label=Settings settings_list.density=Default settings_list.dirty_visualization=Marker settings_list.query=None settings_list.sections=app+chat+lint settings_list.section_label=App settings settings_list.section_description=none settings_list.section_icon=none settings_list.field_count=3 settings_list.section_footer=none settings_list.section_collapsible=false settings_list.default_collapsed=false settings_list.field_label=Format on save settings_list.field_description=none settings_list.control_kind=Toggle settings_list.control_options=2 settings_list.custom_control=none settings_list.set_value=idle settings_list.reset=dirty callback_log={} actions={actions} -> settings_list.label=Workspace settings settings_list.density=Compact settings_list.dirty_visualization=Highlight settings_list.query=format settings_list.sections=app+lint settings_list.section_label=Editor settings_list.section_description=visible settings_list.section_icon=gear settings_list.field_count=5 settings_list.section_footer=policy settings_list.section_collapsible=true settings_list.default_collapsed=true settings_list.field_label=Font size settings_list.field_description=visible settings_list.control_kind=Number settings_list.control_options=4 settings_list.custom_control=button settings_list.set_value=changed settings_list.reset=default",
        example.callback_logs.len()
    )
}

fn key_cap_settings_line(marker: &str) -> String {
    format!("{marker} settings: single key only -> use ShortcutCombo for multi-key combinations")
}

fn chip_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: chip.label=filter:docs chip.leading_icon=filter chip.trailing_icon=none chip.variant=Outline chip.tone=Accent chip.size=Medium chip.interactive=false chip.selected=false chip.disabled=false chip.dismissible=false chip.a11y_label=none chip.focused=false -> chip.label=filter:rust chip.leading_icon=tag chip.trailing_icon=close chip.variant=Filled chip.tone=Danger chip.size=Large chip.interactive=true chip.selected=true chip.disabled=true chip.dismissible=true chip.a11y_label=Filter chip.focused=true"
    )
}

fn attachment_chip_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: attachment.kind=File attachment.name=design.md attachment.meta=none attachment.thumbnail=none attachment.status=Uploading attachment.progress=42 attachment.retry=hidden -> attachment.kind=Image attachment.name=proposal.pdf attachment.meta=size+mime attachment.thumbnail=preview attachment.status=Error attachment.progress=100 attachment.retry=visible"
    )
}

fn chip_group_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: chip_group.label=Filters chip_group.chip_count=3 chip_group.wrap=false chip_group.overflow=Menu chip_group.reorder=false chip_group.gap=0 chip_group.available_width=88 chip_group.overflow_trigger_width=24 chip_group.hidden_count=0 -> chip_group.label=Active filters chip_group.chip_count=5 chip_group.wrap=true chip_group.overflow=ScrollHorizontal chip_group.reorder=true chip_group.gap=8 chip_group.available_width=132 chip_group.overflow_trigger_width=32 chip_group.hidden_count=2"
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
        "{marker} settings: heading=No diagnostics body=mixed text icon=none illustration=none tone=Accent size=Default alignment=Center actions=Primary callback_log={} actions={actions} -> heading=Empty project body=create a file icon=search illustration=folder tone=Danger size=Large alignment=Leading actions=Primary+Secondary",
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
