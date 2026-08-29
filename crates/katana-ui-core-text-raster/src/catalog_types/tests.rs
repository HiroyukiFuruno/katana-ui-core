use super::*;

struct FakeLoader {
    result: Result<
        crate::catalog_types::PlatformEmojiFontObservation,
        crate::catalog_types::PlatformEmojiFontLoadError,
    >,
}

impl crate::catalog_types::PlatformEmojiFontLoader for FakeLoader {
    fn load(
        &mut self,
        _candidate: &crate::catalog_types::PlatformEmojiFontCandidate,
    ) -> Result<
        crate::catalog_types::PlatformEmojiFontObservation,
        crate::catalog_types::PlatformEmojiFontLoadError,
    > {
        self.result.clone()
    }
}

fn candidate(profile: PlatformFontProfile) -> crate::catalog_types::PlatformEmojiFontCandidate {
    PlatformFontCatalogPolicy::for_profile(profile).emoji_candidates[0].clone()
}

fn observation(
    candidate: &crate::catalog_types::PlatformEmojiFontCandidate,
    family: &str,
) -> crate::catalog_types::PlatformEmojiFontObservation {
    crate::catalog_types::PlatformEmojiFontObservation {
        actual_family: family.to_string(),
        source_file_path: candidate.source_file_path.clone(),
        raw_file_sha256: PlatformFontSha256::digest(b"synthetic font bytes"),
    }
}

#[test]
fn profile_policies_select_required_color_emoji_candidates() {
    let cases = [
        (
            PlatformFontProfile::MacOs,
            "Apple Color Emoji",
            "Apple Color Emoji.ttc",
        ),
        (
            PlatformFontProfile::Windows,
            "Segoe UI Emoji",
            "seguiemj.ttf",
        ),
        (
            PlatformFontProfile::Linux,
            "Noto Color Emoji",
            "NotoColorEmoji.ttf",
        ),
    ];
    for (profile, family, file_name) in cases {
        let policy = PlatformFontCatalogPolicy::for_profile(profile);
        let selected = &policy.emoji_candidates[0];
        assert_eq!(family, selected.expected_family);
        assert!(
            selected
                .source_file_path
                .to_string_lossy()
                .ends_with(file_name)
        );
    }
}

#[test]
fn resolved_record_contains_face_identity_hash_and_fingerprint() {
    let mut policy = PlatformFontCatalogPolicy::new(
        PlatformFontProfile::Linux,
        Vec::new(),
        Vec::new(),
        vec![candidate(PlatformFontProfile::Linux)],
    );
    let expected_hash = PlatformFontSha256::digest(b"synthetic font bytes");
    policy.emoji_candidates[0].expected_raw_file_sha256 = Some(expected_hash);
    let selected = policy.emoji_candidates[0].clone();
    let mut loader = FakeLoader {
        result: Ok(observation(&selected, "Noto Color Emoji")),
    };
    let record = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);

    assert!(record.is_available());
    assert_eq!(record.family_identity, "Noto Color Emoji");
    assert_eq!(record.source_file_path, Some(selected.source_file_path));
    assert_eq!(record.raw_file_sha256, Some(expected_hash));
    assert_eq!(record.catalog_fingerprint, policy.fingerprint());
}

#[test]
fn missing_face_is_typed_unavailable() {
    let policy = PlatformFontCatalogPolicy::for_profile(PlatformFontProfile::MacOs);
    let path = policy.emoji_candidates[0].source_file_path.clone();
    let mut loader = FakeLoader {
        result: Err(PlatformEmojiFontLoadError::Missing {
            source_file_path: path.clone(),
        }),
    };
    let record = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);

    assert!(matches!(
        record.availability,
        PlatformColorEmojiAvailability::Unavailable(
            PlatformColorEmojiUnavailableReason::MissingCandidates { .. }
        )
    ));
    assert_eq!(record.source_file_path, None);
}

#[test]
fn wrong_hash_is_typed_error() {
    let mut policy = PlatformFontCatalogPolicy::for_profile(PlatformFontProfile::Windows);
    let expected = PlatformFontSha256::from_bytes([0; 32]);
    policy.emoji_candidates[0].expected_raw_file_sha256 = Some(expected);
    let selected = &policy.emoji_candidates[0];
    let mut loader = FakeLoader {
        result: Ok(observation(selected, "Segoe UI Emoji")),
    };
    let record = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);

    assert!(matches!(
        record.availability,
        PlatformColorEmojiAvailability::Error(PlatformColorEmojiError::HashMismatch { .. })
    ));
}

#[test]
fn family_mismatch_is_typed_error() {
    let mut policy = PlatformFontCatalogPolicy::for_profile(PlatformFontProfile::Windows);
    let expected_hash = PlatformFontSha256::digest(b"synthetic font bytes");
    policy.emoji_candidates[0].expected_raw_file_sha256 = Some(expected_hash);
    let selected = policy.emoji_candidates[0].clone();
    let mut loader = FakeLoader {
        result: Ok(observation(&selected, "Wrong Family")),
    };
    let record = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);

    assert!(matches!(
        record.availability,
        PlatformColorEmojiAvailability::Error(PlatformColorEmojiError::FamilyMismatch { .. })
    ));
}

#[test]
fn catalog_fingerprint_is_stable_for_equal_ordered_policy() {
    let first = PlatformFontCatalogPolicy::for_profile(PlatformFontProfile::MacOs);
    let second = PlatformFontCatalogPolicy::for_profile(PlatformFontProfile::MacOs);
    assert_eq!(first.fingerprint(), second.fingerprint());

    let mut reordered = first.clone();
    reordered.proportional_candidates.reverse();
    assert_ne!(first.fingerprint(), reordered.fingerprint());
}

#[test]
fn linux_default_emoji_candidate_requires_explicit_hash() {
    let policy = PlatformFontCatalogPolicy::for_profile(PlatformFontProfile::Linux);
    assert!(
        policy.emoji_candidates[0]
            .expected_raw_file_sha256
            .is_none()
    );
    let candidate = &policy.emoji_candidates[0];
    let mut loader = FakeLoader {
        result: Ok(observation(candidate, "Noto Color Emoji")),
    };
    let record = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);

    assert!(matches!(
        record.availability,
        PlatformColorEmojiAvailability::Error(PlatformColorEmojiError::MissingExpectedHash { .. })
    ));
}
