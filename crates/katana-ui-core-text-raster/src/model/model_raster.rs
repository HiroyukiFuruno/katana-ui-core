use super::PlatformTextRasterReport;
use super::model_bounds::PlatformTextGraphemeBounds;
use super::{RGBA_ALPHA_INDEX, RGBA_CHANNEL_COUNT};
use sha2::{Digest, Sha256};

const SHA256_HASH_BYTES_LEN: usize = 32;

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

#[derive(Debug, Clone, PartialEq)]
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
    pub fn sha256(&self) -> [u8; SHA256_HASH_BYTES_LEN] {
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
