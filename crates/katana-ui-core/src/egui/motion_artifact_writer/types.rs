use serde::Serialize;
use std::path::PathBuf;

/// KUC-owned motion artifact writer.
#[derive(Debug, Default, Clone, Copy)]
pub struct MotionArtifactWriter;

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
            fps_numerator: super::constants::DEFAULT_FPS_NUMERATOR,
            fps_denominator: super::constants::DEFAULT_FPS_DENOMINATOR,
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
    pub ffmpeg_path: String,
    pub ffmpeg_version: String,
    pub encoder: &'static str,
    pub muxer: &'static str,
    pub decoded_frame_count: usize,
    pub decoded_width: u32,
    pub decoded_height: u32,
    pub source_frame_hashes: Vec<String>,
    pub decoded_frame_hashes: Vec<String>,
    pub canonical_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotionArtifact {
    manifest: MotionArtifactManifest,
    manifest_path: PathBuf,
}

/// Original viewport dimensions for one KUC-issued source frame.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct VariableViewportSourceViewport {
    pub width: u32,
    pub height: u32,
}

/// Opaque semantic evidence bound to the variable-viewport artifact.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VariableViewportSemanticEvidence {
    pub artifact_sha256: String,
    pub root_record_hash: String,
    pub root_record_hashes: Vec<String>,
    pub star_scalar_sequence: Vec<u32>,
    pub ime_preedit_event_seen: bool,
    pub ime_commit_event_seen: bool,
    pub hit_test_count: usize,
    pub accesskit_snapshot_hash: String,
}

/// Manifest for a normalized variable-viewport motion artifact.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VariableViewportMotionArtifactManifest {
    pub schema: &'static str,
    pub source_frame_count: usize,
    pub decoded_frame_count: usize,
    pub width: u32,
    pub height: u32,
    pub source_viewports: Vec<VariableViewportSourceViewport>,
    pub source_png_sha256: Vec<String>,
    pub source_frame_hashes: Vec<String>,
    pub decoded_frame_hashes: Vec<String>,
    pub root_record_hashes: Vec<String>,
    pub semantic_evidence: VariableViewportSemanticEvidence,
    pub frame_sequence_sha256: String,
    pub gif_path: String,
    pub gif_sha256: String,
    pub mp4_path: String,
    pub mp4_sha256: String,
    pub ffmpeg_path: String,
    pub ffmpeg_version: String,
    pub encoder: &'static str,
    pub muxer: &'static str,
    pub canonical_sha256: String,
}

/// A KUC-owned artifact generated from variable viewport receipts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableViewportMotionArtifact {
    manifest: VariableViewportMotionArtifactManifest,
    manifest_path: PathBuf,
}

impl VariableViewportMotionArtifact {
    pub(super) fn from_parts(
        manifest: VariableViewportMotionArtifactManifest,
        manifest_path: PathBuf,
    ) -> Self {
        Self {
            manifest,
            manifest_path,
        }
    }

    #[must_use]
    pub fn manifest(&self) -> &VariableViewportMotionArtifactManifest {
        &self.manifest
    }

    #[must_use]
    pub fn manifest_path(&self) -> &std::path::Path {
        &self.manifest_path
    }
}

impl MotionArtifact {
    pub(super) fn from_parts(manifest: MotionArtifactManifest, manifest_path: PathBuf) -> Self {
        Self {
            manifest,
            manifest_path,
        }
    }

    #[must_use]
    pub fn manifest(&self) -> &MotionArtifactManifest {
        &self.manifest
    }

    #[must_use]
    pub fn manifest_path(&self) -> &std::path::Path {
        &self.manifest_path
    }
}
