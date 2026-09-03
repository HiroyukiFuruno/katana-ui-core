use super::super::text::TextRenderer;
use super::super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::super::ui_tree_canvas_scroll_height_cache::MeasuredNodeHeightCache;
use super::super::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;
use std::cell::RefCell;

pub struct UiTreeCanvasRenderer {
    pub(super) palette: UiTreeCanvasPalette,
    pub(super) text: TextRenderer,
    pub(in crate::raster_host) document_text: TextRenderer,
    pub(in crate::raster_host) export_text: TextRenderer,
    pub(in crate::raster_host) code_text: TextRenderer,
    pub(in crate::raster_host) typography: UiTreeDocumentTypography,
    pub(super) scroll_height_cache: RefCell<MeasuredNodeHeightCache>,
}
