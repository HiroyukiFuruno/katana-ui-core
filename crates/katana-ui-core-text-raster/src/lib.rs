mod catalog;
mod catalog_types;
mod config;
mod font_candidates;
mod layout;
mod model;
mod rasterizer;
mod surface_layout;

pub use catalog::{PlatformFontCatalog, PlatformFontCatalogStats};
pub use catalog_types::{
    PlatformColorEmojiAvailability, PlatformColorEmojiError, PlatformColorEmojiFaceRecord,
    PlatformColorEmojiFaceResolver, PlatformColorEmojiUnavailableReason,
    PlatformEmojiFontCandidate, PlatformEmojiFontLoadError, PlatformEmojiFontLoader,
    PlatformEmojiFontObservation, PlatformFontCatalogError, PlatformFontCatalogFingerprint,
    PlatformFontCatalogPolicy, PlatformFontProfile, PlatformFontSha256,
};
pub use config::PlatformTextRasterConfig;
pub use model::{
    PlatformTextGraphemeAdvance, PlatformTextGraphemeBounds, PlatformTextGraphemeRange,
    PlatformTextHit, PlatformTextMetrics, PlatformTextMetricsFrame, PlatformTextMetricsRequest,
    PlatformTextRaster, PlatformTextRasterCrop, PlatformTextRasterError, PlatformTextRasterReport,
    PlatformTextRasterRequest, PlatformTextRasterStats,
};
pub use rasterizer::PlatformTextRasterizer;

#[cfg(test)]
mod catalog_contract_tests;
#[cfg(test)]
mod surface_layout_tests;
#[cfg(test)]
mod tests;
