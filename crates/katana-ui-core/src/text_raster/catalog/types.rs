use crate::text_raster::catalog_types::{PlatformColorEmojiFaceRecord, PlatformFontCatalogPolicy};
use cosmic_text::FontSystem;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformFontCatalogStats {
    pub font_database_discoveries: usize,
    pub candidate_load_attempts: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlatformRegularFontFace {
    pub(crate) family: String,
    pub(crate) source_file_path: PathBuf,
    pub(crate) selection_family: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct PlatformRegularFontFaces {
    pub(crate) proportional: Option<PlatformRegularFontFace>,
    pub(crate) monospace: Option<PlatformRegularFontFace>,
}

impl PlatformRegularFontFaces {
    pub(crate) fn is_empty(&self) -> bool {
        self.proportional.is_none() && self.monospace.is_none()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &PlatformRegularFontFace> {
        self.proportional.iter().chain(self.monospace.iter())
    }
}

pub struct PlatformFontCatalog {
    pub(super) policy: PlatformFontCatalogPolicy,
    pub(super) font_system: Mutex<FontSystem>,
    pub(super) first_candidate_font_system: Mutex<Option<FontSystem>>,
    pub(super) emoji_face: PlatformColorEmojiFaceRecord,
    pub(super) regular_font_faces: PlatformRegularFontFaces,
    pub(super) stats: PlatformFontCatalogStats,
}
