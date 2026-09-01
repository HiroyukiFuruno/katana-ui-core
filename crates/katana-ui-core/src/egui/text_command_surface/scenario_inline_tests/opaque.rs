#[test]
fn navigation_input_scenario_keeps_source_submission_inside_the_opaque_root() {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::NavigationInput)
        .expect("navigation scenario issues");
    let stages = scenario.stages().to_vec();
    assert_eq!(
        stages.len(),
        4,
        "navigation trace has focus, text, and submit"
    );
    assert!(stages[1].event_count() > 0);
    assert!(stages[2].event_count() > 0);
    assert!(stages[3].event_count() > 0);
    assert!(
        !format!("{stages:?}").contains(NAVIGATION_INPUT_FIXTURE),
        "opaque stages must not expose their raw navigation text through Debug"
    );

    let mut root = retained(FullTextCommandSurfaceScenarioId::NavigationInput);
    let context = egui::Context::default();
    context.enable_accesskit();
    let _ = render(&context, &mut root, Some(&stages[0]));
    let _ = render(&context, &mut root, Some(&stages[1]));
    let typed = render(&context, &mut root, Some(&stages[2]));
    assert_eq!(
        typed
            .events()
            .current_context()
            .source_address_submission_count(),
        0,
        "typing changes retained input without emitting a host submission"
    );
    let submitted = render(&context, &mut root, Some(&stages[3]));
    let context = submitted.events().current_context();
    assert_eq!(context.source_address_submission_count(), 1);
    assert!(
        !format!("{context:?}").contains(NAVIGATION_INPUT_FIXTURE),
        "public root context must not reveal the submitted navigation text"
    );
    assert!(
        !submitted.evidence_composite.rgba_pixels.is_empty(),
        "the same retained KUC root emits a nonempty composite"
    );
}

#[test]
fn workspace_tabs_scenario_keeps_drag_and_artifact_inside_the_same_opaque_root() {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::WorkspaceTabs)
        .expect("workspace tabs scenario issues");
    let stages = scenario.stages().to_vec();
    assert_eq!(
        stages.len(),
        4,
        "workspace tabs trace has start, drag, and release"
    );
    assert!(stages[1].event_count() > 0);
    assert!(stages[2].event_count() > 0);
    assert!(stages[3].event_count() > 0);

    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = retained(FullTextCommandSurfaceScenarioId::WorkspaceTabs);
    let initial = render(&context, &mut root, Some(&stages[0]));
    let _ = render(&context, &mut root, Some(&stages[1]));
    let dragging = render(&context, &mut root, Some(&stages[2]));
    let released = render(&context, &mut root, Some(&stages[3]));
    assert!(
        released
            .artifact_order()
            .contains(&super::super::EguiTextCommandSurfaceChild::StatusBar),
        "workspace tabs consumes the status projection through the opaque lease"
    );
    assert!(
        released
            .artifact_order()
            .contains(&super::super::EguiTextCommandSurfaceChild::DiagnosticsList),
        "workspace tabs consumes the diagnostics projection through the opaque lease"
    );
    assert!(
        released
            .artifact_order()
            .contains(&super::super::EguiTextCommandSurfaceChild::Preview),
        "workspace tabs consumes the generic preview through the opaque lease"
    );
    assert_ne!(
        initial.evidence_composite.pixel_hash, dragging.evidence_composite.pixel_hash,
        "the KUC-owned drag ghost must alter the final root composite"
    );
    assert!(
        released.evidence_composite.non_transparent_pixel_count > 0,
        "tab strip and text surface remain in one nonempty root composite"
    );
    assert!(
        released
            .events()
            .current_context()
            .source_address_submission_count()
            == 0,
        "a generic tab drag cannot be decoded as an unrelated root event"
    );
}

const RGBA_CHANNEL_COUNT: usize = 4;

fn rgba_color_count(pixels: &[u8], color: [u8; RGBA_CHANNEL_COUNT]) -> usize {
    let (pixels, remainder) = pixels.as_chunks::<RGBA_CHANNEL_COUNT>();
    assert!(remainder.is_empty());
    pixels
        .iter()
        .filter(|pixel| **pixel == color)
        .count()
}

#[test]
fn raw_stage_applies_owned_events_and_debug_keeps_payload_opaque() {
    let stage = super::stage(vec![egui::Event::Text("opaque 日本語".to_string())], 1.5);
    let mut input = egui::RawInput::default();
    stage.apply_to(&mut input);
    assert_eq!(stage.event_count(), 1);
    assert_eq!(input.events.len(), 1);
    assert!(format!("{stage:?}").contains("event_count: 1"));
    assert!(!format!("{stage:?}").contains("opaque"));
}

#[test]
fn motion_frame_apply_to_fails_closed_without_required_continuation() {
    let frame = FullTextCommandSurfaceMotionFrame {
        scenario_id: FullTextCommandSurfaceScenarioId::Selection,
        stage: stage(Vec::new(), 1.0),
        provenance_id: String::from("kuc-motion-fake"),
        selection_transition: SelectionMotionTransition::Advance,
        find_transition: FindMotionTransition::None,
        dropdown_transition: DropdownMotionTransition::None,
    };

    let mut input = egui::RawInput::default();
    let mut continuation = None;
    assert!(matches!(
        frame.apply_to(&mut input, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::MissingContinuation)
    ));
}

#[test]
fn motion_frame_capture_continuation_fails_closed_for_invalid_transition_combo() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Selection);
    let output = render(&egui::Context::default(), &mut root, None);
    let locator = output.interaction_locator();

    let mut frame = FullTextCommandSurfaceMotionFrame {
        scenario_id: FullTextCommandSurfaceScenarioId::Selection,
        stage: stage(Vec::new(), 1.0),
        provenance_id: String::from("kuc-motion-fail"),
        selection_transition: SelectionMotionTransition::Begin,
        find_transition: FindMotionTransition::Begin,
        dropdown_transition: DropdownMotionTransition::None,
    };

    let mut no_continuation = None;
    assert!(matches!(
        frame.capture_continuation(locator, &mut no_continuation),
        Err(FullTextCommandSurfaceMotionPlanError::InvalidTransition)
    ));

    let trigger_frame = FullTextCommandSurfaceMotionFrame {
        scenario_id: FullTextCommandSurfaceScenarioId::Selection,
        stage: stage(Vec::new(), 1.0),
        provenance_id: String::from("kuc-motion-trigger"),
        selection_transition: SelectionMotionTransition::None,
        find_transition: FindMotionTransition::None,
        dropdown_transition: DropdownMotionTransition::BeginTrigger,
    };
    let mut continuation = None;
    trigger_frame
        .capture_continuation(locator, &mut continuation)
        .expect("dropdown continuation opens through capture");
    assert_eq!(
        format!(
            "{:?}",
            continuation.as_ref().expect("continuation retained")
        ),
        "KucOpaqueMotionContinuation(..)"
    );

    assert!(matches!(
        trigger_frame.capture_continuation(locator, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation)
    ));

    frame.selection_transition = SelectionMotionTransition::Begin;
    frame.find_transition = FindMotionTransition::None;
    frame.dropdown_transition = DropdownMotionTransition::None;
    assert!(matches!(
        frame.capture_continuation(locator, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation)
    ));

    frame.selection_transition = SelectionMotionTransition::None;
    frame.find_transition = FindMotionTransition::Begin;
    assert!(matches!(
        frame.capture_continuation(locator, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation)
    ));

    frame.selection_transition = SelectionMotionTransition::None;
    frame.find_transition = FindMotionTransition::None;
    frame.dropdown_transition = DropdownMotionTransition::None;
    assert!(matches!(
        frame.capture_continuation(locator, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation)
    ));
}
