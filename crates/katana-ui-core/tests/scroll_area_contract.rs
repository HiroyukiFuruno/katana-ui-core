use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::event::UiEvent;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::layout::{
    Alignment, Length, ScrollArea, ScrollAreaAction, ScrollAreaEvent, ScrollAxis, ScrollEdge,
    ScrollRejectionReason, ScrollbarPlacement, ScrollbarVisibility,
};
use katana_ui_core::render_model::{
    UiAlignItems, UiDimension, UiNodeKind, UiRect, UiScrollAreaAxis, UiScrollbarPlacement,
    UiScrollbarVisibility, UiTree,
};

#[test]
fn scroll_area_render_props_expose_axis_extent_offset_and_scrollbar() {
    let tree = UiTree::new(
        ScrollArea::new()
            .axis(ScrollAxis::Vertical)
            .viewport(320, 240)
            .content_extent(320, 960)
            .offset(0, 840)
            .scrollbar_visibility(ScrollbarVisibility::Always)
            .scrollbar_placement(ScrollbarPlacement::Overlay)
            .edge_threshold(16)
            .gap(Length::px(10.0))
            .align(Alignment::Center)
            .child(Text::new("Scroll item")),
    );
    let props = tree.root().props();

    assert_eq!(UiNodeKind::ScrollArea, tree.root().kind());
    assert_eq!(UiScrollAreaAxis::Vertical, props.scroll_area.axis);
    assert_eq!(0, props.scroll_area.offset_x);
    assert_eq!(720, props.scroll_area.offset_y);
    assert_eq!(320, props.scroll_area.viewport_width);
    assert_eq!(240, props.scroll_area.viewport_height);
    assert_eq!(320, props.scroll_area.content_width);
    assert_eq!(960, props.scroll_area.content_height);
    assert_eq!(
        UiScrollbarVisibility::Always,
        props.scroll_area.scrollbar_visibility
    );
    assert_eq!(
        UiScrollbarPlacement::Overlay,
        props.scroll_area.scrollbar_placement
    );
    assert_eq!(16, props.scroll_area.edge_threshold);
    assert_eq!(UiDimension::Px(10), props.scroll_area.gap);
    assert_eq!(UiAlignItems::Center, props.scroll_area.alignment);
}

#[test]
fn scroll_area_default_and_remaining_render_variants_are_typed() {
    let default_area = ScrollArea::default();
    assert_eq!(0, default_area.offset_x());
    assert_eq!(0, default_area.offset_y());

    let horizontal = UiTree::new(
        ScrollArea::new()
            .axis(ScrollAxis::Horizontal)
            .scrollbar_visibility(ScrollbarVisibility::Hidden)
            .align(Alignment::Start),
    );
    assert_eq!(
        UiScrollAreaAxis::Horizontal,
        horizontal.root().props().scroll_area.axis
    );
    assert_eq!(
        UiScrollbarVisibility::Hidden,
        horizontal.root().props().scroll_area.scrollbar_visibility
    );
    assert_eq!(
        UiAlignItems::Start,
        horizontal.root().props().scroll_area.alignment
    );

    let both = UiTree::new(ScrollArea::new().axis(ScrollAxis::Both));
    assert_eq!(UiScrollAreaAxis::Both, both.root().props().scroll_area.axis);
}

#[test]
fn scroll_actions_clamp_into_view_and_reject_axis_mismatch() {
    let mut area = ScrollArea::new()
        .axis(ScrollAxis::Vertical)
        .viewport(320, 200)
        .content_extent(320, 900)
        .offset(0, 100);
    let into_view = area.apply_scroll_action(ScrollAreaAction::ScrollIntoView {
        target_rect: UiRect::new(0, 760, 120, 80),
    });
    assert_eq!(640, area.offset_y());
    assert_eq!(
        vec![ScrollAreaEvent::Scrolled {
            target: area.state_id().clone(),
            x: 0,
            y: 640,
        }],
        into_view
    );

    let rejected = area.apply_scroll_action(ScrollAreaAction::ScrollBy { dx: 24, dy: 0 });
    assert_eq!(0, area.offset_x());
    assert_eq!(
        vec![ScrollAreaEvent::ScrollCommandRejected {
            target: area.state_id().clone(),
            reason: ScrollRejectionReason::AxisMismatch,
        }],
        rejected
    );
    assert!(matches!(
        UiEvent::Scroll(rejected[0].clone()),
        UiEvent::Scroll(_)
    ));
}

#[test]
fn scroll_area_rejects_scroll_when_content_does_not_overflow() {
    let mut area = ScrollArea::new()
        .axis(ScrollAxis::Both)
        .viewport(320, 200)
        .content_extent(320, 200)
        .offset(120, 80)
        .scrollbar_visibility(ScrollbarVisibility::Always);

    assert_eq!(0, area.offset_x());
    assert_eq!(0, area.offset_y());
    assert_eq!(
        vec![ScrollAreaEvent::ScrollCommandRejected {
            target: area.state_id().clone(),
            reason: ScrollRejectionReason::NoOverflow,
        }],
        area.apply_scroll_action(ScrollAreaAction::ScrollBy { dx: 16, dy: 16 })
    );
}

#[test]
fn nested_scroll_area_events_target_only_the_child_state() {
    let parent = ScrollArea::new()
        .axis(ScrollAxis::Vertical)
        .viewport(400, 300)
        .content_extent(400, 900);
    let mut child = ScrollArea::new()
        .axis(ScrollAxis::Vertical)
        .viewport(240, 160)
        .content_extent(240, 640);
    let parent_id = parent.state_id().clone();
    let child_id = child.state_id().clone();

    let events = child.apply_scroll_action(ScrollAreaAction::ScrollBy { dx: 0, dy: 80 });

    assert_eq!(0, parent.offset_y());
    assert_eq!(80, child.offset_y());
    assert_ne!(parent_id, child_id);
    assert!(events.iter().all(|it| it.target() == &child_id));
}

#[test]
fn scroll_edge_event_is_emitted_when_clamped_to_bottom() {
    let mut area = ScrollArea::new()
        .axis(ScrollAxis::Vertical)
        .viewport(320, 200)
        .content_extent(320, 260);
    let events = area.apply_scroll_action(ScrollAreaAction::ScrollBy { dx: 0, dy: 120 });

    assert_eq!(
        vec![
            ScrollAreaEvent::Scrolled {
                target: area.state_id().clone(),
                x: 0,
                y: 60,
            },
            ScrollAreaEvent::ScrollEdgeReached {
                target: area.state_id().clone(),
                edge: ScrollEdge::Bottom,
            },
        ],
        events
    );
    assert!(events.iter().all(|event| event.target() == area.state_id()));
}

#[test]
fn ui_action_builders_dispatch_scroll_commands() {
    let mut area = ScrollArea::new()
        .axis(ScrollAxis::Both)
        .viewport(100, 100)
        .content_extent(300, 300);
    let result = area.apply_action(&UiAction::scroll_by(area.state_id().clone(), 40, 30));

    assert!(result.handled);
    assert_eq!("offset=40,30", result.after.value);
    assert_eq!("scroll_by", result.callback_log[0].action);

    let edge = area.apply_action(&UiAction::scroll_to(area.state_id().clone(), 200, 200));
    assert!(edge.handled);
    assert!(
        edge.callback_log
            .iter()
            .any(|entry| entry.after.contains("ScrollEdgeReached(Bottom)"))
    );
}

#[test]
fn ui_action_dispatch_covers_scroll_to_into_view_visibility_and_noop() {
    let mut area = ScrollArea::new()
        .axis(ScrollAxis::Both)
        .viewport(100, 100)
        .content_extent(300, 300)
        .offset(80, 80);
    let target = area.state_id().clone();

    assert!(
        area.apply_action(&UiAction::scroll_to(target.clone(), 120, 120))
            .handled
    );
    assert!(
        area.apply_action(&UiAction::scroll_into_view(
            target.clone(),
            UiRect::new(20, 20, 10, 10),
        ))
        .handled
    );
    for visibility in [UiScrollbarVisibility::Auto, UiScrollbarVisibility::Always] {
        assert!(
            area.apply_action(&UiAction::scrollbar_visibility(target.clone(), visibility,))
                .handled
        );
    }
    assert!(!area.apply_action(&UiAction::focus(target)).handled);
}

#[test]
fn scroll_to_current_offset_emits_only_current_edge_contracts() {
    let mut area = ScrollArea::new()
        .axis(ScrollAxis::Both)
        .viewport(100, 100)
        .content_extent(300, 300);
    let events = area.apply_scroll_action(ScrollAreaAction::ScrollTo { x: 0, y: 0 });

    assert_eq!(2, events.len());
    assert!(events.iter().all(|event| matches!(
        event,
        ScrollAreaEvent::ScrollEdgeReached {
            edge: ScrollEdge::Top | ScrollEdge::Left,
            ..
        }
    )));
}

#[test]
fn horizontal_scroll_clamps_vertical_offset_and_reports_top_right_edges() {
    let mut area = ScrollArea::new()
        .axis(ScrollAxis::Horizontal)
        .viewport(100, 100)
        .content_extent(240, 240)
        .offset(20, 80);

    assert_eq!(20, area.offset_x());
    assert_eq!(0, area.offset_y());

    let events = area.apply_scroll_action(ScrollAreaAction::ScrollTo { x: 140, y: 80 });
    assert_eq!(140, area.offset_x());
    assert_eq!(0, area.offset_y());
    assert!(events.iter().any(|event| {
        matches!(
            event,
            ScrollAreaEvent::ScrollEdgeReached {
                edge: ScrollEdge::Right,
                ..
            }
        )
    }));

    let events = area.apply_scroll_action(ScrollAreaAction::ScrollTo { x: 0, y: 80 });
    assert!(events.iter().any(|event| {
        matches!(
            event,
            ScrollAreaEvent::ScrollEdgeReached {
                edge: ScrollEdge::Left,
                ..
            }
        )
    }));
}

#[test]
fn vertical_scroll_reports_top_and_handles_negative_delta() {
    let mut area = ScrollArea::new()
        .axis(ScrollAxis::Vertical)
        .viewport(100, 100)
        .content_extent(100, 300)
        .offset(0, 80);

    let events = area.apply_scroll_action(ScrollAreaAction::ScrollBy { dx: 0, dy: -120 });
    assert_eq!(0, area.offset_y());
    assert!(events.iter().any(|event| {
        matches!(
            event,
            ScrollAreaEvent::ScrollEdgeReached {
                edge: ScrollEdge::Top,
                ..
            }
        )
    }));
}

#[test]
fn invalid_viewport_wrong_target_hidden_scrollbar_and_rejection_are_explicit() {
    let mut invalid = ScrollArea::new()
        .axis(ScrollAxis::Both)
        .viewport(0, 100)
        .content_extent(300, 300);
    assert_eq!(
        vec![ScrollAreaEvent::ScrollCommandRejected {
            target: invalid.state_id().clone(),
            reason: ScrollRejectionReason::InvalidExtent,
        }],
        invalid.apply_scroll_action(ScrollAreaAction::ScrollIntoView {
            target_rect: UiRect::new(20, 20, 40, 40),
        })
    );

    let mut area = ScrollArea::new()
        .axis(ScrollAxis::Vertical)
        .viewport(100, 100)
        .content_extent(100, 300);
    let other = ScrollArea::new();
    let ignored = area.apply_action(&UiAction::scroll_to(other.state_id().clone(), 0, 40));
    assert!(!ignored.handled);
    assert!(ignored.callback_log.is_empty());

    let visibility = area.apply_action(&UiAction::scrollbar_visibility(
        area.state_id().clone(),
        UiScrollbarVisibility::Hidden,
    ));
    assert!(visibility.handled);

    let rejected = area.apply_action(&UiAction::scroll_by(area.state_id().clone(), 20, 0));
    assert!(!rejected.handled);
    assert_eq!(
        "ScrollCommandRejected(AxisMismatch)",
        rejected.callback_log[0].after
    );
}
