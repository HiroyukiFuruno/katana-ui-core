use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::render_model::{UiIconProps, UiSvgPaintPolicy};

pub(crate) const RGBA_CHANNEL_COUNT: usize = 4;
const RED_CHANNEL_INDEX: usize = 0;
const GREEN_CHANNEL_INDEX: usize = 1;
const BLUE_CHANNEL_INDEX: usize = 2;
const ALPHA_CHANNEL_INDEX: usize = 3;

pub(crate) struct SvgPaintProcessor;

impl SvgPaintProcessor {
    pub(crate) fn apply_paint_policy(icon: &UiIconProps, color: RgbaColor) -> String {
        let hex = format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue);
        match icon.paint_policy {
            UiSvgPaintPolicy::CurrentColor | UiSvgPaintPolicy::StrokeAndFill => replace_paint(
                replace_paint(icon.svg_source.clone(), "fill", &hex),
                "stroke",
                &hex,
            ),
            UiSvgPaintPolicy::StrokeOnly => replace_paint(icon.svg_source.clone(), "stroke", &hex),
            UiSvgPaintPolicy::FillOnly => replace_paint(icon.svg_source.clone(), "fill", &hex),
        }
    }

    pub(crate) fn unpremultiply(pixels: &[u8]) -> Vec<u8> {
        let (pixels, _) = pixels.as_chunks::<RGBA_CHANNEL_COUNT>();
        pixels
            .iter()
            .flat_map(|pixel| {
                let alpha = u32::from(pixel[ALPHA_CHANNEL_INDEX]);
                if alpha == 0 {
                    return [0, 0, 0, 0];
                }
                [
                    normalize_channel(pixel[RED_CHANNEL_INDEX], alpha),
                    normalize_channel(pixel[GREEN_CHANNEL_INDEX], alpha),
                    normalize_channel(pixel[BLUE_CHANNEL_INDEX], alpha),
                    pixel[ALPHA_CHANNEL_INDEX],
                ]
            })
            .collect()
    }

    pub(crate) fn apply_alpha(mut pixels: Vec<u8>, alpha: u8) -> Vec<u8> {
        if alpha == u8::MAX {
            return pixels;
        }
        let (pixel_chunks, _) = pixels.as_chunks_mut::<RGBA_CHANNEL_COUNT>();
        for pixel in pixel_chunks {
            pixel[ALPHA_CHANNEL_INDEX] = ((u16::from(pixel[ALPHA_CHANNEL_INDEX])
                * u16::from(alpha))
                / u16::from(u8::MAX)) as u8;
        }
        pixels
    }
}

fn replace_paint(source: String, attribute: &str, hex: &str) -> String {
    ["currentColor", "#FFFFFF", "#ffffff", "white"]
        .into_iter()
        .fold(source, |current, existing| {
            current.replace(
                &format!("{attribute}=\"{existing}\""),
                &format!("{attribute}=\"{hex}\""),
            )
        })
}

fn normalize_channel(channel: u8, alpha: u32) -> u8 {
    ((u32::from(channel) * u32::from(u8::MAX)) / alpha).min(u32::from(u8::MAX)) as u8
}
