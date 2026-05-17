use crate::render_model::UiTree;
use crate::window::WindowId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Surface {
    pub window_id: WindowId,
    pub metrics: SurfaceMetrics,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FrameHandle(String);

impl FrameHandle {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaintRequest {
    window_id: WindowId,
    metrics: SurfaceMetrics,
    tree: UiTree,
}

impl PaintRequest {
    #[must_use]
    pub fn new(window_id: WindowId, metrics: SurfaceMetrics) -> Self {
        Self {
            window_id,
            metrics,
            tree: UiTree::new(crate::layout::Row::new()),
        }
    }

    #[must_use]
    pub fn with_tree(mut self, tree: UiTree) -> Self {
        self.tree = tree;
        self
    }

    #[must_use]
    pub fn metrics(&self) -> &SurfaceMetrics {
        &self.metrics
    }

    #[must_use]
    pub fn window_id(&self) -> &WindowId {
        &self.window_id
    }

    #[must_use]
    pub fn tree(&self) -> &UiTree {
        &self.tree
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SurfaceMetrics {
    pub logical_width: f32,
    pub logical_height: f32,
    pub scale_factor: f32,
    pub dpi: f32,
}

impl SurfaceMetrics {
    #[must_use]
    pub const fn new(logical_width: f32, logical_height: f32, scale_factor: f32, dpi: f32) -> Self {
        Self {
            logical_width,
            logical_height,
            scale_factor,
            dpi,
        }
    }

    #[must_use]
    pub fn physical_width(&self) -> f32 {
        self.logical_width * self.scale_factor
    }
}
