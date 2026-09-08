use super::super::support::{cleanup_stage_output, raw_input, sha256};
use super::super::*;
use super::{binding, token};

const STANDARD_STAGE_SIZE: (f32, f32) = (1280.0, 720.0);
const RESIZED_STAGE_SIZE: (f32, f32) = (900.0, 520.0);

fn assert_input_events(class: GenericInteractionClass, input: &egui::RawInput) {
    match class {
        GenericInteractionClass::TextInput => assert!(matches!(
            input.events.as_slice(),
            [egui::Event::Text(text)] if text == "日本語⭐️"
        )),
        GenericInteractionClass::ImeCommit => assert!(matches!(
            input.events.as_slice(),
            [egui::Event::Ime(egui::ImeEvent::Commit(text))] if text == "日本語⭐️"
        )),
        GenericInteractionClass::Selection
        | GenericInteractionClass::ToolbarActivation
        | GenericInteractionClass::FloatingToolbar
        | GenericInteractionClass::ContextMenu => assert!(input.events.is_empty()),
        GenericInteractionClass::Scroll => assert!(matches!(
            input.events.as_slice(),
            [egui::Event::MouseWheel { .. }]
        )),
        GenericInteractionClass::Search | GenericInteractionClass::AccessibilityActivation => {
            assert!(input.events.is_empty());
        }
        GenericInteractionClass::ViewportResize => assert!(input.events.is_empty()),
    }
}

#[test]
fn all_generic_interaction_classes_map_to_input_and_viewport() {
    for class in [
        GenericInteractionClass::TextInput,
        GenericInteractionClass::ImeCommit,
        GenericInteractionClass::Selection,
        GenericInteractionClass::Scroll,
        GenericInteractionClass::ToolbarActivation,
        GenericInteractionClass::FloatingToolbar,
        GenericInteractionClass::Search,
        GenericInteractionClass::ContextMenu,
        GenericInteractionClass::AccessibilityActivation,
        GenericInteractionClass::ViewportResize,
    ] {
        let input = raw_input(class);
        let expected_size = if class == GenericInteractionClass::ViewportResize {
            RESIZED_STAGE_SIZE
        } else {
            STANDARD_STAGE_SIZE
        };
        assert_eq!(
            input.screen_rect.expect("stage bounds").size(),
            egui::vec2(expected_size.0, expected_size.1)
        );
        assert_input_events(class, &input);
    }
}

#[test]
fn public_error_messages_are_stable() {
    let cases = [
        (
            ConsumerArtifactPlanError::UnsupportedSchemaVersion(99),
            "unsupported consumer artifact schema version 99",
        ),
        (
            ConsumerArtifactPlanError::UnsupportedEffectClass(GenericEffectClass::OpaqueForwarding),
            "opaque forwarding is unsupported by consumer artifact plans",
        ),
        (
            ConsumerArtifactPlanError::EmptyPlan,
            "consumer artifact plan is empty",
        ),
        (
            ConsumerArtifactPlanError::InvalidLeafId,
            "consumer artifact leaf id is invalid",
        ),
        (
            ConsumerArtifactPlanError::DuplicateLeaf("leaf-duplicate".to_owned()),
            "duplicate consumer artifact leaf leaf-duplicate",
        ),
        (
            ConsumerArtifactPlanError::RevisionOverflow,
            "consumer artifact revision overflow",
        ),
        (
            ConsumerArtifactPlanError::StaleRevision {
                stage: 2,
                expected: 5,
                actual: 3,
            },
            "stage 2 revision 3 does not match expected 5",
        ),
        (
            ConsumerArtifactPlanError::StageAlreadyConsumed(7),
            "consumer artifact stage 7 was already consumed",
        ),
        (
            ConsumerArtifactPlanError::IncompleteStageSequence,
            "consumer artifact stages must be executed in full sequence",
        ),
        (
            ConsumerArtifactPlanError::StageExecutionFailed(7),
            "consumer artifact stage 7 failed after root mutation",
        ),
        (
            ConsumerArtifactPlanError::PlanComplete,
            "consumer artifact plan is complete",
        ),
        (
            ConsumerArtifactPlanError::TokenRootMismatch,
            "consumer artifact token does not match retained root",
        ),
        (
            ConsumerArtifactPlanError::Root("kuc root".into()),
            "consumer artifact root failed: kuc root",
        ),
        (
            ConsumerArtifactPlanError::ExistingMedia(std::path::PathBuf::from("/tmp/artifact")),
            "consumer artifact media already exists: /tmp/artifact",
        ),
        (
            ConsumerArtifactPlanError::MissingFrame,
            "consumer artifact stage did not produce a frame",
        ),
        (
            ConsumerArtifactPlanError::Artifact("artifact fail".into()),
            "consumer artifact write failed: artifact fail",
        ),
        (
            ConsumerArtifactPlanError::InvalidPng(std::path::PathBuf::from("/tmp/invalid.png")),
            "consumer artifact PNG is invalid: /tmp/invalid.png",
        ),
        (
            ConsumerArtifactPlanError::UnicodeEvidence("unicode fail".into()),
            "consumer artifact unicode evidence failed: unicode fail",
        ),
        (
            ConsumerArtifactPlanError::ReceiptReuse,
            "consumer artifact receipt was reused",
        ),
        (
            ConsumerArtifactPlanError::ReceiptCrossBind,
            "consumer artifact receipt is bound to another stage",
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(format!("{error}"), expected);
    }
}

mod manifest;

#[test]
fn opaque_artifact_values_expose_safe_debug_output() {
    let leaf = ConsumerArtifactLeafId::new("leaf-debug").expect("leaf");
    assert_eq!(
        format!("{leaf:?}"),
        "ConsumerArtifactLeafId(\"leaf-debug\")"
    );
    assert_eq!(
        format!("{:?}", binding("leaf-binding", 1)),
        "ConsumerArtifactStageBinding { leaf: ConsumerArtifactLeafId(\"leaf-binding\"), interaction: ImeCommit, effect: NoHostEffect, .. }"
    );
    let receipt = ConsumerArtifactForwardingReceipt {
        leaf,
        stage_id: "consumer-stage-0000".to_owned(),
        root_revision: 1,
        consumed: false,
        fingerprint: sha256(b"debug-fingerprint"),
    };
    assert_eq!(
        format!("{receipt:?}"),
        "ConsumerArtifactForwardingReceipt(..)"
    );
}

#[test]
fn cleanup_stage_output_removes_only_failed_stage_artifacts() {
    let output_dir = super::temp_dir("cleanup-stage-output");
    for suffix in [".png", ".manifest.json", ".consumer-artifact.json"] {
        std::fs::write(
            output_dir.join(format!("consumer-stage-0000{suffix}")),
            b"partial artifact",
        )
        .expect("partial artifact fixture");
    }
    let unrelated = output_dir.join("consumer-stage-0001.png");
    std::fs::write(&unrelated, b"other stage").expect("other-stage fixture");

    cleanup_stage_output(&output_dir, "consumer-stage-0000").expect("cleanup succeeds");
    for suffix in [".png", ".manifest.json", ".consumer-artifact.json"] {
        assert!(
            !output_dir
                .join(format!("consumer-stage-0000{suffix}"))
                .exists()
        );
    }
    assert!(unrelated.exists());
}

#[test]
fn cleanup_stage_output_keeps_a_non_file_failure_typed() {
    let output_dir = super::temp_dir("cleanup-stage-output-error");
    std::fs::create_dir(output_dir.join("consumer-stage-0000.png")).expect("directory fixture");

    assert!(matches!(
        cleanup_stage_output(&output_dir, "consumer-stage-0000"),
        Err(ConsumerArtifactPlanError::Artifact(_))
    ));
}

#[test]
fn stage_binding_can_define_distinct_action_target() {
    let binding = ConsumerArtifactStageBinding::new_with_action_target(
        ConsumerArtifactLeafId::new("leaf-1").expect("leaf"),
        "toolbar-action-a",
        GenericInteractionClass::ToolbarActivation,
        GenericEffectClass::NoHostEffect,
        token(1),
    );

    assert_eq!(binding.action_target(), "toolbar-action-a");
}
