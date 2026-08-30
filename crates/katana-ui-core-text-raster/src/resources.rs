use crate::{PlatformFontCatalog, PlatformTextRasterConfig, PlatformTextRasterizer};
use std::sync::Arc;

/// Opaque text-raster resources created from one configuration.
///
/// Every rasterizer created by this value uses the exact catalog that was
/// constructed from its configuration. Callers that need to supply an
/// arbitrary catalog separately must continue to use
/// [`PlatformTextRasterizer::with_catalog`].
pub struct PlatformTextRasterResources {
    catalog: Arc<PlatformFontCatalog>,
    config: PlatformTextRasterConfig,
}

impl PlatformTextRasterResources {
    #[must_use]
    pub fn new(config: PlatformTextRasterConfig) -> Self {
        let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
        Self { catalog, config }
    }

    #[must_use]
    pub fn catalog(&self) -> Arc<PlatformFontCatalog> {
        Arc::clone(&self.catalog)
    }

    #[must_use]
    pub const fn config(&self) -> &PlatformTextRasterConfig {
        &self.config
    }

    #[must_use]
    pub fn rasterizer(&self) -> PlatformTextRasterizer {
        PlatformTextRasterizer::from_matching_catalog(
            Arc::clone(&self.catalog),
            self.config.clone(),
        )
    }
}
