use crate::text_command_surface::EguiTextCommandSurfaceHostRootFrame;
use crate::{FullRootArtifact, FullRootArtifactError, FullRootArtifactWriter};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpaqueRootArtifactReceipt {
    artifact: FullRootArtifact,
}

impl OpaqueRootArtifactReceipt {
    #[must_use]
    pub fn stage_id(&self) -> &str {
        self.artifact.stage_id()
    }

    pub(crate) fn artifact(&self) -> &FullRootArtifact {
        &self.artifact
    }
}

impl From<FullRootArtifact> for OpaqueRootArtifactReceipt {
    fn from(artifact: FullRootArtifact) -> Self {
        Self { artifact }
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
            .map(OpaqueRootArtifactReceipt::from)
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
