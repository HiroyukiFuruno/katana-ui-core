use super::root_event_channels::*;
use super::root_frame_support::*;
use super::search_support::*;
use super::support::*;
use super::*;

#[test]
fn show_returns_a_closed_record_and_forwards_events_only_once() {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input(1, b"document", "日本語 ⭐️"))
        .expect("retain succeeds");
    let frame = run_root_frame_events(&egui::Context::default(), &mut root, Vec::new()).1;
    assert_eq!(frame.record().revision(), 1);
    assert!(frame.record().dimensions().width() > 0);
    assert!(frame.record().dimensions().height() > 0);
    assert_eq!(frame.record().rgba_hash().len(), 64);
    assert_eq!(frame.record().accessibility_snapshot_hash().len(), 64);
    let mut forwarder = recording_forwarder(false);
    let receipt = frame
        .forward_events_once(&mut forwarder)
        .expect("first forwarding succeeds");
    assert_eq!(forwarder.calls, 1);
    assert!(receipt.consumed_once());
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
}

#[test]
fn raw_ime_search_at_current_root_renders_and_forwards_one_opaque_event() -> Result<(), String> {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_search(1)?)
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial_frame = run_search_root_frame(&context, &mut root, Vec::new()).1;
    let query_bounds = focus_search(&context, &mut root, "検索語 ⭐️");
    assert!(query_bounds.width() > 0.0 && query_bounds.height() > 0.0);
    let mut no_search_root = factory
        .retain(input(1, b"document", "本文 ⭐️"))
        .expect("retain without search succeeds");
    let no_search_frame =
        run_search_root_frame(&egui::Context::default(), &mut no_search_root, Vec::new()).1;
    assert_ne!(
        initial_frame.record().record_hash(),
        no_search_frame.record().record_hash()
    );
    let focused = run_search_root_frame(&context, &mut root, Vec::new()).1;
    let preedit = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".to_string(),
            active_range_chars: None,
        })],
    )
    .1;
    assert_eq!(preedit.output.events().event_cardinality(), 0);
    assert_ne!(
        focused.record().record_hash(),
        preedit.record().record_hash()
    );
    let committed = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    )
    .1;
    assert_eq!(committed.output.events().event_cardinality(), 0);
    assert_eq!(
        committed
            .search_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        1
    );
    let mut forwarder = recording_forwarder(false);
    let receipt = committed
        .forward_events_once(&mut forwarder)
        .expect("one-shot search forwarding succeeds");
    assert_eq!((forwarder.calls, receipt.event_cardinality()), (1, 1));
    assert_debug_omits(&format!("{committed:?}"), &["検索語 ⭐️", "かな", "payload"]);
    let transport = forwarder
        .transport_debug
        .as_deref()
        .expect("transport debug exists");
    assert_debug_omits(transport, &["検索語 ⭐️", "かな", "opaque payload"]);
    assert!(transport.contains("<opaque>"));
    assert_debug_omits(
        &format!("{receipt:?}"),
        &["検索語 ⭐️", "かな", "opaque payload"],
    );
    assert_eq!(
        committed.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    Ok(())
}

#[test]
fn physical_ime_commit_routes_exact_text_once_without_debug_leakage() {
    let text_events = Rc::new(RefCell::new(Vec::new()));
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input_with_recorders(
            1,
            text_events.clone(),
            Rc::new(RefCell::new(Vec::new())),
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    focus_search(&context, &mut root, "検索語 ⭐️");
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".to_string(),
            active_range_chars: None,
        })],
    );
    let committed = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    )
    .1;
    let mut forwarder = recording_forwarder(false);
    let receipt = committed
        .forward_events_once(&mut forwarder)
        .expect("physical IME event forwards");
    assert_eq!(
        text_events.borrow().as_slice(),
        &[(SanitizedSearchTextOperation::Query, "⭐️".to_string())]
    );
    assert_eq!(
        (
            committed.output.events().event_cardinality(),
            receipt.event_cardinality(),
            forwarder.calls
        ),
        (0, 1, 1)
    );
    for debug in [
        format!("{committed:?}"),
        format!("{receipt:?}"),
        forwarder
            .transport_debug
            .clone()
            .expect("transport debug exists"),
    ] {
        assert_debug_omits(&debug, &["⭐️", "👩‍💻"]);
    }
    assert_eq!(
        committed.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
}

#[test]
fn sanitized_physical_search_callback_rejection_is_opaque_and_consumed() {
    verify_rejecting_search_callbacks();
}

#[test]
fn sanitized_physical_search_frame_is_stale_after_newer_same_identity_sync() {
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
    focus_search(&context, &mut root, "検索語 ⭐️");
    let frame = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語 ⭐️👩‍💻".to_string(),
        ))],
    )
    .1;
    assert_eq!(
        (
            frame.output.events().event_cardinality(),
            frame.search_events.borrow().as_ref().map_or(0, Vec::len)
        ),
        (0, 1)
    );
    assert!(
        root.synchronize(input_with_rejecting_recorders(
            2,
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0))
        ))
        .expect("newer same-identity synchronization succeeds")
    );
    let mut forwarder = recording_forwarder(false);
    for _ in 0..2 {
        assert_eq!(
            frame.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::StaleFrame)
        );
    }
    assert_eq!(
        (*text_calls.borrow(), *unit_calls.borrow(), forwarder.calls),
        (0, 0, 0)
    );
    assert!(frame.search_events.borrow().is_some());
    assert_debug_omits(&format!("{frame:?}"), &["日本語 ⭐️👩‍💻", "payload"]);
}

#[test]
fn forwarder_error_consumes_root_tab_and_search_batches() -> Result<(), String> {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input_with_search(1)?)
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    focus_search(&context, &mut root, "検索語 ⭐️");
    let frame = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    )
    .1;
    assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 1);
    assert!(frame.tab_closed_events.borrow().is_some());
    let mut forwarder = recording_forwarder(true);
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::Forwarder(()))
    );
    assert_eq!(forwarder.calls, 1);
    assert!(frame.tab_closed_events.borrow().is_none() && frame.search_events.borrow().is_none());
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    Ok(())
}

#[test]
fn public_show_retains_tab_event_without_exposing_it_in_public_frame_data() {
    verify_public_tab_forwarding();
}
