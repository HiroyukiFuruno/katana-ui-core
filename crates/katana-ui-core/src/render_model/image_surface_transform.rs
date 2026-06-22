use serde::{Deserialize, Serialize};

const DEFAULT_ZOOM_PERCENT: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiImageSurfaceTransform {
    pub zoom_percent: u32,
    pub pan_x: i32,
    pub pan_y: i32,
}

impl UiImageSurfaceTransform {
    #[must_use]
    pub fn new(zoom_percent: u32, pan_x: i32, pan_y: i32) -> Self {
        Self {
            zoom_percent,
            pan_x,
            pan_y,
        }
    }

    #[must_use]
    pub fn zoom_factor(self) -> f32 {
        self.zoom_percent.max(1) as f32 / DEFAULT_ZOOM_PERCENT as f32
    }
}

impl Default for UiImageSurfaceTransform {
    fn default() -> Self {
        Self {
            zoom_percent: DEFAULT_ZOOM_PERCENT,
            pan_x: 0,
            pan_y: 0,
        }
    }
}
