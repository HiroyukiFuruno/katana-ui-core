use crate::{
    PlatformColorEmojiAvailability, PlatformColorEmojiFaceResolver, PlatformEmojiFontCandidate,
    PlatformEmojiFontLoadError, PlatformEmojiFontLoader, PlatformEmojiFontObservation,
    PlatformFontCatalog, PlatformFontSha256, PlatformTextRasterConfig, PlatformTextRasterError,
    PlatformTextRasterizer,
};
use std::sync::Arc;

struct ConfigHashLoader {
    family: String,
    source_file_path: std::path::PathBuf,
    raw_file_sha256: PlatformFontSha256,
}

impl PlatformEmojiFontLoader for ConfigHashLoader {
    fn load(
        &mut self,
        _candidate: &PlatformEmojiFontCandidate,
    ) -> Result<PlatformEmojiFontObservation, PlatformEmojiFontLoadError> {
        Ok(PlatformEmojiFontObservation {
            actual_family: self.family.clone(),
            source_file_path: self.source_file_path.clone(),
            raw_file_sha256: self.raw_file_sha256,
        })
    }
}

#[test]
fn custom_config_hash_resolves_and_matches_catalog_configuration() {
    let expected_hash = PlatformFontSha256::digest(b"config supplied font");
    let config = PlatformTextRasterConfig::default().with_emoji_candidate_sha256([expected_hash]);
    let policy = config.catalog_policy();
    if policy.emoji_candidates.is_empty() {
        return;
    }
    let candidate = policy.emoji_candidates[0].clone();
    let mut loader = ConfigHashLoader {
        family: candidate.expected_family.clone(),
        source_file_path: candidate.source_file_path.clone(),
        raw_file_sha256: expected_hash,
    };
    let record = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);

    assert!(matches!(
        record.availability,
        PlatformColorEmojiAvailability::Resolved
    ));
    assert_eq!(record.raw_file_sha256, Some(expected_hash));

    let catalog = PlatformFontCatalog::new(policy.clone());
    let rasterizer = PlatformTextRasterizer::with_catalog(Arc::new(catalog), config);
    assert!(rasterizer.is_ok());
}

#[test]
fn catalog_policy_hash_mismatch_is_rejected_by_with_catalog() {
    let config = PlatformTextRasterConfig::default()
        .with_emoji_candidate_sha256([PlatformFontSha256::from_bytes([0xAB; 32])]);
    let mut policy = config.catalog_policy();
    if policy.emoji_candidates.is_empty() {
        return;
    }
    policy.emoji_candidates[0].expected_raw_file_sha256 =
        Some(PlatformFontSha256::from_bytes([0xCD; 32]));
    let catalog = PlatformFontCatalog::new(policy);

    assert!(matches!(
        PlatformTextRasterizer::with_catalog(Arc::new(catalog), config),
        Err(PlatformTextRasterError::CatalogConfigurationMismatch)
    ));
}
