use super::model::{KucBounds, KucHitTestObservation, KucRgbaCropEvidence};
use super::types::KucUnicodeColorGlyphEvidenceError;
use katana_ui_core_text_raster::PlatformTextRaster;

const RGBA_CHANNEL_COUNT: usize = 4;
const RED_CHANNEL_INDEX: usize = 0;
const GREEN_CHANNEL_INDEX: usize = 1;
const BLUE_CHANNEL_INDEX: usize = 2;
const ALPHA_CHANNEL_INDEX: usize = 3;

pub(super) fn find_range(text: &str, target: &str) -> Option<(usize, usize)> {
    katana_ui_core_text_raster::PlatformTextGraphemeRange::ranges(text)
        .into_iter()
        .find(|range| text.get(range.byte_start..range.byte_end) == Some(target))
        .map(|range| (range.byte_start, range.byte_end))
}

pub(super) fn bounds_for_range(
    raster: &PlatformTextRaster,
    range: (usize, usize),
) -> Result<KucBounds, KucUnicodeColorGlyphEvidenceError> {
    let bounds = raster
        .grapheme_bounds
        .iter()
        .find(|bounds| (bounds.byte_start, bounds.byte_end) == range)
        .ok_or_else(|| {
            KucUnicodeColorGlyphEvidenceError::RootTrace("raster bounds missing".into())
        })?;
    Ok(KucBounds::new(
        bounds.x.max(0.0).floor() as u32,
        bounds.y.max(0.0).floor() as u32,
        bounds.width.ceil().max(1.0) as u32,
        bounds.height.ceil().max(1.0) as u32,
    ))
}

pub(super) fn crop_for_composite(
    rgba_pixels: &[u8],
    canvas_width: u32,
    bounds: KucBounds,
) -> Result<KucRgbaCropEvidence, KucUnicodeColorGlyphEvidenceError> {
    let mut pixels = Vec::new();
    for y in bounds.y..bounds.y.saturating_add(bounds.height) {
        for x in bounds.x..bounds.x.saturating_add(bounds.width) {
            let index = usize::try_from(y)
                .ok()
                .and_then(|row| row.checked_mul(usize::try_from(canvas_width).ok()?))
                .and_then(|row| row.checked_add(usize::try_from(x).ok()?))
                .and_then(|pixel| pixel.checked_mul(RGBA_CHANNEL_COUNT))
                .ok_or_else(|| {
                    KucUnicodeColorGlyphEvidenceError::RootTrace("composite crop overflow".into())
                })?;
            let end = checked_pixel_end(index)?;
            let pixel = rgba_pixels.get(index..end).ok_or_else(|| {
                KucUnicodeColorGlyphEvidenceError::RootTrace("composite crop pixel missing".into())
            })?;
            pixels.push([
                pixel[RED_CHANNEL_INDEX],
                pixel[GREEN_CHANNEL_INDEX],
                pixel[BLUE_CHANNEL_INDEX],
                pixel[ALPHA_CHANNEL_INDEX],
            ]);
        }
    }
    Ok(KucRgbaCropEvidence::new(bounds, pixels))
}

fn checked_pixel_end(index: usize) -> Result<usize, KucUnicodeColorGlyphEvidenceError> {
    index.checked_add(RGBA_CHANNEL_COUNT).ok_or_else(|| {
        KucUnicodeColorGlyphEvidenceError::RootTrace("composite crop overflow".into())
    })
}

pub(super) fn hit_test_observation(
    target: &str,
    raster: &PlatformTextRaster,
    bounds: KucBounds,
) -> Result<KucHitTestObservation, KucUnicodeColorGlyphEvidenceError> {
    let query_x = bounds.x.saturating_add(bounds.width / 2);
    let query_y = bounds.y.saturating_add(bounds.height / 2);
    let hit = raster
        .hit_test(query_x as f32, query_y as f32)
        .ok_or_else(|| {
            KucUnicodeColorGlyphEvidenceError::RootTrace("raster hit-test missed".into())
        })?;
    Ok(KucHitTestObservation {
        target: target.to_string(),
        query_x,
        query_y,
        byte_start: hit.byte_start,
        byte_end: hit.byte_end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core_text_raster::{
        PlatformColorEmojiAvailability, PlatformColorEmojiFaceRecord,
        PlatformFontCatalogFingerprint, PlatformFontProfile, PlatformTextGraphemeBounds,
        PlatformTextRasterReport, PlatformTextRasterStats,
    };

    fn raster() -> PlatformTextRaster {
        PlatformTextRaster {
            text: "a⭐️".into(),
            width: 2,
            height: 1,
            rgba_pixels: vec![[1, 2, 3, 255], [4, 4, 4, 255]],
            grapheme_bounds: vec![PlatformTextGraphemeBounds {
                byte_start: 0,
                byte_end: 1,
                x: 0.25,
                y: 0.0,
                width: 2.25,
                height: 1.0,
            }],
            report: PlatformTextRasterReport {
                resolved_emoji_font_family: None,
                color_emoji_font_available: false,
                emoji_face: PlatformColorEmojiFaceRecord {
                    platform_profile: PlatformFontProfile::Unsupported,
                    family_identity: String::new(),
                    source_file_path: None,
                    raw_file_sha256: None,
                    catalog_fingerprint: PlatformFontCatalogFingerprint::from_bytes([0; 32]),
                    availability: PlatformColorEmojiAvailability::Unavailable(
                        katana_ui_core_text_raster::PlatformColorEmojiUnavailableReason::NoCandidates,
                    ),
                },
                cache_hit: false,
                stats: PlatformTextRasterStats::default(),
            },
        }
    }

    #[test]
    fn range_bounds_crop_and_hit_observations_cover_success_and_failure_paths() {
        assert_eq!(find_range("a⭐️", "⭐️"), Some((1, 7)));
        assert_eq!(find_range("a⭐️", "missing"), None);

        let raster = raster();
        assert_eq!(
            bounds_for_range(&raster, (0, 1)).ok(),
            Some(KucBounds::new(0, 0, 3, 1))
        );
        assert!(bounds_for_range(&raster, (1, 7)).is_err());

        let crop = crop_for_composite(&[1, 2, 3, 4, 5, 6, 7, 8], 2, KucBounds::new(0, 0, 2, 1))
            .expect("two pixels should be captured");
        assert_eq!(crop.pixels, vec![[1, 2, 3, 4], [5, 6, 7, 8]]);
        assert!(crop_for_composite(&[], 1, KucBounds::new(0, 0, 1, 1)).is_err());
        assert!(crop_for_composite(&[], u32::MAX, KucBounds::new(0, u32::MAX - 1, 1, 1),).is_err());
        assert!(checked_pixel_end(usize::MAX).is_err());

        let observed_bounds =
            bounds_for_range(&raster, (0, 1)).expect("raster bounds should exist");
        let hit = hit_test_observation("a", &raster, observed_bounds)
            .expect("center should hit the first grapheme");
        assert_eq!((hit.byte_start, hit.byte_end), (0, 1));
        assert!(hit_test_observation("missing", &raster, KucBounds::new(20, 20, 2, 2)).is_err());
    }
}
