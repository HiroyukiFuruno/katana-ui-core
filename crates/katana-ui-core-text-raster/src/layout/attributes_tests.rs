use super::*;
use crate::{
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
        attrs_for_span(&font(), &span, [u8::MAX; RGBA_CHANNEL_COUNT], &face,),
        Err(PlatformTextRasterError::ColorEmojiUnavailable { .. })
    ));
}
