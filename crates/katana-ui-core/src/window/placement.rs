use super::{WindowId, WindowSize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindowPoint {
    pub x: f32,
    pub y: f32,
}

impl WindowPoint {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindowRect {
    pub origin: WindowPoint,
    pub size: WindowSize,
}

impl WindowRect {
    #[must_use]
    pub const fn new(origin: WindowPoint, size: WindowSize) -> Self {
        Self { origin, size }
    }

    #[must_use]
    pub fn right(self) -> f32 {
        self.origin.x + self.size.width
    }

    #[must_use]
    pub fn bottom(self) -> f32 {
        self.origin.y + self.size.height
    }

    #[must_use]
    pub fn contains_rect(self, other: Self) -> bool {
        other.origin.x >= self.origin.x
            && other.origin.y >= self.origin.y
            && other.right() <= self.right()
            && other.bottom() <= self.bottom()
    }

    #[must_use]
    pub fn intersects(self, other: Self) -> bool {
        self.origin.x < other.right()
            && self.right() > other.origin.x
            && self.origin.y < other.bottom()
            && self.bottom() > other.origin.y
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayBounds {
    pub name: String,
    pub rect: WindowRect,
    pub scale_factor: f32,
}

impl DisplayBounds {
    #[must_use]
    pub fn new(name: impl Into<String>, rect: WindowRect, scale_factor: f32) -> Self {
        Self {
            name: name.into(),
            rect,
            scale_factor,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalWindowPlacement {
    parent_window_id: WindowId,
    modal_window_id: WindowId,
    parent_rect: WindowRect,
    modal_size: WindowSize,
    display: DisplayBounds,
}

impl ModalWindowPlacement {
    #[must_use]
    pub fn same_display(
        parent_window_id: WindowId,
        modal_window_id: WindowId,
        parent_rect: WindowRect,
        modal_size: WindowSize,
        display: DisplayBounds,
    ) -> Self {
        Self {
            parent_window_id,
            modal_window_id,
            parent_rect,
            modal_size,
            display,
        }
    }

    pub fn resolve(&self) -> Result<ModalWindowPlan, ModalWindowPlacementError> {
        if !self.display.rect.intersects(self.parent_rect) {
            return Err(ModalWindowPlacementError::ParentOutsideDisplay);
        }
        if self.modal_size.width > self.display.rect.size.width
            || self.modal_size.height > self.display.rect.size.height
        {
            return Err(ModalWindowPlacementError::ModalLargerThanDisplay);
        }
        let position = self.centered_position();
        let rect = WindowRect::new(position, self.modal_size);
        if !self.display.rect.contains_rect(rect) {
            return Err(ModalWindowPlacementError::ModalOutsideDisplay);
        }
        Ok(ModalWindowPlan {
            parent_window_id: self.parent_window_id.clone(),
            modal_window_id: self.modal_window_id.clone(),
            position,
            size: self.modal_size,
            display_name: self.display.name.clone(),
            frontmost: true,
            same_display: true,
        })
    }

    fn centered_position(&self) -> WindowPoint {
        let desired_x =
            self.parent_rect.origin.x + (self.parent_rect.size.width - self.modal_size.width) / 2.0;
        let desired_y = self.parent_rect.origin.y
            + (self.parent_rect.size.height - self.modal_size.height) / 2.0;
        WindowPoint::new(
            clamp_to_display_axis(
                desired_x,
                self.modal_size.width,
                self.display.rect.origin.x,
                self.display.rect.right(),
            ),
            clamp_to_display_axis(
                desired_y,
                self.modal_size.height,
                self.display.rect.origin.y,
                self.display.rect.bottom(),
            ),
        )
    }
}

fn clamp_to_display_axis(value: f32, length: f32, start: f32, end: f32) -> f32 {
    value.max(start).min(end - length)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalWindowPlan {
    pub parent_window_id: WindowId,
    pub modal_window_id: WindowId,
    pub position: WindowPoint,
    pub size: WindowSize,
    pub display_name: String,
    pub frontmost: bool,
    pub same_display: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalWindowPlacementError {
    ParentOutsideDisplay,
    ModalLargerThanDisplay,
    ModalOutsideDisplay,
}
