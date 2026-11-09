//! Generic retained file selection and search-result dialogs. Host tokens carry no UI meaning.

mod highlight;
mod model;
mod paint;
mod query_history;
mod retained;
mod search_model;

pub use highlight::{SearchHighlightProjection, SearchHighlightProjectionError};
pub use model::{
    FileSelectionItem, FileSelectionProjection, KucDialogControls, KucDialogTarget,
    SelectionDialogEvent, SelectionDialogOperation, SelectionDialogProjection,
    SelectionDialogProjectionError, SelectionDialogProjectionLease,
};
pub use query_history::{SearchQueryArrowKeyPolicy, SearchQueryHistoryDirection};
pub(crate) use retained::{SelectionDialogOutput, SelectionDialogRetainedState};
pub use search_model::{
    SearchFilterValidity, SearchModePresentation, SearchResultMode, SearchResultOption,
    SearchResultProjection, SearchResultRow,
};
