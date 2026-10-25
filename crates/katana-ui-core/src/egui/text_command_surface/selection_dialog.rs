//! Generic retained file selection and search-result dialogs. Host tokens carry no UI meaning.

mod model;
mod paint;
mod retained;

pub use model::{
    FileSelectionItem, FileSelectionProjection, KucDialogControls, KucDialogTarget,
    SearchResultMode, SearchResultOption, SearchResultProjection, SearchResultRow,
    SelectionDialogEvent, SelectionDialogOperation, SelectionDialogProjection,
    SelectionDialogProjectionError, SelectionDialogProjectionLease,
};
pub(crate) use retained::{SelectionDialogOutput, SelectionDialogRetainedState};
