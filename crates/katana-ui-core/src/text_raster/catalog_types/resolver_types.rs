use super::types::{
    PlatformEmojiFontCandidate, PlatformFontCatalogFingerprint, PlatformFontProfile,
    PlatformFontSha256,
};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformFontCatalogError {
    FontSystemLockPoisoned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformEmojiFontObservation {
    pub actual_family: String,
    pub source_file_path: PathBuf,
    pub raw_file_sha256: PlatformFontSha256,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformEmojiFontLoadError {
    Missing {
        source_file_path: PathBuf,
    },
    Io {
        source_file_path: PathBuf,
        message: String,
    },
    InvalidFont {
        source_file_path: PathBuf,
        message: String,
    },
    FaceNotFound {
        source_file_path: PathBuf,
    },
}

pub trait PlatformEmojiFontLoader {
    fn load(
        &mut self,
        candidate: &PlatformEmojiFontCandidate,
    ) -> Result<PlatformEmojiFontObservation, PlatformEmojiFontLoadError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformColorEmojiUnavailableReason {
    UnsupportedPlatformProfile,
    NoCandidates,
    MissingCandidates { source_file_paths: Vec<PathBuf> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformColorEmojiError {
    CandidateLoad {
        source_file_path: PathBuf,
        error: PlatformEmojiFontLoadError,
    },
    SourceFileMismatch {
        expected: PathBuf,
        actual: PathBuf,
    },
    HashMismatch {
        source_file_path: PathBuf,
        expected: PlatformFontSha256,
        actual: PlatformFontSha256,
    },
    MissingExpectedHash {
        source_file_path: PathBuf,
        platform_profile: PlatformFontProfile,
    },
    FamilyMismatch {
        source_file_path: PathBuf,
        expected: String,
        actual: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformColorEmojiAvailability {
    Resolved,
    Unavailable(PlatformColorEmojiUnavailableReason),
    Error(PlatformColorEmojiError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformColorEmojiFaceRecord {
    pub platform_profile: PlatformFontProfile,
    pub family_identity: String,
    pub source_file_path: Option<PathBuf>,
    pub raw_file_sha256: Option<PlatformFontSha256>,
    pub catalog_fingerprint: PlatformFontCatalogFingerprint,
    pub availability: PlatformColorEmojiAvailability,
}

impl PlatformColorEmojiFaceRecord {
    #[must_use]
    pub fn is_available(&self) -> bool {
        matches!(self.availability, PlatformColorEmojiAvailability::Resolved)
    }

    #[must_use]
    pub fn resolved_family(&self) -> Option<&str> {
        self.is_available().then_some(self.family_identity.as_str())
    }
}
