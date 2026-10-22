mod model;
mod model_bidi;
mod types;

pub use model::UiTextSelectionModel;
pub(crate) use types::UiTextCaretPosition;
pub use types::{
    UiTextCaretHit, UiTextGlyphBox, UiTextLineBox, UiTextPasteResult, UiTextSelectionRange,
};
