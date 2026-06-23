use super::super::text::TextRenderer;
use super::super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::super::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;

pub(in crate::visual) struct UiTreeTextRenderer;

#[derive(Clone, Copy)]
pub(in crate::visual) struct UiTreeTextContext<'a> {
    pub(in crate::visual) text: &'a TextRenderer,
    pub(in crate::visual) export_text: &'a TextRenderer,
    pub(in crate::visual) code_text: &'a TextRenderer,
    pub(in crate::visual) palette: UiTreeCanvasPalette,
    pub(in crate::visual) typography: UiTreeDocumentTypography,
}
