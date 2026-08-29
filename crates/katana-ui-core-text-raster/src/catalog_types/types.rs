use sha2::{Digest, Sha256};
use std::path::PathBuf;

const SHA256_BYTE_COUNT: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlatformFontSha256([u8; SHA256_BYTE_COUNT]);

impl PlatformFontSha256 {
    #[must_use]
    pub fn digest(bytes: &[u8]) -> Self {
        Self(Sha256::digest(bytes).into())
    }

    #[must_use]
    pub const fn from_bytes(bytes: [u8; SHA256_BYTE_COUNT]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; SHA256_BYTE_COUNT] {
        &self.0
    }

    #[must_use]
    pub fn to_hex(self) -> String {
        self.0.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlatformFontCatalogFingerprint([u8; SHA256_BYTE_COUNT]);

impl PlatformFontCatalogFingerprint {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; SHA256_BYTE_COUNT]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; SHA256_BYTE_COUNT] {
        &self.0
    }

    #[must_use]
    pub fn to_hex(self) -> String {
        self.0.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformFontProfile {
    MacOs,
    Windows,
    Linux,
    Unsupported,
}

impl PlatformFontProfile {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacOs => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::Unsupported => "unsupported",
        }
    }

    #[must_use]
    pub const fn current() -> Self {
        if cfg!(target_os = "macos") {
            Self::MacOs
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "linux") {
            Self::Linux
        } else {
            Self::Unsupported
        }
    }

    #[must_use]
    pub const fn expected_emoji_family(self) -> Option<&'static str> {
        match self {
            Self::MacOs => Some("Apple Color Emoji"),
            Self::Windows => Some("Segoe UI Emoji"),
            Self::Linux => Some("Noto Color Emoji"),
            Self::Unsupported => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformEmojiFontCandidate {
    pub source_file_path: PathBuf,
    pub expected_family: String,
    pub expected_raw_file_sha256: Option<PlatformFontSha256>,
}

impl PlatformEmojiFontCandidate {
    #[must_use]
    pub fn new(source_file_path: PathBuf, expected_family: impl Into<String>) -> Self {
        Self {
            source_file_path,
            expected_family: expected_family.into(),
            expected_raw_file_sha256: None,
        }
    }

    #[must_use]
    pub fn with_expected_raw_file_sha256(mut self, sha256: PlatformFontSha256) -> Self {
        self.expected_raw_file_sha256 = Some(sha256);
        self
    }
}
