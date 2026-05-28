use super::canvas_clip::CanvasClip;
use super::canvas_color::blend_color;
pub use super::canvas_model::Canvas;
use super::canvas_scale::{normalized_scale, physical_size};
const RECT_BORDER_WIDTH: usize = 1;

impl Canvas {
    #[must_use]
    pub fn new(width: usize, height: usize, color: u32) -> Self {
        Self::new_scaled(width, height, 1.0, color)
    }

    #[must_use]
    pub fn new_scaled(width: usize, height: usize, scale: f32, color: u32) -> Self {
        let scale = normalized_scale(scale);
        Self {
            width: physical_size(width, scale),
            height: physical_size(height, scale),
            logical_width: width,
            logical_height: height,
            scale_factor: scale,
            pixels: vec![color; physical_size(width, scale) * physical_size(height, scale)],
            clip: None,
        }
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub fn logical_width(&self) -> usize {
        self.logical_width
    }

    #[must_use]
    pub fn logical_height(&self) -> usize {
        self.logical_height
    }

    #[must_use]
    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    #[must_use]
    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    #[must_use]
    pub fn non_background_pixels(&self, background: u32) -> usize {
        self.pixels.iter().filter(|it| **it != background).count()
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let Some(rect) = self.visible_rect(x, y, width, height) else {
            return;
        };
        for current_y in rect.y..rect.bottom() {
            let start = current_y * self.width + rect.x;
            let end = current_y * self.width + rect.right();
            self.pixels[start..end].fill(color);
        }
    }

    pub(crate) fn with_clip<F>(&mut self, x: usize, y: usize, width: usize, height: usize, draw: F)
    where
        F: FnOnce(&mut Self),
    {
        let Some(next) = self.to_physical_clip(x, y, width, height) else {
            return;
        };
        let previous = self.clip;
        self.clip = match previous {
            Some(current) => current.intersect(next),
            None => Some(next),
        };
        if self.clip.is_some() {
            draw(self);
        }
        self.clip = previous;
    }

    #[must_use]
    fn visible_rect(&self, x: usize, y: usize, width: usize, height: usize) -> Option<CanvasClip> {
        let rect = self.to_physical_clip(x, y, width, height)?;
        match self.clip {
            Some(clip) => rect.intersect(clip),
            None => Some(rect),
        }
    }

    pub fn stroke_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.fill_rect(x, y, width, RECT_BORDER_WIDTH, color);
        self.fill_rect(
            x,
            y + height - RECT_BORDER_WIDTH,
            width,
            RECT_BORDER_WIDTH,
            color,
        );
        self.fill_rect(x, y, RECT_BORDER_WIDTH, height, color);
        self.fill_rect(
            x + width - RECT_BORDER_WIDTH,
            y,
            RECT_BORDER_WIDTH,
            height,
            color,
        );
    }

    pub fn set(&mut self, x: usize, y: usize, color: u32) {
        let Some((left, right)) = self.physical_span_x(x) else {
            return;
        };
        let Some((top, bottom)) = self.physical_span_y(y) else {
            return;
        };
        for current_y in top..bottom {
            for current_x in left..right {
                self.set_physical(current_x, current_y, color);
            }
        }
    }

    pub fn blend(&mut self, x: usize, y: usize, color: u32, alpha: u8) {
        let Some((left, right)) = self.physical_span_x(x) else {
            return;
        };
        let Some((top, bottom)) = self.physical_span_y(y) else {
            return;
        };
        for current_y in top..bottom {
            for current_x in left..right {
                self.blend_physical(current_x, current_y, color, alpha)
            }
        }
    }

    pub(crate) fn blend_physical(&mut self, x: usize, y: usize, color: u32, alpha: u8) {
        if x >= self.width || y >= self.height || !self.clip.is_none_or(|clip| clip.contains(x, y))
        {
            return;
        }
        let index = y * self.width + x;
        self.pixels[index] = blend_color(self.pixels[index], color, alpha);
    }

    pub(crate) fn set_physical(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height || !self.clip.is_none_or(|clip| clip.contains(x, y))
        {
            return;
        }
        self.pixels[y * self.width + x] = color;
    }

    pub fn blend_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
        alpha: u8,
    ) {
        let Some(rect) = self.visible_rect(x, y, width, height) else {
            return;
        };
        for current_y in rect.y..rect.bottom() {
            let left = current_y * self.width + rect.x;
            let right = current_y * self.width + rect.right();
            for index in left..right {
                let destination = self.pixels[index];
                self.pixels[index] = blend_color(destination, color, alpha);
            }
        }
    }

    fn logical_to_physical_position(&self, logical: usize) -> usize {
        (logical as f64 * f64::from(self.scale_factor)).round() as usize
    }

    fn physical_span_x(&self, x: usize) -> Option<(usize, usize)> {
        let left = self.to_physical_x(x);
        if left >= self.width {
            return None;
        }
        let mut right = self
            .to_physical_x(x.saturating_add(1))
            .saturating_sub(left)
            .max(1)
            .saturating_add(left);
        if right > self.width {
            right = self.width;
        }
        Some((left, right))
    }

    fn physical_span_y(&self, y: usize) -> Option<(usize, usize)> {
        let top = self.to_physical_y(y);
        if top >= self.height {
            return None;
        }
        let mut bottom = self
            .to_physical_y(y.saturating_add(1))
            .saturating_sub(top)
            .max(1)
            .saturating_add(top);
        if bottom > self.height {
            bottom = self.height;
        }
        Some((top, bottom))
    }

    pub(super) fn to_physical_x(&self, x: usize) -> usize {
        self.logical_to_physical_position(x).min(self.width)
    }

    pub(super) fn to_physical_y(&self, y: usize) -> usize {
        self.logical_to_physical_position(y).min(self.height)
    }

    pub(super) fn logical_scale(&self, value: usize) -> usize {
        self.logical_to_physical_position(value)
    }

    fn to_physical_clip(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<CanvasClip> {
        let rect = self.visible_logical_span(x, y, width, height)?;
        if rect.0 >= rect.2 || rect.1 >= rect.3 {
            return None;
        }
        CanvasClip::from_rect(
            rect.0,
            rect.1,
            rect.2 - rect.0,
            rect.3 - rect.1,
            self.width,
            self.height,
        )
    }

    fn visible_logical_span(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize, usize, usize)> {
        if width == 0 || height == 0 {
            return None;
        }
        let left = self.logical_to_physical_position(x).min(self.width);
        let top = self.logical_to_physical_position(y).min(self.height);
        if left >= self.width || top >= self.height {
            return None;
        }
        let right = self
            .logical_to_physical_position(x.saturating_add(width))
            .min(self.width)
            .max(left + 1);
        let bottom = self
            .logical_to_physical_position(y.saturating_add(height))
            .min(self.height)
            .max(top + 1);
        Some((left, top, right, bottom))
    }
}

#[cfg(test)]
mod tests;
