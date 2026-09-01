use serde::Serialize;
use std::path::{Path, PathBuf};

/// Metadata returned after one KUC root frame has been encoded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullRootArtifact {
    pub(super) stage_id: String,
    pub(super) png_path: PathBuf,
    pub(super) manifest_path: PathBuf,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) root_record_hash: String,
    pub(super) pixel_hash: String,
    pub(super) png_sha256: String,
}

/// KUC-owned encoder for one already-composited root frame.
#[derive(Debug, Default, Clone, Copy)]
pub struct FullRootArtifactWriter;

#[derive(Serialize)]
pub(super) struct FullRootArtifactManifest<'a> {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) root_record_hash: &'a str,
    pub(super) pixel_hash: &'a str,
    pub(super) png_sha256: &'a str,
    pub(super) png_path: &'a Path,
}

#[derive(Debug)]
pub enum FullRootArtifactError {
    InvalidPath(&'static str),
    InvalidStageId,
    ZeroDimensions,
    DimensionOverflow,
    RgbaLength {
        expected: usize,
        actual: usize,
    },
    EmptyPixels,
    FrameHashMismatch,
    EmptyPng,
    CreateDirectory(std::io::Error),
    Encode(image::ImageError),
    ManifestEncode(serde_json::Error),
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
}
