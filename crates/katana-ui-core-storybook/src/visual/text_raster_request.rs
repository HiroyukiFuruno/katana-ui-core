use super::text_raster::TextStyle;
use katana_ui_core::theme::FontToken;

pub(super) struct TextRasterDrawRequest<'a> {
    pub(super) text: &'a str,
    pub(super) style: TextStyle,
    pub(super) font: &'a FontToken,
    pub(super) emoji: bool,
    pub(super) origin_x: i32,
    pub(super) origin_y: i32,
    pub(super) scale_factor: f32,
}
