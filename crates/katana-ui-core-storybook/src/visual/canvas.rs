use image::{ImageBuffer, Rgba};
use std::path::Path;

const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;
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
            for current_x in x..right {
                self.set(current_x, current_y, color);
            }
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
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
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
