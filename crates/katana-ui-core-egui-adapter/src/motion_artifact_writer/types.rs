use std::path::PathBuf;

use serde::Serialize;

const DEFAULT_FPS_NUMERATOR: u32 = 1_000;
const DEFAULT_FPS_DENOMINATOR: u32 = 180;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotionArtifactSettings {
    pub expected_frame_count: usize,
    pub width: u32,
    pub height: u32,
    pub fps_numerator: u32,
    pub fps_denominator: u32,
}

impl MotionArtifactSettings {
    #[must_use]
    pub const fn new(expected_frame_count: usize, width: u32, height: u32) -> Self {
        Self {
            expected_frame_count,
            width,
            height,
            fps_numerator: DEFAULT_FPS_NUMERATOR,
            fps_denominator: DEFAULT_FPS_DENOMINATOR,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MotionArtifactManifest {
    pub schema: &'static str,
    pub frame_count: usize,
    pub width: u32,
    pub height: u32,
    pub frame_sequence_sha256: String,
    pub gif_path: String,
    pub gif_sha256: String,
    pub mp4_path: String,
    pub mp4_sha256: String,
    pub root_record_hashes: Vec<String>,
    pub source_artifacts: Vec<MotionSourceArtifact>,
    pub ffmpeg_path: String,
    pub ffmpeg_version: String,
    pub encoder: &'static str,
    pub muxer: &'static str,
    pub decoded_frame_count: usize,
    pub decoded_width: u32,
    pub decoded_height: u32,
    pub canonical_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MotionSourceArtifact {
    pub stage_id: String,
    pub png_path: String,
    pub provenance_path: String,
    pub provenance_sha256: String,
    pub width: u32,
    pub height: u32,
    pub root_record_hash: String,
    pub pixel_hash: String,
    pub png_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotionArtifact {
    pub(crate) manifest: MotionArtifactManifest,
    pub(crate) manifest_path: PathBuf,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MotionArtifactWriter;

impl MotionArtifact {
    #[must_use]
    pub fn manifest(&self) -> &MotionArtifactManifest {
        &self.manifest
    }

    #[must_use]
    pub fn manifest_path(&self) -> &std::path::Path {
        &self.manifest_path
    }
}

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
    SourceCanvasExceedsCanonical {
        path: PathBuf,
        source: (u32, u32),
        canonical: (u32, u32),
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
            Self::SourceCanvasExceedsCanonical {
                path,
                source,
                canonical,
            } => write!(
                f,
                "source canvas exceeds canonical canvas {}: source {source:?}, canonical {canonical:?}",
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
