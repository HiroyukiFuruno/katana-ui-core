mod paint;
mod types;

use paint::{RGBA_CHANNEL_COUNT, SvgPaintProcessor};
use resvg::usvg;
use std::collections::{HashMap, VecDeque};
use tiny_skia::{Pixmap, Transform};
pub use types::{
    UiSvgRaster, UiSvgRasterConfig, UiSvgRasterError, UiSvgRasterMetadata, UiSvgRasterRequest,
    UiSvgRasterStats, UiSvgRasterizer,
};

const DEFAULT_CACHE_CAPACITY: usize = 128;
const DEFAULT_MAX_DIMENSION_PX: u32 = 4_096;

impl Default for UiSvgRasterConfig {
    fn default() -> Self {
        Self {
            cache_capacity: DEFAULT_CACHE_CAPACITY,
            max_dimension_px: DEFAULT_MAX_DIMENSION_PX,
        }
    }
}

impl UiSvgRasterizer {
    #[must_use]
    pub fn new(config: UiSvgRasterConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
            cache_order: VecDeque::new(),
            stats: UiSvgRasterStats::default(),
        }
    }

    #[must_use]
    pub const fn stats(&self) -> UiSvgRasterStats {
        self.stats
    }

    pub fn rasterize(
        &mut self,
        request: &UiSvgRasterRequest,
    ) -> Result<UiSvgRaster, UiSvgRasterError> {
        validate_request(&self.config, request)?;
        let cache_key = cache_key(request);
        if let Some(mut raster) = self.cache.get(&cache_key).cloned() {
            self.stats.cache_hits = self.stats.cache_hits.saturating_add(1);
            self.promote(&cache_key);
            raster.metadata.cache_hit = true;
            return Ok(raster);
        }

        self.stats.cache_misses = self.stats.cache_misses.saturating_add(1);
        let raster = render(request, cache_key.clone())?;
        self.insert(cache_key, raster.clone());
        Ok(raster)
    }

    fn insert(&mut self, cache_key: String, raster: UiSvgRaster) {
        self.remove_from_order(&cache_key);
        if self.cache.len() >= self.config.cache_capacity
            && let Some(oldest) = self.cache_order.pop_front()
        {
            self.cache.remove(&oldest);
            self.stats.evictions = self.stats.evictions.saturating_add(1);
        }
        self.cache_order.push_back(cache_key.clone());
        self.cache.insert(cache_key, raster);
    }

    fn promote(&mut self, cache_key: &str) {
        self.remove_from_order(cache_key);
        self.cache_order.push_back(cache_key.to_string());
    }

    fn remove_from_order(&mut self, cache_key: &str) {
        if let Some(index) = self.cache_order.iter().position(|entry| entry == cache_key) {
            self.cache_order.remove(index);
        }
    }
}

impl Default for UiSvgRasterizer {
    fn default() -> Self {
        Self::new(UiSvgRasterConfig::default())
    }
}

fn validate_request(
    config: &UiSvgRasterConfig,
    request: &UiSvgRasterRequest,
) -> Result<(), UiSvgRasterError> {
    if config.cache_capacity == 0 || config.max_dimension_px == 0 {
        return Err(UiSvgRasterError::InvalidConfiguration {
            cache_capacity: config.cache_capacity,
            max_dimension_px: config.max_dimension_px,
        });
    }
    if request.icon.svg_source.trim().is_empty() {
        return Err(UiSvgRasterError::EmptySource);
    }
    if request.width_px == 0 || request.height_px == 0 {
        return Err(UiSvgRasterError::InvalidDimensions {
            width_px: request.width_px,
            height_px: request.height_px,
        });
    }
    if request.width_px > config.max_dimension_px || request.height_px > config.max_dimension_px {
        return Err(UiSvgRasterError::DimensionsExceedMaximum {
            width_px: request.width_px,
            height_px: request.height_px,
            maximum: config.max_dimension_px,
        });
    }
    let _ = usize::try_from(request.width_px)
        .ok()
        .and_then(|width| {
            usize::try_from(request.height_px)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .and_then(|pixels| pixels.checked_mul(RGBA_CHANNEL_COUNT))
        .ok_or(UiSvgRasterError::PixelBufferOverflow {
            width_px: request.width_px,
            height_px: request.height_px,
        })?;
    Ok(())
}

fn render(
    request: &UiSvgRasterRequest,
    cache_key: String,
) -> Result<UiSvgRaster, UiSvgRasterError> {
    let source = SvgPaintProcessor::apply_paint_policy(&request.icon, request.color);
    let tree = usvg::Tree::from_str(&source, &usvg::Options::default())
        .map_err(|error| UiSvgRasterError::InvalidSvg(error.to_string()))?;
    let mut pixmap = Pixmap::new(request.width_px, request.height_px).ok_or(
        UiSvgRasterError::AllocationFailed {
            width_px: request.width_px,
            height_px: request.height_px,
        },
    )?;
    let size = tree.size();
    let scale_x = request.width_px as f32 / size.width().max(1.0);
    let scale_y = request.height_px as f32 / size.height().max(1.0);
    resvg::render(
        &tree,
        Transform::from_scale(scale_x, scale_y),
        &mut pixmap.as_mut(),
    );
    Ok(UiSvgRaster {
        width_px: request.width_px,
        height_px: request.height_px,
        rgba_unmultiplied: SvgPaintProcessor::apply_alpha(
            SvgPaintProcessor::unpremultiply(pixmap.data()),
            request.color.alpha,
        ),
        metadata: UiSvgRasterMetadata {
            cache_key,
            cache_hit: false,
            paint_policy: request.icon.paint_policy,
        },
    })
}

fn cache_key(request: &UiSvgRasterRequest) -> String {
    format!(
        "{}|{}|{:?}|{}x{}|{:02X}{:02X}{:02X}{:02X}",
        request.icon.svg_source,
        request.icon.view_box,
        request.icon.paint_policy,
        request.width_px,
        request.height_px,
        request.color.red,
        request.color.green,
        request.color.blue,
        request.color.alpha,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::molecule::RgbaColor;
    use crate::render_model::{UiIconProps, UiSvgPaintPolicy};

    const ICON: &str =
        "<svg viewBox=\"0 0 10 10\"><path fill=\"currentColor\" d=\"M0 0h10v10H0z\"/></svg>";

    fn request(width_px: u32, height_px: u32) -> UiSvgRasterRequest {
        UiSvgRasterRequest {
            icon: UiIconProps::new(ICON).paint_policy(UiSvgPaintPolicy::CurrentColor),
            width_px,
            height_px,
            color: RgbaColor::new(18, 171, 52, 255),
        }
    }

    #[test]
    fn rasterizes_current_color_to_unmultiplied_rgba() {
        let mut rasterizer = UiSvgRasterizer::default();
        let raster = rasterizer
            .rasterize(&request(8, 6))
            .expect("raster must succeed");
        assert_eq!((8, 6), (raster.width_px, raster.height_px));
        assert_eq!(
            usize::from(8_u8) * usize::from(6_u8) * 4,
            raster.rgba_unmultiplied.len()
        );
        assert!(
            raster
                .rgba_unmultiplied
                .chunks_exact(4)
                .any(|pixel| pixel[3] != 0 && pixel[1] > pixel[0])
        );
    }

    #[test]
    fn current_color_policy_recolors_white_host_icon_and_applies_alpha() {
        let mut rasterizer = UiSvgRasterizer::default();
        let mut white = request(8, 8);
        white.icon.svg_source =
            "<svg viewBox=\"0 0 10 10\"><path fill=\"#FFFFFF\" d=\"M0 0h10v10H0z\"/></svg>"
                .to_string();
        white.color = RgbaColor::new(18, 171, 52, 128);
        let raster = rasterizer.rasterize(&white).expect("raster must succeed");
        assert!(
            raster
                .rgba_unmultiplied
                .chunks_exact(4)
                .any(|pixel| pixel[3] > 0 && pixel[3] <= 128 && pixel[1] > pixel[0])
        );
    }

    #[test]
    fn stable_request_reuses_cached_pixels() {
        let mut rasterizer = UiSvgRasterizer::default();
        let first = rasterizer.rasterize(&request(8, 8)).expect("first raster");
        let second = rasterizer.rasterize(&request(8, 8)).expect("second raster");
        assert_eq!(first.rgba_unmultiplied, second.rgba_unmultiplied);
        assert!(!first.metadata.cache_hit);
        assert!(second.metadata.cache_hit);
        assert_eq!(1, rasterizer.stats().cache_hits);
    }

    #[test]
    fn color_and_policy_split_cache_identity() {
        let mut rasterizer = UiSvgRasterizer::default();
        let first = rasterizer.rasterize(&request(8, 8)).expect("first raster");
        let mut second_request = request(8, 8);
        second_request.color = RgbaColor::new(171, 18, 52, 255);
        let second = rasterizer
            .rasterize(&second_request)
            .expect("second raster");
        assert_ne!(first.rgba_unmultiplied, second.rgba_unmultiplied);
        let mut policy_request = request(8, 8);
        policy_request.icon.paint_policy = UiSvgPaintPolicy::FillOnly;
        let third = rasterizer
            .rasterize(&policy_request)
            .expect("policy raster");
        assert!(!third.metadata.cache_hit);
        assert_eq!(3, rasterizer.stats().cache_misses);
    }

    #[test]
    fn least_recently_used_entry_is_evicted_at_capacity() {
        let mut rasterizer = UiSvgRasterizer::new(UiSvgRasterConfig {
            cache_capacity: 1,
            max_dimension_px: 16,
        });
        rasterizer.rasterize(&request(8, 8)).expect("first raster");
        rasterizer.rasterize(&request(7, 7)).expect("second raster");
        assert_eq!(1, rasterizer.stats().evictions);
        let third = rasterizer.rasterize(&request(8, 8)).expect("third raster");
        assert!(!third.metadata.cache_hit);
    }

    #[test]
    fn stroke_and_fill_policy_recolors_only_declared_paint_targets() {
        let icon = UiIconProps::new(
            "<svg><path fill=\"currentColor\" stroke=\"currentColor\" d=\"M0 0h1v1H0z\"/></svg>",
        )
        .paint_policy(UiSvgPaintPolicy::StrokeAndFill);
        let painted =
            SvgPaintProcessor::apply_paint_policy(&icon, RgbaColor::new(18, 171, 52, 255));
        assert!(painted.contains("fill=\"#12AB34\""));
        assert!(painted.contains("stroke=\"#12AB34\""));
        let fill_only = UiIconProps::new(
            "<svg><path fill=\"currentColor\" stroke=\"#FFFFFF\" d=\"M0 0h1v1H0z\"/></svg>",
        )
        .paint_policy(UiSvgPaintPolicy::FillOnly);
        let painted_fill =
            SvgPaintProcessor::apply_paint_policy(&fill_only, RgbaColor::new(18, 171, 52, 255));
        assert!(painted_fill.contains("fill=\"#12AB34\""));
        assert!(painted_fill.contains("stroke=\"#FFFFFF\""));
        let stroke_only = UiIconProps::new(
            "<svg><path fill=\"#FFFFFF\" stroke=\"currentColor\" d=\"M0 0h1v1H0z\"/></svg>",
        )
        .paint_policy(UiSvgPaintPolicy::StrokeOnly);
        let painted_stroke =
            SvgPaintProcessor::apply_paint_policy(&stroke_only, RgbaColor::new(18, 171, 52, 255));
        assert!(painted_stroke.contains("fill=\"#FFFFFF\""));
        assert!(painted_stroke.contains("stroke=\"#12AB34\""));
    }

    #[test]
    fn invalid_requests_are_typed_errors_without_glyph_fallback() {
        let mut rasterizer = UiSvgRasterizer::default();
        let mut empty = request(8, 8);
        empty.icon.svg_source = "  ".to_string();
        assert!(matches!(
            rasterizer.rasterize(&empty),
            Err(UiSvgRasterError::EmptySource)
        ));
        assert!(matches!(
            rasterizer.rasterize(&request(0, 8)),
            Err(UiSvgRasterError::InvalidDimensions { .. })
        ));
        let oversized = request(DEFAULT_MAX_DIMENSION_PX + 1, 8);
        assert!(matches!(
            rasterizer.rasterize(&oversized),
            Err(UiSvgRasterError::DimensionsExceedMaximum { .. })
        ));
        let mut invalid = request(8, 8);
        invalid.icon.svg_source = "<svg><path>".to_string();
        assert!(matches!(
            rasterizer.rasterize(&invalid),
            Err(UiSvgRasterError::InvalidSvg(_))
        ));
    }

    #[test]
    fn invalid_configuration_is_typed_error() {
        let mut rasterizer = UiSvgRasterizer::new(UiSvgRasterConfig {
            cache_capacity: 0,
            max_dimension_px: 0,
        });
        assert!(matches!(
            rasterizer.rasterize(&request(8, 8)),
            Err(UiSvgRasterError::InvalidConfiguration {
                cache_capacity: 0,
                max_dimension_px: 0,
            })
        ));
    }

    #[test]
    fn impossible_pixel_buffer_allocation_is_detected_before_render() {
        let mut rasterizer = UiSvgRasterizer::new(UiSvgRasterConfig {
            cache_capacity: 8,
            max_dimension_px: u32::MAX,
        });
        let oversized = request(u32::MAX, u32::MAX);
        assert!(matches!(
            rasterizer.rasterize(&oversized),
            Err(UiSvgRasterError::PixelBufferOverflow {
                width_px: u32::MAX,
                height_px: u32::MAX,
            })
        ));
    }

    #[test]
    fn render_reports_pixmap_allocation_failure_for_an_unrepresentable_axis() {
        let oversized = request(u32::MAX, 1);
        assert!(matches!(
            render(&oversized, "oversized".to_string()),
            Err(UiSvgRasterError::AllocationFailed {
                width_px: u32::MAX,
                height_px: 1,
            })
        ));
    }

    #[test]
    fn paint_processor_handles_alpha_math_and_unpremultiply() {
        let source = vec![0, 0, 0, 0, 64, 32, 16, 128, 255, 255, 255, 255];
        let unpremultiplied = SvgPaintProcessor::unpremultiply(&source);
        assert_eq!(
            unpremultiplied,
            vec![0, 0, 0, 0, 127, 63, 31, 128, 255, 255, 255, 255]
        );
        let half_alpha = SvgPaintProcessor::apply_alpha(unpremultiplied.clone(), 128);
        assert_eq!(
            half_alpha,
            vec![0, 0, 0, 0, 127, 63, 31, 64, 255, 255, 255, 128]
        );
        assert_eq!(
            SvgPaintProcessor::apply_alpha(half_alpha, u8::MAX),
            vec![0, 0, 0, 0, 127, 63, 31, 64, 255, 255, 255, 128]
        );
    }
}
