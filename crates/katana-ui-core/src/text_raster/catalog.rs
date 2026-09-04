mod catalog_cache;
#[cfg(test)]
mod catalog_tests;
mod emoji_loader;
mod selection;
mod types;

use crate::text_raster::catalog_types::{
    PlatformColorEmojiFaceRecord, PlatformColorEmojiFaceResolver, PlatformFontCatalogError,
    PlatformFontCatalogPolicy,
};
use crate::text_raster::config::PlatformTextFaceSelection;
use cosmic_text::{FontSystem, fontdb::Database};
use emoji_loader::SystemEmojiFontLoader;
use std::sync::Mutex;
pub use types::{PlatformFontCatalog, PlatformFontCatalogStats};
pub(crate) use types::{PlatformRegularFontFace, PlatformRegularFontFaces};

#[cfg(test)]
use emoji_loader::{font_file_load_error, load_candidate_font, load_font_file};

impl PlatformFontCatalog {
    #[must_use]
    pub fn new(policy: PlatformFontCatalogPolicy) -> Self {
        let mut font_system = FontSystem::new();
        let (emoji_face, emoji_load_attempts) = {
            let mut loader = SystemEmojiFontLoader {
                font_system: &mut font_system,
                load_attempts: 0,
            };
            let emoji_face = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);
            (emoji_face, loader.load_attempts)
        };
        let (regular_load_attempts, regular_font_faces) =
            catalog_cache::load_regular_candidates(&mut font_system, &policy);
        Self {
            policy,
            font_system: Mutex::new(font_system),
            first_candidate_font_system: Mutex::new(None),
            emoji_face,
            regular_font_faces,
            stats: PlatformFontCatalogStats {
                font_database_discoveries: 1,
                candidate_load_attempts: emoji_load_attempts + regular_load_attempts,
            },
        }
    }

    #[must_use]
    pub const fn policy(&self) -> &PlatformFontCatalogPolicy {
        &self.policy
    }

    #[must_use]
    pub fn emoji_face(&self) -> &PlatformColorEmojiFaceRecord {
        &self.emoji_face
    }

    #[must_use]
    pub const fn stats(&self) -> PlatformFontCatalogStats {
        self.stats
    }

    #[must_use]
    pub(crate) fn regular_font_faces(&self) -> PlatformRegularFontFaces {
        self.regular_font_faces.clone()
    }

    #[must_use]
    pub fn fingerprint(&self) -> crate::text_raster::PlatformFontCatalogFingerprint {
        self.emoji_face.catalog_fingerprint
    }

    pub(crate) fn with_font_system<T>(
        &self,
        operation: impl FnOnce(&mut FontSystem) -> T,
    ) -> Result<T, PlatformFontCatalogError> {
        let mut font_system = self
            .font_system
            .lock()
            .map_err(|_| PlatformFontCatalogError::FontSystemLockPoisoned)?;
        Ok(operation(&mut font_system))
    }

    pub(crate) fn with_font_system_for_face_selection<T>(
        &self,
        face_selection: PlatformTextFaceSelection,
        operation: impl FnOnce(&mut FontSystem) -> T,
    ) -> Result<T, PlatformFontCatalogError> {
        if face_selection == PlatformTextFaceSelection::System || self.regular_font_faces.is_empty()
        {
            return self.with_font_system(operation);
        }
        self.with_first_candidate_font_system(operation)
    }

    fn with_first_candidate_font_system<T>(
        &self,
        operation: impl FnOnce(&mut FontSystem) -> T,
    ) -> Result<T, PlatformFontCatalogError> {
        {
            let mut selected_font_system = self
                .first_candidate_font_system
                .lock()
                .map_err(|_| PlatformFontCatalogError::FontSystemLockPoisoned)?;
            if let Some(font_system) = selected_font_system.as_mut() {
                return Ok(operation(font_system));
            }
        }

        let (locale, database) = self.clone_font_database()?;
        let selected_font_system =
            selection::first_candidate_font_system(locale, database, &self.regular_font_faces);
        let mut cached_font_system = self
            .first_candidate_font_system
            .lock()
            .map_err(|_| PlatformFontCatalogError::FontSystemLockPoisoned)?;
        let font_system = cached_font_system.get_or_insert(selected_font_system);
        Ok(operation(font_system))
    }

    fn clone_font_database(&self) -> Result<(String, Database), PlatformFontCatalogError> {
        let font_system = self
            .font_system
            .lock()
            .map_err(|_| PlatformFontCatalogError::FontSystemLockPoisoned)?;
        Ok((font_system.locale().to_owned(), font_system.db().clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_raster::catalog_types::{
        PlatformEmojiFontCandidate, PlatformEmojiFontLoadError, PlatformFontProfile,
    };

    #[test]
    fn catalog_exposes_requested_policy_and_stable_fingerprint() {
        let policy = PlatformFontCatalogPolicy::new(
            PlatformFontProfile::MacOs,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let catalog = PlatformFontCatalog::new(policy.clone());

        assert_eq!(catalog.policy(), &policy);
        assert_eq!(catalog.fingerprint(), policy.fingerprint());
        let stats = catalog.stats();
        assert_eq!(stats.font_database_discoveries, 1);
        assert_eq!(stats.candidate_load_attempts, 0);
    }

    #[test]
    fn with_font_system_is_callable_and_reentrant()
    -> Result<(), crate::text_raster::catalog_types::PlatformFontCatalogError> {
        let policy = PlatformFontCatalogPolicy::new(
            PlatformFontProfile::MacOs,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let catalog = PlatformFontCatalog::new(policy);
        let first = catalog.with_font_system(|font_system| font_system.db().faces().count())?;
        let second = catalog.with_font_system(|font_system| font_system.db().faces().count())?;
        assert_eq!(first, second);
        Ok(())
    }

    #[test]
    fn font_loader_reports_missing_files() {
        let candidate = PlatformEmojiFontCandidate::new(
            std::env::temp_dir().join(format!("kuc-missing-font-{}.font", std::process::id())),
            "missing",
        );
        let mut font_system = FontSystem::new();
        let error = load_font_file(&mut font_system, &candidate).expect_err("missing font");
        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        assert!(matches!(
            load_candidate_font(&mut font_system, &candidate),
            Err(PlatformEmojiFontLoadError::Io { .. })
        ));
    }
}
