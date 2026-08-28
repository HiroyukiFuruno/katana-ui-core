use super::policy::PlatformFontCatalogPolicy;
use super::resolver_types::{
    PlatformColorEmojiAvailability, PlatformColorEmojiError, PlatformColorEmojiFaceRecord,
    PlatformColorEmojiUnavailableReason, PlatformEmojiFontLoadError, PlatformEmojiFontLoader,
};
use super::types::{PlatformFontCatalogFingerprint, PlatformFontProfile};

pub struct PlatformColorEmojiFaceResolver;

impl PlatformColorEmojiFaceResolver {
    pub fn resolve<L: PlatformEmojiFontLoader>(
        policy: &PlatformFontCatalogPolicy,
        loader: &mut L,
    ) -> PlatformColorEmojiFaceRecord {
        let fingerprint = policy.fingerprint();
        let expected_family = policy
            .platform_profile
            .expected_emoji_family()
            .unwrap_or_default()
            .to_string();
        if policy.platform_profile == PlatformFontProfile::Unsupported {
            return unavailable_record(
                policy,
                expected_family,
                fingerprint,
                PlatformColorEmojiUnavailableReason::UnsupportedPlatformProfile,
            );
        }
        if policy.emoji_candidates.is_empty() {
            return unavailable_record(
                policy,
                expected_family,
                fingerprint,
                PlatformColorEmojiUnavailableReason::NoCandidates,
            );
        }

        let mut missing_candidates = Vec::new();
        let mut last_error = None;
        for candidate in &policy.emoji_candidates {
            match loader.load(candidate) {
                Ok(observation) => {
                    if policy.platform_profile == PlatformFontProfile::Linux
                        && candidate.expected_raw_file_sha256.is_none()
                    {
                        last_error = Some(PlatformColorEmojiError::MissingExpectedHash {
                            source_file_path: candidate.source_file_path.clone(),
                            platform_profile: policy.platform_profile,
                        });
                        continue;
                    }
                    if let Some(expected) = candidate.expected_raw_file_sha256
                        && expected != observation.raw_file_sha256
                    {
                        last_error = Some(PlatformColorEmojiError::HashMismatch {
                            source_file_path: candidate.source_file_path.clone(),
                            expected,
                            actual: observation.raw_file_sha256,
                        });
                        continue;
                    }
                    if candidate.expected_family != observation.actual_family {
                        last_error = Some(PlatformColorEmojiError::FamilyMismatch {
                            source_file_path: candidate.source_file_path.clone(),
                            expected: candidate.expected_family.clone(),
                            actual: observation.actual_family,
                        });
                        continue;
                    }
                    return PlatformColorEmojiFaceRecord {
                        platform_profile: policy.platform_profile,
                        family_identity: candidate.expected_family.clone(),
                        source_file_path: Some(observation.source_file_path),
                        raw_file_sha256: Some(observation.raw_file_sha256),
                        catalog_fingerprint: fingerprint,
                        availability: PlatformColorEmojiAvailability::Resolved,
                    };
                }
                Err(PlatformEmojiFontLoadError::Missing { source_file_path }) => {
                    missing_candidates.push(source_file_path);
                }
                Err(error) => {
                    last_error = Some(PlatformColorEmojiError::CandidateLoad {
                        source_file_path: candidate.source_file_path.clone(),
                        error,
                    });
                }
            }
        }

        if let Some(error) = last_error {
            return error_record(policy, expected_family, fingerprint, error);
        }
        unavailable_record(
            policy,
            expected_family,
            fingerprint,
            PlatformColorEmojiUnavailableReason::MissingCandidates {
                source_file_paths: missing_candidates,
            },
        )
    }
}

fn unavailable_record(
    policy: &PlatformFontCatalogPolicy,
    family_identity: String,
    catalog_fingerprint: PlatformFontCatalogFingerprint,
    reason: PlatformColorEmojiUnavailableReason,
) -> PlatformColorEmojiFaceRecord {
    PlatformColorEmojiFaceRecord {
        platform_profile: policy.platform_profile,
        family_identity,
        source_file_path: None,
        raw_file_sha256: None,
        catalog_fingerprint,
        availability: PlatformColorEmojiAvailability::Unavailable(reason),
    }
}

fn error_record(
    policy: &PlatformFontCatalogPolicy,
    family_identity: String,
    catalog_fingerprint: PlatformFontCatalogFingerprint,
    error: PlatformColorEmojiError,
) -> PlatformColorEmojiFaceRecord {
    PlatformColorEmojiFaceRecord {
        platform_profile: policy.platform_profile,
        family_identity,
        source_file_path: None,
        raw_file_sha256: None,
        catalog_fingerprint,
        availability: PlatformColorEmojiAvailability::Error(error),
    }
}
