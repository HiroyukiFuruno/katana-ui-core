use super::{StorybookButtonOperation, StorybookWindowState};
use crate::visual::dedicated_dod_form_combo_live as combo_live;
use crate::visual::dedicated_dod_form_input_live as input_live;
use crate::visual::dedicated_dod_form_select_live as select_live;
use crate::visual::dedicated_dod_form_selection_list_live as selection_list_live;
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::preview_detail;
use crate::visual::selection_control_metrics;
use crate::visual::selection_screen_state::SelectionScreenAction;

const FIRST_DYNAMIC_OPTION_INDEX: usize = 1;

pub(super) struct SelectionOperation;

impl SelectionOperation {
    pub(super) fn operation_at(
        state: &StorybookWindowState,
        x: usize,
        y: usize,
    ) -> Option<StorybookButtonOperation> {
        let page = state.selected_page;
        let component = preview_detail::component_action_hit_rect(page);
        select_button_operation_at(page, component, x, y)
            .or_else(|| combo_button_operation_at(page, component, x, y))
            .or_else(|| search_button_operation_at(page, component, x, y))
            .or_else(|| selection_list_button_operation_at(page, component, x, y))
            .or_else(|| dynamic_operation_at(state, page, component, x, y))
    }
}

fn select_button_operation_at(
    page: &str,
    component: LayoutRect,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if page != "select-box" {
        return None;
    }
    if select_live::select_state_read_button_rect(component.x, component.y).contains(x, y) {
        return selection_operation(SelectionScreenAction::SelectStateRead);
    }
    if select_live::select_open_button_rect(component.x, component.y).contains(x, y) {
        return selection_operation(SelectionScreenAction::SelectOpen);
    }
    if select_live::select_close_button_rect(component.x, component.y).contains(x, y) {
        return selection_operation(SelectionScreenAction::SelectClose);
    }
    if select_live::select_reset_button_rect(component.x, component.y).contains(x, y) {
        return selection_operation(SelectionScreenAction::SelectReset);
    }
    None
}

fn combo_button_operation_at(
    page: &str,
    component: LayoutRect,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if page != "combo-box" {
        return None;
    }
    if combo_live::combo_state_read_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::ComboStateRead);
    }
    if combo_live::combo_filter_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::ComboFilter);
    }
    if combo_live::combo_select_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::ComboSelect);
    }
    if combo_live::combo_reset_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::ComboReset);
    }
    None
}

fn search_button_operation_at(
    page: &str,
    component: LayoutRect,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if page != "search-box" {
        return None;
    }
    if input_live::search_inline_clear_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::SearchClear);
    }
    if input_live::search_field_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::SearchTypeQuery);
    }
    if input_live::search_state_read_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::SearchStateRead);
    }
    if input_live::search_type_query_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::SearchTypeQuery);
    }
    if input_live::search_submit_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::SearchSubmit);
    }
    if input_live::search_clear_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::SearchClear);
    }
    if input_live::search_case_toggle_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::SearchCaseToggle);
    }
    if input_live::search_regex_toggle_button_rect(component.x, component.y).contains(x, y) {
        return Some(StorybookButtonOperation::SearchRegexToggle);
    }
    None
}

fn selection_list_button_operation_at(
    page: &str,
    component: LayoutRect,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if page != "selection-list" {
        return None;
    }
    if selection_list_live::selection_list_state_read_button_rect(component.x, component.y)
        .contains(x, y)
    {
        return selection_operation(SelectionScreenAction::SelectionListStateRead);
    }
    if selection_list_live::selection_list_select_row_button_rect(component.x, component.y)
        .contains(x, y)
    {
        return selection_operation(SelectionScreenAction::SelectionListSelectRow(
            FIRST_DYNAMIC_OPTION_INDEX,
        ));
    }
    if selection_list_live::selection_list_multi_toggle_button_rect(component.x, component.y)
        .contains(x, y)
    {
        return selection_operation(SelectionScreenAction::SelectionListMultiToggle(
            FIRST_DYNAMIC_OPTION_INDEX,
        ));
    }
    if selection_list_live::selection_list_keyboard_next_button_rect(component.x, component.y)
        .contains(x, y)
    {
        return selection_operation(SelectionScreenAction::SelectionListKeyboardNext);
    }
    if selection_list_live::selection_list_reset_button_rect(component.x, component.y)
        .contains(x, y)
    {
        return selection_operation(SelectionScreenAction::SelectionListReset);
    }
    None
}

fn dynamic_operation_at(
    state: &StorybookWindowState,
    page: &str,
    component: LayoutRect,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    let action = match page {
        "select-box" => selection_control_metrics::select_action_at(
            component,
            state.screen_state.selection.select_open,
            x,
            y,
        ),
        "combo-box" => combo_action_at(state, component, x, y),
        "selection-list" => selection_control_metrics::selection_list_action_at(component, x, y),
        _ => None,
    };
    action.map(StorybookButtonOperation::SelectionControl)
}

fn combo_action_at(
    state: &StorybookWindowState,
    component: LayoutRect,
    x: usize,
    y: usize,
) -> Option<SelectionScreenAction> {
    let action = selection_control_metrics::combo_action_at(
        component,
        state.screen_state.selection.combo_open,
        state.screen_state.selection.combo_filtered,
        x,
        y,
    )?;
    match action {
        SelectionScreenAction::ComboOption(index)
            if state.screen_state.selection.combo_filtered =>
        {
            Some(SelectionScreenAction::ComboOption(
                index + FIRST_DYNAMIC_OPTION_INDEX,
            ))
        }
        _ => Some(action),
    }
}

fn selection_operation(action: SelectionScreenAction) -> Option<StorybookButtonOperation> {
    Some(StorybookButtonOperation::SelectionControl(action))
}
