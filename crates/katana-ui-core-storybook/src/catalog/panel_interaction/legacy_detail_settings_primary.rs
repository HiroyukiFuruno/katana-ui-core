use super::legacy_detail_core::{callback_actions, is_virtualized_page, virtualization_log_after};
use super::legacy_detail_settings_secondary as secondary;
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
        "badge" => {
            format!("{marker} settings: passive status -> use Chip for dismiss / interactive")
        }
        "card" => card_settings_line(marker),
        "panel" => panel_settings_line(marker),
        "scroll-area" => scroll_area_settings_line(example, marker),
        "search-control-strip" => search_control_settings_line(example, marker),
        "modal" => modal_settings_line(example, marker),
        "modal-overlay" => modal_overlay_settings_line(example, marker),
        "drag-and-drop" => drag_and_drop_settings_line(example, marker),
        "context-menu" => context_menu_settings_line(example, marker),
        "color-picker-rgba" => color_picker_settings_line(example, marker),
        "popover" => popover_settings_line(example, marker),
        "hover-card" => hover_card_settings_line(marker),
        "accordion" => accordion_settings_line(example, marker),
        "toolbar" => toolbar_settings_line(marker),
        "split-pane" => split_pane_settings_line(example, marker),
        "text-area" => text_area_settings_line(marker),
        "skeleton" => skeleton_settings_line(example, marker),
        "skeleton-cluster" => skeleton_cluster_settings_line(example, marker),
        "motion" => motion_settings_line(example, marker),
        "command-palette" => command_palette_settings_line(example, marker),
        "diagnostics-list" => {
            secondary::settings_line(example, marker, option, value_type, before, after)
        }
        page if is_virtualized_page(page) => virtualization_settings_line(example, marker),
        _ => secondary::settings_line(example, marker, option, value_type, before, after),
    }
}

fn panel_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: panel.vertical_scroll panel.horizontal_scroll panel.scrollbar_visibility panel.nested_state -> preview overflow x/y, inspector toggle hides panel bars, nested panels keep local offsets"
    )
}

fn card_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: card.label card.header card.footer card.variant card.padding card.clickable card.nested_controls card.child_state -> Project summary custom visible theme_border Large true interactive changed"
    )
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
    let anchor = super::option_value("context_menu.anchor", props);
    let placement_priority = super::option_value("context_menu.placement_priority", props);
    let placement_used = super::option_value("context_menu.placement_used", props);
    let min_width = super::option_value("context_menu.min_width", props);
    let max_height = super::option_value("context_menu.max_height", props);
    let item_kind = super::option_value("context_menu.item_kind", props);
    let log_count = example.callback_logs.len();
    format!(
        "{marker} settings: context_menu.anchor={anchor} context_menu.placement_priority={placement_priority} context_menu.placement_used={placement_used} context_menu.min_width={min_width} context_menu.max_height={max_height} context_menu.item_kind={item_kind} callback_log={log_count} -> context_menu.anchor=Pointer(0,0) context_menu.placement_priority=AboveEnd>BelowStart context_menu.placement_used=AboveEnd context_menu.min_width=280 context_menu.max_height=320 callback_log={log_count}"
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

fn search_control_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: query match_case whole_word regex replace_mode result_count active_index actions={actions} -> query=heading match_case=true whole_word=true regex=true replace_mode=Visible result_count=12 active_index=2"
    )
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
    let actions = callback_actions(example);
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
        "{marker} settings: toolbar.display_mode toolbar.density toolbar.overflow_strategy toolbar.actions toolbar.groups toolbar.context_menu_anchor toolbar.action_priority toolbar.action_accelerator toolbar.action_split toolbar.action_group toolbar.action_tooltip toolbar.action_a11y toolbar.action_disabled toolbar.group_label toolbar.group_divider toolbar.split_disabled toolbar.split_tooltip toolbar.split_a11y -> IconOnly Compact Custom actions=4 groups=2 anchor=pointer priority=critical accelerator=Alt+P split=menu group=edit tooltip=Save file a11y=custom disabled=true divider=false"
    )
}

fn split_pane_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: axis gap alignment overflow ratio_percent min_percent max_percent reset_percent handle_width_px resize_mode; axis=Horizontal/Vertical gap=0 alignment=Start overflow=Fit ratio_percent=50 min_percent=20 max_percent=80 reset_percent=50 handle_width_px=8 resize_mode=Drag+Keyboard children=2 nested=true callback_log={} actions={actions}; state: ratio=50 dragging=false focused_handle=false last_event=RatioChanged; event: ResizeStarted RatioChanged ResizeEnded ResizeRejected; action: split_pane_set_ratio split_pane_resize_by split_pane_reset_ratio; quality: clamp event_order public_api_guard -> axis=Vertical gap=12 alignment=Center overflow=Scroll ratio_percent=64 min_percent=24 max_percent=76 reset_percent=55 handle_width_px=10 resize_mode=Keyboard children=2 nested=true",
        example.callback_logs.len()
    )
}

fn text_area_settings_line(marker: &str) -> String {
    format!(
        "{marker} settings: submit/newline/tab/auto/wrap/resize/scroll/bars Enter,ShiftEnter,MoveFocus,true,Soft,false,false,false,false,false -> ModEnter,Enter,InsertTab,false,None,true,true,true,true,true"
    )
}

fn skeleton_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: shape=Text text_lines=3 last_line_ratio=0.58 line_thickness=8 size=220x44 animation=Shimmer tone=Neutral radius=4 reduced_motion=false accessibility_label=Loading text lines aspect_ratio=none callback_log={} actions={actions} -> shape=Line text_lines=2 last_line_ratio=0.62 line_thickness=12 size=Fill animation=Wave tone=Success radius=14 reduced_motion=true accessibility_label=Loading profile placeholder aspect_ratio=16:9",
        example.callback_logs.len()
    )
}

fn skeleton_cluster_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: preset=ListRow children=2 live_region=Loading list loading reduced_motion=false callback_log={} actions={actions} -> preset=ImageCard children=3 live_region=Loading custom image card reduced_motion=true",
        example.callback_logs.len()
    )
}

fn motion_settings_line(example: &StoryExample, marker: &str) -> String {
    let actions = callback_actions(example);
    format!(
        "{marker} settings: primitive: Fade Slide Scale Shimmer; tokens: duration=Default easing=Emphasized distance=Default; state: instant=false duration=200 distance=8; event: reduced_motion_query override=Ignore context=Storybook; action: motion_reduce motion_tick motion_force motion_ignore motion_override; quality: token_resolution reduced_static override_isolated; callback_log={} actions={actions} -> primitive=Scale duration=Slow easing=Decelerate distance=Spacious reduced_policy=Ignore",
        example.callback_logs.len()
    )
}

fn command_palette_settings_line(example: &StoryExample, marker: &str) -> String {
    format!(
        "{marker} settings: query=open->theme highlight=0->2 row_count=5->50 provider_group=workspace/editor/app shortcut_display=visible/hidden disabled_reason=readonly virtualization enabled=true->false overscan=2->4 row_height_provider=Fixed->Variable visible_range={} -> query=theme highlight=2 row_count=50",
        virtualization_log_after(example)
    )
}

fn virtualization_settings_line(example: &StoryExample, marker: &str) -> String {
    let range = virtualization_log_after(example);
    format!(
        "{marker} settings: virtualization enabled=true->false overscan=2->4 row_height_provider=Fixed->Variable visible_range={range} -> virtualization disabled"
    )
}
