use crate::visual::canvas::Canvas;
use fontdue::{Font, FontSettings};
use std::fs;

const FALLBACK_FONT: &str = "/System/Library/Fonts/SFNS.ttf";
const GLYPH_ALPHA_THRESHOLD: u8 = 32;
const FALLBACK_ADVANCE: usize = 7;
const FALLBACK_WIDTH: usize = 4;
const FALLBACK_HEIGHT: usize = 8;

pub(crate) struct TextRenderer {
    font: Option<Font>,
}

impl TextRenderer {
    pub(crate) fn load() -> Self {
        Self {
            font: fs::read(FALLBACK_FONT)
                .ok()
                .and_then(|bytes| Font::from_bytes(bytes, FontSettings::default()).ok()),
        }
    }

    pub(crate) fn draw(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: usize,
        y: usize,
        size: f32,
        color: u32,
    ) {
        if let Some(font) = self.font.as_ref() {
            draw_font_text(canvas, font, text, x, y, size, color);
        } else {
            draw_fallback_text(canvas, text, x, y, color);
        }
    }
}

fn draw_font_text(
    canvas: &mut Canvas,
    font: &Font,
    text: &str,
    x: usize,
    y: usize,
    size: f32,
    color: u32,
) {
    let mut cursor = x as i32;
    for character in text.chars() {
        let (metrics, bitmap) = font.rasterize(character, size);
        for row in 0..metrics.height {
            for column in 0..metrics.width {
                draw_glyph_pixel(
                    canvas,
                    GlyphPixel {
                        bitmap: &bitmap,
                        metrics: &metrics,
                        cursor,
                        origin_y: y,
                        row,
                        column,
                        color,
                    },
                );
            }
        }
        cursor += metrics.advance_width.ceil() as i32;
    }
}

struct GlyphPixel<'a> {
    bitmap: &'a [u8],
    metrics: &'a fontdue::Metrics,
    cursor: i32,
    origin_y: usize,
    row: usize,
    column: usize,
    color: u32,
}

fn draw_glyph_pixel(canvas: &mut Canvas, pixel: GlyphPixel<'_>) {
    let alpha = pixel.bitmap[pixel.row * pixel.metrics.width + pixel.column];
    if alpha <= GLYPH_ALPHA_THRESHOLD {
        return;
    }
    let current_x = pixel.cursor + pixel.column as i32 + pixel.metrics.xmin;
    let current_y = pixel.origin_y as i32 + pixel.row as i32;
    if current_x < 0 || current_y < 0 {
        return;
    }
    canvas.set(current_x as usize, current_y as usize, pixel.color);
}

fn draw_fallback_text(canvas: &mut Canvas, text: &str, x: usize, y: usize, color: u32) {
    for (index, _) in text.chars().enumerate() {
        canvas.fill_rect(
            x + index * FALLBACK_ADVANCE,
            y,
            FALLBACK_WIDTH,
            FALLBACK_HEIGHT,
            color,
        );
    }
}
