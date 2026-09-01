use crate::text_raster::catalog_types::types::{
    PlatformEmojiFontCandidate, PlatformFontCatalogFingerprint, PlatformFontProfile,
};
use crate::text_raster::font_candidates::PlatformFontCatalogCandidates;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformFontCatalogPolicy {
    pub platform_profile: PlatformFontProfile,
    pub proportional_candidates: Vec<PathBuf>,
    pub monospace_candidates: Vec<PathBuf>,
    pub emoji_candidates: Vec<PlatformEmojiFontCandidate>,
}

impl PlatformFontCatalogPolicy {
    #[must_use]
    pub fn new(
        platform_profile: PlatformFontProfile,
        proportional_candidates: Vec<PathBuf>,
        monospace_candidates: Vec<PathBuf>,
        emoji_candidates: Vec<PlatformEmojiFontCandidate>,
    ) -> Self {
        Self {
            platform_profile,
            proportional_candidates,
            monospace_candidates,
            emoji_candidates,
        }
    }

    #[must_use]
    pub fn for_profile(platform_profile: PlatformFontProfile) -> Self {
        Self::new(
            platform_profile,
            PlatformFontCatalogCandidates::proportional_for(platform_profile),
            PlatformFontCatalogCandidates::monospace_for(platform_profile),
            PlatformFontCatalogCandidates::emoji_for(platform_profile),
        )
    }

    #[must_use]
    pub fn current() -> Self {
        Self::for_profile(PlatformFontProfile::current())
    }

    #[must_use]
    pub fn fingerprint(&self) -> PlatformFontCatalogFingerprint {
        let mut hasher = Sha256::new();
        hash_field(&mut hasher, b"kuc-platform-font-catalog-v1");
        hash_field(&mut hasher, &[self.platform_profile as u8]);
        hash_paths(&mut hasher, &self.proportional_candidates);
        hash_paths(&mut hasher, &self.monospace_candidates);
        hash_field(
            &mut hasher,
            &(self.emoji_candidates.len() as u64).to_be_bytes(),
        );
        for candidate in &self.emoji_candidates {
            hash_path(&mut hasher, &candidate.source_file_path);
            hash_field(&mut hasher, candidate.expected_family.as_bytes());
            match candidate.expected_raw_file_sha256 {
                Some(sha256) => {
                    hash_field(&mut hasher, &[1]);
                    hash_field(&mut hasher, sha256.as_bytes());
                }
                None => hash_field(&mut hasher, &[0]),
            }
        }
        PlatformFontCatalogFingerprint::from_bytes(hasher.finalize().into())
    }
}

fn hash_paths(hasher: &mut Sha256, paths: &[PathBuf]) {
    hash_field(hasher, &(paths.len() as u64).to_be_bytes());
    for path in paths {
        hash_path(hasher, path);
    }
}

fn hash_path(hasher: &mut Sha256, path: &std::path::Path) {
    hash_field(hasher, path.to_string_lossy().as_bytes());
}

fn hash_field(hasher: &mut Sha256, field: &[u8]) {
    hasher.update((field.len() as u64).to_be_bytes());
    hasher.update(field);
}
