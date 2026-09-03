use std::path::PathBuf;

use super::super::error::MotionArtifactError;

/// Error returned by the additive variable-viewport artifact writer.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableViewportMotionArtifactError {
    Motion(MotionArtifactError),
    OccupiedOutputTarget { path: PathBuf },
    InvalidSemanticEvidence(String),
    UnrelatedSemanticEvidence { root_record_hash: String },
}

impl std::fmt::Display for VariableViewportMotionArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Motion(error) => error.fmt(formatter),
            Self::OccupiedOutputTarget { path } => write!(
                formatter,
                "variable viewport output target already exists: {}",
                path.display()
            ),
            Self::InvalidSemanticEvidence(reason) => {
                write!(formatter, "invalid motion semantic evidence: {reason}")
            }
            Self::UnrelatedSemanticEvidence { root_record_hash } => write!(
                formatter,
                "motion semantic evidence root record is absent from the sequence: {root_record_hash}"
            ),
        }
    }
}

impl std::error::Error for VariableViewportMotionArtifactError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Motion(error) => Some(error),
            Self::OccupiedOutputTarget { .. }
            | Self::InvalidSemanticEvidence(_)
            | Self::UnrelatedSemanticEvidence { .. } => None,
        }
    }
}

impl From<MotionArtifactError> for VariableViewportMotionArtifactError {
    fn from(error: MotionArtifactError) -> Self {
        Self::Motion(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_viewport_error_preserves_existing_and_semantic_failures() {
        let motion = VariableViewportMotionArtifactError::from(MotionArtifactError::EmptySequence);
        assert!(motion.to_string().contains("empty"));
        assert!(std::error::Error::source(&motion).is_some());

        let occupied = VariableViewportMotionArtifactError::OccupiedOutputTarget {
            path: "/tmp/output.gif".into(),
        };
        assert!(occupied.to_string().contains("output.gif"));
        assert!(std::error::Error::source(&occupied).is_none());

        let invalid =
            VariableViewportMotionArtifactError::InvalidSemanticEvidence("missing star".into());
        assert!(invalid.to_string().contains("missing star"));
        assert!(std::error::Error::source(&invalid).is_none());

        let unrelated = VariableViewportMotionArtifactError::UnrelatedSemanticEvidence {
            root_record_hash: "other-root".into(),
        };
        assert!(unrelated.to_string().contains("other-root"));
        assert!(std::error::Error::source(&unrelated).is_none());
    }
}
