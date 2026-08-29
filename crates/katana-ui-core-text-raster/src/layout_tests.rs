use super::*;
use crate::{
    PlatformColorEmojiAvailability, PlatformColorEmojiUnavailableReason,
    PlatformFontCatalogFingerprint, PlatformFontProfile,
};

const TEST_FONT_SIZE_PX: f32 = 16.0;
const TEST_SHA256_BYTE_COUNT: usize = 32;

fn font() -> katana_ui_core::theme::FontToken {
    katana_ui_core::theme::FontToken {
        name: "coverage".to_string(),
        family: katana_ui_core::theme::FontFamily::Monospace,
        size: TEST_FONT_SIZE_PX,
        weight: REGULAR_WEIGHT,
    }
}

fn unavailable_emoji_face() -> PlatformColorEmojiFaceRecord {
    PlatformColorEmojiFaceRecord {
        platform_profile: PlatformFontProfile::Unsupported,
        family_identity: String::new(),
        source_file_path: None,
        raw_file_sha256: None,
        catalog_fingerprint: PlatformFontCatalogFingerprint::from_bytes(
            [0; TEST_SHA256_BYTE_COUNT],
        ),
        availability: PlatformColorEmojiAvailability::Unavailable(
            PlatformColorEmojiUnavailableReason::NoCandidates,
        ),
    }
}

#[test]
fn direct_layout_measure_rejects_empty_text_before_shaping() {
    let mut font_system = cosmic_text::FontSystem::new();
    let request = PlatformTextMetricsRequest::from_text("", font(), 1.0);

    assert_eq!(
        TextLayoutRasterizer::measure(&mut font_system, &request, &unavailable_emoji_face(),),
        Err(PlatformTextRasterError::EmptyText)
    );
}
