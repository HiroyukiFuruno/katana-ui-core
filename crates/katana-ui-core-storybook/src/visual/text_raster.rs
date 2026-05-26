use crate::visual::canvas::Canvas;
use cosmic_text::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache, Weight, Wrap,
};
use katana_ui_core::theme::{FontFamily, FontToken};

const TEXT_BUFFER_WIDTH: f32 = 4096.0;
const TEXT_SUPERSAMPLE_SCALE: f32 = 2.0;
const TEXT_SUPERSAMPLE_SAMPLES: u32 = 4;
const REGULAR_WEIGHT: u16 = 400;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;
const OPAQUE_ALPHA: u8 = 0xff;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextCacheStats {
    pub(crate) entries: usize,
    pub(crate) raster_misses: usize,
}

#[derive(Clone, Copy)]
pub(super) struct TextStyle {
    size: f32,
    line_height: f32,
    color: u32,
}

impl TextStyle {
    pub(super) const fn new(size: f32, line_height: f32, color: u32) -> Self {
        Self {
            size,
            line_height,
            color,
        }
    }
}

#[derive(Default)]
pub(super) struct TextRasterCache {
    entries: Vec<TextRasterCacheEntry>,
    raster_misses: usize,
}

impl TextRasterCache {
    pub(super) fn draw(
        &mut self,
        canvas: &mut Canvas,
        text: &str,
        style: TextStyle,
        font: &FontToken,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        origin_x: usize,
        origin_y: usize,
        scale_factor: f32,
    ) {
        let raster_index =
            self.index_or_insert(text, style, font, font_system, swash_cache, scale_factor);
        self.entries[raster_index]
            .raster
            .draw(canvas, origin_x, origin_y, style.color);
    }

    #[cfg(test)]
    pub(super) fn stats(&self) -> TextCacheStats {
        TextCacheStats {
            entries: self.entries.len(),
            raster_misses: self.raster_misses,
        }
    }

    fn index_or_insert(
        &mut self,
        text: &str,
        style: TextStyle,
        font: &FontToken,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        scale_factor: f32,
    ) -> usize {
        if let Some(index) = self
            .entries
            .iter()
            .position(|entry| entry.matches(text, style, font, scale_factor))
        {
            return index;
        }
        self.raster_misses += 1;
        let raster = rasterize_text(text, style, font, font_system, swash_cache, scale_factor);
        self.entries.push(TextRasterCacheEntry {
            text: text.to_string(),
            size_bits: style.size.to_bits(),
            line_height_bits: style.line_height.to_bits(),
            scale_bits: scale_factor.to_bits(),
            family: font.family,
            weight: font.weight,
            raster,
        });
        self.entries.len() - 1
    }
}

struct TextRasterCacheEntry {
    text: String,
    size_bits: u32,
    line_height_bits: u32,
    scale_bits: u32,
    family: FontFamily,
    weight: u16,
    raster: CachedTextRaster,
}

impl TextRasterCacheEntry {
    fn matches(&self, text: &str, style: TextStyle, font: &FontToken, scale_factor: f32) -> bool {
        self.text == text
            && self.size_bits == style.size.to_bits()
            && self.line_height_bits == style.line_height.to_bits()
            && self.scale_bits == scale_factor.to_bits()
            && self.family == font.family
            && self.weight == font.weight
    }
}

struct CachedTextRaster {
    pixels: Vec<CachedTextPixel>,
}

impl CachedTextRaster {
    fn draw(&self, canvas: &mut Canvas, origin_x: usize, origin_y: usize, color: u32) {
        let origin_x = origin_x as i32;
        let origin_y = origin_y as i32;
        for pixel in &self.pixels {
            let x = origin_x + pixel.x;
            let y = origin_y + pixel.y;
            if x < 0 || y < 0 {
                continue;
            }
            canvas.blend_physical(x as usize, y as usize, color, pixel.alpha);
        }
    }
}

struct CachedTextPixel {
    x: i32,
    y: i32,
    alpha: u8,
}

#[derive(Clone, Copy)]
struct SuperSample {
    x: i32,
    y: i32,
    alpha: u8,
}

fn rasterize_text(
    text: &str,
    style: TextStyle,
    font: &FontToken,
    font_system: &mut FontSystem,
    swash_cache: &mut SwashCache,
    scale_factor: f32,
) -> CachedTextRaster {
    let scale_factor = normalized_scale(scale_factor);
    let supersample_scale = TEXT_SUPERSAMPLE_SCALE * scale_factor;
    let metrics = Metrics::new(
        style.size * supersample_scale,
        style.line_height * supersample_scale,
    );
    let mut buffer = Buffer::new(font_system, metrics);
    let mut buffer = buffer.borrow_with(font_system);
    buffer.set_wrap(Wrap::None);
    buffer.set_size(
        Some(TEXT_BUFFER_WIDTH * supersample_scale),
        Some(metrics.line_height),
    );
    buffer.set_text(text, &attrs_for_text(font, text), Shaping::Advanced, None);
    buffer.shape_until_scroll(false);
    CachedTextRaster {
        pixels: raster_pixels(&mut buffer, swash_cache, style.color),
    }
}

fn raster_pixels(
    buffer: &mut cosmic_text::BorrowedWithFontSystem<'_, Buffer>,
    swash_cache: &mut SwashCache,
    color: u32,
) -> Vec<CachedTextPixel> {
    let mut samples = Vec::new();
    buffer.draw(swash_cache, text_color(color), |left, top, _, _, color| {
        if color.a() == 0 {
            return;
        }
        samples.push(SuperSample {
            x: (left as f32 / TEXT_SUPERSAMPLE_SCALE).floor() as i32,
            y: (top as f32 / TEXT_SUPERSAMPLE_SCALE).floor() as i32,
            alpha: color.a(),
        });
    });
    samples.sort_unstable_by_key(|sample| (sample.y, sample.x));
    combine_samples(&samples)
}

fn combine_samples(samples: &[SuperSample]) -> Vec<CachedTextPixel> {
    let mut pixels = Vec::with_capacity(samples.len());
    let mut index = 0;
    while index < samples.len() {
        let current = samples[index];
        let mut alpha_sum = 0u32;
        while index < samples.len()
            && samples[index].x == current.x
            && samples[index].y == current.y
        {
            alpha_sum += u32::from(samples[index].alpha);
            index += 1;
        }
        let alpha = ((alpha_sum as f32 / TEXT_SUPERSAMPLE_SAMPLES as f32).round() as u32)
            .min(u32::from(OPAQUE_ALPHA));
        if alpha != 0 {
            pixels.push(CachedTextPixel {
                x: current.x,
                y: current.y,
                alpha: alpha as u8,
            });
        }
    }
    pixels
}

fn normalized_scale(scale_factor: f32) -> f32 {
    if scale_factor.is_finite() && scale_factor >= 1.0 {
        scale_factor
    } else {
        1.0
    }
}

fn attrs_for_text<'a>(font: &'a FontToken, text: &str) -> Attrs<'a> {
    Attrs::new()
        .family(family_for_text(font.family, text))
        .weight(Weight(font.weight.max(REGULAR_WEIGHT)))
}

fn family_for_text(family: FontFamily, text: &str) -> Family<'static> {
    match family {
        FontFamily::Proportional => Family::SansSerif,
        FontFamily::Monospace if text.is_ascii() => Family::Monospace,
        FontFamily::Monospace => Family::SansSerif,
    }
}

fn text_color(color: u32) -> Color {
    Color::rgba(red(color), green(color), blue(color), OPAQUE_ALPHA)
}

fn red(color: u32) -> u8 {
    ((color >> RED_SHIFT) & CHANNEL_MASK) as u8
}

fn green(color: u32) -> u8 {
    ((color >> GREEN_SHIFT) & CHANNEL_MASK) as u8
}

fn blue(color: u32) -> u8 {
    (color & CHANNEL_MASK) as u8
}
