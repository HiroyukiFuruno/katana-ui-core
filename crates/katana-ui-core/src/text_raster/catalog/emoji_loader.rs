use super::catalog_cache;
use crate::text_raster::catalog_types::{
    PlatformEmojiFontCandidate, PlatformEmojiFontLoadError, PlatformEmojiFontLoader,
    PlatformEmojiFontObservation,
};
use cosmic_text::FontSystem;

pub(super) struct SystemEmojiFontLoader<'a> {
    pub(super) font_system: &'a mut FontSystem,
    pub(super) load_attempts: usize,
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

pub(super) fn load_font_file(
    font_system: &mut FontSystem,
    candidate: &PlatformEmojiFontCandidate,
) -> std::io::Result<()> {
    font_system
        .db_mut()
        .load_font_file(&candidate.source_file_path)
}

pub(super) fn load_candidate_font(
    font_system: &mut FontSystem,
    candidate: &PlatformEmojiFontCandidate,
) -> Result<(), PlatformEmojiFontLoadError> {
    load_font_file(font_system, candidate).map_err(|error| font_file_load_error(candidate, error))
}

pub(super) fn font_file_load_error(
    candidate: &PlatformEmojiFontCandidate,
    error: std::io::Error,
) -> PlatformEmojiFontLoadError {
    PlatformEmojiFontLoadError::Io {
        source_file_path: candidate.source_file_path.clone(),
        message: error.to_string(),
    }
}
