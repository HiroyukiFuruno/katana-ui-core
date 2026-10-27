//! Generic retained file selection and search-result dialogs. Host tokens carry no UI meaning.

mod highlight;
mod model;
mod paint;
mod query_history;
mod retained;

pub use highlight::{SearchHighlightProjection, SearchHighlightProjectionError};
pub use model::{
    FileSelectionItem, FileSelectionProjection, KucDialogControls, KucDialogTarget,
    SearchResultMode, SearchResultOption, SearchResultProjection, SearchResultRow,
    SelectionDialogEvent, SelectionDialogOperation, SelectionDialogProjection,
    SelectionDialogProjectionError, SelectionDialogProjectionLease,
};
pub use query_history::{SearchQueryArrowKeyPolicy, SearchQueryHistoryDirection};
pub(crate) use retained::{SelectionDialogOutput, SelectionDialogRetainedState};
