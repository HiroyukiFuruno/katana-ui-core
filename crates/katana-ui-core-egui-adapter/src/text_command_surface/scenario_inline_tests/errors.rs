#[test]
fn scenario_and_motion_continuation_errors_keep_typed_context() {
    for (error, expected) in [
        (
            FullTextCommandSurfaceScenarioError::LeaseConsumed,
            "scenario lease was already consumed",
        ),
        (
            FullTextCommandSurfaceScenarioError::InvalidProjection,
            "scenario projection is invalid",
        ),
        (
            FullTextCommandSurfaceScenarioError::RevisionExhausted,
            "scenario session revision is exhausted",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }

    for (error, expected) in [
        (
            KucOpaqueMotionContinuationError::Selection(
                KucTextSelectionContinuationError::Unavailable,
            ),
            "selection continuation failed: current root frame has no selectable text area",
        ),
        (
            KucOpaqueMotionContinuationError::Search(KucSearchTraceContinuationError::Unavailable),
            "search continuation failed: search trace is unavailable",
        ),
        (
            KucOpaqueMotionContinuationError::Click(KucOpaqueClickContinuationError::NotApplied),
            "click continuation failed: click continuation step was not applied",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn motion_frame_debug_and_error_display_keep_opaque_context() {
    let frame = FullTextCommandSurfaceMotionPlan::issue(
        FullTextCommandSurfaceMotionPlan::minimum_frame_count(),
    )
    .expect("complete motion plan")
    .frames()[0]
        .clone();
    assert_eq!(frame.event_count(), frame.stage.event_count());
    let debug = format!("{frame:?}");
    assert!(debug.contains("FullTextCommandSurfaceMotionFrame"));
    assert!(debug.contains("event_count:"));
    assert!(!debug.contains("日本語"));

    let errors = [
        FullTextCommandSurfaceMotionPlanError::IncompleteCatalogue {
            requested: 1,
            minimum: 2,
        },
        FullTextCommandSurfaceMotionPlanError::MissingContinuation,
        FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation,
        FullTextCommandSurfaceMotionPlanError::InvalidTransition,
        FullTextCommandSurfaceMotionPlanError::Selection(
            KucTextSelectionContinuationError::Unavailable,
        ),
        FullTextCommandSurfaceMotionPlanError::Search(KucSearchTraceContinuationError::Unavailable),
        FullTextCommandSurfaceMotionPlanError::Dropdown(KucInteractionLocatorError::Missing),
        FullTextCommandSurfaceMotionPlanError::Continuation(
            KucOpaqueMotionContinuationError::Click(KucOpaqueClickContinuationError::NotApplied),
        ),
    ];
    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn scenario_terminal_sinks_accept_opaque_submission_and_proposal() {
    let mut strip = SourceAddressStrip::new(SourceAddressPresentation::new(
        "表示",
        "ツールチップ",
        "アクセシビリティ",
    ));
    let _ = strip.apply_action(SourceAddressAction::SetDraft(String::from("opaque")));
    let submission = match strip.apply_action(SourceAddressAction::Submit) {
        Some(SourceAddressEvent::Submitted(submission)) => submission,
        _ => panic!("enabled source address should submit"),
    };
    NavigationInputAcknowledgementPort
        .forward_submission(submission)
        .expect("navigation sink accepts the one-shot submission");

    WorkspaceTabsAcknowledgementPort
        .forward_proposal(TabStripProposal::new(
            1,
            TabStripCorrelation::from_opaque_bytes([1]),
            TabStripProposalOperation::SelectPrevious,
        ))
        .expect("workspace-tabs sink consumes the opaque proposal");
}
