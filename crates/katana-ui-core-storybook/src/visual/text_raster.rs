use crate::visual::canvas::Canvas;
use crate::visual::text_raster_cache_entry::{
    RichTextRasterCacheEntry, RichTextRasterCacheKey, TextRasterCacheEntry, TextRasterCacheKey,
};
use crate::visual::text_raster_rasterize::{rasterize_rich_line, rasterize_text};
use crate::visual::text_raster_request::TextRasterDrawRequest;
use cosmic_text::{FontSystem, SwashCache};
use katana_ui_core::theme::FontToken;
use std::collections::HashMap;

pub(super) const TEXT_BUFFER_WIDTH: f32 = 4096.0;
pub(super) const TEXT_SUPERSAMPLE_SCALE: f32 = 2.0;
pub(super) const TEXT_RASTER_VERTICAL_GUARD_RATIO: f32 = 0.35;
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextCacheStats {
    pub(crate) entries: usize,
    pub(crate) raster_misses: usize,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct TextStyle {
    pub(super) size: f32,
    pub(super) line_height: f32,
    pub(super) color: u32,
    pub(super) italic: bool,
    pub(super) raster_vertical_scale: f32,
}

pub(super) struct RichTextRasterSpan<'a> {
    pub(super) text: &'a str,
    pub(super) style: TextStyle,
    pub(super) font: &'a FontToken,
    pub(super) emoji: bool,
}

impl TextStyle {
    pub(super) const fn new(size: f32, line_height: f32, color: u32) -> Self {
        Self {
            size,
            line_height,
            color,
            italic: false,
            raster_vertical_scale: 1.0,
        }
    }

    pub(super) const fn italic(mut self, value: bool) -> Self {
        self.italic = value;
        self
    }

    pub(super) const fn raster_vertical_scale(mut self, value: f32) -> Self {
        self.raster_vertical_scale = value;
        self
    }

    pub(super) const fn color(&self) -> u32 {
        self.color
    }

    pub(super) const fn is_italic(&self) -> bool {
        self.italic
    }
}

#[derive(Default)]
pub(super) struct TextRasterCache {
    entries: Vec<TextRasterCacheEntry>,
    entry_index: HashMap<TextRasterCacheKey, usize>,
    rich_entries: Vec<RichTextRasterCacheEntry>,
    rich_entry_index: HashMap<RichTextRasterCacheKey, usize>,
    raster_misses: usize,
}

impl TextRasterCache {
    pub(super) fn draw(
        &mut self,
        canvas: &mut Canvas,
        request: TextRasterDrawRequest<'_>,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
    ) {
        let raster_index = self.index_or_insert(
            request.text,
            request.style,
            request.font,
            request.emoji,
            font_system,
            swash_cache,
            request.scale_factor,
        );
        self.entries[raster_index].raster.draw(
            canvas,
            request.origin_x,
            request.origin_y,
            request.style.color,
            request.style.raster_vertical_scale,
        );
    }

    pub(super) fn measure_width(
        &mut self,
        text: &str,
        style: TextStyle,
        font: &FontToken,
        emoji: bool,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        scale_factor: f32,
    ) -> usize {
        let raster_index = self.index_or_insert(
            text,
            style,
            font,
            emoji,
            font_system,
            swash_cache,
            scale_factor,
        );
        self.entries[raster_index].raster.width()
    }

    pub(super) fn measure_width_uncached(
        text: &str,
        style: TextStyle,
        font: &FontToken,
        emoji: bool,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        scale_factor: f32,
    ) -> usize {
        rasterize_text(
            text,
            style,
            font,
            emoji,
            font_system,
            swash_cache,
            scale_factor,
        )
        .width()
    }

    pub(super) fn draw_rich_line(
        &mut self,
        canvas: &mut Canvas,
        spans: &[RichTextRasterSpan<'_>],
        origin_x: i32,
        origin_y: i32,
        scale_factor: f32,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
    ) {
        let Some(default_color) = spans.first().map(|span| span.style.color) else {
            return;
        };
        let raster_vertical_scale = spans
            .iter()
            .map(|span| span.style.raster_vertical_scale)
            .filter(|scale| scale.is_finite() && *scale > 1.0)
            .fold(1.0, f32::max);
        let raster_index = self.rich_index_or_insert(spans, font_system, swash_cache, scale_factor);
        self.rich_entries[raster_index].raster.draw(
            canvas,
            origin_x,
            origin_y,
            default_color,
            raster_vertical_scale,
        );
    }

    #[cfg(test)]
    pub(super) fn stats(&self) -> TextCacheStats {
        TextCacheStats {
            entries: self.entries.len() + self.rich_entries.len(),
            raster_misses: self.raster_misses,
        }
    }

    fn index_or_insert(
        &mut self,
        text: &str,
        style: TextStyle,
        font: &FontToken,
        emoji: bool,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        scale_factor: f32,
    ) -> usize {
        let key = TextRasterCacheKey::new(text, style, font, emoji, scale_factor);
        if let Some(index) = self.entry_index.get(&key).copied() {
            return index;
        }
        self.raster_misses += 1;
        let raster = rasterize_text(
            text,
            style,
            font,
            emoji,
            font_system,
            swash_cache,
            scale_factor,
        );
        let index = self.entries.len();
        self.entries.push(TextRasterCacheEntry { raster });
        self.entry_index.insert(key, index);
        index
    }

    fn rich_index_or_insert(
        &mut self,
        spans: &[RichTextRasterSpan<'_>],
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        scale_factor: f32,
    ) -> usize {
        let key = RichTextRasterCacheKey::new(spans, scale_factor);
        if let Some(index) = self.rich_entry_index.get(&key).copied() {
            return index;
        }
        self.raster_misses += 1;
        let raster = rasterize_rich_line(spans, font_system, swash_cache, scale_factor);
        let index = self.rich_entries.len();
        self.rich_entries
            .push(RichTextRasterCacheEntry::new(raster));
        self.rich_entry_index.insert(key, index);
        index
    }
}
