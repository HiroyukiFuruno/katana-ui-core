use super::state::DiagnosticsListState;
use super::{DiagnosticId, DiagnosticItem, DiagnosticsListOptions, DiagnosticsListPlanner};

pub(super) fn selected_visible_id(
    state: &DiagnosticsListState,
    items: &[DiagnosticItem],
    options: &DiagnosticsListOptions,
) -> Option<DiagnosticId> {
    let selected_id = state.selected_id.as_ref()?;
    DiagnosticsListPlanner::visible_items_for_scope(
        items,
        options,
        state.selected_scope_key.as_ref(),
    )
    .into_iter()
    .find(|item| &item.id == selected_id)
    .map(|item| item.id.clone())
}
