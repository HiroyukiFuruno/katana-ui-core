use crate::text_raster::catalog_types::{
    PlatformFontCatalogPolicy, PlatformFontProfile, PlatformFontSha256,
};
use std::path::PathBuf;

const DEFAULT_CACHE_CAPACITY: usize = 128;
#[cfg(any(test, target_os = "linux"))]
const SHA256_HEX_LENGTH: usize = 64;
#[cfg(any(test, target_os = "linux"))]
const SHA256_BYTE_LENGTH: usize = 32;
#[cfg(any(test, target_os = "linux"))]
const HEX_CHARS_PER_BYTE: usize = 2;
#[cfg(any(test, target_os = "linux"))]
const HEX_RADIX: u32 = 16;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlatformTextFaceSelection {
    #[default]
    System,
    FirstCandidate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformTextRasterConfig {
    pub proportional_candidates: Vec<PathBuf>,
    pub monospace_candidates: Vec<PathBuf>,
    pub emoji_candidates: Vec<PathBuf>,
    pub emoji_candidate_sha256: Vec<PlatformFontSha256>,
    pub cache_capacity: usize,
}

impl Default for PlatformTextRasterConfig {
    fn default() -> Self {
        let policy = PlatformFontCatalogPolicy::current();
        Self {
            proportional_candidates: policy.proportional_candidates,
            monospace_candidates: policy.monospace_candidates,
            emoji_candidates: policy
                .emoji_candidates
                .into_iter()
                .map(|candidate| candidate.source_file_path)
                .collect(),
            emoji_candidate_sha256: compile_time_emoji_candidate_sha256(),
            cache_capacity: DEFAULT_CACHE_CAPACITY,
        }
    }
}

fn compile_time_emoji_candidate_sha256() -> Vec<PlatformFontSha256> {
    #[cfg(target_os = "linux")]
    {
        /* WHY: Linux の配布フォントは更新され得るため、信頼済みビルド環境が固定した値だけを使う。 */
        option_env!("KUC_PINNED_LINUX_EMOJI_SHA256")
            .and_then(parse_sha256)
            .into_iter()
            .collect()
    }
    #[cfg(not(target_os = "linux"))]
    {
        Vec::new()
    }
}

#[cfg(any(test, target_os = "linux"))]
fn parse_sha256(value: &str) -> Option<PlatformFontSha256> {
    if value.len() != SHA256_HEX_LENGTH {
        return None;
    }
    let mut bytes = [0; SHA256_BYTE_LENGTH];
    for (index, byte) in bytes.iter_mut().enumerate() {
        let start = index * HEX_CHARS_PER_BYTE;
        *byte = u8::from_str_radix(&value[start..start + HEX_CHARS_PER_BYTE], HEX_RADIX).ok()?;
    }
    Some(PlatformFontSha256::from_bytes(bytes))
}

impl PlatformTextRasterConfig {
    #[must_use]
    pub fn with_emoji_candidate_sha256(
        mut self,
        hashes: impl IntoIterator<Item = PlatformFontSha256>,
    ) -> Self {
        self.emoji_candidate_sha256 = hashes.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_cache_capacity(mut self, cache_capacity: usize) -> Self {
        self.cache_capacity = cache_capacity.max(1);
        self
    }

    #[must_use]
    pub fn catalog_policy(&self) -> PlatformFontCatalogPolicy {
        let profile = PlatformFontProfile::current();
        PlatformFontCatalogPolicy::new(
            profile,
            self.proportional_candidates.clone(),
            self.monospace_candidates.clone(),
            self.emoji_candidates
                .iter()
                .enumerate()
                .map(|(index, candidate_path)| {
                    let expected_family = profile.expected_emoji_family().unwrap_or_default();
                    let mut candidate =
                        crate::text_raster::catalog_types::PlatformEmojiFontCandidate::new(
                            candidate_path.clone(),
                            expected_family,
                        );
                    if let Some(expected_raw_file_sha256) = self.emoji_candidate_sha256.get(index) {
                        candidate =
                            candidate.with_expected_raw_file_sha256(*expected_raw_file_sha256);
                    }
                    candidate
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_face_selection_defaults_to_system_resolution() {
        assert_eq!(
            PlatformTextFaceSelection::default(),
            PlatformTextFaceSelection::System
        );
    }

    #[test]
    fn pinned_linux_emoji_hash_parser_accepts_exact_sha256() {
        let value = "e5899ed38b8ed83e08bd3ac5de09791e9d19d288333a796de1d35ad17396f1ec";
        assert_eq!(
            parse_sha256(value).map(PlatformFontSha256::to_hex),
            Some(value.to_string())
        );
    }

    #[test]
    fn pinned_linux_emoji_hash_parser_rejects_invalid_values() {
        assert_eq!(parse_sha256("too-short"), None);
        assert_eq!(
            parse_sha256("z5899ed38b8ed83e08bd3ac5de09791e9d19d288333a796de1d35ad17396f1ec"),
            None
        );
    }
}
