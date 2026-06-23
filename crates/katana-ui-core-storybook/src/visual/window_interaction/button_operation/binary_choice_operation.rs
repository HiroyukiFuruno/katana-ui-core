use super::StorybookButtonOperation;
use crate::visual::dedicated_dod_form_binary_choice_live as binary_choice_live;
use crate::visual::preview_detail;

pub(super) fn operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    checkbox_operation_at(page, x, y).or_else(|| radio_operation_at(page, x, y))
}

fn checkbox_operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if page != "checkbox" {
        return None;
    }
    let base = preview_detail::component_action_hit_rect(page);
    for index in 0..2 {
        if binary_choice_live::row_rect(index, base.x, base.y).contains(x, y) {
            return Some(StorybookButtonOperation::CheckboxToggle(index));
        }
    }
    if binary_choice_live::checkbox_state_read_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::CheckboxStateRead);
    }
    if binary_choice_live::checkbox_toggle_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::CheckboxToggleFocused);
    }
    if binary_choice_live::checkbox_reset_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::CheckboxReset);
    }
    None
}

fn radio_operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if page != "radio" {
        return None;
    }
    let base = preview_detail::component_action_hit_rect(page);
    if binary_choice_live::row_rect(0, base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::RadioSelect);
    }
    if binary_choice_live::row_rect(1, base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::RadioSelectIndex(1));
    }
    if binary_choice_live::radio_state_read_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::RadioStateRead);
    }
    if binary_choice_live::radio_select_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::RadioSelect);
    }
    if binary_choice_live::radio_reset_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::RadioReset);
    }
    None
}
