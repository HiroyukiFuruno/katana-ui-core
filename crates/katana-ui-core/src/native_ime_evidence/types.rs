use serde::{Deserialize, Serialize};

const RGBA_CHANNEL_COUNT: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeImeEvidenceArtifact {
    pub schema_version: String,
    pub revision: String,
    pub platform: String,
    pub run: NativeRunBinding,
    pub ime: NativeImeTranscript,
    pub scalar_sequence: Vec<u32>,
    pub rgba_crop: RgbaCropObservation,
    pub observations: NativeObservationHashes,
    pub frame_observations: NativeFrameObservations,
    pub artifact_sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeRunBinding {
    pub run_id: String,
    pub challenge: String,
    pub producer_id: String,
    pub runner_id: String,
    pub input_method: String,
    pub origin: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeImeTranscript {
    pub preedit: String,
    pub commit: String,
    pub preedit_sequence: u64,
    pub commit_sequence: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RgbaCropObservation {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<[u8; RGBA_CHANNEL_COUNT]>,
    pub control_width: u32,
    pub control_height: u32,
    pub control_pixels: Vec<[u8; RGBA_CHANNEL_COUNT]>,
    pub has_colored_pixels: bool,
    pub rgba_sha256: String,
    pub control_rgba_sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeObservationHashes {
    pub measurement_sha256: String,
    pub caret_sha256: String,
    pub hit_test_sha256: String,
    pub accesskit_sha256: String,
    pub frame_sha256: String,
    pub root_sha256: String,
    pub receipt_sha256: String,
    pub font_file_sha256: String,
    pub font_catalog_sha256: String,
    pub root_id: String,
    pub frame_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HitTestObservation {
    pub target: String,
    pub target_bounds: Bounds,
    pub range_start: u32,
    pub range_end: u32,
    pub query_x: u32,
    pub query_y: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessKitObservation {
    pub role: String,
    pub value: String,
    pub bounds: Bounds,
    pub node_id: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeFrameObservations {
    pub final_text: String,
    pub committed_text: String,
    pub committed_range_start: u32,
    pub committed_range_end: u32,
    pub measurement_width: u32,
    pub measurement_height: u32,
    pub caret: Bounds,
    pub hit_tests: Vec<HitTestObservation>,
    pub accesskit: AccessKitObservation,
    pub root_id: String,
    pub frame_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeImeVerificationExpectations {
    pub revision: String,
    pub platform: String,
    pub run_id: String,
    pub challenge: String,
    pub producer_id: String,
    pub runner_id: String,
    pub attested_artifact_sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeImeVerificationError {
    MissingField(&'static str),
    BindingMismatch(&'static str),
    UnsupportedPlatform,
    InvalidSchema,
    InvalidOrigin,
    InvalidTranscript,
    InvalidScalars,
    InvalidCrop,
    InvalidObservation(&'static str),
    InvalidHash(&'static str),
    DigestMismatch,
    Serialization(String),
}
