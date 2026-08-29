use super::super::types::TextSelectionPhase;
use super::super::{KucOpaqueTextSelectionContinuation, KucTextSelectionContinuationError};
use super::common::{
    FRAME_STEP_TWO, KUC_TEXT_SELECTION_FRAME_FIFTH, KUC_TEXT_SELECTION_FRAME_FOURTH,
    KUC_TEXT_SELECTION_FRAME_SECOND, KUC_TEXT_SELECTION_FRAME_START,
    KUC_TEXT_SELECTION_FRAME_THIRD, locator, locator_for_continue, search_text_geometry_points,
    text_selection_locator_for_continue,
};

#[test]
fn text_selection_continuation_walks_phases_and_reports_end_state_failures() {
    let start = || KucOpaqueTextSelectionContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_TEXT_SELECTION_FRAME_START,
        geometry: search_text_geometry_points(),
        phase: TextSelectionPhase::Aim,
        applied: false,
    };
    let mut input = egui::RawInput::default();

    assert!(matches!(
        start().advance(&text_selection_locator_for_continue(
            "root",
            KUC_TEXT_SELECTION_FRAME_SECOND,
        )),
        Err(KucTextSelectionContinuationError::NotApplied)
    ));

    {
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert!(matches!(
            continuation.advance(&locator(
                "other",
                KUC_TEXT_SELECTION_FRAME_SECOND,
                Vec::new()
            )),
            Err(KucTextSelectionContinuationError::RootMismatch)
        ));
    }

    {
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert!(matches!(
            continuation.advance(&locator(
                "root",
                KUC_TEXT_SELECTION_FRAME_SECOND + FRAME_STEP_TWO,
                Vec::new(),
            )),
            Err(KucTextSelectionContinuationError::FrameDiscontinuity)
        ));
    }

    let mut next = start();
    assert_eq!(next.apply_to_raw_input_once(&mut input), Ok(()));
    let mut next = next
        .advance(&text_selection_locator_for_continue(
            "root",
            KUC_TEXT_SELECTION_FRAME_SECOND,
        ))
        .expect("next phase")
        .expect("midpoint phase");
    assert_eq!(next.apply_to_raw_input_once(&mut input), Ok(()));

    let mut next = next
        .advance(&text_selection_locator_for_continue(
            "root",
            KUC_TEXT_SELECTION_FRAME_THIRD,
        ))
        .expect("next phase")
        .expect("end phase");
    assert_eq!(next.apply_to_raw_input_once(&mut input), Ok(()));

    let mut next = next
        .advance(&text_selection_locator_for_continue(
            "root",
            KUC_TEXT_SELECTION_FRAME_FOURTH,
        ))
        .expect("next phase")
        .expect("release phase");
    assert_eq!(next.apply_to_raw_input_once(&mut input), Ok(()));

    let mut release = KucOpaqueTextSelectionContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_TEXT_SELECTION_FRAME_FOURTH,
        geometry: search_text_geometry_points(),
        phase: TextSelectionPhase::Release,
        applied: false,
    };
    assert_eq!(release.apply_to_raw_input_once(&mut input), Ok(()));
    assert!(
        release
            .advance(&text_selection_locator_for_continue(
                "root",
                KUC_TEXT_SELECTION_FRAME_FIFTH,
            ))
            .is_ok()
    );

    let new_release = || KucOpaqueTextSelectionContinuation {
        root_identity: "root".to_owned(),
        frame_serial: KUC_TEXT_SELECTION_FRAME_FOURTH,
        geometry: search_text_geometry_points(),
        phase: TextSelectionPhase::Release,
        applied: true,
    };
    let mut locator = text_selection_locator_for_continue("root", KUC_TEXT_SELECTION_FRAME_FIFTH);
    locator.selection_established = false;
    assert!(matches!(
        new_release().advance(&locator),
        Err(KucTextSelectionContinuationError::SelectionNotEstablished)
    ));

    let locator = locator_for_continue(
        "root",
        KUC_TEXT_SELECTION_FRAME_FIFTH,
        Vec::new(),
        true,
        false,
        true,
        false,
    );
    assert!(matches!(
        new_release().advance(&locator),
        Err(KucTextSelectionContinuationError::FloatingNotVisible)
    ));
}
