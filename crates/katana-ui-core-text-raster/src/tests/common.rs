use crate::{PlatformTextGraphemeBounds, PlatformTextRaster};
use katana_ui_core::theme::{FontFamily, FontToken};

pub(super) const RGBA_CHANNEL_COUNT: usize = crate::model::RGBA_CHANNEL_COUNT;
const RGBA_ALPHA_INDEX: usize = crate::model::RGBA_ALPHA_INDEX;
pub(super) const TEXT_COLOR: [u8; RGBA_CHANNEL_COUNT] = [245, 245, 245, 255];
const EDITOR_FONT_SIZE_PX: f32 = 18.0;
const EDITOR_FONT_WEIGHT: u16 = 400;

pub(super) fn font() -> FontToken {
    FontToken {
        name: "editor".to_string(),
        family: FontFamily::Monospace,
        size: EDITOR_FONT_SIZE_PX,
        weight: EDITOR_FONT_WEIGHT,
    }
}

pub(super) fn has_alpha_pixels(
    raster: &PlatformTextRaster,
    bounds: &PlatformTextGraphemeBounds,
    scale: f32,
) -> bool {
    let left = (bounds.x * scale).floor().max(0.0) as usize;
    let top = (bounds.y * scale).floor().max(0.0) as usize;
    let right = ((bounds.x + bounds.width) * scale).ceil() as usize;
    let bottom = ((bounds.y + bounds.height) * scale).ceil() as usize;
    (top..bottom.min(raster.height)).any(|y| {
        (left..right.min(raster.width))
            .any(|x| raster.rgba_pixels[y * raster.width + x][RGBA_ALPHA_INDEX] != 0)
    })
}

pub(super) fn grapheme_pixels(
    raster: &PlatformTextRaster,
    bounds: &PlatformTextGraphemeBounds,
    scale: f32,
) -> Vec<[u8; RGBA_CHANNEL_COUNT]> {
    let left = (bounds.x * scale).floor().max(0.0) as usize;
    let top = (bounds.y * scale).floor().max(0.0) as usize;
    let right = ((bounds.x + bounds.width) * scale).ceil() as usize;
    let bottom = ((bounds.y + bounds.height) * scale).ceil() as usize;

    (top..bottom.min(raster.height))
        .flat_map(|y| {
            (left..right.min(raster.width)).map(move |x| raster.rgba_pixels[y * raster.width + x])
        })
        .collect()
}
