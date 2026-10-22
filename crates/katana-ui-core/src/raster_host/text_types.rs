use katana_ui_core::text_raster::PlatformTextRasterizer;
use katana_ui_core::theme::FontToken;
use std::cell::RefCell;

pub struct TextRenderer {
    pub(super) rasterizer: RefCell<PlatformTextRasterizer>,
    pub(super) font: FontToken,
    pub(super) independent_raster_config: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct RichTextLineSpan {
    pub(super) text: String,
    pub(super) style: RichTextStyle,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct RichTextBidiContext {
    pub(in crate::raster_host) prefix: String,
    pub(in crate::raster_host) suffix: String,
}

impl RichTextLineSpan {
    pub(crate) fn with_text(&self, text: String) -> Self {
        Self {
            text,
            style: self.style,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RichTextStyle {
    pub(super) size: f32,
    pub(super) color: u32,
    pub(super) bold: bool,
    pub(super) italic: bool,
    pub(super) monospace: bool,
    pub(super) emoji: bool,
    pub(super) raster_vertical_scale: f32,
}
