use crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame;
use crate::egui::{FullRootArtifact, FullRootArtifactError, FullRootArtifactWriter};
use std::path::Path;

mod semantic;
pub(crate) use semantic::MotionFrameSemanticEvidence;

#[derive(Debug, Clone)]
pub struct OpaqueRootArtifactReceipt {
    artifact: FullRootArtifact,
    motion_semantics: Option<MotionFrameSemanticEvidence>,
}

impl PartialEq for OpaqueRootArtifactReceipt {
    fn eq(&self, other: &Self) -> bool {
        self.artifact == other.artifact
    }
}

impl Eq for OpaqueRootArtifactReceipt {}

impl OpaqueRootArtifactReceipt {
    #[must_use]
    pub fn stage_id(&self) -> &str {
        self.artifact.stage_id()
    }

    pub(crate) fn artifact(&self) -> &FullRootArtifact {
        &self.artifact
    }

    pub(crate) fn motion_semantics(&self) -> Option<&MotionFrameSemanticEvidence> {
        self.motion_semantics.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn from_test_parts(
        artifact: FullRootArtifact,
        motion_semantics: Option<MotionFrameSemanticEvidence>,
    ) -> Self {
        Self {
            artifact,
            motion_semantics,
        }
    }
}

impl From<FullRootArtifact> for OpaqueRootArtifactReceipt {
    fn from(artifact: FullRootArtifact) -> Self {
        Self {
            artifact,
            motion_semantics: None,
        }
    }
}

#[derive(Debug)]
pub enum OpaqueRootArtifactReceiptError {
    Artifact(FullRootArtifactError),
}

impl std::fmt::Display for OpaqueRootArtifactReceiptError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Artifact(error) => {
                write!(formatter, "opaque root artifact receipt failed: {error}")
            }
        }
    }
}

impl std::error::Error for OpaqueRootArtifactReceiptError {}

#[derive(Debug, Default, Clone, Copy)]
pub struct OpaqueRootArtifactReceiptWriter;

impl OpaqueRootArtifactReceiptWriter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn write(
        &self,
        frame: &EguiTextCommandSurfaceHostRootFrame,
        output_dir: &Path,
        stage_id: &str,
    ) -> Result<OpaqueRootArtifactReceipt, OpaqueRootArtifactReceiptError> {
        FullRootArtifactWriter::new()
            .write(frame, output_dir, stage_id)
            .map(|artifact| OpaqueRootArtifactReceipt {
                artifact,
                motion_semantics: Some(semantic::motion_semantics(frame)),
            })
            .map_err(OpaqueRootArtifactReceiptError::Artifact)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OpaqueMotionReceiptSequence {
    receipts: Vec<OpaqueRootArtifactReceipt>,
}

impl OpaqueMotionReceiptSequence {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(
        &mut self,
        stage_id: &str,
        receipt: OpaqueRootArtifactReceipt,
    ) -> Result<(), OpaqueMotionReceiptSequenceError> {
        if receipt.stage_id() != stage_id {
            return Err(OpaqueMotionReceiptSequenceError::StageMismatch {
                expected: stage_id.to_owned(),
                actual: receipt.stage_id().to_owned(),
            });
        }
        self.receipts.push(receipt);
        Ok(())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.receipts.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }

    pub(crate) fn receipts(&self) -> &[OpaqueRootArtifactReceipt] {
        &self.receipts
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpaqueMotionReceiptSequenceError {
    StageMismatch { expected: String, actual: String },
}

impl std::fmt::Display for OpaqueMotionReceiptSequenceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StageMismatch { expected, actual } => {
                write!(
                    formatter,
                    "opaque motion receipt stage mismatch: expected {expected}, got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for OpaqueMotionReceiptSequenceError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn receipt(stage: &str) -> OpaqueRootArtifactReceipt {
        FullRootArtifact::from_test_parts(
            stage.to_owned(),
            std::path::PathBuf::from(format!("{stage}.png")),
            std::path::PathBuf::from(format!("{stage}.manifest.json")),
            1,
            1,
            "record".to_owned(),
            "pixel".to_owned(),
            "png".to_owned(),
        )
        .into()
    }

    #[test]
    fn opaque_receipt_sequence_preserves_order_and_rejects_stage_mismatch() {
        let mut sequence = OpaqueMotionReceiptSequence::new();
        assert!(sequence.is_empty());
        sequence
            .push("frame-000", receipt("frame-000"))
            .expect("matching stage must append");
        assert_eq!(sequence.len(), 1);
        assert_eq!(sequence.receipts()[0].stage_id(), "frame-000");
        assert_eq!(sequence.receipts()[0].artifact().width(), 1);

        let error = sequence
            .push("frame-001", receipt("wrong"))
            .expect_err("mismatched stage must fail closed");
        assert!(error.to_string().contains("frame-001"));
        let _: &dyn std::error::Error = &error;
    }

    #[test]
    fn opaque_receipt_writer_error_display_preserves_typed_source() {
        let error = OpaqueRootArtifactReceiptError::Artifact(FullRootArtifactError::InvalidStageId);
        assert!(
            error
                .to_string()
                .contains("opaque root artifact receipt failed")
        );
        let _: &dyn std::error::Error = &error;
        let _ = OpaqueRootArtifactReceiptWriter::new();
    }

    #[test]
    fn opaque_receipt_equality_ignores_private_semantic_cache() {
        let artifact = FullRootArtifact::from_test_parts(
            "frame-000".into(),
            "frame-000.png".into(),
            "frame-000.manifest.json".into(),
            1,
            1,
            "record".into(),
            "pixel".into(),
            "png".into(),
        );
        let without_cache = OpaqueRootArtifactReceipt::from(artifact.clone());
        let with_cache = OpaqueRootArtifactReceipt::from_test_parts(
            artifact,
            Some(MotionFrameSemanticEvidence {
                root_record_hash: "record".into(),
                star_scalar_sequence: vec![0x2b50, 0xfe0f],
                star_chromatic_pixel_count: 1,
                control_star_chromatic_pixel_count: 0,
                star_hit_test_seen: true,
                ime_preedit_event_seen: true,
                ime_commit_event_seen: true,
                expected_accesskit_text_input_value: "⭐️入力".into(),
                accesskit_text_input_nodes: Vec::new(),
                accesskit_snapshot_hash: "accesskit".into(),
            }),
        );
        assert_eq!(without_cache, with_cache);
    }
}
