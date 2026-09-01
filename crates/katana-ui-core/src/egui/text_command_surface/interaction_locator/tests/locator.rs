use super::super::targets::{
    TEXT_SURFACE_CONTEXT_TARGET_ID, append_generic_targets, append_text_surface_context_target,
};
use super::super::{
    AccessKitEvidence, AccessKitTargetClass, KucInteractionActionClass, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueInteractionRequest,
};
use super::common::{
    KUC_FRAME_STEP_ONE, KUC_LOCATOR_OWNER_FRAME, KUC_LOCATOR_REQUEST_REVISION,
    KUC_LOCATOR_STALE_FRAME, REQUEST_EVENT_COUNT_THREE, TEST_BOUNDS_SIZE_PX, ZERO_I32,
    evidence_for_toolbar_match, locator, target,
};

#[test]
fn request_is_opaque_and_raw_input_is_mutated_once() {
    let locator = locator(
        "root",
        KUC_LOCATOR_REQUEST_REVISION,
        vec![target("bold", KucInteractionActionClass::Toolbar, false)],
    );
    assert_eq!(locator.state_revision(), KUC_LOCATOR_REQUEST_REVISION);
    let selector = KucInteractionSelector::new("bold", KucInteractionActionClass::Toolbar);
    let mut request = locator.request(selector).expect("current action");
    let mut input = egui::RawInput::default();

    request
        .apply_to_raw_input_once(&mut input)
        .expect("first application");
    assert_eq!(input.events.len(), REQUEST_EVENT_COUNT_THREE);
    assert_eq!(
        request.apply_to_raw_input_once(&mut input),
        Err(KucInteractionRequestError::AlreadyQueued)
    );
    assert_eq!(input.events.len(), REQUEST_EVENT_COUNT_THREE);

    let queued_locator = super::common::locator(
        "queued-root",
        KUC_LOCATOR_REQUEST_REVISION,
        vec![target("queued", KucInteractionActionClass::Toolbar, false)],
    );
    let queued_request = queued_locator
        .request(KucInteractionSelector::new(
            "queued",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("current queued action");
    let mut queued_input = egui::RawInput::default();
    queued_locator
        .queue_request(queued_request, &mut queued_input)
        .expect("current request queues through its owner locator");
    assert_eq!(queued_input.events.len(), REQUEST_EVENT_COUNT_THREE);
}

#[test]
fn locator_rejects_disabled_missing_ambiguous_and_duplicate_without_raw_mutation() {
    let disabled = locator(
        "root",
        KUC_FRAME_STEP_ONE,
        vec![target("disabled", KucInteractionActionClass::Toolbar, true)],
    );
    assert!(matches!(
        disabled.request(KucInteractionSelector::new(
            "disabled",
            KucInteractionActionClass::Toolbar
        )),
        Err(KucInteractionLocatorError::Disabled)
    ));
    let missing = locator("root", KUC_FRAME_STEP_ONE, Vec::new());
    assert!(matches!(
        missing.request(KucInteractionSelector::new(
            "missing",
            KucInteractionActionClass::Toolbar
        )),
        Err(KucInteractionLocatorError::Missing)
    ));

    let ambiguous = locator(
        "root",
        KUC_FRAME_STEP_ONE,
        vec![
            target("same", KucInteractionActionClass::Toolbar, false),
            target("same", KucInteractionActionClass::Toolbar, false),
        ],
    );
    let selector = KucInteractionSelector::new("same", KucInteractionActionClass::Toolbar);
    assert!(matches!(
        ambiguous.request(selector.clone()),
        Err(KucInteractionLocatorError::Ambiguous)
    ));
    assert!(matches!(
        ambiguous.request(selector),
        Err(KucInteractionLocatorError::Duplicate)
    ));
}

#[test]
fn same_bounds_match_their_own_identity_and_class_evidence() {
    let shared_bounds = crate::render_model::UiRect::new(
        ZERO_I32,
        ZERO_I32,
        TEST_BOUNDS_SIZE_PX,
        TEST_BOUNDS_SIZE_PX,
    );
    let evidence = vec![
        AccessKitEvidence {
            response_id: egui::Id::new(("test", "first")),
            bounds: shared_bounds,
            label: "first".to_owned(),
            disabled: false,
            target_identity: "first".to_owned(),
            target_class: AccessKitTargetClass::Toolbar,
        },
        AccessKitEvidence {
            response_id: egui::Id::new(("test", "second")),
            bounds: shared_bounds,
            label: "second".to_owned(),
            disabled: false,
            target_identity: "second".to_owned(),
            target_class: AccessKitTargetClass::Toolbar,
        },
    ];
    assert_eq!(
        evidence_for_toolbar_match(&evidence, "first").label,
        "first"
    );
    assert_eq!(
        evidence_for_toolbar_match(&evidence, "second").label,
        "second"
    );
}

#[test]
fn root_and_revision_mismatch_do_not_mutate_raw_input() {
    let owner = locator(
        "owner",
        KUC_LOCATOR_OWNER_FRAME,
        vec![target("action", KucInteractionActionClass::Toolbar, false)],
    );
    let other_root = locator(
        "other",
        KUC_LOCATOR_OWNER_FRAME,
        vec![target("action", KucInteractionActionClass::Toolbar, false)],
    );
    let request = other_root
        .request(KucInteractionSelector::new(
            "action",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("other root action");
    let mut input = egui::RawInput::default();

    assert_eq!(
        owner.queue_request(request, &mut input),
        Err(KucInteractionRequestError::RootMismatch)
    );
    assert!(input.events.is_empty());

    let stale = KucOpaqueInteractionRequest {
        root_identity: "owner".to_owned(),
        state_revision: KUC_LOCATOR_STALE_FRAME,
        correlation_fingerprint: "correlation-8".to_owned(),
        events: vec![egui::Event::Copy],
        queued: false,
    };
    assert_eq!(
        owner.queue_request(stale, &mut input),
        Err(KucInteractionRequestError::Stale)
    );
    assert!(input.events.is_empty());

    let stale_correlation = KucOpaqueInteractionRequest {
        root_identity: "owner".to_owned(),
        state_revision: KUC_LOCATOR_OWNER_FRAME,
        correlation_fingerprint: "different-event-batch".to_owned(),
        events: vec![egui::Event::Copy],
        queued: false,
    };
    assert_eq!(
        owner.queue_request(stale_correlation, &mut input),
        Err(KucInteractionRequestError::Stale)
    );
    assert!(input.events.is_empty());
}

#[test]
fn disabled_context_target_is_retained_and_fails_closed() {
    let evidence = vec![
        target(
            TEXT_SURFACE_CONTEXT_TARGET_ID,
            KucInteractionActionClass::TextSurfaceContextTarget,
            true,
        )
        .evidence,
    ];
    let mut targets = Vec::new();
    append_text_surface_context_target(&mut targets, &evidence);

    assert_eq!(targets.len(), 1);
    assert!(targets[0].disabled);
    assert_eq!(
        locator("root", KUC_FRAME_STEP_ONE, targets)
            .request_context_open()
            .expect_err("disabled context target must not be requested"),
        KucInteractionLocatorError::Disabled
    );
}

#[test]
fn generic_accesskit_targets_preserve_every_action_class() {
    let expected = [
        ("status", KucInteractionActionClass::StatusBarSegment),
        ("scope", KucInteractionActionClass::DiagnosticsScope),
        (
            "severity",
            KucInteractionActionClass::DiagnosticsSeverityFilter,
        ),
        ("diagnostic", KucInteractionActionClass::DiagnosticsItem),
        ("fix", KucInteractionActionClass::DiagnosticsFix),
    ];
    let evidence = expected
        .iter()
        .map(|(identity, class)| target(identity, *class, false).evidence)
        .collect::<Vec<_>>();
    let mut targets = Vec::new();
    append_generic_targets(&mut targets, &evidence);

    assert_eq!(targets.len(), expected.len());
    for (identity, class) in expected {
        assert!(targets.iter().any(|target| {
            target.action_identity == identity && target.action_class == class && !target.disabled
        }));
    }
}

#[test]
fn mismatched_bound_evidence_is_ignored_after_an_actual_surface_frame() {
    use crate::atom::TextArea;
    use crate::egui::text_command_surface::{
        EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, TextCommandSurfaceStyle,
    };
    use crate::molecule::command_chrome::{CommandChromeAction, CommandChromeToolbar};
    use crate::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};

    let text = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("locator-evidence").value("locator evidence"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 320, 180),
    ));
    let mut surface =
        EguiTextCommandSurface::new(text).with_toolbar(CommandChromeToolbar::new().action(
            CommandChromeAction::new("mismatch-action", "Mismatch action"),
        ));
    let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
        crate::text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("adapter");
    let style = TextCommandSurfaceStyle::standard().expect("style");
    let context = egui::Context::default();
    let mut output = None;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 180.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            output =
                Some(adapter.show_with_tab_strip(ui, &mut surface, &style, None, None, None, None));
        },
    );
    let mut output = output
        .expect("actual frame ran")
        .expect("actual frame output");
    let actual_evidence = output.accesskit_evidence.clone();
    assert!(actual_evidence.iter().any(|entry| {
        entry.target_identity == "mismatch-action"
            && entry.target_class == AccessKitTargetClass::Toolbar
    }));
    let batch = super::super::super::root_event::build_event_batch(&mut output, None)
        .expect("root event batch");
    let event_context = batch.current_context();
    let mismatched = crate::egui::text_command_surface::accesskit_evidence::bind_frame(
        actual_evidence,
        "other-root",
        &event_context,
    );

    let locator = super::super::KucInteractionLocator::from_output(
        "root",
        &event_context,
        KUC_FRAME_STEP_ONE,
        &output,
        &mismatched,
    );

    assert!(matches!(
        locator.request(KucInteractionSelector::new(
            "mismatch-action",
            KucInteractionActionClass::Toolbar,
        )),
        Err(KucInteractionLocatorError::Missing)
    ));
}
