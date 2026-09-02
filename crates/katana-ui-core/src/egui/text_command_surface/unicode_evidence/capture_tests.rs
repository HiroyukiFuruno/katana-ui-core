use super::*;
use crate::text_raster::{
    PlatformEmojiFontCandidate, PlatformFontCatalog, PlatformFontCatalogPolicy,
    PlatformFontProfile, PlatformFontSha256, PlatformTextRasterConfig,
};
use std::fs;
use std::path::PathBuf;

fn pinned_system_font_catalog() -> Option<(PlatformFontCatalogPolicy, PlatformFontCatalog)> {
    let policy = PlatformFontCatalogPolicy::current();
    let catalog = PlatformFontCatalog::new(policy.clone());
    let mut candidates = Vec::<PlatformEmojiFontCandidate>::new();
    for candidate in catalog.policy().emoji_candidates.iter() {
        let Some(bytes) = fs::read(&candidate.source_file_path).ok() else {
            continue;
        };
        let hash = PlatformFontSha256::digest(&bytes);
        candidates.push(
            PlatformEmojiFontCandidate::new(
                candidate.source_file_path.clone(),
                &candidate.expected_family,
            )
            .with_expected_raw_file_sha256(hash),
        );
        if candidates.len() == 1 {
            break;
        }
    }

    if candidates.is_empty() {
        return None;
    }

    let policy = PlatformFontCatalogPolicy::new(
        policy.platform_profile,
        policy.proportional_candidates,
        policy.monospace_candidates,
        candidates,
    );
    let catalog = PlatformFontCatalog::new(policy.clone());
    if catalog.emoji_face().is_available() {
        Some((policy, catalog))
    } else {
        None
    }
}

fn invalid_font_file() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "kuc-invalid-unicode-evidence-font-{}.ttf",
        std::process::id()
    ));
    fs::write(&path, b"not-a-font").expect("failed writing invalid font fixture");
    path
}

#[test]
fn required_range_finds_existing_target() {
    assert_eq!(
        required_range("a⭐️b", "⭐️").expect("grapheme range exists"),
        (1, 7)
    );
}

#[test]
fn required_range_reports_missing_target() {
    assert!(matches!(
        required_range("hello", "missing"),
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(_))
    ));
}

#[test]
fn capture_composite_validation_rejects_empty_pixels_and_accepts_real_rgba() {
    assert!(matches!(
        ensure_non_empty_composite(&[]),
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "retained root produced an empty RGBA composite"
    ));
    assert!(ensure_non_empty_composite(&[1, 2, 3, 4]).is_ok());
}

#[test]
fn capture_fails_closed_when_the_runtime_face_is_not_pinned() {
    let result = KucUnicodeColorGlyphEvidenceCapture::capture(
        KucUnicodeColorGlyphEvidenceOptions::default(),
    );
    if let Err(error) = result {
        assert!(matches!(
            error,
            KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. }
                | KucUnicodeColorGlyphEvidenceError::ColorEmojiFaceError { .. }
                | KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. }
                | KucUnicodeColorGlyphEvidenceError::RootTrace(_)
                | KucUnicodeColorGlyphEvidenceError::Raster(_)
        ));
    }
}

#[test]
fn capture_rejects_unavailable_runtime_emoji_face() {
    let path = PathBuf::from("/tmp/kuc-unicode-evidence-missing-font.ttf");
    let candidate = PlatformEmojiFontCandidate::new(path.clone(), "Missing");
    let policy = PlatformFontCatalogPolicy::new(
        PlatformFontProfile::current(),
        vec![],
        vec![],
        vec![candidate],
    );
    let catalog = PlatformFontCatalog::new(policy.clone());
    let error = ensure_face_is_resolved_and_pinned(&catalog, &policy);
    assert!(matches!(
        error,
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. })
    ));
}

#[test]
fn capture_rejects_error_runtime_emoji_face() {
    let path = invalid_font_file();
    let hash =
        PlatformFontSha256::digest(&fs::read(&path).expect("failed reading invalid font fixture"));
    let candidate = PlatformEmojiFontCandidate::new(path.clone(), "Invalid font")
        .with_expected_raw_file_sha256(hash);
    let policy = PlatformFontCatalogPolicy::new(
        PlatformFontProfile::current(),
        Vec::new(),
        Vec::new(),
        vec![candidate],
    );
    let catalog = PlatformFontCatalog::new(policy.clone());
    let error = ensure_face_is_resolved_and_pinned(&catalog, &policy);
    assert!(matches!(
        error,
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiFaceError { .. })
    ));
    let _ = fs::remove_file(&path);
}

#[test]
fn capture_rejects_a_resolved_face_when_the_supplied_policy_does_not_pin_it() {
    let Some((catalog_policy, catalog)) = pinned_system_font_catalog() else {
        return;
    };
    let mut unpinned_policy = catalog_policy.clone();
    unpinned_policy.emoji_candidates[0].expected_raw_file_sha256 =
        Some(PlatformFontSha256::digest(b"different bytes"));
    assert!(matches!(
        ensure_face_is_resolved_and_pinned(&catalog, &unpinned_policy),
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnpinned { .. })
    ));
}

#[test]
fn capture_extract_accesskit_text_input_rejects_missing_text_input_node() {
    let update = egui::accesskit::TreeUpdate {
        nodes: vec![],
        tree: Some(egui::accesskit::Tree {
            root: 0.into(),
            toolkit_name: None,
            toolkit_version: None,
        }),
        tree_id: egui::accesskit::TreeId::ROOT,
        focus: 0.into(),
    };
    let error = extract_accesskit_text_input(&update).expect_err("multiline input is mandatory");
    assert!(matches!(
        error,
        KucUnicodeColorGlyphEvidenceError::MissingAccessKitNode
    ));
}

#[test]
fn capture_extract_accesskit_text_input_rejects_input_without_value() {
    let update = egui::accesskit::TreeUpdate {
        nodes: vec![(
            egui::accesskit::NodeId(1),
            egui::accesskit::Node::new(egui::accesskit::Role::MultilineTextInput),
        )],
        tree: Some(egui::accesskit::Tree {
            root: 0.into(),
            toolkit_name: None,
            toolkit_version: None,
        }),
        tree_id: egui::accesskit::TreeId::ROOT,
        focus: 0.into(),
    };
    let error = extract_accesskit_text_input(&update).expect_err("value is required");
    assert!(matches!(
        error,
        KucUnicodeColorGlyphEvidenceError::InvalidAccessKitNode { reason } if reason == "multiline text input value is missing"
    ));
}

#[test]
fn capture_extract_accesskit_text_input_rejects_input_without_bounds() {
    let mut node = egui::accesskit::Node::new(egui::accesskit::Role::MultilineTextInput);
    node.set_value("fixture");

    let update = egui::accesskit::TreeUpdate {
        nodes: vec![(egui::accesskit::NodeId(1), node)],
        tree: Some(egui::accesskit::Tree {
            root: 0.into(),
            toolkit_name: None,
            toolkit_version: None,
        }),
        tree_id: egui::accesskit::TreeId::ROOT,
        focus: 0.into(),
    };
    let error = extract_accesskit_text_input(&update).expect_err("bounds are required");
    assert!(matches!(
        error,
        KucUnicodeColorGlyphEvidenceError::InvalidAccessKitNode { reason } if reason == "multiline text input bounds are missing"
    ));
}

#[test]
fn capture_runs_through_trace_flow_when_font_is_pinned() {
    let Some((policy, catalog)) = pinned_system_font_catalog() else {
        return;
    };
    let config = PlatformTextRasterConfig::default().with_emoji_candidate_sha256(
        policy
            .emoji_candidates
            .iter()
            .filter_map(|candidate| candidate.expected_raw_file_sha256)
            .collect::<Vec<_>>(),
    );
    let options = KucUnicodeColorGlyphEvidenceOptions {
        config,
        ..Default::default()
    };
    assert!(ensure_face_is_resolved_and_pinned(&catalog, &policy).is_ok());
    let evidence = KucUnicodeColorGlyphEvidenceCapture::capture(options)
        .expect("pinned font from fontdb should be usable for capture");
    assert_eq!(
        evidence.profile.profile_id,
        PlatformFontProfile::current().as_str()
    );
    assert_eq!(
        evidence.root_record_hash, evidence.root_frame_hash,
        "semantic evidence must bind to the enclosing root record used by opaque receipts"
    );
    assert!(!evidence.accesskit_text_input.value.is_empty());
}
