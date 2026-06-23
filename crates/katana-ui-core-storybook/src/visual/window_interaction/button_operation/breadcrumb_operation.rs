use super::{StorybookButtonOperation, StorybookWindowState};
use crate::visual::{dedicated_breadcrumb, preview_detail};

const ROOT_INDEX: usize = 0;
const SRC_INDEX: usize = 1;
const FILE_INDEX: usize = 2;

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if state.selected_page != "breadcrumb" {
        return None;
    }
    let component = preview_detail::component_action_hit_rect(state.selected_page);
    if dedicated_breadcrumb::root_crumb_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::BreadcrumbSelection(ROOT_INDEX));
    }
    if dedicated_breadcrumb::src_crumb_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::BreadcrumbSelection(SRC_INDEX));
    }
    if dedicated_breadcrumb::file_crumb_rect(component.x, component.y).contains(x, y) {
        Some(StorybookButtonOperation::BreadcrumbSelection(FILE_INDEX))
    } else {
        None
    }
}
