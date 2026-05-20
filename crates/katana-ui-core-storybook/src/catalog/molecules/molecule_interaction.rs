use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::placement::Placement;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::{UiNodeId, UiStateId};
use katana_ui_core::{atom, molecule};

const TOOLTIP_DELAY_MS: u16 = 240;
const TOOLTIP_MAX_WIDTH: u16 = 280;
const FIRST_OPTION_INDEX: usize = 0;
const SECOND_OPTION_INDEX: usize = 1;
const HOVER_CARD_OPEN_DELAY_MS: u16 = 100;
const HOVER_CARD_CLOSE_DELAY_MS: u16 = 50;
const ACCORDION_TREE_DEPTH: u8 = 2;
const POPOVER_OFFSET_X: i16 = 12;
const POPOVER_OFFSET_Y: i16 = 8;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        StoryCatalog::story(
            "menu",
            molecule::Menu::new("Menu")
                .child(atom::Button::new("Open"))
                .child(atom::Button::new("Close")),
        ),
        tooltip_story(),
        modal_story(),
        accordion_story(),
        combo_box_story(),
        menu_button_story(),
        modal_overlay_story(),
        notification_toast_story(),
        popover_story(),
        hover_card_story(),
        segmented_toggle_story(),
        select_box_story(),
    ]
}

fn tooltip_story() -> StoryExample {
    let mut tooltip = molecule::Tooltip::new("Tooltip")
        .hover_trigger(true)
        .delay_ms(TOOLTIP_DELAY_MS)
        .max_width(TOOLTIP_MAX_WIDTH)
        .child(atom::Icon::new("Info"))
        .child(atom::Text::new("Hint"));
    let target = tooltip.state_id().clone();
    let result = tooltip.apply_action(&UiAction::hover(target, true));
    StoryCatalog::interactive_story("tooltip", tooltip, result.callback_log)
}

fn modal_story() -> StoryExample {
    let mut modal = molecule::Modal::new("Modal")
        .open(true)
        .native_window_mode(true)
        .title("Preferences")
        .panel_size("medium")
        .footer("Cancel / Save")
        .escape_dismiss(true)
        .focus_return("trigger:open-modal")
        .dismiss_policy("parent_interaction=Block")
        .backdrop("native-window")
        .child(atom::Button::new(
            "native window preset=native window action=open_native_window",
        ))
        .child(atom::Text::new(
            "escape close option=escape_dismiss=true event=ModalEscaped",
        ))
        .child(atom::Text::new(
            "focus return state=modal -> trigger focused preset=focus return",
        ))
        .child(atom::Text::new(
            "parent block parent_interaction=Block event=ParentInteractionBlocked",
        ))
        .child(atom::Text::new("Body"))
        .child(atom::Button::new(
            "Close title footer size title=Preferences footer=Cancel / Save size=medium",
        ));
    let target = modal.state_id().clone();
    let result = modal.apply_action(&UiAction::modal_escape(target));
    StoryCatalog::interactive_story(
        "modal",
        modal,
        modal_logs(result.target, result.callback_log),
    )
}

fn accordion_story() -> StoryExample {
    let mut accordion = molecule::Accordion::new("Accordion")
        .open(false)
        .controlled(true)
        .multiple(true)
        .indicator_position("leading")
        .trigger_area(molecule::DisclosureTriggerArea::IconAndText)
        .toggle_icon("<svg data-icon=\"chevron\"/>")
        .tree_mode(true)
        .depth(ACCORDION_TREE_DEPTH)
        .selected(true)
        .show_lines(true)
        .reduced_motion(true)
        .body_border(true)
        .child(atom::Button::new("closed expanded=false disabled=false"))
        .child(atom::Text::new("open expanded=true body_border=true"))
        .child(atom::Text::new("disabled disabled=true block toggle"))
        .child(atom::Text::new("controlled controlled=true request"))
        .child(atom::Text::new("multiple multiple=true group item"))
        .child(atom::Text::new(
            "tree mode tree_mode=true depth=2 selected=true",
        ))
        .child(atom::Text::new("reduced motion reduced_motion=true"))
        .child(atom::Text::new(
            "trigger areas trigger_area=IconAndText toggle_icon=chevron",
        ));
    let target = accordion.state_id().clone();
    let result = accordion.apply_action(&UiAction::accordion_toggle(target));
    StoryCatalog::interactive_story("accordion", accordion, accordion_logs(result.callback_log))
}

fn accordion_logs(mut logs: Vec<UiCallbackLog>) -> Vec<UiCallbackLog> {
    let target = katana_ui_core::render_model::UiStateId::new("state:Accordion:storybook");
    logs.push(UiCallbackLog::new(
        target.clone(),
        "accordion_trigger_area",
        "trigger_area=IconOnly",
        "trigger_area=IconAndText event=TriggerAreaChanged",
    ));
    logs.push(UiCallbackLog::new(
        target.clone(),
        "accordion_controlled_request",
        "controlled=true expanded=false",
        "controlled=true request=Expand event=ControlledExpandRequested",
    ));
    logs.push(UiCallbackLog::new(
        target.clone(),
        "accordion_group_toggle",
        "multiple=true expanded=item-a",
        "multiple=true expanded=item-a,item-b event=GroupToggle",
    ));
    logs.push(UiCallbackLog::new(
        target,
        "accordion_disabled_block",
        "disabled=true expanded=false",
        "disabled=true blocked=true event=DisabledToggleBlocked",
    ));
    logs
}

fn combo_box_story() -> StoryExample {
    let mut combo = molecule::ComboBox::new("Combo box")
        .open(true)
        .item(molecule::ChoiceItem::new("one", "One"))
        .item(molecule::ChoiceItem::new("two", "Two"))
        .child(atom::Input::new("Search"))
        .child(atom::Text::new("Option"));
    let target = combo.state_id().clone();
    let result = combo.apply_action(&UiAction::select_box_selected(target, SECOND_OPTION_INDEX));
    StoryCatalog::interactive_story("combo-box", combo, result.callback_log)
}

fn menu_button_story() -> StoryExample {
    let mut menu = molecule::MenuButton::new("Menu button")
        .open(true)
        .item(molecule::ChoiceItem::new("open", "Open"))
        .child(atom::Button::new("Trigger"))
        .child(molecule::Menu::new("Menu"));
    let target = menu.state_id().clone();
    let result = menu.apply_action(&UiAction::select_box_selected(target, FIRST_OPTION_INDEX));
    StoryCatalog::interactive_story("menu-button", menu, result.callback_log)
}

fn modal_overlay_story() -> StoryExample {
    let mut overlay = molecule::ModalOverlay::new("Modal overlay")
        .open(true)
        .backdrop("dimmer")
        .escape_dismiss(true)
        .focus_trap(true)
        .focus_return("trigger:open-overlay")
        .outside_click_dismiss(true)
        .dismiss_policy("backdrop=true escape=true dismiss_disabled=false")
        .child(
            molecule::Modal::new("Overlay dialog")
                .open(true)
                .title("Confirm")
                .panel_size("small")
                .footer("Cancel / Continue")
                .child(atom::Text::new("same_window_overlay=true")),
        )
        .child(atom::Text::new(
            "overlay dialog preset=overlay dialog same_window_overlay=true",
        ))
        .child(atom::Button::new(
            "backdrop close action=modal_backdrop_click backdrop_close=true",
        ))
        .child(atom::Text::new(
            "escape close action=modal_escape escape_close=true",
        ))
        .child(atom::Text::new(
            "focus trap option=focus_trap=true event=FocusTrapCycled",
        ))
        .child(atom::Text::new(
            "focus return state=overlay -> trigger focused preset=focus return",
        ))
        .child(atom::Text::new(
            "dismiss disabled preset=dismiss disabled backdrop=false",
        ));
    let target = overlay.state_id().clone();
    let result = overlay.apply_action(&UiAction::modal_backdrop_click(target));
    StoryCatalog::interactive_story(
        "modal-overlay",
        overlay,
        modal_overlay_logs(result.target, result.callback_log),
    )
}

fn modal_logs(target: UiStateId, mut handled_logs: Vec<UiCallbackLog>) -> Vec<UiCallbackLog> {
    let mut logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "modal_escape",
            "state=open native_window_mode=true",
            "state=closed event=ModalEscaped",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_focus_return",
            "focus=modal.close_button",
            "focus=trigger:open-modal event=FocusReturned",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_parent_block",
            "parent_interaction=Block parent_click=requested",
            "parent_interaction=Block blocked=true event=ParentInteractionBlocked",
        ),
        UiCallbackLog::new(
            target,
            "modal_native_window_open",
            "window=closed parent=storybook",
            "window=opened same_display=true frontmost=true event=NativeWindowOpened",
        ),
    ];
    logs.append(&mut handled_logs);
    logs
}

fn modal_overlay_logs(
    target: UiStateId,
    mut handled_logs: Vec<UiCallbackLog>,
) -> Vec<UiCallbackLog> {
    let mut logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "modal_backdrop_click",
            "state=open backdrop_close=true",
            "state=closed event=OverlayBackdropClosed",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_escape",
            "state=open escape_close=true",
            "state=closed event=OverlayEscaped",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_focus_trap",
            "focus=first-field tab=forward",
            "focus=primary-action event=FocusTrapCycled",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_focus_return",
            "focus=overlay.primary-action",
            "focus=trigger:open-overlay event=FocusReturned",
        ),
        UiCallbackLog::new(
            target,
            "modal_dismiss_disabled",
            "dismiss_disabled=true backdrop_click=requested",
            "dismiss_disabled=true state=open event=DismissBlocked",
        ),
    ];
    logs.append(&mut handled_logs);
    logs
}

fn notification_toast_story() -> StoryExample {
    let mut toast = molecule::NotificationToast::new("Notification")
        .open(true)
        .child(atom::Badge::new("Info"))
        .child(atom::Text::new("Message"));
    let target = toast.state_id().clone();
    let result = toast.apply_action(&UiAction::dismiss(target));
    StoryCatalog::interactive_story("notification-toast", toast, result.callback_log)
}

fn popover_story() -> StoryExample {
    let anchor = "node:toolbar.more-actions";
    let focus_return = "trigger:popover-anchor";
    let slots = molecule::PopoverSlots::new()
        .heading("Quick actions")
        .body("Operate on the current selection")
        .footer("Esc closes and returns focus")
        .action(molecule::PopoverActionSlot::new("copy-action", "Copy"));
    let popover = molecule::Popover::new("Popover")
        .open(true)
        .anchor_summary(anchor)
        .placement("bottom-start")
        .offset(POPOVER_OFFSET_X, POPOVER_OFFSET_Y)
        .width("320px")
        .outside_click_dismiss(true)
        .escape_dismiss(true)
        .arrow(molecule::PopoverArrowSpec::new(true, 10, "surface-raised"))
        .slots(slots)
        .focus_management(molecule::PopoverFocusManagement::FirstInteractive)
        .focus_return_target(UiNodeId::new(focus_return))
        .focus_handling("FirstInteractive + return:trigger:popover-anchor")
        .keep_open_on_inner_focus(true)
        .auto_flip_priority([Placement::BottomStart, Placement::TopStart])
        .child(atom::Button::new(
            "anchor preset=anchor anchor=node:toolbar.more-actions action=popover_open",
        ))
        .child(atom::Text::new(
            "placement preset=placement placement=bottom-start resolved=bottom-start",
        ))
        .child(atom::Text::new(
            "auto flip preset=auto flip priority=BottomStart>TopStart event=PopoverAutoFlipped",
        ))
        .child(atom::Text::new(
            "offset width preset=offset width offset=12,8 width=320px",
        ))
        .child(atom::Text::new(
            "outside+escape close preset=outside+escape close outside=true escape=true",
        ))
        .child(atom::Text::new(
            "focus handling preset=focus handling focus=FirstInteractive focus_return=trigger:popover-anchor",
        ))
        .child(atom::Button::new(
            "slot content preset=slot content heading/body/footer/action=Copy",
        ));
    let target = popover.state_id().clone();
    StoryCatalog::interactive_story(
        "popover",
        popover,
        popover_logs(target, anchor, focus_return),
    )
}

fn popover_logs(target: UiStateId, anchor: &str, focus_return: &str) -> Vec<UiCallbackLog> {
    vec![
        UiCallbackLog::new(
            target.clone(),
            "popover_open",
            format!("state=open=false anchor={anchor} placement=bottom-start"),
            "state=open=true event=PopoverOpened focus=copy-action",
        ),
        UiCallbackLog::new(
            target.clone(),
            "popover_outside_close",
            "state=open=true outside_click=true dismiss_on_outside_click=true",
            format!("state=closed event=PopoverOutsideClosed focus={focus_return}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "popover_escape_close",
            "state=open=true escape=true dismiss_on_escape=true",
            format!("state=closed event=PopoverEscapeClosed focus={focus_return}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "popover_auto_flip",
            "preferred=BottomStart viewport_edge=bottom priority=BottomStart>TopStart",
            "resolved=TopStart event=PopoverAutoFlipped",
        ),
        UiCallbackLog::new(
            target.clone(),
            "popover_focus_return",
            "focus=copy-action close=escape",
            format!("focus={focus_return} event=PopoverFocusReturned"),
        ),
        UiCallbackLog::new(
            target,
            "popover_slot_action",
            "slot=copy-action content=selected-row",
            "slot=copy-action event=PopoverSlotActionInvoked action=Copy",
        ),
    ]
}

fn hover_card_story() -> StoryExample {
    let slots = molecule::PopoverSlots::new()
        .heading("Capability")
        .body("Shows rich hover and focus content")
        .footer("Keeps open while the card is focused")
        .action(molecule::PopoverActionSlot::new(
            "configure-action",
            "Configure",
        ));
    let mut hover_card = molecule::HoverCard::new("Hover card")
        .open_delay_ms(HOVER_CARD_OPEN_DELAY_MS)
        .close_delay_ms(HOVER_CARD_CLOSE_DELAY_MS)
        .pointer_follow(true)
        .slots(slots);
    let opened =
        hover_card.apply_hover_card_action(molecule::HoverCardAction::AnchorPointerEntered);
    let kept = hover_card.apply_hover_card_action(molecule::HoverCardAction::CardPointerEntered);
    let target = katana_ui_core::render_model::UiStateId::new("state:HoverCard:storybook");
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "hover_card_open",
            "open=false",
            format!("event={opened:?}"),
        ),
        UiCallbackLog::new(
            target,
            "hover_card_keep_open",
            "close=scheduled",
            format!("event={kept:?}"),
        ),
    ];
    StoryCatalog::interactive_story("hover-card", hover_card, logs)
}

fn segmented_toggle_story() -> StoryExample {
    let mut segmented = molecule::SegmentedToggle::new("Segmented toggle")
        .item(molecule::ChoiceItem::new("preview", "Preview"))
        .item(molecule::ChoiceItem::new("code", "Code"))
        .selected_index(1)
        .child(atom::Toggle::new("Preview"))
        .child(atom::Toggle::new("Code"));
    let target = segmented.state_id().clone();
    let result = segmented.apply_action(&UiAction::segmented_toggle_selected(target, 0));
    StoryCatalog::interactive_story("segmented-toggle", segmented, result.callback_log)
}

fn select_box_story() -> StoryExample {
    let mut select = molecule::SelectBox::new("Select box")
        .open(true)
        .placement("bottom-start")
        .item(molecule::ChoiceItem::new("light", "Light"))
        .item(molecule::ChoiceItem::new("dark", "Dark"))
        .child(atom::Button::new("Trigger"))
        .child(molecule::List::new("Options"));
    let target = select.state_id().clone();
    let result = select.apply_action(&UiAction::select_box_selected(target, SECOND_OPTION_INDEX));
    StoryCatalog::interactive_story("select-box", select, result.callback_log)
}
