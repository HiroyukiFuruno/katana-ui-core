use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DndPoint {
    pub x: f32,
    pub y: f32,
}

impl DndPoint {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DndRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl DndRect {
    #[must_use]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[must_use]
    pub fn contains(&self, point: DndPoint) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    #[must_use]
    pub fn vertical_ratio(&self, point: DndPoint) -> f32 {
        ratio(point.y - self.y, self.height)
    }

    #[must_use]
    pub fn horizontal_ratio(&self, point: DndPoint) -> f32 {
        ratio(point.x - self.x, self.width)
    }
}

fn ratio(offset: f32, size: f32) -> f32 {
    if size <= 0.0 {
        return 0.0;
    }
    (offset / size).clamp(0.0, 1.0)
}
