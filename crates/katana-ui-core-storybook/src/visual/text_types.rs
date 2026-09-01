use katana_ui_core::text_raster::PlatformTextRasterizer;
use katana_ui_core::theme::FontToken;
use std::cell::RefCell;

pub struct TextRenderer {
    pub(super) rasterizer: RefCell<PlatformTextRasterizer>,
    pub(super) font: FontToken,
}

#[derive(Debug, Clone)]
pub(crate) struct RichTextLineSpan {
    pub(super) text: String,
    pub(super) style: RichTextStyle,
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
