use crate::catalog_types::{PlatformFontCatalogPolicy, PlatformFontProfile, PlatformFontSha256};
use std::path::PathBuf;

const DEFAULT_CACHE_CAPACITY: usize = 128;

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
            emoji_candidate_sha256: Vec::new(),
            cache_capacity: DEFAULT_CACHE_CAPACITY,
        }
    }
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
                    let mut candidate = crate::catalog_types::PlatformEmojiFontCandidate::new(
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
