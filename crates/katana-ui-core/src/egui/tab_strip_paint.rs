//! KUC-owned paint model for a retained tab strip.

use crate::render_model::UiRect;
use serde::Serialize;

const TAB_STRIP_RGBA_CHANNELS: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TabStripPaintTexture {
    pub(crate) identity: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) rgba_pixels: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TabStripPaintOperationKind {
    Fill {
        bounds: UiRect,
        color_rgba: [u8; TAB_STRIP_RGBA_CHANNELS],
    },
    Texture {
        bounds: UiRect,
        texture: TabStripPaintTexture,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TabStripPaintOperation {
    pub(crate) clip_bounds: UiRect,
    pub(crate) kind: TabStripPaintOperationKind,
}

/// KUC tab-strip pixels and geometry. Labels, targets, operation kinds, and
/// structural ids never cross this artifact boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TabStripPaintPlan {
    pub(crate) surface_bounds: UiRect,
    pub(crate) operations: Vec<TabStripPaintOperation>,
}
