use super::super::text::TextRenderer;
use super::super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::super::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;

pub(in crate::raster_host) struct UiTreeTextRenderer;

#[derive(Clone, Copy)]
pub(in crate::raster_host) struct UiTreeTextContext<'a> {
    pub(in crate::raster_host) text: &'a TextRenderer,
    pub(in crate::raster_host) export_text: &'a TextRenderer,
    pub(in crate::raster_host) code_text: &'a TextRenderer,
    pub(in crate::raster_host) palette: UiTreeCanvasPalette,
    pub(in crate::raster_host) typography: UiTreeDocumentTypography,
}
