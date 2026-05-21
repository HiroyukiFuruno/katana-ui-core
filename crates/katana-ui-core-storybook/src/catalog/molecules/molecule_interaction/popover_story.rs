use super::{
    POPOVER_OFFSET_X, POPOVER_OFFSET_Y, Placement, StoryCatalog, StoryExample, UiCallbackLog,
    UiNodeId, UiStateId, atom, molecule,
};

const POPOVER_ARROW_SIZE: u16 = 10;

pub(super) fn popover_story() -> StoryExample {
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
        .arrow(molecule::PopoverArrowSpec::new(
            true,
            POPOVER_ARROW_SIZE,
            "surface-raised",
        ))
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
