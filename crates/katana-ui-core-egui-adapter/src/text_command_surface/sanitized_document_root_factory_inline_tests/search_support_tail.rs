use super::search_support::*;
use super::support::*;
use super::*;

pub(super) fn input_with_recorders_and_unit_targets(
    revision: u64,
    text_events: Rc<RefCell<Vec<(SanitizedSearchTextOperation, String)>>>,
    unit_events: Rc<RefCell<Vec<SanitizedSearchUnitOperation>>>,
    unit_targets_enabled: bool,
) -> SanitizedDocumentRootInput {
    input_with_recorders_and_unit_targets_and_state(
        revision,
        text_events,
        unit_events,
        unit_targets_enabled,
        false,
        false,
        false,
    )
}

pub(super) fn input_with_recorders_and_unit_targets_and_state(
    revision: u64,
    text_events: Rc<RefCell<Vec<(SanitizedSearchTextOperation, String)>>>,
    unit_events: Rc<RefCell<Vec<SanitizedSearchUnitOperation>>>,
    unit_targets_enabled: bool,
    match_case_state: bool,
    whole_word_state: bool,
    regex_state: bool,
) -> SanitizedDocumentRootInput {
    let text_target = |operation| {
        let events = text_events.clone();
        let _ = operation;
        SanitizedSearchTarget::from_opaque_bytes([0]).with_text_capability(move |actual, value| {
            events.borrow_mut().push((actual, value));
            Ok::<(), ()>(())
        })
    };
    let unit_target = |operation| {
        let events = unit_events.clone();
        let _ = operation;
        SanitizedSearchTarget::from_opaque_bytes([0]).with_unit_capability(move |actual| {
            events.borrow_mut().push(actual);
            Ok::<(), ()>(())
        })
    };
    let mut builder = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(search_localized())
        .match_case_state(match_case_state)
        .whole_word_state(whole_word_state)
        .regex_state(regex_state)
        .query_target(text_target(SanitizedSearchTextOperation::Query))
        .replacement_target(text_target(SanitizedSearchTextOperation::Replacement))
        .close_enabled(unit_targets_enabled)
        .close_target(unit_target(SanitizedSearchUnitOperation::Close))
        .next_enabled(unit_targets_enabled)
        .next_target(unit_target(SanitizedSearchUnitOperation::Next))
        .previous_enabled(unit_targets_enabled)
        .previous_target(unit_target(SanitizedSearchUnitOperation::Previous))
        .replace_enabled(true)
        .replace_target(text_target(SanitizedSearchTextOperation::Replace))
        .replace_all_enabled(true)
        .replace_all_target(text_target(SanitizedSearchTextOperation::ReplaceAll));
    if unit_targets_enabled {
        builder = builder
            .match_case_target(unit_target(SanitizedSearchUnitOperation::MatchCase(false)))
            .whole_word_target(unit_target(SanitizedSearchUnitOperation::WholeWord(false)))
            .regex_target(unit_target(SanitizedSearchUnitOperation::Regex(false)));
    }
    input(revision, b"document", "本文 ⭐️")
        .with_search_projection(builder.build().expect("search projection is valid"))
}

pub(super) fn run_unit_operation_case(
    operation: SanitizedSearchUnitOperation,
    input_events: impl FnOnce(&egui::Context, &mut super::SanitizedDocumentRoot) -> Vec<egui::Event>,
) {
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders_and_unit_targets(
            1,
            Rc::new(RefCell::new(Vec::new())),
            unit_events.clone(),
            true,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let events = input_events(&context, &mut root);
    let (_, frame) = run_search_root_frame(&context, &mut root, events);
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = frame
        .forward_events_once(&mut forwarder)
        .expect("unit operation forwarding succeeds");
    assert_eq!(unit_events.borrow().as_slice(), &[operation]);
    assert_eq!(
        frame.output.events().event_cardinality(),
        0,
        "sanitized search consumes its physical text event exclusively"
    );
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    assert!(!format!("{receipt:?}").contains("payload"));
}

pub(super) fn assert_search_leaf_forwarded_once(
    leaf: &str,
    frame: &SanitizedDocumentRootFrame,
    forwarder: &mut RecordingForwarder,
) {
    assert_eq!(
        frame.output.events().event_cardinality(),
        0,
        "{leaf}: root batch must not duplicate the sanitized physical search event"
    );
    assert_eq!(
        frame.search_events.borrow().as_ref().map_or(0, Vec::len),
        1,
        "{leaf}: exactly one sanitized search receipt must be retained"
    );
    let receipt = frame
        .forward_events_once(forwarder)
        .unwrap_or_else(|error| panic!("{leaf}: forwarding failed: {error:?}"));
    assert_eq!(
        receipt.event_cardinality(),
        1,
        "{leaf}: receipt cardinality"
    );
    assert_eq!(forwarder.calls, 1, "{leaf}: outer forwarder calls");

    for (name, debug) in [
        ("frame", format!("{frame:?}")),
        (
            "transport",
            forwarder
                .transport_debug
                .clone()
                .expect("forwarder captured transport Debug"),
        ),
        ("receipt", format!("{receipt:?}")),
    ] {
        for forbidden in ["日本語", "置換後", "⭐️", "👩‍💻", "かな", "opaque payload"]
        {
            assert!(
                !debug.contains(forbidden),
                "{leaf}: {name} Debug leaked `{forbidden}`: {debug}"
            );
        }
    }
    assert_eq!(
        frame.forward_events_once(forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed),
        "{leaf}: replay must be AlreadyConsumed"
    );
    assert_eq!(
        forwarder.calls, 1,
        "{leaf}: replay changed outer forwarding"
    );
}

pub(super) fn accesskit_click(node: egui::accesskit::NodeId) -> egui::Event {
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        action: egui::accesskit::Action::Click,
        target_tree: egui::accesskit::TreeId::ROOT,
        target_node: node,
        data: None,
    })
}

pub(super) type SearchTextEvents = Rc<RefCell<Vec<(SanitizedSearchTextOperation, String)>>>;
pub(super) type SearchUnitEvents = Rc<RefCell<Vec<SanitizedSearchUnitOperation>>>;

pub(super) fn recorded_search_case() -> (
    egui::Context,
    super::SanitizedDocumentRoot,
    SearchTextEvents,
    SearchUnitEvents,
) {
    let text_events = Rc::new(RefCell::new(Vec::new()));
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let root = SanitizedDocumentRootFactory::new()
        .retain(input_with_recorders(
            1,
            text_events.clone(),
            unit_events.clone(),
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    (context, root, text_events, unit_events)
}
