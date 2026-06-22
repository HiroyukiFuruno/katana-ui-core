use crate::visual::text_raster::{
    RichTextRasterSpan, TEXT_BUFFER_WIDTH, TEXT_RASTER_VERTICAL_GUARD_RATIO,
    TEXT_SUPERSAMPLE_SCALE, TextStyle,
};
use crate::visual::text_raster_font::{attrs_for_rich_span, attrs_for_text};
use crate::visual::text_raster_pixels::{CachedTextRaster, raster_pixels};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache, Wrap};
use katana_ui_core::theme::FontToken;

pub(super) fn rasterize_text(
    text: &str,
    style: TextStyle,
    font: &FontToken,
    emoji: bool,
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
        Some(raster_buffer_height(
            metrics.line_height,
            style.size,
            supersample_scale,
        )),
    );
    buffer.set_text(
        text,
        &attrs_for_text(font, text, emoji, style.italic),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(false);
    CachedTextRaster::new(raster_pixels(&mut buffer, swash_cache, style.color))
}

pub(super) fn rasterize_rich_line(
    spans: &[RichTextRasterSpan<'_>],
    font_system: &mut FontSystem,
    swash_cache: &mut SwashCache,
    scale_factor: f32,
) -> CachedTextRaster {
    let Some(first) = spans.first() else {
        return CachedTextRaster::new(Vec::new());
    };
    let scale_factor = normalized_scale(scale_factor);
    let supersample_scale = TEXT_SUPERSAMPLE_SCALE * scale_factor;
    let metrics = Metrics::new(
        first.style.size * supersample_scale,
        first.style.line_height * supersample_scale,
    );
    let mut buffer = Buffer::new(font_system, metrics);
    let mut buffer = buffer.borrow_with(font_system);
    buffer.set_wrap(Wrap::None);
    buffer.set_size(
        Some(TEXT_BUFFER_WIDTH * supersample_scale),
        Some(raster_buffer_height(
            metrics.line_height,
            first.style.size,
            supersample_scale,
        )),
    );
    let rich_text = spans
        .iter()
        .map(|span| {
            (
                span.text,
                attrs_for_rich_span(span, supersample_scale / TEXT_SUPERSAMPLE_SCALE),
            )
        })
        .collect::<Vec<_>>();
    buffer.set_rich_text(rich_text, &Attrs::new(), Shaping::Advanced, None);
    buffer.shape_until_scroll(false);
    CachedTextRaster::new(raster_pixels(&mut buffer, swash_cache, first.style.color))
}

fn raster_buffer_height(line_height: f32, font_size: f32, supersample_scale: f32) -> f32 {
    line_height + font_size * supersample_scale * TEXT_RASTER_VERTICAL_GUARD_RATIO
}

fn normalized_scale(scale_factor: f32) -> f32 {
    if scale_factor.is_finite() && scale_factor >= 1.0 {
        scale_factor
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raster_buffer_height_keeps_vertical_guard_for_descenders() {
        let line_height = 35.94;
        let font_size = 24.79;
        let supersample_scale = 4.0;

        assert!(
            raster_buffer_height(line_height, font_size, supersample_scale) > line_height + 30.0
        );
    }
}
