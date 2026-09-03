#![cfg(feature = "egui")]
#![cfg(feature = "storybook-artifacts")]

use katana_ui_core::egui::{
    MotionArtifactError, MotionArtifactWriter, OpaqueMotionReceiptSequence,
    VariableViewportMotionArtifact, VariableViewportMotionArtifactError,
    VariableViewportMotionArtifactManifest, VariableViewportSemanticEvidence,
    VariableViewportSourceViewport,
};
use std::path::Path;

type VariableViewportWriter =
    fn(
        &MotionArtifactWriter,
        &OpaqueMotionReceiptSequence,
        &Path,
    ) -> Result<VariableViewportMotionArtifact, VariableViewportMotionArtifactError>;

#[test]
fn variable_viewport_motion_contract_is_public_and_additive() {
    let writer: VariableViewportWriter = MotionArtifactWriter::write_opaque_variable_viewport;
    let _ = writer;
    let _: Option<VariableViewportMotionArtifactManifest> = None;
    let _: Option<VariableViewportMotionArtifactError> = None;
    let _: Option<VariableViewportSourceViewport> = None;
    let _: Option<VariableViewportSemanticEvidence> = None;

    let fixed_writer = MotionArtifactWriter::new();
    let empty = OpaqueMotionReceiptSequence::new();
    assert!(matches!(
        fixed_writer.write_opaque(&empty, Path::new("unused")),
        Err(MotionArtifactError::EmptySequence)
    ));
}
