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

#[derive(Serialize)]
pub(super) struct FullRootArtifactManifest<'a> {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) root_record_hash: &'a str,
    pub(super) pixel_hash: &'a str,
    pub(super) png_sha256: &'a str,
    pub(super) png_path: &'a Path,
}

pub(super) struct FrameRootMetadata<'a> {
    pub stage_id: &'a str,
    pub png_path: PathBuf,
    pub manifest_path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub root_record_hash: &'a str,
    pub pixel_hash: String,
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
