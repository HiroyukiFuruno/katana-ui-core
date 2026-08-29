use super::targets::{accesskit_class, evidence_for};
use super::types::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueInteractionRequest, LocatorTarget,
};
use super::{AccessKitEvidence, AccessKitTargetClass, HashSet, RefCell};

const TEST_BOUNDS_SIZE_PX: u32 = 10;

fn locator(root: &str, revision: u64, targets: Vec<LocatorTarget>) -> KucInteractionLocator {
    KucInteractionLocator {
        root_identity: root.to_owned(),
        state_revision: revision,
        frame_serial: revision,
        correlation_fingerprint: format!("correlation-{revision}"),
        targets,
        ambiguous_bounds: Vec::new(),
        hidden: HashSet::new(),
        requested: RefCell::new(HashSet::new()),
        selection_geometry: None,
        selection_established: false,
        floating_visible: false,
        search_visible: false,
        search_query_focused: false,
    }
}

fn target(id: &str, class: KucInteractionActionClass, disabled: bool) -> LocatorTarget {
    LocatorTarget {
        action_identity: id.to_owned(),
        action_class: class,
        disabled,
        evidence: AccessKitEvidence {
            response_id: egui::Id::new(("test", id)),
            bounds: katana_ui_core::render_model::UiRect::new(
                0,
                0,
                TEST_BOUNDS_SIZE_PX,
                TEST_BOUNDS_SIZE_PX,
            ),
            label: id.to_owned(),
            disabled,
            target_identity: id.to_owned(),
            target_class: accesskit_class(class),
        },
    }
}

#[test]
fn request_is_opaque_and_raw_input_is_mutated_once() {
    let locator = locator(
        "root",
        4,
        vec![target("bold", KucInteractionActionClass::Toolbar, false)],
    );
    let selector = KucInteractionSelector::new("bold", KucInteractionActionClass::Toolbar);
    let mut request = locator.request(selector).expect("current action");
    let mut input = egui::RawInput::default();
    request
        .apply_to_raw_input_once(&mut input)
        .expect("first application");
    assert_eq!(input.events.len(), 3);
    assert_eq!(
        request.apply_to_raw_input_once(&mut input),
        Err(KucInteractionRequestError::AlreadyQueued)
    );
    assert_eq!(input.events.len(), 3);
}

#[test]
fn locator_rejects_disabled_missing_ambiguous_and_duplicate_without_raw_mutation() {
    let disabled = locator(
        "root",
        1,
        vec![target("disabled", KucInteractionActionClass::Toolbar, true)],
    );
    assert!(matches!(
        disabled.request(KucInteractionSelector::new(
            "disabled",
            KucInteractionActionClass::Toolbar
        )),
        Err(KucInteractionLocatorError::Disabled)
    ));
    let missing = locator("root", 1, Vec::new());
    assert!(matches!(
        missing.request(KucInteractionSelector::new(
            "missing",
            KucInteractionActionClass::Toolbar
        )),
        Err(KucInteractionLocatorError::Missing)
    ));
    let ambiguous = locator(
        "root",
        1,
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
    let shared_bounds = katana_ui_core::render_model::UiRect::new(0, 0, 10, 10);
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
        evidence_for(
            &evidence,
            "first",
            KucInteractionActionClass::Toolbar,
            false,
        )
        .expect("first current-frame evidence")
        .label,
        "first"
    );
    assert_eq!(
        evidence_for(
            &evidence,
            "second",
            KucInteractionActionClass::Toolbar,
            false,
        )
        .expect("second current-frame evidence")
        .label,
        "second"
    );
}

#[test]
fn root_and_revision_mismatch_do_not_mutate_raw_input() {
    let owner = locator(
        "owner",
        9,
        vec![target("action", KucInteractionActionClass::Toolbar, false)],
    );
    let other_root = locator(
        "other",
        9,
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
        state_revision: 8,
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
