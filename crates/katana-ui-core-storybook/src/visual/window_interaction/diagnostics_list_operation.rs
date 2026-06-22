use super::StorybookWindowState;
use crate::visual::preview_detail;

const PAGE: &str = "diagnostics-list";
const TOOL_PRESET_INDEX: usize = 2;
const BULK_PRESET_INDEX: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum DiagnosticsListStoryAction {
    ToggleFixPreview,
    OpenBulkPreview,
    SelectItem,
    FocusList,
    HoverItem,
    KeyboardNavigate,
    ScrollRetention,
}

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<DiagnosticsListStoryAction> {
    if state.selected_page != PAGE {
        return None;
    }
    if !preview_detail::component_action_hit_rect(PAGE).contains(x, y) {
        return None;
    }
    Some(match state.preset_index {
        TOOL_PRESET_INDEX => DiagnosticsListStoryAction::SelectItem,
        BULK_PRESET_INDEX => DiagnosticsListStoryAction::OpenBulkPreview,
        _ => DiagnosticsListStoryAction::ToggleFixPreview,
    })
}
