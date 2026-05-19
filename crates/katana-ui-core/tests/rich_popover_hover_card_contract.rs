use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::interaction::placement::{
    AnchorKind, Placement, PlacementEngine, PlacementRequest, Point, Rect, Size,
};
use katana_ui_core::molecule::{
    HoverCard, HoverCardAction, HoverCardDelayState, HoverCardEvent, Popover, PopoverActionSlot,
    PopoverArrowSpec, PopoverFocusManagement, PopoverSlots,
};
use katana_ui_core::render_model::UiNodeId;

const OPEN_DELAY_MS: u16 = 100;
const CLOSE_DELAY_MS: u16 = 50;

#[test]
fn placement_engine_flips_clamps_and_aligns_arrow() {
    let viewport = Rect::new(0, 0, 300, 200);
    let request = PlacementRequest::new(
        AnchorKind::virtual_rect(Rect::new(140, 180, 20, 10)),
        Placement::Bottom,
        Size::new(100, 80),
        viewport,
    )
    .priority([Placement::Bottom, Placement::Top])
    .offset(8)
    .clamp_margin(8)
    .arrow_size(10);

    let result = PlacementEngine::resolve(&request);

    assert_eq!(Placement::Top, result.placement_used);
    assert_eq!(Point::new(100, 92), result.position);
    assert_eq!(Some(50), result.arrow_offset);
    assert!(!result.clamped);
}

#[test]
fn placement_engine_covers_anchor_and_placement_matrix() {
    let viewport = Rect::new(0, 0, 640, 480);
    let anchors = [
        AnchorKind::node_rect(UiNodeId::new("toolbar-button"), Rect::new(220, 160, 40, 28)),
        AnchorKind::virtual_rect(Rect::new(300, 220, 80, 32)),
        AnchorKind::pointer(Point::new(420, 240)),
    ];
    let placements = [
        Placement::Top,
        Placement::TopStart,
        Placement::TopEnd,
        Placement::Right,
        Placement::Bottom,
        Placement::BottomStart,
        Placement::BottomEnd,
        Placement::Left,
    ];

    for anchor in anchors {
        for placement in placements {
            let request =
                PlacementRequest::new(anchor.clone(), placement, Size::new(120, 60), viewport)
                    .offset(6)
                    .clamp_margin(4)
                    .arrow_size(8);
            let result = PlacementEngine::resolve(&request);

            assert!(viewport.contains_panel(result.position, Size::new(120, 60)));
            assert!(result.arrow_offset.is_some());
        }
    }
}

#[test]
fn hover_card_open_and_close_delays_are_state_transitions() {
    let mut hover_card = HoverCard::new("Diagnostics")
        .open_delay_ms(OPEN_DELAY_MS)
        .close_delay_ms(CLOSE_DELAY_MS);

    let scheduled = hover_card.apply_hover_card_action(HoverCardAction::AnchorPointerEntered);
    let early =
        hover_card.apply_hover_card_action(HoverCardAction::TimerElapsed(OPEN_DELAY_MS - 1));
    let opened = hover_card.apply_hover_card_action(HoverCardAction::TimerElapsed(1));
    let closing = hover_card.apply_hover_card_action(HoverCardAction::AnchorPointerLeft);
    let still_open =
        hover_card.apply_hover_card_action(HoverCardAction::TimerElapsed(CLOSE_DELAY_MS - 1));
    let closed = hover_card.apply_hover_card_action(HoverCardAction::TimerElapsed(1));

    assert_eq!(HoverCardEvent::OpenScheduled, scheduled);
    assert_eq!(HoverCardEvent::DelayPending, early);
    assert_eq!(HoverCardEvent::Opened, opened);
    assert_eq!(HoverCardEvent::CloseScheduled, closing);
    assert_eq!(HoverCardEvent::DelayPending, still_open);
    assert_eq!(HoverCardEvent::Closed, closed);
    assert!(!hover_card.is_open());
}

#[test]
fn hover_card_pointer_and_focus_keep_card_open() {
    let mut hover_card = HoverCard::new("Capability")
        .open_delay_ms(0)
        .close_delay_ms(CLOSE_DELAY_MS)
        .pointer_follow(true)
        .slot_action(PopoverActionSlot::new("configure-action", "Configure"));

    assert_eq!(
        HoverCardEvent::Opened,
        hover_card.apply_hover_card_action(HoverCardAction::AnchorPointerEntered)
    );
    assert_eq!(
        HoverCardEvent::CloseScheduled,
        hover_card.apply_hover_card_action(HoverCardAction::AnchorPointerLeft)
    );
    assert_eq!(
        HoverCardEvent::KeptOpen,
        hover_card.apply_hover_card_action(HoverCardAction::CardPointerEntered)
    );
    assert_eq!(
        HoverCardEvent::KeptOpen,
        hover_card.apply_hover_card_action(HoverCardAction::TimerElapsed(CLOSE_DELAY_MS))
    );
    assert_eq!(HoverCardDelayState::PausedClose, hover_card.delay_state());

    let focused = hover_card.apply_hover_card_action(HoverCardAction::InnerFocusEntered(
        UiNodeId::new("configure-action"),
    ));

    assert_eq!(HoverCardEvent::KeptOpen, focused);
    assert!(hover_card.is_open());
    assert!(hover_card.pointer_follow_model());
}

#[test]
fn popover_focus_arrow_slots_and_keep_open_are_contract_options() {
    let anchor = UiNodeId::new("toolbar-anchor");
    let first_action = PopoverActionSlot::new("copy-action", "Copy");
    let slots = PopoverSlots::new()
        .heading("Quick actions")
        .body("Operate on the current selection")
        .action(first_action.clone());
    let mut popover = Popover::new("Actions")
        .open(true)
        .arrow(PopoverArrowSpec::new(true, 10, "surface-raised"))
        .slots(slots)
        .focus_management(PopoverFocusManagement::FirstInteractive)
        .focus_return_target(anchor.clone())
        .keep_open_on_inner_focus(true)
        .auto_flip_priority([Placement::BottomStart, Placement::TopStart]);

    let focus = popover.apply_action(&UiAction::focus(popover.state_id().clone()));
    let dismiss = popover.apply_action(&UiAction::dismiss(popover.state_id().clone()));

    assert_eq!(Some(first_action.node_id), popover.open_focus_target());
    assert_eq!(Some(anchor), popover.close_focus_target());
    assert!(popover.arrow_model().visible);
    assert_eq!("Quick actions", popover.slots_model().heading);
    assert_eq!(2, popover.auto_flip_priority_model().len());
    assert!(focus.handled);
    assert!(dismiss.handled);
    assert!(dismiss.after.open);
}
