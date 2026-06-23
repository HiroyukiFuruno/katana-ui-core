use super::text_raster::{RichTextRasterSpan, TextStyle};
use super::text_raster_pixels::CachedTextRaster;
use katana_ui_core::theme::{FontFamily, FontToken};

pub(super) struct TextRasterCacheEntry {
    pub(super) raster: CachedTextRaster,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct TextRasterCacheKey {
    text: String,
    size_bits: u32,
    line_height_bits: u32,
    scale_bits: u32,
    family: u8,
    font_name: String,
    weight: u16,
    italic: bool,
    emoji: bool,
}

impl TextRasterCacheKey {
    pub(super) fn new(
        text: &str,
        style: TextStyle,
        font: &FontToken,
        emoji: bool,
        scale_factor: f32,
    ) -> Self {
        Self {
            text: text.to_string(),
            size_bits: style.size.to_bits(),
            line_height_bits: style.line_height.to_bits(),
            scale_bits: scale_factor.to_bits(),
            family: font_family_key(font.family),
            font_name: font.name.clone(),
            weight: font.weight,
            italic: style.italic,
            emoji,
        }
    }
}

pub(super) struct RichTextRasterCacheEntry {
    pub(super) raster: CachedTextRaster,
}

impl RichTextRasterCacheEntry {
    pub(super) fn new(raster: CachedTextRaster) -> Self {
        Self { raster }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct RichTextRasterCacheKey {
    spans: Vec<RichTextRasterCacheSpan>,
    scale_bits: u32,
}

impl RichTextRasterCacheKey {
    pub(super) fn new(spans: &[RichTextRasterSpan<'_>], scale_factor: f32) -> Self {
        Self {
            spans: spans.iter().map(RichTextRasterCacheSpan::from).collect(),
            scale_bits: scale_factor.to_bits(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct RichTextRasterCacheSpan {
    text: String,
    size_bits: u32,
    line_height_bits: u32,
    color: u32,
    italic: bool,
    family: u8,
    font_name: String,
    weight: u16,
    emoji: bool,
}

impl RichTextRasterCacheSpan {
    fn from(span: &RichTextRasterSpan<'_>) -> Self {
        Self {
            text: span.text.to_string(),
            size_bits: span.style.size.to_bits(),
            line_height_bits: span.style.line_height.to_bits(),
            color: span.style.color,
            italic: span.style.italic,
            family: font_family_key(span.font.family),
            font_name: span.font.name.clone(),
            weight: span.font.weight,
            emoji: span.emoji,
        }
    }
}

fn font_family_key(family: FontFamily) -> u8 {
    match family {
        FontFamily::Proportional => 0,
        FontFamily::Monospace => 1,
    }
}
