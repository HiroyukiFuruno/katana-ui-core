use super::layout_metrics::LayoutRect;
use katana_ui_core::text_selection::UiTextSelectionModel;

#[derive(Debug, Clone)]
pub struct SelectableTextRun {
    pub(in crate::raster_host) text: String,
    pub(in crate::raster_host) rect: LayoutRect,
    pub(in crate::raster_host) model: UiTextSelectionModel,
    pub(in crate::raster_host) logical_line: Option<u64>,
    pub(in crate::raster_host) visual_line: Option<u64>,
    pub(in crate::raster_host) separator_before: String,
    pub(in crate::raster_host) resolved_bidi_rtl: Option<bool>,
}

impl PartialEq for SelectableTextRun {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.rect == other.rect
            && self.model == other.model
            && self.separator_before == other.separator_before
            && self.resolved_bidi_rtl == other.resolved_bidi_rtl
    }
}

impl Eq for SelectableTextRun {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::raster_host) struct TextSelection {
    pub(in crate::raster_host) start: (usize, usize),
    pub(in crate::raster_host) end: (usize, usize),
}
