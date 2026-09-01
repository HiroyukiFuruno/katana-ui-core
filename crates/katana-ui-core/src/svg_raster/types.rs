use crate::molecule::RgbaColor;
use crate::render_model::{UiIconProps, UiSvgPaintPolicy};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiSvgRasterConfig {
    pub cache_capacity: usize,
    pub max_dimension_px: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiSvgRasterRequest {
    pub icon: UiIconProps,
    pub width_px: u32,
    pub height_px: u32,
    pub color: RgbaColor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiSvgRasterMetadata {
    pub cache_key: String,
    pub cache_hit: bool,
    pub paint_policy: UiSvgPaintPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiSvgRaster {
    pub width_px: u32,
    pub height_px: u32,
    pub rgba_unmultiplied: Vec<u8>,
    pub metadata: UiSvgRasterMetadata,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UiSvgRasterStats {
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub evictions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiSvgRasterError {
    EmptySource,
    InvalidConfiguration {
        cache_capacity: usize,
        max_dimension_px: u32,
    },
    InvalidDimensions {
        width_px: u32,
        height_px: u32,
    },
    DimensionsExceedMaximum {
        width_px: u32,
        height_px: u32,
        maximum: u32,
    },
    PixelBufferOverflow {
        width_px: u32,
        height_px: u32,
    },
    InvalidSvg(String),
    AllocationFailed {
        width_px: u32,
        height_px: u32,
    },
}

#[derive(Debug)]
pub struct UiSvgRasterizer {
    pub(super) config: UiSvgRasterConfig,
    pub(super) cache: HashMap<String, UiSvgRaster>,
    pub(super) cache_order: VecDeque<String>,
    pub(super) stats: UiSvgRasterStats,
}
