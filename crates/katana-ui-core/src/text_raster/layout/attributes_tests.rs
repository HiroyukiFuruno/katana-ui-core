use super::*;
use crate::text_raster::catalog::{PlatformRegularFontFace, PlatformRegularFontFaces};
use crate::text_raster::{
    PlatformColorEmojiAvailability, PlatformColorEmojiUnavailableReason,
    PlatformFontCatalogFingerprint, PlatformFontProfile,
};
use cosmic_text::{Stretch, Style as FontStyle, Weight};
use std::path::PathBuf;

const TEST_FONT_SIZE_PX: f32 = 16.0;
const TEST_FACE_INDEX: u32 = 3;
const TEST_FACE_WEIGHT: u16 = 300;

fn font() -> FontToken {
    FontToken {
        name: "coverage".to_string(),
        family: FontFamily::Monospace,
        size: TEST_FONT_SIZE_PX,
        weight: REGULAR_WEIGHT,
    }
}

fn selected_non_regular_faces() -> ResolvedTextFaces {
    let proportional = PlatformRegularFontFace {
        family: "Candidate".to_owned(),
        source_file_path: PathBuf::from("candidate.ttc"),
        index: TEST_FACE_INDEX,
        weight: TEST_FACE_WEIGHT,
        style: FontStyle::Oblique,
        stretch: Stretch::Condensed,
        selection_family: "__candidate__".to_owned(),
    };
    let monospace = PlatformRegularFontFace {
        selection_family: "__monospace__".to_owned(),
        ..proportional.clone()
    };
    ResolvedTextFaces::from_candidate_faces(PlatformRegularFontFaces {
        proportional: vec![proportional],
        monospace: vec![monospace],
    })
}

fn selected_distinct_regular_and_monospace_faces() -> ResolvedTextFaces {
    let proportional = PlatformRegularFontFace {
        family: "Candidate".to_owned(),
        source_file_path: PathBuf::from("candidate.ttc"),
        index: TEST_FACE_INDEX,
        weight: TEST_FACE_WEIGHT,
        style: FontStyle::Oblique,
        stretch: Stretch::Condensed,
        selection_family: "__candidate__".to_owned(),
    };
    let monospace = PlatformRegularFontFace {
        family: "Monospace".to_owned(),
        source_file_path: PathBuf::from("monospace.ttc"),
        index: TEST_FACE_INDEX,
        weight: 650,
        style: FontStyle::Italic,
        stretch: Stretch::Expanded,
        selection_family: "__monospace__".to_owned(),
    };
    ResolvedTextFaces::from_candidate_faces(PlatformRegularFontFaces {
        proportional: vec![proportional],
        monospace: vec![monospace],
    })
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
    for text in ["日本語", "a日本"] {
        assert_eq!(
            attrs_for_span(
                &font(),
                &UiTextSpan::plain(text),
                [u8::MAX; RGBA_CHANNEL_COUNT],
                &face,
                &text_faces,
            )
            .expect("non-ASCII generic fallback family")
            .family,
            cosmic_text::Family::Name("KatanA proportional"),
            "a generic monospace token must allow non-ASCII glyph fallback: {text}"
        );
    }
}

#[test]
fn regular_candidate_faces_do_not_replace_emoji_or_generic_fallback_contracts() {
    let resolved_emoji_face = PlatformColorEmojiFaceRecord {
        platform_profile: PlatformFontProfile::Unsupported,
        family_identity: "KatanA emoji".to_owned(),
        source_file_path: None,
        raw_file_sha256: None,
        catalog_fingerprint: PlatformFontCatalogFingerprint::from_bytes([0; 32]),
        availability: PlatformColorEmojiAvailability::Resolved,
    };
    let text_faces = ResolvedTextFaces::from_first_candidates(
        Some("KatanA proportional".to_owned()),
        Some("KatanA monospace".to_owned()),
    );
    let mut emoji_span = UiTextSpan::plain("⭐");
    emoji_span.style.emoji = true;

    let emoji_attrs = attrs_for_span(
        &font(),
        &emoji_span,
        [u8::MAX; RGBA_CHANNEL_COUNT],
        &resolved_emoji_face,
        &text_faces,
    )
    .expect("emoji face remains first-class");
    assert_eq!(
        emoji_attrs.family,
        cosmic_text::Family::Name("KatanA emoji")
    );
    assert_eq!(emoji_attrs.weight, Weight(REGULAR_WEIGHT));
    assert_eq!(emoji_attrs.style, FontStyle::Normal);
    let generic_faces = ResolvedTextFaces::default();
    let mut proportional_font = font();
    proportional_font.family = FontFamily::Proportional;
    assert_eq!(
        attrs_for_span(
            &proportional_font,
            &UiTextSpan::plain("System"),
            [u8::MAX; RGBA_CHANNEL_COUNT],
            &resolved_emoji_face,
            &generic_faces,
        )
        .expect("system regular fallback")
        .family,
        cosmic_text::Family::SansSerif
    );
}

#[test]
fn selected_candidate_face_keeps_its_attrs_for_all_latin_style_requests() {
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
    let faces = selected_non_regular_faces();
    for (bold, italic, token_weight, monospace) in [
        (false, false, 300, false),
        (true, false, 700, false),
        (false, true, 650, false),
        (false, false, 900, true),
    ] {
        let mut token = font();
        token.weight = token_weight;
        let mut span = UiTextSpan::plain("Latin");
        span.style.bold = bold;
        span.style.italic = italic;
        span.style.monospace = monospace;
        let attrs = attrs_for_span(&token, &span, [u8::MAX; RGBA_CHANNEL_COUNT], &face, &faces)
            .expect("candidate attrs");
        assert_eq!(attrs.family, cosmic_text::Family::Name("__monospace__"));
        assert_eq!(attrs.weight, Weight(300));
        assert_eq!(attrs.style, FontStyle::Oblique);
        assert_eq!(attrs.stretch, Stretch::Condensed);
    }
}

#[test]
fn non_ascii_monospace_span_uses_proportional_fallback_face_attrs() {
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
    let faces = selected_distinct_regular_and_monospace_faces();

    for text in ["ASCII", "日本語", "a日本"] {
        let mut span = UiTextSpan::plain(text);
        span.style.monospace = true;
        let attrs = attrs_for_span(&font(), &span, [u8::MAX; RGBA_CHANNEL_COUNT], &face, &faces)
            .expect("monospace candidate attrs");

        assert_eq!(
            attrs.family,
            if text.is_ascii() {
                cosmic_text::Family::Name("__monospace__")
            } else {
                cosmic_text::Family::Name("__candidate__")
            },
        );
        if text.is_ascii() {
            assert_eq!(attrs.weight, Weight(650));
            assert_eq!(attrs.style, FontStyle::Italic);
            assert_eq!(attrs.stretch, Stretch::Expanded);
        } else {
            assert_eq!(attrs.weight, Weight(TEST_FACE_WEIGHT));
            assert_eq!(attrs.style, FontStyle::Oblique);
            assert_eq!(attrs.stretch, Stretch::Condensed);
        }
    }
}
