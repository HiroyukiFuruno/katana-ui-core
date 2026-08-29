use super::common::{TEXT_COLOR, font, grapheme_pixels};
use crate::{
    PlatformFontCatalog, PlatformTextRasterConfig, PlatformTextRasterError,
    PlatformTextRasterRequest, PlatformTextRasterizer,
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
fn non_finite_scale_factor_is_rejected_before_pixel_buffer_allocation()
-> Result<(), Box<dyn std::error::Error>> {
    let mut request = PlatformTextRasterRequest::from_text("safe scale", font(), TEXT_COLOR);
    request.scale_factor = f32::INFINITY;
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());

    match rasterizer.rasterize(&request) {
        Ok(_) => return Err("non-finite scale factor was accepted".into()),
        Err(error) => assert_eq!(error, PlatformTextRasterError::NonFiniteLayoutExtent),
    }
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

    let error = match rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        "⭐️",
        font(),
        TEXT_COLOR,
    )) {
        Ok(_) => return Err("emoji silently fell back to SansSerif".into()),
        Err(error) => error,
    };
    let PlatformTextRasterError::ColorEmojiUnavailable { face } = error else {
        return Err("missing color emoji must return a typed availability error".into());
    };
    assert!(!face.is_available());
    assert!(matches!(
        face.availability,
        crate::PlatformColorEmojiAvailability::Unavailable(_)
    ));
    Ok(())
}
