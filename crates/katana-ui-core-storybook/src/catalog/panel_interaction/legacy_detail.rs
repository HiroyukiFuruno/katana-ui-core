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
        let settings = if example.page == "badge" {
            badge_settings_line(&marker)
        } else if example.page == "scroll-area" {
            scroll_area_settings_line(example, &marker)
        } else if example.page == "search-control-strip" {
            search_control_settings_line(example, &marker)
        } else if example.page == "modal" {
            modal_settings_line(example, &marker)
        } else if example.page == "modal-overlay" {
            modal_overlay_settings_line(example, &marker)
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
        } else if example.page == "collapsible-panel" {
            collapsible_panel_settings_line(example, &marker)
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
        } else if example.page == "color-picker-rgba" {
            color_picker_settings_line(example, &marker)
        } else if example.page == "popover" {
            popover_settings_line(example, &marker)
        } else if example.page == "hover-card" {
            hover_card_settings_line(&marker)
        } else if example.page == "accordion" {
            accordion_settings_line(example, &marker)
        } else if example.page == "toolbar" {
            toolbar_settings_line(&marker)
        } else if example.page == "split-pane" {
            split_pane_settings_line(example, &marker)
        } else if example.page == "text-area" {
            text_area_settings_line(&marker)
        } else if example.page == "skeleton" {
            skeleton_settings_line(example, &marker)
        } else if example.page == "skeleton-cluster" {
            skeleton_cluster_settings_line(example, &marker)
        } else if example.page == "motion" {
            motion_settings_line(example, &marker)
        } else if example.page == "window-control-button-group" {
            window_control_button_group_settings_line(example, &marker)
        } else if is_virtualized_page(example.page) {
            virtualization_settings_line(example, &marker)
        } else {
            format!("{marker} settings: {option} ({value_type}) {before} -> {after}")
        };
        let state = if example.page == "search-control-strip" {
            search_control_state_line(example, &marker)
        } else if example.page == "scroll-area" {
            scroll_area_state_line(example, &marker)
        } else {
            state_line(example, &marker, option, &after_props)
        };
        let event = if example.page == "search-control-strip" {
            search_control_event_line(example, &marker)
        } else if example.page == "scroll-area" {
            scroll_area_event_line(example, &marker)
        } else {
            event_line(example, &marker)
        };
        let action = if example.page == "search-control-strip" {
            search_control_action_line(example, &marker)
        } else if example.page == "scroll-area" {
            scroll_area_action_line(example, &marker)
        } else {
            action_line(example, &marker)
        };
        let quality = if example.page == "search-control-strip" {
            search_control_quality_line(&marker)
        } else if example.page == "scroll-area" {
            scroll_area_quality_line(&marker)
        } else {
            quality_line(spec, &marker)
        };

        Self {
            page: example.page.to_string(),
            settings,
            state,
            event,
            action,
            preset: preset_line(example.page, &marker),
            quality,
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

fn color_picker_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: mode=RGBA/RGB channels=R64,G128,B255,A204 blending=Normal/Additive eyedropper=storybook-eyedropper readonly=false disabled=false plane=saturation/value hue=214 alpha=204 preview=transparent-checker actions={actions} -> mode=RGB channels=R72,G136,B240,A188 blending=Additive readonly=true disabled=true"
    )
}

fn scroll_area_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: axis offset viewport content scrollbar visibility placement edge_threshold actions={actions} -> axis=Both offset=40,180 viewport=320x220 content=860x1400 scrollbar=Always placement=Reserved"
    )
}

fn scroll_area_state_line(example: &StoryExample, marker: &str) -> String {
    let props = example.tree.root().props();
    format!(
        "{marker} state: id={} state: offset={},{} viewport={}x{} content={}x{} edge=none",
        props.state_id.as_str(),
        props.scroll_area.offset_x,
        props.scroll_area.offset_y,
        props.scroll_area.viewport_width,
        props.scroll_area.viewport_height,
        props.scroll_area.content_width,
        props.scroll_area.content_height
    )
}

fn scroll_area_event_line(example: &StoryExample, marker: &str) -> String {
    format!(
        "{marker} event: Scrolled ScrollEdgeReached ScrollCommandRejected callback_log={}",
        example.callback_logs.len()
    )
}

fn scroll_area_action_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} action: scroll_to scroll_by scroll_into_view scrollbar_visibility actions={actions}"
    )
}

fn scroll_area_quality_line(marker: &str) -> String {
    format!("{marker} quality: nested_state_identity clamp edge_event axis_rejection")
}

fn search_control_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: query match_case whole_word regex replace_mode result_count active_index actions={actions} -> query=heading match_case=true whole_word=true regex=true replace_mode=Visible result_count=12 active_index=2"
    )
}

fn search_control_state_line(example: &StoryExample, marker: &str) -> String {
    let props = example.tree.root().props();
    format!(
        "{marker} state: id={} state: query={} match_case={} whole_word={} regex={} replace={} result={}",
        props.state_id.as_str(),
        props.search_control.query,
        props.search_control.match_case,
        props.search_control.whole_word,
        props.search_control.use_regex,
        props.search_control.replace_value,
        props.search_control.result_summary
    )
}

fn search_control_event_line(example: &StoryExample, marker: &str) -> String {
    format!(
        "{marker} event: SearchQueryChanged SearchOptionChanged SearchNavigationRequested ReplaceRequested callback_log={}",
        example.callback_logs.len()
    )
}

fn search_control_action_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!("{marker} action: query option navigate replace result-position actions={actions}")
}

fn search_control_quality_line(marker: &str) -> String {
    format!("{marker} quality: typed options state_id result_count event_contract")
}

fn popover_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: option=anchor=node:toolbar.more-actions placement=bottom-start auto_flip=BottomStart>TopStart offset=12,8 width=320px outside_close=true escape_close=true focus_handling=FirstInteractive focus_return=trigger:popover-anchor slot=heading/body/footer/action action={actions} event=PopoverOpened+PopoverOutsideClosed+PopoverEscapeClosed+PopoverAutoFlipped+PopoverFocusReturned+PopoverSlotActionInvoked state=open=false->true->closed focus=copy-action->trigger:popover-anchor preset=anchor/placement/auto flip/offset width/outside+escape close/focus handling/slot content -> placement=top-start auto_flip=TopStart offset=0,0 width=240px focus_handling=None"
    )
}

fn hover_card_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: delay/placement/arrow/focus/slot open100,Pointer,true,keep,heading -> open0,TopStart,false,blur,footer"
    )
}

fn accordion_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: expanded=false disabled=false controlled=true multiple=true indicator=leading trigger_area=IconAndText toggle_icon=chevron tree_mode=true depth=2 selected=true show_lines=true reduced_motion=true body_border=true callback_log={} actions={actions} -> expanded=true trigger_area=WholeElement selected=false",
        example.callback_logs.len()
    )
}

fn modal_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: option=native_window_mode=true escape_close=true focus_return=trigger:open-modal parent_interaction=Block title=Preferences footer=Cancel / Save size=medium action={actions} event=NativeWindowOpened+ModalEscaped+FocusReturned+ParentInteractionBlocked state=open->closed preset=native window/focus return/parent block -> native_window_mode=true"
    )
}

fn modal_overlay_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: option=same_window_overlay=true backdrop_close=true escape_close=true focus_trap=true focus_return=trigger:open-overlay dismiss_disabled=true action={actions} event=OverlayBackdropClosed+OverlayEscaped+FocusTrapCycled+FocusReturned+DismissBlocked state=open->closed/open preset=overlay dialog/backdrop close/focus trap/dismiss disabled -> same_window_overlay=true"
    )
}

fn toolbar_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: action/priority/overflow/display/density count4,search10,Menu,IconLeading,Default -> count5,search90,Hide,LabelOnly,Compact"
    )
}

fn split_pane_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: axis ratio min max reset handle resize_mode; axis=Horizontal/Vertical ratio=50 min=20 max=80 reset=50 handle=8 resize_mode=Drag+Keyboard children=2 nested=true callback_log={} actions={actions}; state: ratio=50 dragging=false focused_handle=false last_event=RatioChanged; event: ResizeStarted RatioChanged ResizeEnded ResizeRejected; action: split_pane_set_ratio split_pane_resize_by split_pane_reset_ratio; quality: clamp event_order public_api_guard -> axis=Vertical ratio=56 min=20 max=80 reset=50 handle=10 resize_mode=Keyboard children=2 nested=true",
        example.callback_logs.len()
    )
}

fn text_area_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: submit/newline/tab/auto/wrap Enter,ShiftEnter,MoveFocus,true,Soft -> ModEnter,Enter,InsertTab,false,Hard"
    )
}

fn skeleton_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: shape=Text size=220x44 animation=Shimmer tone=Neutral radius=4 reduced_motion=false accessibility_label=Loading text lines callback_log={} actions={actions} -> shape=Line size=220x44 animation=Wave tone=Success radius=4 reduced_motion=true accessibility_label=Reduced loading text",
        example.callback_logs.len()
    )
}

fn skeleton_cluster_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: preset=ListRow children=2 live_region=Loading list loading reduced_motion=false callback_log={} actions={actions} -> preset=ImageCard children=3 live_region=Loading image card loading reduced_motion=false",
        example.callback_logs.len()
    )
}

fn motion_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: primitive: Fade Slide Scale Shimmer; tokens: duration=Default easing=Emphasized distance=Default; state: instant=false duration=200 distance=8; event: reduced_motion_query override=Ignore context=Storybook; action: motion_reduce motion_tick motion_force motion_ignore motion_override; quality: token_resolution reduced_static override_isolated; callback_log={} actions={actions} -> primitive=Scale duration=Slow easing=Decelerate distance=Spacious reduced_policy=Ignore",
        example.callback_logs.len()
    )
}

fn window_control_button_group_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: position=Leading/Trailing/Auto size=Compact/Default/Tall controls=Close+Minimize+Maximize+Restore visibility=Always/Hover/FullscreenHover callback_log={} actions={actions} state=visible event=ControlPressed+VisibilityChanged+FullscreenChanged action=window_control_press -> position=Trailing size=Tall controls=Close visibility=Hover",
        example.callback_logs.len()
    )
}

fn virtualization_settings_line(example: &StoryExample, marker: &str) -> String {
    let range = virtualization_log_after(example);
    format!(
        "{marker} settings: virtualization enabled=true->false overscan=2->4 row_height_provider=Fixed->Variable visible_range={range} -> virtualization disabled"
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
        "{marker} settings: group_by=Severity sort_by=Severity severity_filter=Error+Warning bulk_action=Preview fix_preview=Expanded actions={actions} virtualization enabled=true->false overscan=2->4 row_height_provider=Fixed->Variable visible_range={} -> group_by=Source sort_by=Location severity_filter=Error bulk_action=Apply fix_preview=Collapsed",
        virtualization_log_after(example)
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

fn collapsible_panel_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{marker} settings: mode=Expanded width=240 pinned=true expand_on_hover=true resize_handle=true callback_log={} actions={actions} -> mode=IconOnly width=320 pinned=false expand_on_hover=true resize_handle=true",
        example.callback_logs.len()
    )
}

fn callback_actions(example: &StoryExample) -> String {
    example
        .callback_logs
        .iter()
        .map(|it| it.action.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn action_line(example: &StoryExample, marker: &str) -> String {
    if let Some(log) = virtualization_log(example).or_else(|| example.callback_logs.first()) {
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

fn is_virtualized_page(page: &str) -> bool {
    matches!(
        page,
        "list" | "selection-list" | "tree-view" | "command-palette" | "diagnostics-list"
    )
}

fn virtualization_log(
    example: &StoryExample,
) -> Option<&katana_ui_core::interaction::UiCallbackLog> {
    example
        .callback_logs
        .iter()
        .find(|it| it.action.contains("virtualization_range"))
}

fn virtualization_log_after(example: &StoryExample) -> &str {
    virtualization_log(example).map_or("missing", |it| it.after.as_str())
}
