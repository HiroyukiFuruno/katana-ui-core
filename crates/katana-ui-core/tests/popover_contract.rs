use katana_ui_core::atom::{Button, Text};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::interaction::placement::{
    AnchorKind, Placement, PlacementRequest, Point, Rect, Size,
};
use katana_ui_core::molecule::{
    Popover, PopoverActionSlot, PopoverArrowSpec, PopoverFocusManagement, PopoverSlots,
};
use katana_ui_core::render_model::{
    UiNodeId, UiNodeKind, UiPopoverFocusManagement, UiPopoverPlacement, UiTree,
};

#[test]
fn popover_render_props_expose_anchor_placement_offset_width_and_dismiss_contract() {
    let tree = UiTree::new(
        Popover::new("Actions")
            .open(true)
            .anchor_summary("toolbar.action")
            .placement("bottom-start")
            .offset(8, 12)
            .width("320px")
            .focus_handling("return-to-anchor")
            .outside_click_dismiss(true)
            .escape_dismiss(true)
            .arrow(PopoverArrowSpec::new(true, 10, "surface-raised"))
            .slots(
                PopoverSlots::new()
                    .heading("Quick actions")
                    .body("Operate on the selection")
                    .footer("2 actions")
                    .action(PopoverActionSlot::new("copy-action", "Copy")),
            )
            .focus_management(PopoverFocusManagement::FirstInteractive)
            .auto_flip_priority([Placement::BottomStart, Placement::TopStart])
            .child(Button::new("Copy"))
            .child(Text::new("Complex content")),
    );
    let props = tree.root().props();

    assert_eq!(UiNodeKind::Popover, tree.root().kind());
    assert_eq!("toolbar.action", props.popover.anchor);
    assert_eq!(UiPopoverPlacement::BottomStart, props.popover.placement);
    assert_eq!((8, 12), (props.popover.offset_x, props.popover.offset_y));
    assert_eq!("320px", props.popover.width);
    assert_eq!("return-to-anchor", props.popover.focus_handling);
    assert!(props.popover.dismiss_on_outside_click);
    assert!(props.popover.dismiss_on_escape);
    assert!(props.popover.arrow_visible);
    assert_eq!(10, props.popover.arrow_size_px);
    assert_eq!("surface-raised", props.popover.arrow_tone);
    assert_eq!("Quick actions", props.popover.heading);
    assert_eq!(1, props.popover.action_count);
    assert_eq!(
        UiPopoverFocusManagement::FirstInteractive,
        props.popover.focus_management
    );
    assert_eq!(2, props.popover.auto_flip_priority.len());
    assert_eq!(2, tree.root().children().len());
}

#[test]
fn popover_resolves_placement_flip_offset_and_arrow_position() {
    let popover = Popover::new("Actions");
    let request = PlacementRequest::new(
        AnchorKind::virtual_rect(Rect::new(560, 560, 80, 32)),
        Placement::BottomStart,
        Size::new(220, 120),
        Rect::new(0, 0, 800, 640),
    )
    .priority([Placement::BottomStart, Placement::TopStart])
    .offset(8)
    .clamp_margin(12)
    .arrow_size(10);

    let result = popover.resolve_panel_placement(&request);

    assert_eq!(Placement::TopStart, result.placement_used);
    assert_eq!(Point::new(560, 432), result.position);
    assert_eq!(Some(40), result.arrow_offset);
    assert!(!result.clamped);
}

#[test]
fn popover_outside_escape_and_focus_return_are_deterministic() {
    let focus_return = UiNodeId::new("toolbar-anchor");
    let mut popover = Popover::new("Actions")
        .open(true)
        .outside_click_dismiss(true)
        .escape_dismiss(true)
        .focus_return_target(focus_return.clone());

    let outside = popover.apply_action(&UiAction::modal_backdrop_click(popover.state_id().clone()));
    popover = popover.open(true);
    let escape = popover.apply_action(&UiAction::modal_escape(popover.state_id().clone()));

    assert!(outside.handled);
    assert!(!outside.after.open);
    assert_eq!("outside", outside.after.dismiss_reason);
    assert_eq!("focus_return=toolbar-anchor", outside.after.value);
    assert!(escape.handled);
    assert!(!escape.after.open);
    assert_eq!("escape", escape.after.dismiss_reason);
    assert_eq!(Some(focus_return), popover.close_focus_target());
}

#[test]
fn popover_keep_open_on_inner_focus_blocks_dismiss_until_focus_leaves() {
    let mut popover = Popover::new("Actions")
        .open(true)
        .keep_open_on_inner_focus(true);

    let focus = popover.apply_action(&UiAction::focus(popover.state_id().clone()));
    let dismiss_while_focused =
        popover.apply_action(&UiAction::dismiss(popover.state_id().clone()));
    let blur = popover.apply_action(&UiAction::blur(popover.state_id().clone()));
    let dismiss_after_blur = popover.apply_action(&UiAction::dismiss(popover.state_id().clone()));

    assert!(focus.after.open);
    assert!(dismiss_while_focused.after.open);
    assert!(!blur.after.focused);
    assert!(!dismiss_after_blur.after.open);
}

#[test]
fn popover_hover_focus_toggle_and_escape_are_core_actions() {
    let mut popover = Popover::new("Actions")
        .open(false)
        .outside_click_dismiss(true)
        .escape_dismiss(true)
        .keep_open_on_inner_focus(true)
        .focus_return_target(UiNodeId::new("toolbar-anchor"));

    let opened = popover.apply_action(&UiAction::popover_toggle(popover.state_id().clone()));
    let hovered = popover.apply_action(&UiAction::hover(popover.state_id().clone(), true));
    let focused = popover.apply_action(&UiAction::focus(popover.state_id().clone()));
    let escaped = popover.apply_action(&UiAction::modal_escape(popover.state_id().clone()));

    assert!(opened.handled);
    assert!(opened.after.open);
    assert!(hovered.handled);
    assert!(hovered.after.hovered);
    assert!(focused.handled);
    assert!(focused.after.focused);
    assert!(escaped.handled);
    assert!(!escaped.after.open);
    assert_eq!("escape", escaped.after.dismiss_reason);
}
