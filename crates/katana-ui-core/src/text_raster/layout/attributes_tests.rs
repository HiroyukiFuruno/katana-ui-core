use super::*;
use crate::text_raster::{
    PlatformColorEmojiAvailability, PlatformColorEmojiUnavailableReason,
    PlatformFontCatalogFingerprint, PlatformFontProfile,
};

const TEST_FONT_SIZE_PX: f32 = 16.0;

fn font() -> FontToken {
    FontToken {
        name: "coverage".to_string(),
        family: FontFamily::Monospace,
        size: TEST_FONT_SIZE_PX,
        weight: REGULAR_WEIGHT,
    }
}

#[test]
fn emoji_attributes_fail_closed_without_a_resolved_family() {
    let face = PlatformColorEmojiFaceRecord {
        platform_profile: PlatformFontProfile::Unsupported,
        family_identity: String::new(),
        source_file_path: None,
        raw_file_sha256: None,
        catalog_fingerprint: PlatformFontCatalogFingerprint::from_bytes([0; 32]),
        availability: PlatformColorEmojiAvailability::Unavailable(
            PlatformColorEmojiUnavailableReason::NoCandidates,
        ),
    };
    let mut span = UiTextSpan::plain("⭐");
    span.style.emoji = true;

    assert!(matches!(
        attrs_for_span(
            &font(),
            &span,
            [u8::MAX; RGBA_CHANNEL_COUNT],
            &face,
            &ResolvedTextFaces::default(),
        ),
        Err(PlatformTextRasterError::ColorEmojiUnavailable { .. })
    ));
}

#[test]
fn first_candidate_faces_replace_generic_regular_and_monospace_families() {
    let face = PlatformColorEmojiFaceRecord {
        platform_profile: PlatformFontProfile::Unsupported,
        family_identity: String::new(),
        source_file_path: None,
        raw_file_sha256: None,
        catalog_fingerprint: PlatformFontCatalogFingerprint::from_bytes([0; 32]),
        availability: PlatformColorEmojiAvailability::Unavailable(
            PlatformColorEmojiUnavailableReason::NoCandidates,
        ),
    };
    let text_faces = ResolvedTextFaces::from_first_candidates(
        Some("KatanA proportional".to_owned()),
        Some("KatanA monospace".to_owned()),
    );
    let mut proportional_font = font();
    proportional_font.family = FontFamily::Proportional;
    let proportional_span = UiTextSpan::plain("Regular");
    let monospace_span = UiTextSpan::plain("Code");

    assert_eq!(
        attrs_for_span(
            &proportional_font,
            &proportional_span,
            [u8::MAX; RGBA_CHANNEL_COUNT],
            &face,
            &text_faces,
        )
        .expect("regular candidate family")
        .family,
        cosmic_text::Family::Name("KatanA proportional")
    );
    assert_eq!(
        attrs_for_span(
            &font(),
            &monospace_span,
            [u8::MAX; RGBA_CHANNEL_COUNT],
            &face,
            &text_faces,
        )
        .expect("monospace candidate family")
        .family,
        cosmic_text::Family::Name("KatanA monospace")
    );
}
