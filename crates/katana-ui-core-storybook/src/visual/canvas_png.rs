use image::{ImageBuffer, Rgba};
use std::path::Path;

use super::canvas_model::Canvas;

const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;
const OPAQUE_ALPHA: u8 = 255;

impl Canvas {
    pub fn save_png(&self, path: &Path) -> image::ImageResult<()> {
        let mut image =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::new(self.width() as u32, self.height() as u32);
        for (index, pixel) in self.pixels().iter().enumerate() {
            let x = (index % self.width()) as u32;
            let y = (index / self.width()) as u32;
            let red = ((pixel >> RED_SHIFT) & CHANNEL_MASK) as u8;
            let green = ((pixel >> GREEN_SHIFT) & CHANNEL_MASK) as u8;
            let blue = (pixel & CHANNEL_MASK) as u8;
            image.put_pixel(x, y, Rgba([red, green, blue, OPAQUE_ALPHA]));
        }
        image.save(path)
    }
}
