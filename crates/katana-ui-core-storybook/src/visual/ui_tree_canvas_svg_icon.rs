use super::canvas::Canvas;
use katana_ui_core::render_model::UiIconProps;
use resvg::usvg;
use tiny_skia::{Pixmap, Transform};

const HEX_COLOR_LENGTH: usize = 7;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;

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
        let physical_size = canvas.logical_scale(size).max(1);
        let Some(pixmap) = Self::rasterize(icon, physical_size, color) else {
            return false;
        };
        for y in 0..physical_size {
            for x in 0..physical_size {
                let Some(pixel) = pixmap.pixel(x as u32, y as u32) else {
                    continue;
                };
                let alpha = pixel.alpha();
                if alpha == 0 {
                    continue;
                }
                let color = (u32::from(pixel.red()) << RED_SHIFT)
                    | (u32::from(pixel.green()) << GREEN_SHIFT)
                    | u32::from(pixel.blue());
                canvas.blend_physical(
                    physical_left.saturating_add(x),
                    physical_top.saturating_add(y),
                    color,
                    alpha,
                );
            }
        }
        true
    }

    fn rasterize(icon: &UiIconProps, size: usize, color: u32) -> Option<Pixmap> {
        let svg = Self::themed_svg_source(icon, color);
        let options = usvg::Options::default();
        let tree = usvg::Tree::from_str(&svg, &options).ok()?;
        let source_size = tree.size();
        let mut pixmap = Pixmap::new(size as u32, size as u32)?;
        let scale_x = size as f32 / source_size.width().max(1.0);
        let scale_y = size as f32 / source_size.height().max(1.0);
        resvg::render(
            &tree,
            Transform::from_scale(scale_x, scale_y),
            &mut pixmap.as_mut(),
        );
        Some(pixmap)
    }

    fn themed_svg_source(icon: &UiIconProps, color: u32) -> String {
        let hex = color_hex(color);
        icon.svg_source
            .replace("fill=\"#FFFFFF\"", &format!("fill=\"{hex}\""))
            .replace("fill=\"#ffffff\"", &format!("fill=\"{hex}\""))
            .replace("fill=\"currentColor\"", &format!("fill=\"{hex}\""))
            .replace("stroke=\"#FFFFFF\"", &format!("stroke=\"{hex}\""))
            .replace("stroke=\"#ffffff\"", &format!("stroke=\"{hex}\""))
            .replace("stroke=\"currentColor\"", &format!("stroke=\"{hex}\""))
    }
}

fn color_hex(color: u32) -> String {
    let red = (color >> RED_SHIFT) & CHANNEL_MASK;
    let green = (color >> GREEN_SHIFT) & CHANNEL_MASK;
    let blue = color & CHANNEL_MASK;
    let hex = format!("#{red:02X}{green:02X}{blue:02X}");
    debug_assert_eq!(HEX_COLOR_LENGTH, hex.len());
    hex
}

#[cfg(test)]
mod tests {
    use super::SvgIconRaster;
    use crate::test_assert::KucTestExpect;
    use crate::visual::canvas::Canvas;
    use katana_ui_core::render_model::UiIconProps;

    const BACKGROUND: u32 = 0x101010;
    const TEXT: u32 = 0xeeeeee;
    const MATERIAL_PAN_UP: &str = r##"<svg fill="#FFFFFF" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"><path d="M440-647 244-451q-12 12-28 11.5T188-452q-11-12-11.5-28t11.5-28l264-264q6-6 13-8.5t15-2.5q8 0 15 2.5t13 8.5l264 264q11 11 11 27.5T772-452q-12 12-28.5 12T715-452L520-647v447q0 17-11.5 28.5T480-160q-17 0-28.5-11.5T440-200v-447Z"/></svg>"##;
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
    fn katana_stroke_icon_tints_white_stroke_to_requested_color() {
        let icon = UiIconProps::new(KATANA_STROKE_PAN_UP)
            .role("surface.pan-up")
            .view_box("0 0 16 16");

        let themed = SvgIconRaster::themed_svg_source(&icon, 0x12AB34);

        assert!(themed.contains("stroke=\"#12AB34\""));
        assert!(!themed.contains("stroke=\"#FFFFFF\""));
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
