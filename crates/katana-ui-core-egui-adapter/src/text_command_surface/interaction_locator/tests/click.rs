use super::super::{
    KucInteractionActionClass, KucInteractionLocatorError, KucInteractionSelector,
    KucOpaqueClickContinuationError,
};
use super::common::{
    CLICK_EVENT_COUNT_THREE, CLICK_FRAME_PRESS, CLICK_FRAME_RELEASE, CLICK_FRAME_SOURCE,
    KUC_FRAME_STEP_ONE, click_geometry_locator, click_selector, locator, target,
};

#[test]
fn click_continuation_applies_once_and_advances_until_release() {
    let start = || {
        locator(
            "root",
            CLICK_FRAME_SOURCE,
            vec![target("open", KucInteractionActionClass::Toolbar, false)],
        )
        .begin_click(click_selector("open", KucInteractionActionClass::Toolbar))
        .expect("click continuation started")
    };

    {
        let mut input = egui::RawInput::default();
        let not_applied = start();
        assert!(matches!(
            not_applied.advance(&click_geometry_locator("root", CLICK_FRAME_PRESS)),
            Err(KucOpaqueClickContinuationError::NotApplied)
        ));
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert_eq!(input.events.len(), 1);
        assert_eq!(
            continuation.apply_to_raw_input_once(&mut input),
            Err(KucOpaqueClickContinuationError::AlreadyApplied)
        );
    }

    {
        let mut input = egui::RawInput::default();
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert_eq!(input.events.len(), 1);
        assert!(matches!(
            continuation.advance(&locator(
                "other",
                CLICK_FRAME_PRESS,
                vec![target("open", KucInteractionActionClass::Toolbar, false)],
            )),
            Err(KucOpaqueClickContinuationError::RootMismatch)
        ));
    }

    {
        let mut input = egui::RawInput::default();
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert_eq!(input.events.len(), 1);
        assert!(matches!(
            continuation.advance(&locator(
                "root",
                CLICK_FRAME_RELEASE,
                vec![target("open", KucInteractionActionClass::Toolbar, false)],
            )),
            Err(KucOpaqueClickContinuationError::FrameDiscontinuity)
        ));
    }

    {
        let mut input = egui::RawInput::default();
        let mut continuation = start();
        assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
        assert_eq!(input.events.len(), 1);
        assert!(matches!(
            continuation.advance(&locator(
                "root",
                CLICK_FRAME_PRESS,
                vec![target("open", KucInteractionActionClass::Toolbar, true)],
            )),
            Err(KucOpaqueClickContinuationError::Interaction(
                KucInteractionLocatorError::Disabled
            ))
        ));
    }

    let mut input = egui::RawInput::default();

    let mut next = start();
    assert_eq!(next.apply_to_raw_input_once(&mut input), Ok(()));
    let mut next = next
        .advance(&locator(
            "root",
            CLICK_FRAME_PRESS,
            vec![target("open", KucInteractionActionClass::Toolbar, false)],
        ))
        .expect("press phase")
        .expect("click press");
    assert_eq!(next.apply_to_raw_input_once(&mut input), Ok(()));
    assert_eq!(input.events.len(), 2);

    let mut final_continuation = next
        .advance(&locator(
            "root",
            CLICK_FRAME_RELEASE,
            vec![target("open", KucInteractionActionClass::Toolbar, false)],
        ))
        .expect("release phase")
        .expect("click release");
    assert_eq!(
        final_continuation.apply_to_raw_input_once(&mut input),
        Ok(())
    );
    assert_eq!(input.events.len(), CLICK_EVENT_COUNT_THREE);
    assert!(
        final_continuation
            .advance(&click_geometry_locator(
                "root",
                CLICK_FRAME_RELEASE + KUC_FRAME_STEP_ONE
            ))
            .is_ok()
    );
}

#[test]
fn click_continuation_reports_missing_action_if_target_disappears() {
    let source = locator(
        "root",
        KUC_FRAME_STEP_ONE,
        vec![target("open", KucInteractionActionClass::Toolbar, false)],
    );
    let mut continuation = source
        .begin_click(KucInteractionSelector::new(
            "open",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("click continuation started");
    let mut input = egui::RawInput::default();

    assert_eq!(continuation.apply_to_raw_input_once(&mut input), Ok(()));
    let next_frame = locator("root", KUC_FRAME_STEP_ONE + 1, Vec::new());
    assert!(matches!(
        continuation.advance(&next_frame),
        Err(KucOpaqueClickContinuationError::Interaction(
            KucInteractionLocatorError::Missing
        ))
    ));
}
