use super::super::text_raster::{TextRasterCache, TextStyle};
use cosmic_text::{FontSystem, SwashCache};
use katana_ui_core::theme::FontToken;
use std::cell::RefCell;

pub struct TextRenderer {
    pub(super) font_system: RefCell<FontSystem>,
    pub(super) swash_cache: RefCell<SwashCache>,
    pub(super) raster_cache: RefCell<TextRasterCache>,
    pub(super) font: FontToken,
}

#[derive(Debug, Clone)]
pub(crate) struct RichTextLineSpan {
    pub(super) text: String,
    pub(super) font: FontToken,
    pub(super) style: TextStyle,
    pub(super) emoji: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RichTextStyle {
    pub(super) size: f32,
    pub(super) color: u32,
    pub(super) bold: bool,
    pub(super) italic: bool,
    pub(super) emoji: bool,
    pub(super) raster_vertical_scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextOrigin {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextBox {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) horizontal_align: TextHorizontalAlign,
    pub(super) vertical_align: TextVerticalAlign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextHorizontalAlign {
    Start,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextVerticalAlign {
    Top,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TextVerticalBox {
    pub(super) y: usize,
    pub(super) height: f32,
}
