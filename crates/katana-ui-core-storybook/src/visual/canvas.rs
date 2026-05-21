use image::{ImageBuffer, Rgba};
use std::path::Path;

use super::canvas_round_rect;

const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;
const ALPHA_MAX: u32 = 255;
const OPAQUE_ALPHA: u8 = 255;
const RECT_BORDER_WIDTH: usize = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Canvas {
    #[must_use]
    pub fn new(width: usize, height: usize, color: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![color; width * height],
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
    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    #[must_use]
    pub fn non_background_pixels(&self, background: u32) -> usize {
        self.pixels.iter().filter(|it| **it != background).count()
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let right = x.saturating_add(width).min(self.width);
        let bottom = y.saturating_add(height).min(self.height);
        for current_y in y..bottom {
            let start = current_y * self.width + x.min(self.width);
            let end = current_y * self.width + right;
            self.pixels[start..end].fill(color);
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

    pub fn fill_round_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        radius: usize,
        color: u32,
    ) {
        canvas_round_rect::fill(self, x, y, width, height, radius, color);
    }

    pub fn set(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    #[must_use]
    pub fn viewport_y(&self, offset_y: usize, height: usize, fill: u32) -> Self {
        let mut viewport = Self::new(self.width, height, fill);
        for target_y in 0..height {
            let source_y = offset_y + target_y;
            if source_y >= self.height {
                break;
            }
            let source_start = source_y * self.width;
            let target_start = target_y * self.width;
            viewport.pixels[target_start..target_start + self.width]
                .copy_from_slice(&self.pixels[source_start..source_start + self.width]);
        }
        viewport
    }

    pub fn blend(&mut self, x: usize, y: usize, color: u32, alpha: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = y * self.width + x;
        self.pixels[index] = blend_color(self.pixels[index], color, alpha);
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
        let right = x.saturating_add(width).min(self.width);
        let bottom = y.saturating_add(height).min(self.height);
        for current_y in y..bottom {
            for current_x in x..right {
                self.blend(current_x, current_y, color, alpha);
            }
        }
    }

    pub fn save_png(&self, path: &Path) -> image::ImageResult<()> {
        let mut image =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::new(self.width as u32, self.height as u32);
        for (index, pixel) in self.pixels.iter().enumerate() {
            let x = (index % self.width) as u32;
            let y = (index / self.width) as u32;
            let red = ((pixel >> RED_SHIFT) & CHANNEL_MASK) as u8;
            let green = ((pixel >> GREEN_SHIFT) & CHANNEL_MASK) as u8;
            let blue = (pixel & CHANNEL_MASK) as u8;
            image.put_pixel(x, y, Rgba([red, green, blue, OPAQUE_ALPHA]));
        }
        image.save(path)
    }
}

fn blend_color(destination: u32, source: u32, alpha: u8) -> u32 {
    let alpha = u32::from(alpha);
    let inverse = ALPHA_MAX - alpha;
    let red = blend_channel(destination, source, alpha, inverse, RED_SHIFT);
    let green = blend_channel(destination, source, alpha, inverse, GREEN_SHIFT);
    let blue = blend_channel(destination, source, alpha, inverse, 0);
    (red << RED_SHIFT) | (green << GREEN_SHIFT) | blue
}

fn blend_channel(destination: u32, source: u32, alpha: u32, inverse: u32, shift: u32) -> u32 {
    let destination_channel = (destination >> shift) & CHANNEL_MASK;
    let source_channel = (source >> shift) & CHANNEL_MASK;
    (source_channel * alpha + destination_channel * inverse) / ALPHA_MAX
}
