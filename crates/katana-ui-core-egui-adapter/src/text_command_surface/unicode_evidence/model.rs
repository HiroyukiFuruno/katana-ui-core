use super::constants::RGBA_CHANNEL_COUNT;
use katana_ui_core::render_model::UiRect;
use katana_ui_core_text_raster::{
    PlatformColorEmojiFaceRecord, PlatformFontCatalogPolicy, PlatformFontProfile,
    PlatformTextRasterConfig,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct KucUnicodeColorGlyphEvidenceCapture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucUnicodeColorGlyphEvidenceOptions {
    pub root_identity: String,
    pub config: PlatformTextRasterConfig,
}

impl Default for KucUnicodeColorGlyphEvidenceOptions {
    fn default() -> Self {
        Self {
            root_identity: "kuc.unicode-color-glyph-evidence".to_string(),
            config: PlatformTextRasterConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucUnicodeColorGlyphEvidenceInput {
    pub profile: PlatformFontProfile,
    pub catalog_policy: PlatformFontCatalogPolicy,
    pub face: PlatformColorEmojiFaceRecord,
    pub final_text: String,
    pub ime: KucImeTraceEvidence,
    pub caret: KucCaretObservation,
    pub hit_tests: Vec<KucHitTestObservation>,
    pub star_crop: KucRgbaCropEvidence,
    pub control_crop: KucRgbaCropEvidence,
    pub accesskit_text_snapshot_hash: String,
    pub root_frame_hash: String,
    pub root_record_hash: String,
    pub root_rgba_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucImeTraceEvidence {
    pub preedit_scalars: Vec<u32>,
    pub commit_scalars: Vec<u32>,
    pub preedit_event_seen: bool,
    pub commit_event_seen: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KucCaretObservation {
    pub bounds: KucBounds,
}

impl KucCaretObservation {
    #[must_use]
    pub fn from_ui_rect(bounds: UiRect) -> Self {
        Self {
            bounds: KucBounds::from_ui_rect(bounds),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucHitTestObservation {
    pub target: String,
    pub query_x: u32,
    pub query_y: u32,
    pub byte_start: usize,
    pub byte_end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct KucBounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl KucBounds {
    #[must_use]
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[must_use]
    pub fn from_ui_rect(rect: UiRect) -> Self {
        Self::new(
            rect.x.max(0) as u32,
            rect.y.max(0) as u32,
            rect.width,
            rect.height,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucRgbaCropEvidence {
    pub bounds: KucBounds,
    pub pixels: Vec<[u8; RGBA_CHANNEL_COUNT]>,
}

impl KucRgbaCropEvidence {
    #[must_use]
    pub fn new(bounds: KucBounds, pixels: Vec<[u8; RGBA_CHANNEL_COUNT]>) -> Self {
        Self { bounds, pixels }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucUnicodeColorGlyphEvidenceProfile {
    pub profile_id: String,
    pub catalog_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct KucUnicodeColorGlyphEvidence {
    pub schema: String,
    pub schema_version: u32,
    pub profile: KucUnicodeColorGlyphEvidenceProfileArtifact,
    pub catalog_face: KucColorEmojiFaceArtifact,
    pub graphemes: Vec<KucGraphemeArtifact>,
    pub ime: KucImeArtifact,
    pub caret: KucCaretArtifact,
    pub hit_tests: Vec<KucHitTestArtifact>,
    pub star: KucRgbaCropArtifact,
    pub control_star: KucRgbaCropArtifact,
    pub chromatic_pixel_delta: i64,
    pub accesskit_text_snapshot_hash: String,
    pub root_frame_hash: String,
    pub root_record_hash: String,
    pub root_rgba_hash: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub artifact_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct KucUnicodeColorGlyphEvidenceProfileArtifact {
    pub profile_id: String,
    pub catalog_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct KucColorEmojiFaceArtifact {
    pub profile_id: String,
    pub family: String,
    pub source_file_path: String,
    pub raw_file_sha256: String,
    pub catalog_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct KucGraphemeArtifact {
    pub byte_start: usize,
    pub byte_end: usize,
    pub scalar_sequence: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct KucImeArtifact {
    pub preedit_scalar_sequence: Vec<u32>,
    pub commit_scalar_sequence: Vec<u32>,
    pub preedit_event_seen: bool,
    pub commit_event_seen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct KucCaretArtifact {
    pub bounds: KucBounds,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct KucHitTestArtifact {
    pub target: String,
    pub query_x: u32,
    pub query_y: u32,
    pub byte_start: usize,
    pub byte_end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct KucRgbaCropArtifact {
    pub bounds: KucBounds,
    pub rgba_sha256: String,
    pub pixel_count: usize,
    pub chromatic_pixel_count: usize,
}

impl KucUnicodeColorGlyphEvidence {
    #[must_use = "serialize the evidence before writing it"]
    pub fn canonical_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    #[must_use]
    pub fn artifact_sha256(&self) -> &str {
        &self.artifact_sha256
    }
}
