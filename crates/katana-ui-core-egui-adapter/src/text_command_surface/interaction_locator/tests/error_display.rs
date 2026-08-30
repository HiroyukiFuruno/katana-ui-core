use super::super::{
    KucInteractionLocatorError, KucInteractionRequestError, KucOpaqueClickContinuationError,
    KucSearchTraceContinuationError, KucTextSelectionContinuationError,
};

#[test]
fn every_interaction_error_variant_has_a_stable_display_message() {
    for (error, expected) in [
        (
            KucInteractionLocatorError::Missing,
            "interaction action is missing",
        ),
        (
            KucInteractionLocatorError::Disabled,
            "interaction action is disabled",
        ),
        (
            KucInteractionLocatorError::Hidden,
            "interaction action is hidden",
        ),
        (
            KucInteractionLocatorError::Ambiguous,
            "interaction action is ambiguous",
        ),
        (
            KucInteractionLocatorError::Duplicate,
            "interaction action is duplicated",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }

    for (error, expected) in [
        (
            KucInteractionRequestError::RootMismatch,
            "interaction request belongs to another root",
        ),
        (
            KucInteractionRequestError::Stale,
            "interaction request is stale",
        ),
        (
            KucInteractionRequestError::Duplicate,
            "interaction request is duplicated",
        ),
        (
            KucInteractionRequestError::AlreadyQueued,
            "interaction request is already queued",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }

    for (error, expected) in [
        (
            KucTextSelectionContinuationError::Unavailable,
            "current root frame has no selectable text area",
        ),
        (
            KucTextSelectionContinuationError::RootMismatch,
            "text-selection continuation belongs to another root",
        ),
        (
            KucTextSelectionContinuationError::FrameDiscontinuity,
            "text-selection continuation requires the next root frame",
        ),
        (
            KucTextSelectionContinuationError::NotApplied,
            "text-selection continuation was not applied",
        ),
        (
            KucTextSelectionContinuationError::AlreadyApplied,
            "text-selection continuation was already applied",
        ),
        (
            KucTextSelectionContinuationError::SelectionNotEstablished,
            "text-selection continuation did not establish selection",
        ),
        (
            KucTextSelectionContinuationError::FloatingNotVisible,
            "text-selection continuation did not open floating output",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn continuation_error_wrappers_preserve_their_failure_class() {
    for (error, expected) in [
        (
            KucSearchTraceContinuationError::Unavailable,
            "search trace is unavailable",
        ),
        (
            KucSearchTraceContinuationError::RootMismatch,
            "search trace belongs to another root",
        ),
        (
            KucSearchTraceContinuationError::FrameDiscontinuity,
            "search trace requires the next root frame",
        ),
        (
            KucSearchTraceContinuationError::NotApplied,
            "search trace step was not applied",
        ),
        (
            KucSearchTraceContinuationError::AlreadyApplied,
            "search trace step was already applied",
        ),
        (
            KucSearchTraceContinuationError::FocusNotEstablished,
            "search query focus was not established",
        ),
        (
            KucSearchTraceContinuationError::CloseNotApplied,
            "search close did not hide the retained strip",
        ),
        (
            KucSearchTraceContinuationError::Interaction(KucInteractionLocatorError::Disabled),
            "search target failed: interaction action is disabled",
        ),
        (
            KucSearchTraceContinuationError::Request(KucInteractionRequestError::Stale),
            "search request failed: interaction request is stale",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }

    for (error, expected) in [
        (
            KucOpaqueClickContinuationError::RootMismatch,
            "click continuation belongs to another root",
        ),
        (
            KucOpaqueClickContinuationError::FrameDiscontinuity,
            "click continuation requires the next root frame",
        ),
        (
            KucOpaqueClickContinuationError::NotApplied,
            "click continuation step was not applied",
        ),
        (
            KucOpaqueClickContinuationError::AlreadyApplied,
            "click continuation step was already applied",
        ),
        (
            KucOpaqueClickContinuationError::Interaction(KucInteractionLocatorError::Hidden),
            "click target failed: interaction action is hidden",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }
}
