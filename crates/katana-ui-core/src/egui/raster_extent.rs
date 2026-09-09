//! Logical geometry derived from physical text-raster dimensions.

use crate::text_raster::PlatformTextRaster;

pub(crate) struct LogicalRasterExtent;

impl LogicalRasterExtent {
    /// Converts a physical raster dimension into the logical extent used by egui.
    #[must_use]
    pub(crate) fn from_physical(physical_extent: usize, scale_factor: f32) -> u32 {
        (physical_extent as f32 / Self::normalized_scale_factor(scale_factor))
            .ceil()
            .max(1.0) as u32
    }

    /// Returns the logical dimensions used for layout, paint, clipping, and hits.
    #[must_use]
    pub(crate) fn size(raster: &PlatformTextRaster, scale_factor: f32) -> (u32, u32) {
        (
            Self::from_physical(raster.width, scale_factor),
            Self::from_physical(raster.height, scale_factor),
        )
    }

    fn normalized_scale_factor(scale_factor: f32) -> f32 {
        if scale_factor.is_finite() && scale_factor > 0.0 {
            scale_factor
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LogicalRasterExtent;

    #[test]
    fn logical_extent_rounds_physical_dimensions_up_at_supported_scales() {
        assert_eq!(37, LogicalRasterExtent::from_physical(37, 1.0));
        assert_eq!(25, LogicalRasterExtent::from_physical(37, 1.5));
        assert_eq!(19, LogicalRasterExtent::from_physical(37, 2.0));
        assert_eq!(37, LogicalRasterExtent::from_physical(37, 0.0));
        assert_eq!(37, LogicalRasterExtent::from_physical(37, f32::NAN));
        assert_eq!(37, LogicalRasterExtent::from_physical(37, -1.0));
    }
}
