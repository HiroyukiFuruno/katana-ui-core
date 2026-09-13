use super::ConsumerArtifactPlanError;
use crate::egui::text_command_surface::KucUnicodeColorGlyphEvidenceError;

/// 既存の実行APIの互換性を保ちつつ、Unicode証跡の原因を型で判別できるエラー。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConsumerArtifactPlanExecutionError {
    Plan(ConsumerArtifactPlanError),
    UnicodeEvidence(KucUnicodeColorGlyphEvidenceError),
}

impl ConsumerArtifactPlanExecutionError {
    pub(super) fn into_legacy(self) -> ConsumerArtifactPlanError {
        match self {
            Self::Plan(error) => error,
            Self::UnicodeEvidence(error) => {
                ConsumerArtifactPlanError::UnicodeEvidence(error.to_string())
            }
        }
    }
}

impl From<ConsumerArtifactPlanError> for ConsumerArtifactPlanExecutionError {
    fn from(error: ConsumerArtifactPlanError) -> Self {
        Self::Plan(error)
    }
}

impl std::fmt::Display for ConsumerArtifactPlanExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plan(error) => error.fmt(f),
            Self::UnicodeEvidence(error) => {
                write!(f, "consumer artifact unicode evidence failed: {error}")
            }
        }
    }
}

impl std::error::Error for ConsumerArtifactPlanExecutionError {}
