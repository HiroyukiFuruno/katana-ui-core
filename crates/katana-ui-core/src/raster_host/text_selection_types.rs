use super::layout_metrics::LayoutRect;
use katana_ui_core::text_selection::UiTextSelectionModel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectableTextRun {
    pub(in crate::raster_host) text: String,
    pub(in crate::raster_host) rect: LayoutRect,
    pub(in crate::raster_host) model: UiTextSelectionModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::raster_host) struct TextSelection {
    pub(in crate::raster_host) start: (usize, usize),
    pub(in crate::raster_host) end: (usize, usize),
}
