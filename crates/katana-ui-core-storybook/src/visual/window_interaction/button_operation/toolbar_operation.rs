use super::StorybookButtonOperation;
use crate::visual::dedicated_toolbar;
use crate::visual::preview_detail;

const ACTION_DISABLED_PRESET_INDEX: usize = 12;
const SPLIT_DISABLED_PRESET_INDEX: usize = 15;
const SPLIT_ACTION_INDEX: usize = 1;

pub(super) fn operation_at(
    state: &super::StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    hovered_action_index_at(state.selected_page, x, y)
        .map(StorybookButtonOperation::ToolbarActionButton)
}

pub(super) fn hovered_action_index_at(page: &str, x: usize, y: usize) -> Option<usize> {
    if page != "toolbar" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(page);
    dedicated_toolbar::action_index_at(origin.x, origin.y, x, y)
}

pub(super) fn is_action_disabled(preset_index: usize, action_index: usize) -> bool {
    preset_index == ACTION_DISABLED_PRESET_INDEX
        || (preset_index == SPLIT_DISABLED_PRESET_INDEX && action_index == SPLIT_ACTION_INDEX)
}
