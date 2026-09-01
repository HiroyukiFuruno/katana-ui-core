use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MotionArtifactError {
    EmptySequence,
    FrameCount {
        expected: usize,
        actual: usize,
    },
    DuplicateStage(String),
    StaleStage {
        expected: String,
        actual: String,
    },
    MissingPng(PathBuf),
    MissingProvenance(PathBuf),
    InvalidPng {
        path: PathBuf,
        reason: String,
    },
    WrongDimensions {
        path: PathBuf,
        expected: (u32, u32),
        actual: (u32, u32),
    },
    BadPngSha {
        path: PathBuf,
    },
    EmptyPixels(PathBuf),
    InvalidSettings,
    Encoder(String),
    Io(String),
    Json(String),
}

impl std::fmt::Display for MotionArtifactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySequence => f.write_str("motion receipt sequence is empty"),
            Self::FrameCount { expected, actual } => {
                write!(f, "expected {expected} frames, got {actual}")
            }
            Self::DuplicateStage(stage) => write!(f, "duplicate stage: {stage}"),
            Self::StaleStage { expected, actual } => {
                write!(f, "stale stage: expected {expected}, got {actual}")
            }
            Self::MissingPng(path) => write!(f, "missing PNG: {}", path.display()),
            Self::MissingProvenance(path) => {
                write!(f, "missing root provenance: {}", path.display())
            }
            Self::InvalidPng { path, reason } => {
                write!(f, "invalid PNG {}: {reason}", path.display())
            }
            Self::WrongDimensions {
                path,
                expected,
                actual,
            } => write!(
                f,
                "wrong dimensions {}: expected {expected:?}, got {actual:?}",
                path.display()
            ),
            Self::BadPngSha { path } => write!(f, "PNG SHA-256 mismatch: {}", path.display()),
            Self::EmptyPixels(path) => write!(f, "PNG has no non-empty pixels: {}", path.display()),
            Self::InvalidSettings => f.write_str("invalid motion artifact settings"),
            Self::Encoder(error) => write!(f, "motion encoder failure: {error}"),
            Self::Io(error) => write!(f, "motion artifact I/O failure: {error}"),
            Self::Json(error) => write!(f, "motion manifest failure: {error}"),
        }
    }
}

impl std::error::Error for MotionArtifactError {}

#[cfg(test)]
mod tests {
    use super::MotionArtifactError;

    #[test]
    fn motion_artifact_error_display_is_covered() {
        let errors = [
            MotionArtifactError::EmptySequence.to_string(),
            MotionArtifactError::FrameCount {
                expected: 3,
                actual: 1,
            }
            .to_string(),
            MotionArtifactError::DuplicateStage("stage-0".to_owned()).to_string(),
            MotionArtifactError::StaleStage {
                expected: "frame-000".to_owned(),
                actual: "frame-001".to_owned(),
            }
            .to_string(),
            MotionArtifactError::MissingPng(std::path::PathBuf::from("/tmp/a")).to_string(),
            MotionArtifactError::MissingProvenance(std::path::PathBuf::from("/tmp/a.json"))
                .to_string(),
            MotionArtifactError::InvalidPng {
                path: "/tmp/a.png".into(),
                reason: "bad".to_owned(),
            }
            .to_string(),
            MotionArtifactError::WrongDimensions {
                path: "/tmp/a.png".into(),
                expected: (1, 1),
                actual: (2, 2),
            }
            .to_string(),
            MotionArtifactError::BadPngSha {
                path: "/tmp/a.png".into(),
            }
            .to_string(),
            MotionArtifactError::EmptyPixels("/tmp/a.png".into()).to_string(),
            MotionArtifactError::InvalidSettings.to_string(),
            MotionArtifactError::Encoder("ffmpeg".to_owned()).to_string(),
            MotionArtifactError::Io("io".to_owned()).to_string(),
            MotionArtifactError::Json("json".to_owned()).to_string(),
        ];
        assert!(errors[0].contains("empty"));
        assert!(errors[1].contains("expected 3"));
        assert!(errors[2].contains("duplicate stage"));
        assert!(errors[3].contains("stale stage"));
        assert!(errors[4].contains("missing PNG"));
        assert!(errors[5].contains("missing root provenance"));
        assert!(errors[6].contains("invalid PNG"));
        assert!(errors[7].contains("wrong dimensions"));
        assert!(errors[8].contains("PNG SHA-256 mismatch"));
        assert!(errors[9].contains("PNG has no non-empty pixels"));
        assert!(errors[10].contains("invalid motion artifact settings"));
        assert!(errors[11].contains("motion encoder failure"));
        assert!(errors[12].contains("motion artifact I/O failure"));
        assert!(errors[13].contains("motion manifest failure"));
    }
}
