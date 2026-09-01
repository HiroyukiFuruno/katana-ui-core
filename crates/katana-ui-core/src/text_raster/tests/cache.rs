use super::common::{TEXT_COLOR, font};
use crate::text_raster::{
    PlatformTextRasterConfig, PlatformTextRasterRequest, PlatformTextRasterizer,
};

#[test]
fn identical_request_reuses_font_database_and_raster_cache()
-> Result<(), Box<dyn std::error::Error>> {
    let request = PlatformTextRasterRequest::from_text("cache 日本語", font(), TEXT_COLOR);
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let first = rasterizer.rasterize(&request)?;
    let second = rasterizer.rasterize(&request)?;

    assert!(!first.report.cache_hit);
    assert!(second.report.cache_hit);
    assert_eq!(first.rgba_pixels, second.rgba_pixels);
    assert_eq!(1, second.report.stats.font_database_loads);
    assert_eq!(1, second.report.stats.cache_entries);
    assert_eq!(1, second.report.stats.cache_hits);
    assert_eq!(1, second.report.stats.cache_misses);
    Ok(())
}

#[test]
fn bounded_cache_evicts_the_oldest_request() -> Result<(), Box<dyn std::error::Error>> {
    let mut rasterizer =
        PlatformTextRasterizer::new(PlatformTextRasterConfig::default().with_cache_capacity(1));
    let first = PlatformTextRasterRequest::from_text("first", font(), TEXT_COLOR);
    let second = PlatformTextRasterRequest::from_text("second", font(), TEXT_COLOR);

    rasterizer.rasterize(&first)?;
    rasterizer.rasterize(&second)?;
    let reloaded = rasterizer.rasterize(&first)?;

    assert!(!reloaded.report.cache_hit);
    assert_eq!(1, reloaded.report.stats.cache_entries);
    assert_eq!(0, reloaded.report.stats.cache_hits);
    assert_eq!(3, reloaded.report.stats.cache_misses);
    assert_eq!(1, reloaded.report.stats.font_database_loads);
    Ok(())
}
