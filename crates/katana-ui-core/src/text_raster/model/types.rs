use super::{RGBA_ALPHA_INDEX, RGBA_CHANNEL_COUNT};
use crate::text_raster::catalog_types::PlatformColorEmojiFaceRecord;
use sha2::{Digest, Sha256};
use unicode_segmentation::UnicodeSegmentation;

const SHA256_DIGEST_BYTE_COUNT: usize = 32;

#[derive(Debug, Clone, PartialEq)]
pub struct PlatformTextRaster {
    pub text: String,
    pub width: usize,
    pub height: usize,
    pub rgba_pixels: Vec<[u8; RGBA_CHANNEL_COUNT]>,
    pub grapheme_bounds: Vec<PlatformTextGraphemeBounds>,
    pub report: PlatformTextRasterReport,
}

impl PlatformTextRaster {
    #[must_use]
    pub fn hit_test(&self, x: f32, y: f32) -> Option<PlatformTextHit> {
        self.grapheme_bounds
            .iter()
            .find(|bounds| bounds.contains(x, y))
            .map(PlatformTextHit::from)
    }

    #[must_use]
    pub fn chromatic_pixel_count(&self) -> usize {
        self.rgba_pixels
            .iter()
            .filter(|pixel| pixel[RGBA_ALPHA_INDEX] != 0)
            .filter(|pixel| pixel[0] != pixel[1] || pixel[1] != pixel[2])
            .count()
    }

    #[must_use]
    pub fn grapheme_crop(
        &self,
        bounds: &PlatformTextGraphemeBounds,
        scale: f32,
    ) -> Option<PlatformTextRasterCrop> {
        let scale = (scale.is_finite() && scale > 0.0).then_some(scale)?;
        let left = (bounds.x * scale).floor().max(0.0) as usize;
        let top = (bounds.y * scale).floor().max(0.0) as usize;
        let right = ((bounds.x + bounds.width) * scale).ceil() as usize;
        let bottom = ((bounds.y + bounds.height) * scale).ceil() as usize;
        let right = right.min(self.width);
        let bottom = bottom.min(self.height);
        if left >= right || top >= bottom {
            return None;
        }
        let pixels = (top..bottom)
            .flat_map(|y| (left..right).map(move |x| self.rgba_pixels[y * self.width + x]))
            .collect();
        Some(PlatformTextRasterCrop {
            width: right - left,
            height: bottom - top,
            pixels,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformTextRasterCrop {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[u8; RGBA_CHANNEL_COUNT]>,
}

impl PlatformTextRasterCrop {
    #[must_use]
    pub fn chromatic_pixel_count(&self) -> usize {
        self.pixels
            .iter()
            .filter(|pixel| pixel[RGBA_ALPHA_INDEX] != 0)
            .filter(|pixel| pixel[0] != pixel[1] || pixel[1] != pixel[2])
            .count()
    }

    #[must_use]
    pub fn sha256(&self) -> [u8; SHA256_DIGEST_BYTE_COUNT] {
        let mut hasher = Sha256::new();
        hasher.update((self.width as u64).to_be_bytes());
        hasher.update((self.height as u64).to_be_bytes());
        for pixel in &self.pixels {
            hasher.update(pixel);
        }
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlatformTextGraphemeBounds {
    pub byte_start: usize,
    pub byte_end: usize,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl PlatformTextGraphemeBounds {
    #[must_use]
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformTextGraphemeRange {
    pub byte_start: usize,
    pub byte_end: usize,
}

impl PlatformTextGraphemeRange {
    #[must_use]
    pub fn ranges(text: &str) -> Vec<Self> {
        text.grapheme_indices(true)
            .map(|(byte_start, grapheme)| Self {
                byte_start,
                byte_end: byte_start + grapheme.len(),
            })
            .collect()
    }
    #[must_use]
    pub fn previous(text: &str, byte_offset: usize) -> Option<Self> {
        let byte_offset = clamp_to_char_boundary(text, byte_offset);
        Self::ranges(text)
            .into_iter()
            .rev()
            .find(|range| range.byte_end <= byte_offset)
    }
    #[must_use]
    pub fn next(text: &str, byte_offset: usize) -> Option<Self> {
        let byte_offset = clamp_to_char_boundary(text, byte_offset);
        Self::ranges(text)
            .into_iter()
            .find(|range| range.byte_start >= byte_offset)
    }
}

fn clamp_to_char_boundary(text: &str, byte_offset: usize) -> usize {
    let mut boundary = byte_offset.min(text.len());
    while boundary > 0 && !text.is_char_boundary(boundary) {
        boundary -= 1;
    }
    boundary
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformTextHit {
    pub byte_start: usize,
    pub byte_end: usize,
}

impl From<&PlatformTextGraphemeBounds> for PlatformTextHit {
    fn from(bounds: &PlatformTextGraphemeBounds) -> Self {
        Self {
            byte_start: bounds.byte_start,
            byte_end: bounds.byte_end,
        }
    }
}

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
    MetricsFrameScaleMismatch {
        expected_bits: u32,
        actual_bits: u32,
    },
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
            Self::MetricsFrameScaleMismatch {
                expected_bits,
                actual_bits,
            } => write!(
                formatter,
                "platform text metrics frame scale changed ({expected_bits} != {actual_bits})"
            ),
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
