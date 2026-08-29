use super::*;
use std::collections::VecDeque;

struct SequenceLoader {
    results: VecDeque<Result<PlatformEmojiFontObservation, PlatformEmojiFontLoadError>>,
}

impl PlatformEmojiFontLoader for SequenceLoader {
    fn load(
        &mut self,
        _candidate: &PlatformEmojiFontCandidate,
    ) -> Result<PlatformEmojiFontObservation, PlatformEmojiFontLoadError> {
        self.results
            .pop_front()
            .expect("sequence loader must have remaining result")
    }
}

fn candidate(profile: PlatformFontProfile) -> PlatformEmojiFontCandidate {
    PlatformFontCatalogPolicy::for_profile(profile).emoji_candidates[0].clone()
}

fn observation(
    candidate: &PlatformEmojiFontCandidate,
    family: &str,
) -> PlatformEmojiFontObservation {
    PlatformEmojiFontObservation {
        actual_family: family.to_string(),
        source_file_path: candidate.source_file_path.clone(),
        raw_file_sha256: PlatformFontSha256::digest(b"synthetic font bytes"),
    }
}

#[test]
fn multi_candidate_fallback_prefers_first_resolved_candidate() {
    let mut policy = PlatformFontCatalogPolicy::new(
        PlatformFontProfile::Windows,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let mut first = candidate(PlatformFontProfile::Windows);
    let mut second = candidate(PlatformFontProfile::Windows);
    first.source_file_path = std::path::PathBuf::from("/first-windows-emoji.ttf");
    second.source_file_path = std::path::PathBuf::from("/second-windows-emoji.ttf");

    let expected_hash = observation(&second, "Segoe UI Emoji").raw_file_sha256;
    let wrong_hash = PlatformFontSha256::digest(b"mismatch");
    first.expected_raw_file_sha256 = Some(wrong_hash);
    second.expected_raw_file_sha256 = Some(expected_hash);
    policy.emoji_candidates = vec![first.clone(), second.clone()];

    let mut loader = SequenceLoader {
        results: VecDeque::from([
            Ok(observation(&first, "Segoe UI Emoji")),
            Ok(observation(&second, "Segoe UI Emoji")),
        ]),
    };
    let record = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);

    assert!(record.is_available());
    assert_eq!(record.source_file_path, Some(second.source_file_path));
    assert_eq!(record.raw_file_sha256, Some(expected_hash));
}

#[test]
fn multi_candidate_loader_errors_keep_last_error_when_no_candidate_resolves() {
    let mut policy = PlatformFontCatalogPolicy::new(
        PlatformFontProfile::Windows,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let mut first = candidate(PlatformFontProfile::Windows);
    let mut second = candidate(PlatformFontProfile::Windows);
    first.source_file_path = std::path::PathBuf::from("/first-windows-emoji.ttf");
    second.source_file_path = std::path::PathBuf::from("/second-windows-emoji.ttf");
    let expected_hash = PlatformFontSha256::digest(b"expected");
    first.expected_raw_file_sha256 = Some(expected_hash);
    second.expected_raw_file_sha256 = Some(expected_hash);
    policy.emoji_candidates = vec![first, second];
    let second_error_path = std::path::PathBuf::from("/second-windows-emoji.ttf");

    let mut loader = SequenceLoader {
        results: VecDeque::from([
            Err(PlatformEmojiFontLoadError::Io {
                source_file_path: std::path::PathBuf::from("/first-windows-emoji.ttf"),
                message: "first failed".to_string(),
            }),
            Err(PlatformEmojiFontLoadError::Io {
                source_file_path: second_error_path.clone(),
                message: "second failed".to_string(),
            }),
        ]),
    };
    let record = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);

    assert!(matches!(
        record.availability,
        PlatformColorEmojiAvailability::Error(PlatformColorEmojiError::CandidateLoad {
            source_file_path,
            error: PlatformEmojiFontLoadError::Io {
                source_file_path: error_source,
                message,
            },
        }) if source_file_path == second_error_path && error_source == second_error_path && message == "second failed"
    ));
}
