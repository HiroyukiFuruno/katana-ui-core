use super::super::model::{
    KucAccessKitNodeObservation, KucCaretObservation, KucGraphemeArtifact, KucImeTraceEvidence,
    KucRgbaCropEvidence, KucUnicodeColorGlyphEvidenceInput,
};
use super::*;
use katana_ui_core::render_model::UiRect;

const SHA256_BYTE_COUNT: usize = 32;

fn input() -> KucUnicodeColorGlyphEvidenceInput {
    KucUnicodeColorGlyphEvidenceInput {
        profile: katana_ui_core_text_raster::PlatformFontProfile::Linux,
        catalog_policy: katana_ui_core_text_raster::PlatformFontCatalogPolicy::new(
            katana_ui_core_text_raster::PlatformFontProfile::Linux,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        face: katana_ui_core_text_raster::PlatformColorEmojiFaceRecord {
            platform_profile: katana_ui_core_text_raster::PlatformFontProfile::Linux,
            family_identity: "fixture".to_owned(),
            source_file_path: None,
            raw_file_sha256: None,
            catalog_fingerprint:
                katana_ui_core_text_raster::PlatformFontCatalogFingerprint::from_bytes(
                    [0; SHA256_BYTE_COUNT],
                ),
            availability: katana_ui_core_text_raster::PlatformColorEmojiAvailability::Resolved,
        },
        final_text: "text".to_owned(),
        ime: KucImeTraceEvidence {
            preedit_scalars: Vec::new(),
            commit_scalars: Vec::new(),
            preedit_event_seen: true,
            commit_event_seen: true,
        },
        caret: KucCaretObservation::from_ui_rect(UiRect::new(0, 0, 1, 1)),
        hit_tests: Vec::new(),
        star_crop: KucRgbaCropEvidence::new(
            super::super::model::KucBounds::new(0, 0, 1, 1),
            vec![[1, 0, 0, 255]],
        ),
        control_crop: KucRgbaCropEvidence::new(
            super::super::model::KucBounds::new(0, 0, 1, 1),
            vec![[1, 1, 1, 255]],
        ),
        accesskit_text_input: None,
        accesskit_text_snapshot_hash: "snapshot".to_owned(),
        root_frame_hash: "frame".to_owned(),
        root_record_hash: "record".to_owned(),
        root_rgba_hash: "rgba".to_owned(),
    }
}

#[test]
fn validation_rejects_missing_ime_events_and_invalid_caret() {
    let mut value = input();
    value.ime.preedit_event_seen = false;
    assert!(matches!(
        validate_ime_events(&value),
        Err(KucUnicodeColorGlyphEvidenceError::MissingImeEvent { kind: "preedit" })
    ));
    value.ime.preedit_event_seen = true;
    value.ime.commit_event_seen = false;
    assert!(matches!(
        validate_ime_events(&value),
        Err(KucUnicodeColorGlyphEvidenceError::MissingImeEvent { kind: "commit" })
    ));
    value.caret = KucCaretObservation::from_ui_rect(UiRect::new(0, 0, 0, 1));
    assert!(matches!(
        validate_caret(&value),
        Err(KucUnicodeColorGlyphEvidenceError::InvalidCaret)
    ));
}

#[test]
fn validation_rejects_invalid_accesskit_node_and_hashes() {
    let mut value = input();
    assert!(matches!(
        validate_accesskit_node(&value),
        Err(KucUnicodeColorGlyphEvidenceError::MissingAccessKitNode)
    ));
    value.accesskit_text_input = Some(KucAccessKitNodeObservation {
        node_id: String::new(),
        role: "wrong".to_owned(),
        value: String::new(),
        scalar_sequence: Vec::new(),
        bounds: super::super::model::KucBounds::new(0, 0, 0, 0),
    });
    assert!(matches!(
        validate_accesskit_node(&value),
        Err(KucUnicodeColorGlyphEvidenceError::InvalidAccessKitNode { .. })
    ));
    value.accesskit_text_input = Some(KucAccessKitNodeObservation {
        node_id: "node".to_owned(),
        role: "WrongRole".to_owned(),
        value: "text".to_owned(),
        scalar_sequence: grapheme::scalars("text"),
        bounds: super::super::model::KucBounds::new(0, 0, 1, 1),
    });
    assert!(matches!(
        validate_accesskit_node(&value),
        Err(KucUnicodeColorGlyphEvidenceError::AccessKitRoleMismatch { .. })
    ));
    value.accesskit_text_input.as_mut().unwrap().role = "MultilineTextInput".to_owned();
    value.accesskit_text_input.as_mut().unwrap().value = "other".to_owned();
    assert!(matches!(
        validate_accesskit_node(&value),
        Err(KucUnicodeColorGlyphEvidenceError::AccessKitValueMismatch)
    ));
    value.accesskit_text_input.as_mut().unwrap().value = "text".to_owned();
    value.accesskit_text_input.as_mut().unwrap().scalar_sequence = vec![1];
    assert!(matches!(
        validate_accesskit_node(&value),
        Err(KucUnicodeColorGlyphEvidenceError::AccessKitScalarSequenceMismatch { .. })
    ));
    value.accesskit_text_snapshot_hash.clear();
    assert!(matches!(
        validate_hashes(&value),
        Err(KucUnicodeColorGlyphEvidenceError::EmptyEvidenceHash {
            field: "accesskit_text_snapshot_hash"
        })
    ));
}

#[test]
fn validation_rejects_indistinguishable_crops() {
    let crop = super::super::model::KucRgbaCropArtifact {
        bounds: super::super::model::KucBounds::new(0, 0, 1, 1),
        rgba_sha256: "same".to_owned(),
        pixel_count: 1,
        chromatic_pixel_count: 1,
    };
    assert!(matches!(
        chromatic_pixel_delta(&crop, &crop),
        Err(KucUnicodeColorGlyphEvidenceError::IndistinguishableCrops)
    ));
    let more_chromatic = super::super::model::KucRgbaCropArtifact {
        chromatic_pixel_count: 2,
        ..crop.clone()
    };
    assert!(matches!(
        chromatic_pixel_delta(&crop, &more_chromatic),
        Err(KucUnicodeColorGlyphEvidenceError::IndistinguishableCrops)
    ));
}

#[test]
fn validation_rejects_corrupt_grapheme_artifact_ranges() {
    let corrupted_range = vec![KucGraphemeArtifact {
        byte_start: 1,
        byte_end: 2,
        scalar_sequence: grapheme::scalars("a"),
    }];
    assert!(matches!(
        grapheme::required_range("a", "a", &corrupted_range),
        Err(KucUnicodeColorGlyphEvidenceError::RequiredGraphemeMissing { target }) if target == "a"
    ));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn validation_rejects_chromatic_delta_that_exceeds_i64() {
    let star = super::super::model::KucRgbaCropArtifact {
        bounds: super::super::model::KucBounds::new(0, 0, 1, 1),
        rgba_sha256: "star".to_owned(),
        pixel_count: usize::MAX,
        chromatic_pixel_count: usize::MAX,
    };
    let control = super::super::model::KucRgbaCropArtifact {
        rgba_sha256: "control".to_owned(),
        pixel_count: 0,
        chromatic_pixel_count: 0,
        ..star.clone()
    };
    assert!(matches!(
        chromatic_pixel_delta(&star, &control),
        Err(KucUnicodeColorGlyphEvidenceError::Serialization(message))
            if message == "chromatic pixel delta exceeds i64"
    ));
}
