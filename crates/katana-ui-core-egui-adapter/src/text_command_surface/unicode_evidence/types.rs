use katana_ui_core_text_raster::PlatformColorEmojiFaceRecord;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KucUnicodeColorGlyphEvidenceError {
    ColorEmojiUnavailable {
        face: Box<PlatformColorEmojiFaceRecord>,
    },
    ColorEmojiFaceError {
        face: Box<PlatformColorEmojiFaceRecord>,
    },
    ColorEmojiUnpinned {
        profile_id: String,
    },
    ProfileMismatch {
        expected: String,
        actual: String,
    },
    CatalogFingerprintMismatch {
        expected: String,
        actual: String,
    },
    ExpectedScalarSequenceChanged {
        target: String,
        expected: Vec<u32>,
        actual: Vec<u32>,
    },
    RequiredGraphemeMissing {
        target: String,
    },
    MissingImeEvent {
        kind: &'static str,
    },
    InvalidCaret,
    MissingAccessKitNode,
    AccessKitRoleMismatch {
        expected: String,
        actual: String,
    },
    InvalidAccessKitNode {
        reason: &'static str,
    },
    AccessKitValueMismatch,
    AccessKitScalarSequenceMismatch {
        expected: Vec<u32>,
        actual: Vec<u32>,
    },
    MissingHitTest {
        target: String,
    },
    InvalidHitTest {
        target: String,
    },
    InvalidCrop {
        target: String,
        reason: &'static str,
    },
    IndistinguishableCrops,
    EmptyEvidenceHash {
        field: &'static str,
    },
    RootTrace(String),
    Raster(String),
    Serialization(String),
}

impl fmt::Display for KucUnicodeColorGlyphEvidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for KucUnicodeColorGlyphEvidenceError {}
