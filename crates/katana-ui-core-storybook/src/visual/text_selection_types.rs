use super::layout_metrics::LayoutRect;
use katana_ui_core::text_selection::UiTextSelectionModel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectableTextRun {
    pub(in crate::visual) text: String,
    pub(in crate::visual) rect: LayoutRect,
    pub(in crate::visual) model: UiTextSelectionModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct TextSelection {
    pub(in crate::visual) start: (usize, usize),
    pub(in crate::visual) end: (usize, usize),
}
