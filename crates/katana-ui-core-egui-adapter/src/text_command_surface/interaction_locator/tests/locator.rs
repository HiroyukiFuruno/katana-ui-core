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
    let shared_bounds = katana_ui_core::render_model::UiRect::new(
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
}
