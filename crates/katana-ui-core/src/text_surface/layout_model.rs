use crate::render_model::UiRect;
use crate::text_selection::UiTextSelectionModel;
use crate::text_selection::UiTextSelectionRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceGraphemeBox {
    pub grapheme_index: usize,
    pub byte_start: usize,
    pub byte_end: usize,
    pub bounds: UiRect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceLineBox {
    pub logical_row: usize,
    pub bounds: UiRect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceCompositionLayout {
    pub source_start: usize,
    pub source_end: usize,
    pub preedit: String,
    pub preedit_range: UiTextSelectionRange,
    pub caret_range: UiTextSelectionRange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceLayout {
    pub identity: String,
    pub content_bounds: UiRect,
    pub graphemes: Vec<TextSurfaceGraphemeBox>,
    pub lines: Vec<TextSurfaceLineBox>,
    pub(super) text: String,
    pub(super) composition: Option<TextSurfaceCompositionLayout>,
    pub(super) selection_model: UiTextSelectionModel,
}
