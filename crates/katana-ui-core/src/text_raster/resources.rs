use crate::text_raster::{
    PlatformFontCatalog, PlatformTextFaceSelection, PlatformTextRasterConfig,
    PlatformTextRasterizer,
};
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
        self.rasterizer_with_face_selection(PlatformTextFaceSelection::System)
    }

    #[must_use]
    pub fn rasterizer_with_face_selection(
        &self,
        face_selection: PlatformTextFaceSelection,
    ) -> PlatformTextRasterizer {
        PlatformTextRasterizer::from_matching_catalog_with_face_selection(
            Arc::clone(&self.catalog),
            self.config.clone(),
            face_selection,
        )
    }
}
