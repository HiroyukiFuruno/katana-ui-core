use super::interaction_locator_types::LocatorTarget;
use super::interaction_locator_utils::evidence_for;
use super::*;
use crate::text_command_surface::accesskit_evidence::{AccessKitEvidence, AccessKitTargetClass};
use std::cell::RefCell;
use std::collections::HashSet;

const TARGET_EXTENT: u32 = 10;

#[cfg(test)]
mod tests {
    use super::*;

    fn locator(root: &str, revision: u64, targets: Vec<LocatorTarget>) -> KucInteractionLocator {
        locator_with_correlation(root, revision, &format!("correlation-{revision}"), targets)
    }

    fn locator_with_correlation(
        root: &str,
        revision: u64,
        correlation_fingerprint: &str,
        targets: Vec<LocatorTarget>,
    ) -> KucInteractionLocator {
        KucInteractionLocator {
            root_identity: root.to_owned(),
            state_revision: revision,
            correlation_fingerprint: correlation_fingerprint.to_owned(),
            targets,
            ambiguous_bounds: Vec::new(),
            hidden: HashSet::new(),
            requested: RefCell::new(HashSet::new()),
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
                    TARGET_EXTENT,
                    TARGET_EXTENT,
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
    fn locator_rejects_hidden_and_geometry_ambiguous_targets() {
        let mut hidden = locator(
            "root",
            1,
            vec![target("hidden", KucInteractionActionClass::Toolbar, false)],
        );
        hidden
            .hidden
            .insert(("hidden".to_owned(), KucInteractionActionClass::Toolbar));
        assert!(matches!(
            hidden.request(KucInteractionSelector::new(
                "hidden",
                KucInteractionActionClass::Toolbar
            )),
            Err(KucInteractionLocatorError::Hidden)
        ));

        let mut ambiguous = locator(
            "root",
            1,
            vec![target(
                "ambiguous",
                KucInteractionActionClass::Toolbar,
                false,
            )],
        );
        ambiguous
            .ambiguous_bounds
            .push(katana_ui_core::render_model::UiRect::new(
                0,
                0,
                TARGET_EXTENT,
                TARGET_EXTENT,
            ));
        assert!(matches!(
            ambiguous.request(KucInteractionSelector::new(
                "ambiguous",
                KucInteractionActionClass::Toolbar
            )),
            Err(KucInteractionLocatorError::Ambiguous)
        ));
    }

    #[test]
    fn context_target_resolves_to_secondary_pointer_events() {
        let locator = locator(
            "root",
            3,
            vec![target(
                TEXT_SURFACE_CONTEXT_TARGET_ID,
                KucInteractionActionClass::TextSurfaceContextTarget,
                false,
            )],
        );
        let request = locator
            .request_context_open()
            .expect("current text-surface context target");

        let buttons: Vec<_> = request
            .events
            .iter()
            .filter_map(|event| match event {
                egui::Event::PointerButton {
                    button, pressed, ..
                } => Some((*button, *pressed)),
                _ => None,
            })
            .collect();
        assert_eq!(
            buttons,
            vec![
                (egui::PointerButton::Secondary, true),
                (egui::PointerButton::Secondary, false),
            ]
        );
    }

    #[test]
    fn duplicate_context_targets_are_rejected_as_ambiguous() {
        let locator = locator(
            "root",
            3,
            vec![
                target(
                    TEXT_SURFACE_CONTEXT_TARGET_ID,
                    KucInteractionActionClass::TextSurfaceContextTarget,
                    false,
                ),
                target(
                    TEXT_SURFACE_CONTEXT_TARGET_ID,
                    KucInteractionActionClass::TextSurfaceContextTarget,
                    false,
                ),
            ],
        );

        assert!(matches!(
            locator.request_context_open(),
            Err(KucInteractionLocatorError::Ambiguous)
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
    fn disabled_text_surface_context_evidence_is_retained_as_disabled_target() {
        let evidence = vec![AccessKitEvidence {
            response_id: egui::Id::new("disabled-context"),
            bounds: katana_ui_core::render_model::UiRect::new(1, 2, 3, 4),
            label: "context".to_owned(),
            disabled: true,
            target_identity: TEXT_SURFACE_CONTEXT_TARGET_ID.to_owned(),
            target_class: AccessKitTargetClass::TextSurfaceContextTarget,
        }];
        let mut targets = Vec::new();
        super::interaction_locator_appenders::append_text_surface_context_target(
            &mut targets,
            &evidence,
        );
        assert_eq!(targets.len(), 1);
        assert!(targets[0].disabled);
        assert_eq!(
            targets[0].action_class,
            KucInteractionActionClass::TextSurfaceContextTarget
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
        let stale_correlation = KucOpaqueInteractionRequest {
            root_identity: "owner".to_owned(),
            state_revision: 9,
            correlation_fingerprint: "wrong-correlation".to_owned(),
            events: vec![egui::Event::Copy],
            queued: false,
        };
        assert_eq!(
            owner.queue_request(stale_correlation, &mut input),
            Err(KucInteractionRequestError::Stale)
        );
        assert!(input.events.is_empty());
        let current = owner
            .request(KucInteractionSelector::new(
                "action",
                KucInteractionActionClass::Toolbar,
            ))
            .expect("owner action");
        assert!(owner.queue_request(current, &mut input).is_ok());
        assert!(!input.events.is_empty());
    }

    #[test]
    fn request_generated_by_previous_frame_is_rejected_after_correlation_changes() {
        let previous = locator_with_correlation(
            "root",
            9,
            "correlation-previous",
            vec![target("action", KucInteractionActionClass::Toolbar, false)],
        );
        let current = locator_with_correlation(
            "root",
            9,
            "correlation-current",
            vec![target("action", KucInteractionActionClass::Toolbar, false)],
        );
        let request = previous
            .request(KucInteractionSelector::new(
                "action",
                KucInteractionActionClass::Toolbar,
            ))
            .expect("previous-frame action");
        let mut input = egui::RawInput::default();

        assert_eq!(
            current.queue_request(request, &mut input),
            Err(KucInteractionRequestError::Stale)
        );
        assert!(input.events.is_empty());
    }
}
