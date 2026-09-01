use super::command_support::assert_command_forwarded_once;
use super::root_frame_support::*;
use super::support::*;
use super::*;
#[test]
fn physical_text_selection_controls_floating_surface_lifecycle() {
    use crate::egui::text_command_surface::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget,
    };

    let top_calls = Rc::new(RefCell::new(0));
    let floating_calls = Rc::new(RefCell::new(0));
    let top_projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "トップ操作 日本語 ⭐️")
            .item(SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes(b"top-target-secret")
                    .with_unit_capability({
                        let calls = top_calls.clone();
                        move || {
                            *calls.borrow_mut() += 1;
                            Ok::<(), ()>(())
                        }
                    }),
                0,
                "トップ 日本語 ⭐️",
            ))]);
    let floating_projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(0, "選択操作 日本語 ⭐️")
            .item(SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes(b"floating-target-secret")
                    .with_unit_capability({
                        let calls = floating_calls.clone();
                        move || {
                            *calls.borrow_mut() += 1;
                            Ok::<(), ()>(())
                        }
                    }),
                0,
                "太字 日本語 ⭐️",
            ))]);
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(
            input(1, b"floating-document", "本文 日本語 ⭐️👩‍💻")
                .with_command_projection(top_projection)
                .with_floating_command_projection(floating_projection),
        )
        .expect("retain succeeds");
    let context = egui::Context::default();

    let (_, initial) = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
    );
    assert!(
        initial
            .output
            .evidence_text
            .record
            .frame
            .selection
            .range
            .is_collapsed()
    );
    assert!(
        initial
            .output
            .floating
            .as_ref()
            .is_none_or(|value| value.record.is_none())
    );

    let text_frame = &initial.output.evidence_text.record.frame;
    let content_bounds = text_frame.content_bounds;
    assert!(content_bounds.width > 24);
    let start = egui::pos2(
        content_bounds.x as f32 + 8.0,
        content_bounds.y as f32 + content_bounds.height as f32 / 2.0,
    );
    let midpoint = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 / 2.0,
        start.y,
    );
    let end = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 - 8.0,
        start.y,
    );

    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, true)]);
    let (_, dragging) = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(midpoint)],
    );
    let (_, _selected) =
        run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(end)]);
    let (_, released) =
        run_root_frame_events(&context, &mut root, vec![pointer_button(end, false)]);
    let selection = &released.output.evidence_text.record.frame.selection.range;
    let ordered_selection = selection.ordered();
    assert!(ordered_selection.start < ordered_selection.end);
    assert!(
        dragging
            .output
            .evidence_text
            .events
            .iter()
            .any(|event| matches!(
                event,
                crate::text_surface::TextSurfaceEvent::SelectionChanged { .. }
            ))
    );
    assert!(
        released
            .output
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_some()
    );

    let escaped = run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::Escape)]).1;
    assert!(
        escaped
            .output
            .floating
            .as_ref()
            .is_none_or(|value| value.record.is_none())
    );
    let escaped_next = run_root_frame_events(&context, &mut root, Vec::new()).1;
    assert!(
        escaped_next
            .output
            .evidence_text
            .record
            .frame
            .accessibility
            .root
            .focused
    );

    let collapsed =
        run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::ArrowRight)]).1;
    let collapsed_selection = &collapsed.output.evidence_text.record.frame.selection.range;
    assert!(collapsed_selection.is_collapsed());
    assert!(
        collapsed
            .output
            .floating
            .as_ref()
            .is_none_or(|value| value.record.is_none())
    );

    let text_frame = &collapsed.output.evidence_text.record.frame;
    let content_bounds = text_frame.content_bounds;
    let start = egui::pos2(
        content_bounds.x as f32 + 8.0,
        content_bounds.y as f32 + content_bounds.height as f32 / 2.0,
    );
    let midpoint = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 / 2.0,
        start.y,
    );
    let end = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 - 8.0,
        start.y,
    );
    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, true)]);
    let _ = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(midpoint)],
    );
    let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(end)]);
    let (_, reselected) =
        run_root_frame_events(&context, &mut root, vec![pointer_button(end, false)]);
    let floating = reselected
        .output
        .floating
        .as_ref()
        .and_then(|value| value.record.as_ref())
        .expect("reselection opens floating surface");
    let floating_action = reselected
        .floating_action_rects()
        .first()
        .copied()
        .expect("floating action bounds are available only in the test frame");
    let floating_point = egui::pos2(
        floating_action.x as f32 + floating_action.width as f32 / 2.0,
        floating_action.y as f32 + floating_action.height as f32 / 2.0,
    );
    let _ = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(floating_point)],
    );
    let _ = run_root_frame_events(
        &context,
        &mut root,
        vec![pointer_button(floating_point, true)],
    );
    let floating_clicked = run_root_frame_events(
        &context,
        &mut root,
        vec![pointer_button(floating_point, false)],
    )
    .1;
    let mut floating_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    assert_command_forwarded_once(&floating_clicked, &floating_calls, &mut floating_forwarder);
    assert_eq!(*top_calls.borrow(), 0);
    assert_eq!(*floating_calls.borrow(), 1);
    let public_record_debug = format!("{:?}", floating_clicked.record());
    for forbidden in ["panel_bounds", "actions", "floating-target-secret"] {
        assert!(!public_record_debug.contains(forbidden));
    }
    for forbidden in [
        "floating-target-secret",
        "top-target-secret",
        "太字 日本語",
        "トップ 日本語",
        "選択操作 日本語",
    ] {
        assert!(!format!("{floating_clicked:?}").contains(forbidden));
        assert!(
            !floating_forwarder
                .transport_debug
                .as_deref()
                .expect("transport debug")
                .contains(forbidden)
        );
    }
    let text_bounds = reselected.output.evidence_text.record.frame.content_bounds;
    let outside = egui::pos2(
        text_bounds.x.saturating_sub(1) as f32,
        text_bounds.y.saturating_sub(1) as f32,
    );
    assert!((outside.x.round() as i32) < text_bounds.x);
    assert!((outside.y.round() as i32) < text_bounds.y);
    assert!(
        (outside.x.round() as i32) < floating.panel_bounds.x
            || (outside.x.round() as i32)
                >= floating
                    .panel_bounds
                    .x
                    .saturating_add_unsigned(floating.panel_bounds.width)
    );
    assert!(
        (outside.y.round() as i32) < floating.panel_bounds.y
            || (outside.y.round() as i32)
                >= floating
                    .panel_bounds
                    .y
                    .saturating_add_unsigned(floating.panel_bounds.height)
    );

    let outside_dismissed =
        run_root_frame_events(&context, &mut root, vec![pointer_button(outside, true)]).1;
    assert!(
        outside_dismissed
            .output
            .floating
            .as_ref()
            .is_none_or(|value| value.record.is_none())
    );
    let outside_dismissed_next = run_root_frame_events(&context, &mut root, Vec::new()).1;
    assert!(
        outside_dismissed_next
            .output
            .evidence_text
            .record
            .frame
            .accessibility
            .root
            .focused
    );
}
