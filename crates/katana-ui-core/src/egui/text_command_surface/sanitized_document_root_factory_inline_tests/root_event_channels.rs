use super::root_frame_support::*;
use super::search_support::*;
use super::search_support_tail::*;
use super::support::*;
use super::*;

pub(super) fn recording_forwarder(reject_forwarding: bool) -> RecordingForwarder {
    RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding,
    }
}

pub(super) fn focus_search(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    label: &str,
) -> egui::Rect {
    let output = run_search_root_frame(context, root, Vec::new()).0;
    let bounds = accesskit_bounds(&output, egui::accesskit::Role::TextInput, label);
    for pressed in [true, false] {
        let frame = run_search_root_frame(
            context,
            root,
            vec![pointer_button(bounds.center(), pressed)],
        )
        .1;
        assert_eq!(frame.output.events().event_cardinality(), 0);
    }
    bounds
}

pub(super) fn assert_debug_omits(debug: &str, forbidden: &[&str]) {
    for value in forbidden {
        assert!(!debug.contains(value), "Debug leaked `{value}`: {debug}");
    }
}

pub(super) fn verify_public_tab_forwarding() {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input_with_tabs(1))
        .expect("retain succeeds");
    let context = egui::Context::default();
    let first = run_root_frame_events(&context, &mut root, Vec::new()).1;
    let mut root_forwarder = recording_forwarder(false);
    let root_receipt = first
        .forward_events_once(&mut root_forwarder)
        .expect("root-only forwarding succeeds");
    assert_eq!(
        (root_forwarder.calls, root_receipt.event_cardinality()),
        (1, 0)
    );
    let target = first
        .tab_rects()
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("second tab rect exists");
    let pressed = run_root_frame_events(&context, &mut root, vec![pointer_button(target, true)]).1;
    assert_eq!(pressed.tab_closed_event_count(), 0);
    let mut no_event_forwarder = recording_forwarder(false);
    let no_event_receipt = pressed
        .forward_events_once(&mut no_event_forwarder)
        .expect("no-event forwarding succeeds");
    assert_eq!(no_event_forwarder.calls, 1);
    let released =
        run_root_frame_events(&context, &mut root, vec![pointer_button(target, false)]).1;
    assert_eq!(released.tab_closed_event_count(), 1);
    let mut forwarder = recording_forwarder(false);
    let receipt = released
        .forward_events_once(&mut forwarder)
        .expect("released tab event forwarding succeeds");
    assert_eq!((forwarder.calls, receipt.event_cardinality()), (1, 1));
    assert_ne!(
        root_receipt.event_batch_fingerprint(),
        receipt.event_batch_fingerprint()
    );
    assert_ne!(
        no_event_receipt.event_batch_fingerprint(),
        receipt.event_batch_fingerprint()
    );
    assert_ne!(
        root_receipt.correlation_fingerprint(),
        receipt.correlation_fingerprint()
    );
    assert_ne!(
        no_event_receipt.correlation_fingerprint(),
        receipt.correlation_fingerprint()
    );
    assert_debug_omits(
        &format!("{released:?}"),
        &["次の文書", "sanitized-tab-0-1", "payload"],
    );
    let transport = forwarder
        .transport_debug
        .as_deref()
        .expect("forwarder recorded transport Debug");
    assert_debug_omits(
        transport,
        &["次の文書", "sanitized-tab-0-1", "opaque payload"],
    );
    assert!(transport.contains("<opaque>"));
    assert_debug_omits(
        &format!("{receipt:?}"),
        &["次の文書", "sanitized-tab-0-1", "opaque payload"],
    );
    assert_eq!(
        released.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!((forwarder.calls, released.record().revision()), (1, 1));
}

pub(super) fn verify_rejecting_search_callbacks() {
    for operation in ["query", "replacement", "option"] {
        let text_calls = Rc::new(RefCell::new(0));
        let unit_calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(input_with_rejecting_recorders(
                1,
                text_calls.clone(),
                unit_calls.clone(),
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        context.enable_accesskit();
        let initial_output = run_search_root_frame(&context, &mut root, Vec::new()).0;
        let event = if operation == "option" {
            let node = accesskit_button(&initial_output, "大文字小文字 ⭐️").0;
            vec![accesskit_click(node)]
        } else {
            let (label, text) = if operation == "query" {
                ("検索語 ⭐️", "日本語 ⭐️👩‍💻")
            } else {
                ("置換 ⭐️", "置換後 ⭐️👩‍💻")
            };
            focus_search(&context, &mut root, label);
            vec![egui::Event::Ime(egui::ImeEvent::Commit(text.to_string()))]
        };
        let frame = run_search_root_frame(&context, &mut root, event).1;
        assert_eq!(
            (
                frame.output.events().event_cardinality(),
                frame.search_events.borrow().as_ref().map_or(0, Vec::len)
            ),
            (0, 1)
        );
        let mut forwarder = RetainingForwarder {
            calls: 0,
            transport_debug: None,
            transport: None,
        };
        assert!(
            frame.forward_events_once(&mut forwarder).is_ok(),
            "{operation} outer forward"
        );
        assert_eq!(
            (*text_calls.borrow(), *unit_calls.borrow(), forwarder.calls),
            (0, 0, 1)
        );
        assert!(frame.search_events.borrow().is_none());
        assert_debug_omits(
            &format!("{frame:?}"),
            &["日本語 ⭐️👩‍💻", "置換後 ⭐️👩‍💻", "payload"],
        );
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect),
            "{operation} host dispatch rejection"
        );
        assert_eq!(*text_calls.borrow(), usize::from(operation != "option"));
        assert_eq!(*unit_calls.borrow(), usize::from(operation == "option"));
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
        );
        assert_eq!(
            frame.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
        );
        assert_eq!(forwarder.calls, 1);
    }
}

#[test]
fn missing_tab_event_batch_fails_closed_without_calling_forwarder() {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input(1, b"document", "本文"))
        .expect("retain succeeds");
    let frame = run_root_frame_events(&egui::Context::default(), &mut root, Vec::new()).1;
    let _ = frame.tab_closed_events.borrow_mut().take();
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };

    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::InconsistentTabEventBatch)
    );
    assert_eq!(forwarder.calls, 0);
}

#[test]
fn consumed_child_event_channels_fail_closed_through_real_root_forwarding() {
    for channel in ["search", "command", "context menu"] {
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(input(1, b"event-channel", "本文 ⭐️"))
            .expect("retain succeeds");
        let context = egui::Context::default();
        let frame = run_root_frame_events(&context, &mut root, Vec::new()).1;
        match channel {
            "search" => {
                let _ = frame.search_events.borrow_mut().take();
            }
            "command" => {
                let _ = frame.command_events.borrow_mut().take();
            }
            "context menu" => {
                let _ = frame.context_menu_events.borrow_mut().take();
            }
            _ => unreachable!(),
        }
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };

        let result = frame.forward_events_once(&mut forwarder);
        assert!(
            matches!(
                (channel, result),
                (
                    "search",
                    Err(SanitizedDocumentRootEventForwardError::InconsistentSearchEventBatch)
                ) | (
                    "command",
                    Err(SanitizedDocumentRootEventForwardError::InconsistentCommandEventBatch)
                ) | (
                    "context menu",
                    Err(SanitizedDocumentRootEventForwardError::InconsistentContextMenuEventBatch)
                )
            ),
            "{channel} channel must fail through its typed consistency error"
        );
        assert_eq!(forwarder.calls, 0);
    }
}
