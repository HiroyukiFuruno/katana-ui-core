use crate::text_raster::catalog_types::PlatformColorEmojiFaceRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformTextRasterReport {
    pub resolved_emoji_font_family: Option<String>,
    pub color_emoji_font_available: bool,
    pub emoji_face: PlatformColorEmojiFaceRecord,
    pub cache_hit: bool,
    pub stats: PlatformTextRasterStats,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlatformTextRasterStats {
    pub cache_entries: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub font_database_loads: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformTextRasterError {
    EmptyText,
    NonFiniteLayoutExtent,
    CatalogAccess,
    CatalogConfigurationMismatch,
    ColorEmojiUnavailable {
        face: Box<PlatformColorEmojiFaceRecord>,
    },
    RasterTooLarge {
        width: usize,
        height: usize,
        max_pixels: usize,
    },
}

impl std::fmt::Display for PlatformTextRasterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyText => {
                formatter.write_str("platform text raster request must not be empty")
            }
            Self::NonFiniteLayoutExtent => {
                formatter.write_str("platform text raster layout extent must be finite")
            }
            Self::CatalogAccess => formatter.write_str("platform font catalog is unavailable"),
            Self::CatalogConfigurationMismatch => {
                formatter.write_str("platform text raster catalog configuration does not match")
            }
            Self::ColorEmojiUnavailable { face } => write!(
                formatter,
                "platform color emoji is unavailable for {:?}: {:?}",
                face.platform_profile, face.availability
            ),
            Self::RasterTooLarge {
                width,
                height,
                max_pixels,
            } => write!(
                formatter,
                "platform text raster {width}x{height} exceeds {max_pixels} pixel limit"
            ),
        }
    }
}

impl std::error::Error for PlatformTextRasterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raster_error_display_covers_all_public_variants() {
        let hash = crate::text_raster::PlatformFontSha256::digest(b"error");
        let record = crate::text_raster::PlatformColorEmojiFaceRecord {
            platform_profile: crate::text_raster::PlatformFontProfile::Unsupported,
            family_identity: "Family".to_string(),
            source_file_path: None,
            raw_file_sha256: Some(hash),
            catalog_fingerprint: crate::text_raster::PlatformFontCatalogFingerprint::from_bytes([0; 32]),
            availability: crate::text_raster::PlatformColorEmojiAvailability::Unavailable(
                crate::text_raster::PlatformColorEmojiUnavailableReason::NoCandidates,
            ),
        };
        let variants = [
            PlatformTextRasterError::EmptyText,
            PlatformTextRasterError::NonFiniteLayoutExtent,
            PlatformTextRasterError::CatalogAccess,
            PlatformTextRasterError::CatalogConfigurationMismatch,
            PlatformTextRasterError::ColorEmojiUnavailable {
                face: Box::new(record),
            },
            PlatformTextRasterError::RasterTooLarge {
                width: 1,
                height: 2,
                max_pixels: 3,
            },
        ];

        for error in variants {
            let text = error.to_string();
            assert!(
                !text.is_empty(),
                "display string for variant must be non-empty: {error:?}"
            );
        }
    }

    #[test]
    fn raster_error_implements_error_trait() {
        let error: &dyn std::error::Error = &PlatformTextRasterError::EmptyText;
        assert_eq!(
            error.to_string(),
            "platform text raster request must not be empty"
        );
    }
}
