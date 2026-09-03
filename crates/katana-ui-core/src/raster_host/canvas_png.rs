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

#[cfg(test)]
mod tests {
    use super::Canvas;
    use crate::test_assert::KucTestExpect;

    #[test]
    fn png_export_preserves_canvas_rgb_channels() {
        let canvas = Canvas::new(1, 1, 0x12_34_56);
        let path = std::env::temp_dir().join(format!(
            "katana-ui-core-canvas-png-{}-{}.png",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .kuc_expect("system clock must be after the Unix epoch")
                .as_nanos()
        ));

        canvas
            .save_png(&path)
            .kuc_expect("canvas PNG export must succeed");
        let pixels = image::open(&path)
            .kuc_expect("canvas PNG output must be readable")
            .to_rgba8();
        assert_eq!([0x12, 0x34, 0x56, 0xff], pixels.get_pixel(0, 0).0);
        std::fs::remove_file(&path).kuc_expect("test PNG must be removable");
    }
}
