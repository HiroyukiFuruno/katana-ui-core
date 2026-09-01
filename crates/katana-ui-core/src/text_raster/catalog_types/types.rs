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
        #[cfg(target_os = "macos")]
        return Self::MacOs;
        #[cfg(target_os = "windows")]
        return Self::Windows;
        #[cfg(target_os = "linux")]
        return Self::Linux;
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        return Self::Unsupported;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_profile_and_candidate_accessors_cover_the_catalog_value_surface() {
        let sha = PlatformFontSha256::digest(b"font");
        assert_eq!(sha.as_bytes().len(), SHA256_BYTE_COUNT);
        assert_eq!(PlatformFontSha256::from_bytes(*sha.as_bytes()), sha);
        assert_eq!(sha.to_hex().len(), SHA256_BYTE_COUNT * 2);

        let fingerprint = PlatformFontCatalogFingerprint::from_bytes([7; SHA256_BYTE_COUNT]);
        assert_eq!(fingerprint.as_bytes(), &[7; SHA256_BYTE_COUNT]);
        assert_eq!(fingerprint.to_hex(), "07".repeat(SHA256_BYTE_COUNT));

        for (profile, name, family) in [
            (
                PlatformFontProfile::MacOs,
                "macos",
                Some("Apple Color Emoji"),
            ),
            (
                PlatformFontProfile::Windows,
                "windows",
                Some("Segoe UI Emoji"),
            ),
            (
                PlatformFontProfile::Linux,
                "linux",
                Some("Noto Color Emoji"),
            ),
            (PlatformFontProfile::Unsupported, "unsupported", None),
        ] {
            assert_eq!(profile.as_str(), name);
            assert_eq!(profile.expected_emoji_family(), family);
        }
        #[cfg(target_os = "linux")]
        assert_eq!(PlatformFontProfile::current(), PlatformFontProfile::Linux);

        let candidate = PlatformEmojiFontCandidate::new(PathBuf::from("font.ttf"), "Family")
            .with_expected_raw_file_sha256(sha);
        assert_eq!(candidate.expected_family, "Family");
        assert_eq!(candidate.expected_raw_file_sha256, Some(sha));
    }
}
