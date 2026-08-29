use super::model::{KucBounds, KucHitTestObservation, KucRgbaCropEvidence};
use super::types::KucUnicodeColorGlyphEvidenceError;
use katana_ui_core_text_raster::PlatformTextRaster;

const RGBA_CHANNEL_COUNT: usize = 4;

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
            let end = index.checked_add(RGBA_CHANNEL_COUNT).ok_or_else(|| {
                KucUnicodeColorGlyphEvidenceError::RootTrace("composite crop overflow".into())
            })?;
            let pixel = rgba_pixels.get(index..end).ok_or_else(|| {
                KucUnicodeColorGlyphEvidenceError::RootTrace("composite crop pixel missing".into())
            })?;
            let pixel: [u8; RGBA_CHANNEL_COUNT] = pixel.try_into().map_err(|_| {
                KucUnicodeColorGlyphEvidenceError::RootTrace(
                    "composite crop pixel width changed".into(),
                )
            })?;
            pixels.push(pixel);
        }
    }
    Ok(KucRgbaCropEvidence::new(bounds, pixels))
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

    #[test]
    fn find_range_uses_grapheme_boundaries_and_rejects_missing_text() {
        assert_eq!(find_range("a⭐️b", "⭐️"), Some((1, 7)));
        assert_eq!(find_range("a⭐️b", "⭐"), None);
    }

    #[test]
    fn bounds_for_range_requires_the_exact_raster_grapheme() {
        let font = katana_ui_core::theme::FontToken {
            name: "coverage-test".to_owned(),
            family: katana_ui_core::theme::FontFamily::Proportional,
            size: 14.0,
            weight: 400,
        };
        let request = katana_ui_core_text_raster::PlatformTextRasterRequest::from_text(
            "⭐️",
            font,
            [255, 255, 255, 255],
        );
        let mut rasterizer = katana_ui_core_text_raster::PlatformTextRasterizer::new(
            katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
        );
        let raster = rasterizer
            .rasterize(&request)
            .expect("default rasterizer resolves the test text");
        let bounds = bounds_for_range(&raster, (0, "⭐️".len())).expect("range is present");
        assert!(bounds.width > 0);
        assert!(bounds.height > 0);
        assert!(matches!(
            bounds_for_range(&raster, (0, 1)),
            Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message)) if message == "raster bounds missing"
        ));
    }

    #[test]
    fn crop_for_composite_reports_empty_and_missing_pixels() {
        let bounds = KucBounds::new(0, 0, 1, 1);
        assert!(matches!(
            crop_for_composite(&[], 1, bounds),
            Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message)) if message == "composite crop pixel missing"
        ));
        assert_eq!(
            crop_for_composite(&[1, 2, 3, 4], 1, bounds)
                .expect("one RGBA pixel is a valid crop")
                .pixels,
            vec![[1, 2, 3, 4]]
        );
    }
}
