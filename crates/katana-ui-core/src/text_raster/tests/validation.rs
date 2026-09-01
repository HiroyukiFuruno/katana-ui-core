use super::common::{TEXT_COLOR, font, grapheme_pixels};
use crate::text_raster::{
    PlatformFontCatalog, PlatformTextMetricsFrame, PlatformTextMetricsRequest,
    PlatformTextRasterConfig, PlatformTextRasterError, PlatformTextRasterRequest,
    PlatformTextRasterizer,
};
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn appending_editor_input_keeps_existing_grapheme_bounds_and_pixels_stable()
-> Result<(), Box<dyn std::error::Error>> {
    let before = "入力: かな⭐️ ";
    let after = format!("{before}Markdown");
    let mut before_request = PlatformTextRasterRequest::from_text(before, font(), TEXT_COLOR);
    before_request.max_width_px = Some(900.0);
    before_request.scale_factor = 2.0;
    let mut after_request = PlatformTextRasterRequest::from_text(&after, font(), TEXT_COLOR);
    after_request.max_width_px = before_request.max_width_px;
    after_request.scale_factor = before_request.scale_factor;
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());

    let before_raster = match rasterizer.rasterize(&before_request) {
        Ok(raster) => raster,
        Err(PlatformTextRasterError::ColorEmojiUnavailable { face }) => {
            assert!(!face.is_available());
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let after_raster = rasterizer.rasterize(&after_request)?;

    for before_bounds in &before_raster.grapheme_bounds {
        let after_bounds = after_raster
            .grapheme_bounds
            .iter()
            .find(|candidate| {
                candidate.byte_start == before_bounds.byte_start
                    && candidate.byte_end == before_bounds.byte_end
            })
            .ok_or("existing grapheme bound missing after input")?;
        assert_eq!(
            before_bounds, after_bounds,
            "input changed an existing glyph position"
        );
        assert_eq!(
            grapheme_pixels(&before_raster, before_bounds, before_request.scale_factor),
            grapheme_pixels(&after_raster, after_bounds, after_request.scale_factor),
            "input changed an existing grapheme's color glyph or pixel output"
        );
    }
    Ok(())
}

#[test]
fn non_finite_wrap_width_uses_the_safe_fallback_width() -> Result<(), Box<dyn std::error::Error>> {
    let mut request = PlatformTextRasterRequest::from_text("safe width", font(), TEXT_COLOR);
    request.max_width_px = Some(f32::INFINITY);
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());

    let raster = match rasterizer.rasterize(&request) {
        Ok(raster) => raster,
        Err(PlatformTextRasterError::ColorEmojiUnavailable { face }) => {
            assert!(!face.is_available());
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };

    assert!(raster.width > 0);
    assert!(raster.height > 0);
    Ok(())
}

#[test]
fn non_finite_scale_factor_is_rejected_before_pixel_buffer_allocation() {
    let mut request = PlatformTextRasterRequest::from_text("safe scale", font(), TEXT_COLOR);
    request.scale_factor = f32::INFINITY;
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());

    assert_eq!(
        rasterizer.rasterize(&request).unwrap_err(),
        PlatformTextRasterError::NonFiniteLayoutExtent
    );
}

#[test]
fn empty_text_is_rejected_without_changing_rasterizer_stats() {
    let request = PlatformTextRasterRequest::from_text("", font(), TEXT_COLOR);
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let before = rasterizer.stats();

    assert_eq!(
        rasterizer.rasterize(&request).unwrap_err(),
        PlatformTextRasterError::EmptyText
    );
    assert_eq!(rasterizer.stats(), before);
}

#[test]
fn metrics_reject_empty_text_and_frame_scale_drift() {
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let empty = PlatformTextMetricsRequest::from_text("", font(), 1.0);
    assert_eq!(
        rasterizer.measure_text(&empty),
        Err(PlatformTextRasterError::EmptyText)
    );

    let mut frame = PlatformTextMetricsFrame::new();
    frame.begin(1.0).expect("valid frame scale");
    let drifted = PlatformTextMetricsRequest::from_text("scale drift", font(), 2.0);
    let error = frame
        .measure_text(&mut rasterizer, &drifted)
        .expect_err("frame scale drift must fail closed");
    assert!(matches!(
        error,
        PlatformTextRasterError::MetricsFrameScaleMismatch { .. }
    ));
    assert!(error.to_string().contains("scale changed"));
}

#[test]
fn metrics_frame_reuses_records_and_utf8_ranges_clamp_to_char_boundaries()
-> Result<(), Box<dyn std::error::Error>> {
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let request = PlatformTextMetricsRequest::from_text("metrics", font(), 1.0);
    let mut frame = PlatformTextMetricsFrame::new();
    let first = frame.measure_text(&mut rasterizer, &request)?;
    let second = frame.measure_text(&mut rasterizer, &request)?;
    assert_eq!(first, second);
    assert_eq!(frame.records().len(), 1);

    assert!(crate::text_raster::PlatformTextGraphemeRange::previous("é", 1).is_none());
    assert_eq!(
        crate::text_raster::PlatformTextGraphemeRange::next("é", 1),
        Some(crate::text_raster::PlatformTextGraphemeRange {
            byte_start: 0,
            byte_end: "é".len(),
        })
    );
    Ok(())
}

#[test]
fn raster_hit_crop_color_and_hash_contracts_cover_valid_and_rejected_bounds()
-> Result<(), Box<dyn std::error::Error>> {
    let request = PlatformTextRasterRequest::from_text("ab", font(), TEXT_COLOR);
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let raster = rasterizer.rasterize(&request)?;
    let first = raster
        .grapheme_bounds
        .first()
        .ok_or("text raster must expose first grapheme bounds")?;

    let hit = raster
        .hit_test(first.x + first.width / 2.0, first.y + first.height / 2.0)
        .ok_or("point inside grapheme must hit")?;
    assert_eq!(
        (hit.byte_start, hit.byte_end),
        (first.byte_start, first.byte_end)
    );
    assert!(raster.hit_test(-1.0, -1.0).is_none());
    assert!(raster.grapheme_crop(first, f32::NAN).is_none());
    assert!(raster.grapheme_crop(first, 0.0).is_none());

    let crop = raster
        .grapheme_crop(first, request.scale_factor)
        .ok_or("visible grapheme must produce a crop")?;
    assert!(crop.width > 0);
    assert!(crop.height > 0);
    assert_eq!(crop.pixels.len(), crop.width * crop.height);
    assert_ne!(crop.sha256(), [0; 32]);
    let _ = crop.chromatic_pixel_count();

    let mut colored = raster.clone();
    colored.rgba_pixels[0] = [255, 0, 0, 255];
    assert!(colored.chromatic_pixel_count() >= 1);
    let outside = crate::text_raster::PlatformTextGraphemeBounds {
        byte_start: 0,
        byte_end: 1,
        x: colored.width as f32 + 10.0,
        y: colored.height as f32 + 10.0,
        width: 1.0,
        height: 1.0,
    };
    assert!(colored.grapheme_crop(&outside, 1.0).is_none());
    Ok(())
}

#[test]
fn missing_color_emoji_is_typed_and_does_not_replace_regular_text_family()
-> Result<(), Box<dyn std::error::Error>> {
    let missing_emoji =
        PathBuf::from("/kuc-test-font-catalog/missing-color-emoji-face-that-must-not-exist.ttf");
    let config = PlatformTextRasterConfig {
        proportional_candidates: Vec::new(),
        monospace_candidates: Vec::new(),
        emoji_candidates: vec![missing_emoji],
        emoji_candidate_sha256: Vec::new(),
        cache_capacity: 4,
    };
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
    let mut rasterizer = PlatformTextRasterizer::with_catalog(catalog, config)?;

    let regular = rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        "regular text",
        font(),
        TEXT_COLOR,
    ))?;
    assert!(!regular.text.is_empty());
    assert!(!regular.report.color_emoji_font_available);

    let error = rasterizer
        .rasterize(&PlatformTextRasterRequest::from_text(
            "⭐️",
            font(),
            TEXT_COLOR,
        ))
        .expect_err("emoji must not silently fall back to SansSerif");
    let PlatformTextRasterError::ColorEmojiUnavailable { face } = error else {
        return Err("missing color emoji must return a typed availability error".into());
    };
    assert!(
        PlatformTextRasterError::ColorEmojiUnavailable { face: face.clone() }
            .to_string()
            .contains("unavailable")
    );
    assert!(!face.is_available());
    assert!(matches!(
        face.availability,
        crate::text_raster::PlatformColorEmojiAvailability::Unavailable(_)
    ));
    Ok(())
}

#[test]
fn raster_error_display_contract_covers_every_public_variant() {
    let errors = [
        PlatformTextRasterError::EmptyText,
        PlatformTextRasterError::NonFiniteLayoutExtent,
        PlatformTextRasterError::CatalogAccess,
        PlatformTextRasterError::ColorEmojiUnavailable {
            face: Box::new(crate::text_raster::PlatformColorEmojiFaceRecord {
                platform_profile: crate::text_raster::PlatformFontProfile::Unsupported,
                family_identity: String::new(),
                source_file_path: None,
                raw_file_sha256: None,
                catalog_fingerprint: crate::text_raster::PlatformFontCatalogFingerprint::from_bytes(
                    [0; 32],
                ),
                availability: crate::text_raster::PlatformColorEmojiAvailability::Unavailable(
                    crate::text_raster::PlatformColorEmojiUnavailableReason::NoCandidates,
                ),
            }),
        },
        PlatformTextRasterError::CatalogConfigurationMismatch,
        PlatformTextRasterError::CatalogAccess,
        PlatformTextRasterError::RasterTooLarge {
            width: 10,
            height: 20,
            max_pixels: 100,
        },
    ];
    for error in errors {
        assert!(!error.to_string().is_empty());
        let _: &dyn std::error::Error = &error;
    }
}
