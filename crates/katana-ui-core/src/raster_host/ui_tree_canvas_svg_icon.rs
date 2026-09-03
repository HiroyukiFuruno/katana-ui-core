use super::canvas::Canvas;
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::render_model::UiIconProps;
use katana_ui_core::svg_raster::{UiSvgRasterRequest, UiSvgRasterizer};
use std::cell::RefCell;

const RGBA_CHANNEL_COUNT: usize = 4;
const RED_CHANNEL_INDEX: usize = 0;
const GREEN_CHANNEL_INDEX: usize = 1;
const BLUE_CHANNEL_INDEX: usize = 2;
const ALPHA_CHANNEL_INDEX: usize = 3;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;

thread_local! {
    static SVG_RASTERIZER: RefCell<UiSvgRasterizer> = RefCell::new(UiSvgRasterizer::default());
}

pub(super) struct SvgIconRaster;

impl SvgIconRaster {
    pub(super) fn draw(
        canvas: &mut Canvas,
        icon: &UiIconProps,
        left: usize,
        top: usize,
        size: usize,
        color: u32,
    ) -> bool {
        let physical_left = canvas.to_physical_x(left);
        let physical_top = canvas.to_physical_y(top);
        let physical_size = canvas.logical_scale(size).max(1).min(u32::MAX as usize) as u32;
        let request = UiSvgRasterRequest {
            icon: icon.clone(),
            width_px: physical_size,
            height_px: physical_size,
            color: rgba_color(color),
        };
        let raster = SVG_RASTERIZER.with(|rasterizer| rasterizer.borrow_mut().rasterize(&request));
        let Ok(raster) = raster else {
            return false;
        };

        let raster_width = raster.width_px as usize;
        let raster_height = raster.height_px as usize;
        for y in 0..raster_height {
            for x in 0..raster_width {
                let index = (y * raster_width + x) * RGBA_CHANNEL_COUNT;
                let pixel = &raster.rgba_unmultiplied[index..index + RGBA_CHANNEL_COUNT];
                if pixel[ALPHA_CHANNEL_INDEX] == 0 {
                    continue;
                }
                let color = (u32::from(pixel[RED_CHANNEL_INDEX]) << RED_SHIFT)
                    | (u32::from(pixel[GREEN_CHANNEL_INDEX]) << GREEN_SHIFT)
                    | u32::from(pixel[BLUE_CHANNEL_INDEX]);
                canvas.blend_physical(
                    physical_left.saturating_add(x),
                    physical_top.saturating_add(y),
                    color,
                    pixel[ALPHA_CHANNEL_INDEX],
                );
            }
        }
        true
    }
}

fn rgba_color(color: u32) -> RgbaColor {
    RgbaColor::new(
        ((color >> RED_SHIFT) & CHANNEL_MASK) as u8,
        ((color >> GREEN_SHIFT) & CHANNEL_MASK) as u8,
        (color & CHANNEL_MASK) as u8,
        u8::MAX,
    )
}

#[cfg(test)]
mod tests {
    use super::SvgIconRaster;
    use crate::raster_host::canvas::Canvas;
    use crate::test_assert::KucTestExpect;
    use katana_ui_core::render_model::UiIconProps;

    const BACKGROUND: u32 = 0x101010;
    const TEXT: u32 = 0xeeeeee;
    const GREEN: u32 = 0x12ab34;
    const MATERIAL_PAN_UP: &str = r##"<svg fill="#FFFFFF" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"><path d="M440-647 244-451q-12 12-28 11.5T188-452q-11-12-11.5-28t11.5-28l264-264q6-6 13-8.5t15-2.5q8 0 15 2.5t13 8.5l264 264q11 11 11 27.5T772-452q0 17-11.5 28.5T743-439L520-647v447q0 17-11.5 28.5T480-160q-17 0-28.5-11.5T440-200v-447Z"/></svg>"##;
    const KATANA_STROKE_PAN_UP: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 10 8 4 12 10"/></svg>"##;

    #[test]
    fn material_symbols_negative_viewbox_renders_as_glyph_not_square_fragment() {
        let icon = UiIconProps::new(MATERIAL_PAN_UP)
            .role("surface.pan-up")
            .view_box("0 -960 960 960");
        let mut canvas = Canvas::new(32, 32, BACKGROUND);

        assert!(SvgIconRaster::draw(&mut canvas, &icon, 4, 4, 24, TEXT));

        let bounds = color_bounds(&canvas, BACKGROUND).kuc_expect("icon pixels should exist");
        assert!(
            bounds.width() >= 10 && bounds.height() >= 14,
            "material icon must occupy a real glyph area, not a tiny clipped square: {bounds:?}"
        );
    }

    #[test]
    fn katana_stroke_icon_uses_public_runtime_white_paint_normalization() {
        let icon = UiIconProps::new(KATANA_STROKE_PAN_UP)
            .role("surface.pan-up")
            .view_box("0 0 16 16");
        let mut canvas = Canvas::new(24, 24, BACKGROUND);

        assert!(SvgIconRaster::draw(&mut canvas, &icon, 4, 4, 16, GREEN));
        assert!(canvas.pixels().iter().any(|pixel| {
            let red = (*pixel >> 16) & 0xff;
            let green = (*pixel >> 8) & 0xff;
            green > red
        }));
    }

    #[test]
    fn katana_stroke_icon_rasterizes_at_canvas_scale_without_blocky_expansion() {
        let icon = UiIconProps::new(KATANA_STROKE_PAN_UP)
            .role("surface.pan-up")
            .view_box("0 0 16 16");
        let mut canvas = Canvas::new_scaled(32, 32, 2.0, BACKGROUND);

        assert!(SvgIconRaster::draw(&mut canvas, &icon, 4, 4, 24, TEXT));

        let bounds = color_bounds(&canvas, BACKGROUND).kuc_expect("icon pixels should exist");
        assert!(
            bounds.width() >= 24 && bounds.height() >= 20,
            "2x icon should be rasterized in physical pixels, not copied from a 1x glyph: {bounds:?}"
        );
        assert!(
            anti_aliased_pixel_count(&canvas, BACKGROUND, TEXT) > 0,
            "2x icon should keep SVG antialias pixels instead of blocky logical expansion"
        );
    }

    #[test]
    fn invalid_svg_returns_false_without_mutating_the_canvas() {
        let icon = UiIconProps::new("<svg");
        let mut canvas = Canvas::new(8, 8, BACKGROUND);

        assert!(!SvgIconRaster::draw(&mut canvas, &icon, 0, 0, 8, TEXT));
        assert!(canvas.pixels().iter().all(|pixel| *pixel == BACKGROUND));
    }

    #[derive(Debug)]
    struct Bounds {
        min_x: usize,
        min_y: usize,
        max_x: usize,
        max_y: usize,
    }

    impl Bounds {
        fn width(&self) -> usize {
            self.max_x.saturating_sub(self.min_x).saturating_add(1)
        }

        fn height(&self) -> usize {
            self.max_y.saturating_sub(self.min_y).saturating_add(1)
        }
    }

    fn color_bounds(canvas: &Canvas, background: u32) -> Option<Bounds> {
        let mut bounds: Option<Bounds> = None;
        for (index, pixel) in canvas.pixels().iter().enumerate() {
            if *pixel == background {
                continue;
            }
            let x = index % canvas.width();
            let y = index / canvas.width();
            match &mut bounds {
                Some(value) => {
                    value.min_x = value.min_x.min(x);
                    value.min_y = value.min_y.min(y);
                    value.max_x = value.max_x.max(x);
                    value.max_y = value.max_y.max(y);
                }
                None => {
                    bounds = Some(Bounds {
                        min_x: x,
                        min_y: y,
                        max_x: x,
                        max_y: y,
                    });
                }
            }
        }
        bounds
    }

    fn anti_aliased_pixel_count(canvas: &Canvas, background: u32, foreground: u32) -> usize {
        canvas
            .pixels()
            .iter()
            .filter(|pixel| **pixel != background && **pixel != foreground)
            .count()
    }
}
