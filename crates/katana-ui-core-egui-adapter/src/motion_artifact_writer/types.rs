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
    pub canonical_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotionArtifact {
    manifest: MotionArtifactManifest,
    manifest_path: PathBuf,
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
