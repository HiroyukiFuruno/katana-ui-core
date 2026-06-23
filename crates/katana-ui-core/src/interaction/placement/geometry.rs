use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    #[must_use]
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[must_use]
    pub fn contains_panel(self, position: Point, size: Size) -> bool {
        position.x >= self.x
            && position.y >= self.y
            && position.x + size.width as i32 <= self.right()
            && position.y + size.height as i32 <= self.bottom()
    }

    pub(crate) fn center_x(self) -> i32 {
        self.x + self.width as i32 / 2
    }

    pub(crate) fn center_y(self) -> i32 {
        self.y + self.height as i32 / 2
    }

    pub(crate) fn right(self) -> i32 {
        self.x + self.width as i32
    }

    pub(crate) fn bottom(self) -> i32 {
        self.y + self.height as i32
    }
}
