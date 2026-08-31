use super::super::types::SearchTracePhase;
use super::super::{
    KucInteractionLocatorError, KucInteractionRequestError, KucOpaqueSearchTraceContinuation,
    KucSearchTraceContinuationError as TraceError,
};
use super::common::{
    KUC_SEARCH_TRACE_FRAME_CLOSE, KUC_SEARCH_TRACE_FRAME_COMMIT, KUC_SEARCH_TRACE_FRAME_FOCUS,
    KUC_SEARCH_TRACE_FRAME_NEXT, KUC_SEARCH_TRACE_FRAME_PREEDIT, KUC_SEARCH_TRACE_FRAME_PREVIOUS,
    KUC_SEARCH_TRACE_FRAME_QUERY, KUC_SEARCH_TRACE_FRAME_VERIFY, search_locator_for_continue,
};

#[test]
fn search_trace_continuation_walks_phases_and_reports_error_states() {
    let mut input = egui::RawInput::default();

    let not_applied = KucOpaqueSearchTraceContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_SEARCH_TRACE_FRAME_QUERY,
        phase: SearchTracePhase::Preedit,
        applied: false,
    };
    assert!(matches!(
        not_applied.advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_FOCUS,
            true,
            true
        )),
        Err(TraceError::NotApplied)
    ));

    let mut already_applied = KucOpaqueSearchTraceContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_SEARCH_TRACE_FRAME_QUERY,
        phase: SearchTracePhase::Preedit,
        applied: false,
    };
    assert_eq!(already_applied.apply_to_raw_input_once(&mut input), Ok(()));
    assert_eq!(
        already_applied.apply_to_raw_input_once(&mut input),
        Err(TraceError::AlreadyApplied)
    );

    {
        let mut trace =
            search_locator_for_continue("root", KUC_SEARCH_TRACE_FRAME_QUERY, false, false)
                .begin_search_trace()
                .expect("search trace starts from query");
        assert_eq!(trace.apply_to_raw_input_once(&mut input), Ok(()));
        assert!(matches!(
            trace.advance(&search_locator_for_continue(
                "root",
                KUC_SEARCH_TRACE_FRAME_FOCUS,
                false,
                true
            )),
            Err(TraceError::FocusNotEstablished)
        ));
    }

    let mut step = search_locator_for_continue("root", KUC_SEARCH_TRACE_FRAME_QUERY, false, false)
        .begin_search_trace()
        .expect("search trace starts from query");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_FOCUS,
            true,
            true,
        ))
        .expect("search preedit")
        .expect("preedit phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_PREEDIT,
            true,
            true,
        ))
        .expect("search commit")
        .expect("commit phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_COMMIT,
            true,
            true,
        ))
        .expect("search next")
        .expect("next phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_NEXT,
            true,
            true,
        ))
        .expect("search previous")
        .expect("previous phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_PREVIOUS,
            true,
            true,
        ))
        .expect("search close")
        .expect("close phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));

    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_CLOSE,
            true,
            true,
        ))
        .expect("search verification")
        .expect("verify-closed phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));

    assert!(matches!(
        step.advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_VERIFY,
            true,
            true
        )),
        Err(TraceError::CloseNotApplied)
    ));

    let mut step = search_locator_for_continue("root", KUC_SEARCH_TRACE_FRAME_QUERY, false, false)
        .begin_search_trace()
        .expect("search trace starts from query");
    assert_eq!(
        step.apply_to_raw_input_once(&mut egui::RawInput::default()),
        Ok(())
    );
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_FOCUS,
            true,
            true,
        ))
        .expect("search preedit")
        .expect("preedit phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_PREEDIT,
            true,
            true,
        ))
        .expect("search commit")
        .expect("commit phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_COMMIT,
            true,
            true,
        ))
        .expect("search next")
        .expect("next phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_NEXT,
            true,
            true,
        ))
        .expect("search previous")
        .expect("previous phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));
    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_PREVIOUS,
            true,
            true,
        ))
        .expect("search close")
        .expect("close phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));

    step = step
        .advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_CLOSE,
            true,
            false,
        ))
        .expect("search verification")
        .expect("verify-closed phase");
    assert_eq!(step.apply_to_raw_input_once(&mut input), Ok(()));

    assert!(matches!(
        step.advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_VERIFY,
            true,
            false
        )),
        Ok(None)
    ));
}

#[test]
fn search_trace_propagates_an_already_consumed_inner_request() {
    let mut trace = search_locator_for_continue("root", KUC_SEARCH_TRACE_FRAME_QUERY, false, false)
        .begin_search_trace()
        .expect("search trace starts from query");
    let SearchTracePhase::Focus(request) = &mut trace.phase else {
        panic!("search trace must start with a focus request");
    };
    request
        .apply_to_raw_input_once(&mut egui::RawInput::default())
        .expect("inner focus request applies once");

    assert_eq!(
        trace.apply_to_raw_input_once(&mut egui::RawInput::default()),
        Err(TraceError::Request(
            KucInteractionRequestError::AlreadyQueued
        ))
    );
}

#[test]
fn search_trace_rejects_frame_gaps_and_missing_control_routes() {
    let wrong_root = KucOpaqueSearchTraceContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_SEARCH_TRACE_FRAME_QUERY,
        phase: SearchTracePhase::Preedit,
        applied: true,
    };
    assert!(matches!(
        wrong_root.advance(&search_locator_for_continue(
            "other",
            KUC_SEARCH_TRACE_FRAME_FOCUS,
            true,
            true,
        )),
        Err(TraceError::RootMismatch)
    ));

    let discontinuous = KucOpaqueSearchTraceContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_SEARCH_TRACE_FRAME_QUERY,
        phase: SearchTracePhase::Preedit,
        applied: true,
    };
    assert!(matches!(
        discontinuous.advance(&search_locator_for_continue(
            "root",
            KUC_SEARCH_TRACE_FRAME_PREEDIT,
            true,
            true,
        )),
        Err(TraceError::FrameDiscontinuity)
    ));

    let missing_next = KucOpaqueSearchTraceContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_SEARCH_TRACE_FRAME_QUERY,
        phase: SearchTracePhase::Commit,
        applied: true,
    };
    assert!(matches!(
        missing_next.advance(&super::common::locator(
            "root",
            KUC_SEARCH_TRACE_FRAME_FOCUS,
            Vec::new(),
        )),
        Err(TraceError::Unavailable)
    ));

    let next_request =
        search_locator_for_continue("root", KUC_SEARCH_TRACE_FRAME_QUERY, true, true)
            .search_control_request("next")
            .expect("next request exists");
    let missing_previous = KucOpaqueSearchTraceContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_SEARCH_TRACE_FRAME_QUERY,
        phase: SearchTracePhase::Next(next_request),
        applied: true,
    };
    assert!(matches!(
        missing_previous.advance(&super::common::locator(
            "root",
            KUC_SEARCH_TRACE_FRAME_FOCUS,
            Vec::new(),
        )),
        Err(TraceError::Unavailable)
    ));

    let previous_request =
        search_locator_for_continue("root", KUC_SEARCH_TRACE_FRAME_QUERY, true, true)
            .search_control_request("previous")
            .expect("previous request exists");
    let missing_close = KucOpaqueSearchTraceContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_SEARCH_TRACE_FRAME_QUERY,
        phase: SearchTracePhase::Previous(previous_request),
        applied: true,
    };
    assert!(matches!(
        missing_close.advance(&super::common::locator(
            "root",
            KUC_SEARCH_TRACE_FRAME_FOCUS,
            Vec::new(),
        )),
        Err(TraceError::Unavailable)
    ));

    let locator = search_locator_for_continue("root", KUC_SEARCH_TRACE_FRAME_QUERY, true, true);
    locator
        .begin_search_trace()
        .expect("first search trace owns the query request");
    assert!(matches!(
        locator.begin_search_trace(),
        Err(TraceError::Interaction(
            KucInteractionLocatorError::Duplicate
        ))
    ));
}
