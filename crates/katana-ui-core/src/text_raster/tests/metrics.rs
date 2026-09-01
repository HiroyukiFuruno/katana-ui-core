use super::common::font;
use crate::text_raster::{PlatformTextMetricsRequest, PlatformTextRasterConfig, PlatformTextRasterizer};

#[test]
fn metrics_are_deterministic_for_the_same_catalog_and_scale()
-> Result<(), Box<dyn std::error::Error>> {
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let request = PlatformTextMetricsRequest::from_text("ASCII 日本語", font(), 1.0);

    let first = rasterizer.measure_text(&request)?;
    let second = rasterizer.measure_text(&request)?;

    assert_eq!(first, second);
    Ok(())
}

#[test]
fn scale_factor_affects_resolved_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let base = rasterizer.measure_text(&PlatformTextMetricsRequest::from_text(
        "ASCII 日本語",
        font(),
        1.0,
    ))?;
    let scaled = rasterizer.measure_text(&PlatformTextMetricsRequest::from_text(
        "ASCII 日本語",
        font(),
        2.0,
    ))?;

    assert_eq!(base.text, scaled.text);
    assert_eq!(base.scale_factor, 1.0);
    assert_eq!(scaled.scale_factor, 2.0);
    assert!(scaled.ascent_px > base.ascent_px);
    assert!(scaled.descent_px > base.descent_px);
    assert!(scaled.line_height_px > base.line_height_px);
    assert!(scaled.advance_px > base.advance_px);
    Ok(())
}

#[test]
fn mixed_text_preserves_vs16_identity_without_replacement() -> Result<(), Box<dyn std::error::Error>>
{
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let with_variation = PlatformTextMetricsRequest::from_text("日本語 A ⭐️", font(), 1.0);
    let without_variation = PlatformTextMetricsRequest::from_text("日本語 A ☆", font(), 1.0);

    let variation = rasterizer.measure_text(&with_variation)?;
    let outline = rasterizer.measure_text(&without_variation)?;

    assert_eq!(variation.text, "日本語 A ⭐️");
    assert_eq!(outline.text, "日本語 A ☆");
    assert!(variation.text.contains("⭐️"));
    assert!(!variation.text.contains('☆'));
    assert_ne!(variation.text, outline.text);
    assert_ne!(variation.grapheme_advances, outline.grapheme_advances);
    Ok(())
}
