use super::*;

#[test]
fn find_range_finds_matching_grapheme() {
    assert_eq!(find_range("a⭐️b", "⭐️"), Some((1, 7)));
}

#[test]
fn find_range_returns_none_for_missing_grapheme() {
    assert_eq!(find_range("abc", "⭐️"), None);
}

#[test]
fn bounds_for_range_accepts_exact_raster_grapheme_only() {
    let font = crate::theme::FontToken {
        name: "coverage-test".to_owned(),
        family: crate::theme::FontFamily::Proportional,
        size: 14.0,
        weight: 400,
    };
    let request =
        crate::text_raster::PlatformTextRasterRequest::from_text("⭐️", font, [255, 255, 255, 255]);
    let mut rasterizer = crate::text_raster::PlatformTextRasterizer::new(
        crate::text_raster::PlatformTextRasterConfig::default(),
    );
    let raster = rasterizer
        .rasterize(&request)
        .expect("default rasterizer resolves the test text");
    let bounds = bounds_for_range(&raster, (0, "⭐️".len())).expect("range is present");
    assert!(bounds.width > 0);
    assert!(bounds.height > 0);
    assert!(matches!(
        bounds_for_range(&raster, (0, 1)),
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "raster bounds missing"
    ));
}

#[test]
fn crop_for_composite_reports_overflow_and_missing_pixels() {
    let index_overflow = crop_for_composite(&[], u32::MAX, KucBounds::new(0, 1 << 31, 1, 1));
    assert!(matches!(
        index_overflow,
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "composite crop overflow"
    ));
    let missing = crop_for_composite(&[], 1, KucBounds::new(0, 0, 1, 1));
    assert!(matches!(
        missing,
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "composite crop pixel missing"
    ));
}

#[test]
fn crop_for_composite_reports_realistic_64bit_overflow_candidate_values() {
    let overflow = crop_for_composite(
        &[],
        1 << 30,
        KucBounds::new((1 << 31) - 1, u32::MAX - 1, 1, 1),
    );
    assert!(matches!(
        overflow,
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "composite crop overflow"
    ));
}

#[test]
fn crop_for_composite_preserves_rgba_pixels() {
    let crop = crop_for_composite(&[1, 2, 3, 4], 1, KucBounds::new(0, 0, 1, 1))
        .expect("one RGBA pixel is a valid crop");
    assert_eq!(crop.pixels, vec![[1, 2, 3, 4]]);
}

#[test]
fn hit_test_observation_reports_a_real_raster_miss() {
    let font = crate::theme::FontToken {
        name: "hit-test-coverage".to_owned(),
        family: crate::theme::FontFamily::Proportional,
        size: 14.0,
        weight: 400,
    };
    let request = crate::text_raster::PlatformTextRasterRequest::from_text(
        "actual raster",
        font,
        [255, 255, 255, 255],
    );
    let mut rasterizer = crate::text_raster::PlatformTextRasterizer::new(
        crate::text_raster::PlatformTextRasterConfig::default(),
    );
    let raster = rasterizer
        .rasterize(&request)
        .expect("default rasterizer resolves the hit-test fixture");
    assert!(matches!(
        hit_test_observation(
            "outside",
            &raster,
            KucBounds::new(u32::MAX - 1, u32::MAX - 1, 1, 1),
        ),
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "raster hit-test missed"
    ));
}
