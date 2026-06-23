use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::interaction::placement::{
    AnchorKind, Placement, PlacementConsumer, PlacementEngine, PlacementRequest, Point, Rect, Size,
};
use katana_ui_core::molecule::{
    ComboBox, ContextMenuAnchor, ContextMenuPlacement, ContextMenuPlacementResolver,
    ContextMenuSize, ContextMenuViewport, HoverCard, HoverCardAction, HoverCardDelayState,
    HoverCardEvent, Menu, MenuButton, Popover, PopoverActionSlot, PopoverArrowSpec,
    PopoverFocusManagement, PopoverSlots, SelectBox, Tooltip,
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
fn placement_engine_resolve_function_is_deterministic() {
    let request = PlacementRequest::new(
        AnchorKind::virtual_rect(Rect::new(240, 180, 40, 24)),
        Placement::BottomEnd,
        Size::new(120, 80),
        Rect::new(0, 0, 320, 220),
    )
    .priority([Placement::BottomEnd, Placement::TopEnd])
    .offset(4)
    .clamp_margin(8)
    .arrow_size(10);

    let first = PlacementEngine::resolve_placement(&request);
    let second = PlacementEngine::resolve_placement(&request);

    assert_eq!(first, second);
    assert_eq!(Placement::TopEnd, first.placement_used);
    assert!(first.arrow_offset.is_some());
}

#[test]
fn placement_consumers_share_default_priority_contract() {
    let viewport = Rect::new(0, 0, 320, 220);
    let request = PlacementRequest::new(
        AnchorKind::virtual_rect(Rect::new(120, 190, 40, 24)),
        Placement::BottomStart,
        Size::new(120, 80),
        viewport,
    )
    .offset(4)
    .clamp_margin(8);
    let top_consumers = [PlacementConsumer::Tooltip, PlacementConsumer::HoverCard];
    let panel_consumers = [
        PlacementConsumer::Popover,
        PlacementConsumer::ContextMenu,
        PlacementConsumer::Menu,
        PlacementConsumer::MenuButton,
        PlacementConsumer::SelectBox,
        PlacementConsumer::ComboBox,
    ];

    for consumer in top_consumers {
        let result = PlacementEngine::resolve_for(consumer, &request);
        assert_eq!(Placement::Top, result.placement_used);
        assert!(viewport.contains_panel(result.position, Size::new(120, 80)));
    }

    for consumer in panel_consumers {
        let result = PlacementEngine::resolve_for(consumer, &request);
        assert_eq!(Placement::TopStart, result.placement_used);
        assert!(viewport.contains_panel(result.position, Size::new(120, 80)));
    }
}

#[test]
fn disclosure_and_selection_molecules_delegate_panel_placement_to_shared_engine() {
    let viewport = Rect::new(0, 0, 320, 220);
    let request = PlacementRequest::new(
        AnchorKind::virtual_rect(Rect::new(120, 190, 40, 24)),
        Placement::BottomStart,
        Size::new(120, 80),
        viewport,
    )
    .offset(4)
    .clamp_margin(8);
    let tooltip = Tooltip::new("Tooltip");
    let popover = Popover::new("Popover");
    let hover_card = HoverCard::new("Hover card");
    let menu = Menu::new("Menu");
    let menu_button = MenuButton::new("Menu button");
    let select_box = SelectBox::new("Select box");
    let combo_box = ComboBox::new("Combo box");

    assert_eq!(
        Placement::Top,
        tooltip.resolve_panel_placement(&request).placement_used
    );
    assert_eq!(
        Placement::Top,
        hover_card.resolve_panel_placement(&request).placement_used
    );
    assert_eq!(
        Placement::TopStart,
        popover.resolve_panel_placement(&request).placement_used
    );
    assert_eq!(
        Placement::TopStart,
        menu.resolve_panel_placement(&request).placement_used
    );
    assert_eq!(
        Placement::TopStart,
        menu_button.resolve_panel_placement(&request).placement_used
    );
    assert_eq!(
        Placement::TopStart,
        select_box.resolve_panel_placement(&request).placement_used
    );
    assert_eq!(
        Placement::TopStart,
        combo_box.resolve_panel_placement(&request).placement_used
    );
}

#[test]
fn context_menu_resolver_uses_shared_placement_engine_defaults() {
    let result = ContextMenuPlacementResolver::resolve(
        &ContextMenuAnchor::VirtualRect(katana_ui_core::molecule::ContextMenuRect::new(
            120, 190, 40, 24,
        )),
        ContextMenuSize::new(120, 80),
        ContextMenuViewport::new(320, 220),
        &[],
    );

    assert_eq!(ContextMenuPlacement::AboveStart, result.placement);
    assert_eq!(120, result.x);
    assert_eq!(110, result.y);
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
fn hover_card_exposes_typed_content_slots() {
    let slots = PopoverSlots::new()
        .heading("Diagnostic detail")
        .body("Unused import")
        .footer("source: rustc")
        .action(PopoverActionSlot::new("fix-action", "Apply fix"));
    let hover_card = HoverCard::new("Diagnostic").slots(slots);

    assert_eq!("Diagnostic detail", hover_card.slots_model().heading);
    assert_eq!("Unused import", hover_card.slots_model().body);
    assert_eq!("source: rustc", hover_card.slots_model().footer);
    assert_eq!(1, hover_card.slots_model().actions.len());
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
    assert_eq!(10, popover.arrow_model().size_px);
    assert_eq!("surface-raised", popover.arrow_model().tone);
    assert_eq!("Quick actions", popover.slots_model().heading);
    assert_eq!(2, popover.auto_flip_priority_model().len());
    assert!(focus.handled);
    assert!(dismiss.handled);
    assert!(dismiss.after.open);
}
