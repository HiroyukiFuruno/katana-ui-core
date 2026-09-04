mod catalog_cache;
#[cfg(test)]
mod catalog_tests;

use crate::text_raster::catalog_types::{
    PlatformColorEmojiFaceRecord, PlatformColorEmojiFaceResolver, PlatformEmojiFontCandidate,
    PlatformEmojiFontLoadError, PlatformEmojiFontLoader, PlatformEmojiFontObservation,
    PlatformFontCatalogError, PlatformFontCatalogPolicy,
};
use cosmic_text::FontSystem;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformFontCatalogStats {
    pub font_database_discoveries: usize,
    pub candidate_load_attempts: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct PlatformRegularFontFamilies {
    pub(crate) proportional: Option<String>,
    pub(crate) monospace: Option<String>,
}

pub struct PlatformFontCatalog {
    policy: PlatformFontCatalogPolicy,
    font_system: Mutex<FontSystem>,
    emoji_face: PlatformColorEmojiFaceRecord,
    regular_font_families: PlatformRegularFontFamilies,
    stats: PlatformFontCatalogStats,
}

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
        let (regular_load_attempts, regular_font_families) =
            catalog_cache::load_regular_candidates(&mut font_system, &policy);
        Self {
            policy,
            font_system: Mutex::new(font_system),
            emoji_face,
            regular_font_families,
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
    pub(crate) fn regular_font_families(&self) -> PlatformRegularFontFamilies {
        self.regular_font_families.clone()
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
}

struct SystemEmojiFontLoader<'a> {
    font_system: &'a mut FontSystem,
    load_attempts: usize,
}

impl PlatformEmojiFontLoader for SystemEmojiFontLoader<'_> {
    fn load(
        &mut self,
        candidate: &PlatformEmojiFontCandidate,
    ) -> Result<PlatformEmojiFontObservation, PlatformEmojiFontLoadError> {
        self.load_attempts += 1;
        let raw_file_sha256 = catalog_cache::read_cached_file_hash(&candidate.source_file_path)
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    PlatformEmojiFontLoadError::Missing {
                        source_file_path: candidate.source_file_path.clone(),
                    }
                } else {
                    PlatformEmojiFontLoadError::Io {
                        source_file_path: candidate.source_file_path.clone(),
                        message: error.to_string(),
                    }
                }
            })?;
        load_candidate_font(self.font_system, candidate)?;
        let actual_family = catalog_cache::family_from_loaded_file(
            self.font_system,
            &candidate.source_file_path,
            &candidate.expected_family,
        )
        .ok_or_else(|| PlatformEmojiFontLoadError::FaceNotFound {
            source_file_path: candidate.source_file_path.clone(),
        })?;
        Ok(PlatformEmojiFontObservation {
            actual_family,
            source_file_path: candidate.source_file_path.clone(),
            raw_file_sha256,
        })
    }
}

fn load_font_file(
    font_system: &mut FontSystem,
    candidate: &PlatformEmojiFontCandidate,
) -> std::io::Result<()> {
    font_system
        .db_mut()
        .load_font_file(&candidate.source_file_path)
}

fn load_candidate_font(
    font_system: &mut FontSystem,
    candidate: &PlatformEmojiFontCandidate,
) -> Result<(), PlatformEmojiFontLoadError> {
    load_font_file(font_system, candidate).map_err(|error| font_file_load_error(candidate, error))
}

fn font_file_load_error(
    candidate: &PlatformEmojiFontCandidate,
    error: std::io::Error,
) -> PlatformEmojiFontLoadError {
    PlatformEmojiFontLoadError::Io {
        source_file_path: candidate.source_file_path.clone(),
        message: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_raster::catalog_types::PlatformFontProfile;

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
