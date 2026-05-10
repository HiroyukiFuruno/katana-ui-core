use super::types::{SwatchShape, SwatchSize};

const CELL_SM: f32 = 16.0;
const CELL_MD: f32 = 24.0;
const CELL_LG: f32 = 32.0;
const RING_WIDTH_SM: f32 = 1.5;
const RING_WIDTH_MD: f32 = 2.0;
const RING_WIDTH_LG: f32 = 2.5;
const ROUNDED_RECT_RADIUS: f32 = 6.0;

pub(super) fn cell_size(size: SwatchSize) -> f32 {
    match size {
        SwatchSize::Sm => CELL_SM,
        SwatchSize::Md => CELL_MD,
        SwatchSize::Lg => CELL_LG,
    }
}

pub(super) fn ring_width(size: SwatchSize) -> f32 {
    match size {
        SwatchSize::Sm => RING_WIDTH_SM,
        SwatchSize::Md => RING_WIDTH_MD,
        SwatchSize::Lg => RING_WIDTH_LG,
    }
}

pub(super) fn border_radius(size: SwatchSize, shape: SwatchShape) -> f32 {
    match shape {
        SwatchShape::RoundedRect => ROUNDED_RECT_RADIUS,
        SwatchShape::Circle => cell_size(size) / 2.0,
    }
}
