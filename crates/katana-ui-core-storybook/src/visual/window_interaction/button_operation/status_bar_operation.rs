use super::StorybookButtonOperation;
use crate::visual::dedicated_status_bar;
use crate::visual::preview_detail;

pub(super) fn operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    segment_index_at(page, x, y).map(StorybookButtonOperation::StatusBarSegment)
}

pub(super) fn segment_index_at(page: &str, x: usize, y: usize) -> Option<usize> {
    if page != "status-bar" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(page);
    dedicated_status_bar::segment_index_at(origin.x, origin.y, x, y)
}
