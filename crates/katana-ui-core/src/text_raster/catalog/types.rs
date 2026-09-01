use crate::text_raster::catalog_types::{PlatformColorEmojiFaceRecord, PlatformFontCatalogPolicy};
use cosmic_text::FontSystem;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformFontCatalogStats {
    pub font_database_discoveries: usize,
    pub candidate_load_attempts: usize,
}

pub struct PlatformFontCatalog {
    pub(super) policy: PlatformFontCatalogPolicy,
    pub(super) font_system: Mutex<FontSystem>,
    pub(super) emoji_face: PlatformColorEmojiFaceRecord,
    pub(super) stats: PlatformFontCatalogStats,
}
