use super::legacy_dod_options::{
    option_state_summary, option_value, props_with_option, resolved_after_value,
};
use super::legacy_dod_specs::{LegacyDodSpec, legacy_dod_specs};
use crate::catalog::{StoryExample, StoryPresetLabels};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryDetailContent {
    pub page: String,
    pub settings: String,
    pub state: String,
    pub event: String,
    pub action: String,
    pub preset: String,
    pub quality: String,
}

impl StoryDetailContent {
    #[must_use]
    pub fn from_example(example: &StoryExample) -> Self {
        let spec = spec_for(example.page);
        let marker = marker_for(spec, example.page);
        let option = spec.map_or(fallback_option(example.page), |it| it.option);
        let value_type = spec.map_or("StorybookOption", |it| it.value_type);
        let props = example.tree.root().props();
        let before = option_value(option, props);
        let configured_after = spec.map_or(fallback_after(example.page), |it| it.after);
        let resolved_after = resolved_after_value(option, value_type, configured_after, &before);
        let after_props = props_with_option(props, option, &resolved_after);
        let after = option_value(option, &after_props);
        let action = action_line(example, &marker);
        let settings = if example.page == "badge" {
            badge_settings_line(&marker)
        } else if example.page == "banner" {
            banner_settings_line(example, &marker)
        } else if example.page == "toast-stack-manager" {
            toast_stack_settings_line(example, &marker)
        } else if example.page == "notification-toast" {
            notification_toast_settings_line(&marker)
        } else if example.page == "status-bar" {
            status_bar_settings_line(example, &marker)
        } else if example.page == "shortcut-combo" {
            shortcut_combo_settings_line(example, &marker)
        } else if example.page == "shortcut-cheatsheet" {
            shortcut_cheatsheet_settings_line(example, &marker)
        } else if example.page == "settings-list" {
            settings_list_settings_line(example, &marker)
        } else if example.page == "key-cap" {
            key_cap_settings_line(&marker)
        } else if example.page == "chip" {
            chip_settings_line(&marker)
        } else if example.page == "attachment-chip" {
            attachment_chip_settings_line(&marker)
        } else if example.page == "chip-group" {
            chip_group_settings_line(&marker)
        } else if example.page == "diagnostics-list" {
            diagnostics_settings_line(example, &marker)
        } else if example.page == "empty-state" {
            empty_state_settings_line(example, &marker)
        } else if example.page == "closeable-tab-strip" {
            closeable_tab_strip_settings_line(example, &marker)
        } else if example.page == "drag-and-drop" {
            drag_and_drop_settings_line(example, &marker)
        } else if example.page == "context-menu" {
            context_menu_settings_line(example, &marker)
        } else if example.page == "popover" {
            popover_settings_line(&marker)
        } else if example.page == "hover-card" {
            hover_card_settings_line(&marker)
        } else if example.page == "toolbar" {
            toolbar_settings_line(&marker)
        } else if example.page == "text-area" {
            text_area_settings_line(&marker)
        } else {
            format!("{marker} settings: {option} ({value_type}) {before} -> {after}")
        };

        Self {
            page: example.page.to_string(),
            settings,
            state: state_line(example, &marker, option, &after_props),
            event: event_line(example, &marker),
            action,
            preset: preset_line(example.page, &marker),
            quality: quality_line(spec, &marker),
        }
    }
}

fn spec_for(page: &str) -> Option<&'static LegacyDodSpec> {
    legacy_dod_specs().find(|it| it.page == page)
}

fn fallback_option(page: &str) -> &'static str {
    if page == "context-menu" {
        return "context_menu.anchor";
    }
    "theme_id"
}

fn fallback_after(page: &str) -> &'static str {
    if page == "context-menu" {
        return "Pointer(192,128)";
    }
    "dark"
}

fn marker_for(spec: Option<&LegacyDodSpec>, page: &str) -> String {
    spec.map_or_else(
        || format!("catalog-{page}"),
        |it| format!("legacy-{}", it.marker),
    )
}

fn state_line(
    example: &StoryExample,
    marker: &str,
    option: &str,
    after_props: &katana_ui_core::render_model::UiProps,
) -> String {
    let props = example.tree.root().props();
    format!(
        "{marker} state: id={} before={} after={}",
        props.state_id.as_str(),
        option_state_summary(option, props),
        option_state_summary(option, after_props)
    )
}

fn event_line(example: &StoryExample, marker: &str) -> String {
    if let Some(log) = example.callback_logs.first() {
        return format!("{marker} event: {} -> {}", log.action, log.after);
    }
    format!("{marker} event: passive-ui")
}

fn drag_and_drop_settings_line(example: &StoryExample, marker: &str) -> String {
    let before = example.callback_logs.first().map_or(
        "accept=missing autoscroll=missing keyboard_draggable=missing",
        |it| it.before.as_str(),
    );
    let after = example.callback_logs.first().map_or(
        "accept=missing autoscroll=missing keyboard_draggable=missing",
        |it| it.after.as_str(),
    );
    format!("{marker} settings: accept/autoscroll/keyboard_draggable {before} -> {after}")
}

fn context_menu_settings_line(example: &StoryExample, marker: &str) -> String {
    let props = example.tree.root().props();
    let anchor = option_value("context_menu.anchor", props);
    let placement = option_value("context_menu.placement", props);
    let item_kind = option_value("context_menu.item_kind", props);
    let log_count = example.callback_logs.len();
    format!(
        "{marker} settings: context_menu.anchor={anchor} context_menu.placement={placement} context_menu.item_kind={item_kind} callback_log={log_count} -> context_menu.anchor=Pointer(0,0) context_menu.placement=AboveEnd context_menu.item_kind=Toggle callback_log={log_count}"
    )
}

fn popover_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: placement/arrow/focus/slot BottomStart,true,FirstInteractive,heading -> TopStart,false,None,footer"
    )
}

fn hover_card_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: delay/placement/arrow/focus/slot open100,Pointer,true,keep,heading -> open0,TopStart,false,blur,footer"
    )
}

fn toolbar_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: action/priority/overflow/display/density count4,search10,Menu,IconLeading,Default -> count5,search90,Hide,LabelOnly,Compact"
    )
}

fn text_area_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: submit/newline/tab/auto/wrap Enter,ShiftEnter,MoveFocus,true,Soft -> ModEnter,Enter,InsertTab,false,Hard"
    )
}

fn badge_settings_line(marker: &str) -> String {
    format!("{marker} settings: passive status -> use Chip for dismiss / interactive")
}

fn banner_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: severity=Warning density=Compact actions=2 details=Closed dismissible=true callback_log={} actions={actions} -> severity=Danger density=Default actions=1 details=Open dismissible=false",
        example.callback_logs.len()
    )
}

fn toast_stack_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
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
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: mode=SingleMessage segments=1 density=Default callback_log={} actions={actions} -> mode=MultiSegment segments=4 density=Compact",
        example.callback_logs.len()
    )
}

fn shortcut_combo_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
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
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
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
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: group_by=Severity sort_by=Severity severity_filter=Error+Warning bulk_action=Preview fix_preview=Expanded actions={actions} -> group_by=Source sort_by=Location severity_filter=Error bulk_action=Apply fix_preview=Collapsed"
    )
}

fn empty_state_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: tone=Accent size=Default alignment=Center actions=Primary callback_log={} actions={actions} -> tone=Danger size=Large alignment=Leading actions=Primary+Secondary",
        example.callback_logs.len()
    )
}

fn closeable_tab_strip_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: tab.count=6 pinned=false dirty=false group=docs actions={actions} callback_log={} -> tab.count=7 pinned=true dirty=true group=preview",
        example.callback_logs.len()
    )
}

fn action_line(example: &StoryExample, marker: &str) -> String {
    if let Some(log) = example.callback_logs.first() {
        return format!(
            "{marker} action: {} before={} after={}",
            log.action, log.before, log.after
        );
    }
    format!("{marker} action: none")
}

fn preset_line(page: &str, marker: &str) -> String {
    if page == "closeable-tab-strip" {
        return format!("{marker} preset: default / overflow / pinned / groups / dirty / dragging");
    }
    let presets = StoryPresetLabels::for_page(page);
    format!("{marker} preset: {}", presets.join(" / "))
}

fn quality_line(spec: Option<&LegacyDodSpec>, marker: &str) -> String {
    if marker == "catalog-closeable-tab-strip" {
        return format!(
            "{marker} quality: settings=tab_add/delete/pin/dirty/group state/event/action/preset markers fixed"
        );
    }
    let option = spec.map_or("theme_id", |it| it.option);
    format!("{marker} quality: settings={option} state/event/action/preset markers fixed")
}
