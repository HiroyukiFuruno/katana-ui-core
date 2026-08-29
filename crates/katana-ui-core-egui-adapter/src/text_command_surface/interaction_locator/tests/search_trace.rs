use super::super::types::SearchTracePhase;
use super::super::{
    KucOpaqueSearchTraceContinuation, KucSearchTraceContinuationError as TraceError,
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
