use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::event::UiEvent;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::layout::{
    ScrollArea, ScrollAreaAction, ScrollAreaEvent, ScrollAxis, ScrollEdge, ScrollRejectionReason,
    ScrollbarPlacement, ScrollbarVisibility,
};
use katana_ui_core::render_model::{
    UiNodeKind, UiRect, UiScrollAreaAxis, UiScrollbarPlacement, UiScrollbarVisibility, UiTree,
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
}
