use super::layout_metrics::LayoutRect;
use super::selection_screen_state::SelectionScreenAction;

pub(super) const TRIGGER_X: usize = 18;
pub(super) const TRIGGER_Y: usize = 32;
pub(super) const TRIGGER_WIDTH: usize = 166;
pub(super) const TRIGGER_HEIGHT: usize = 28;
pub(super) const SELECT_OPTIONS_Y: usize = 60;
pub(super) const SELECT_OPTION_HEIGHT: usize = 14;
pub(super) const SELECT_OPTION_COUNT: usize = 4;
pub(super) const COMBO_OPTIONS_Y: usize = 66;
pub(super) const COMBO_OPTION_HEIGHT: usize = 18;
pub(super) const COMBO_OPTION_COUNT: usize = 2;
pub(super) const SELECTION_LIST_Y: usize = 56;
pub(super) const SELECTION_LIST_ROW_HEIGHT: usize = 14;
pub(super) const SELECTION_LIST_ROW_COUNT: usize = 4;
pub(super) const OPTION_ROW_INSET: usize = 4;
pub(super) const OPTION_ROW_WIDTH_REDUCTION: usize = 8;
pub(super) const STATUS_X: usize = 204;
pub(super) const STATUS_Y: usize = 36;
pub(super) const STATUS_WIDTH: usize = 120;
pub(super) const STATUS_HEIGHT: usize = 20;
pub(super) const STATUS_GAP: usize = 8;
pub(super) const TEXT_X: usize = 10;
pub(super) const TEXT_Y: usize = 6;

pub(super) fn select_action_at(
    component: LayoutRect,
    is_open: bool,
    x: usize,
    y: usize,
) -> Option<SelectionScreenAction> {
    if trigger_rect(component).contains(x, y) {
        if is_open {
            return Some(SelectionScreenAction::SelectClose);
        }
        return Some(SelectionScreenAction::SelectOpen);
    }
    if !is_open {
        return None;
    }
    select_option_index_at(component, x, y).map(SelectionScreenAction::SelectOption)
}

pub(super) fn combo_action_at(
    component: LayoutRect,
    is_open: bool,
    is_filtered: bool,
    x: usize,
    y: usize,
) -> Option<SelectionScreenAction> {
    if !is_open {
        if trigger_rect(component).contains(x, y) {
            return Some(SelectionScreenAction::ComboFilter);
        }
        return None;
    }
    if trigger_rect(component).contains(x, y) {
        return Some(SelectionScreenAction::ComboFilter);
    }
    combo_option_index_at(component, is_filtered, x, y).map(SelectionScreenAction::ComboOption)
}

pub(super) fn selection_list_action_at(
    component: LayoutRect,
    x: usize,
    y: usize,
) -> Option<SelectionScreenAction> {
    selection_list_index_at(component, x, y).map(SelectionScreenAction::SelectionListToggle)
}

pub(super) fn trigger_rect(component: LayoutRect) -> LayoutRect {
    LayoutRect::new(
        component.x + TRIGGER_X,
        component.y + TRIGGER_Y,
        TRIGGER_WIDTH,
        TRIGGER_HEIGHT,
    )
}

fn select_option_index_at(component: LayoutRect, x: usize, y: usize) -> Option<usize> {
    option_index_at(
        component,
        x,
        y,
        SELECT_OPTIONS_Y,
        SELECT_OPTION_HEIGHT,
        SELECT_OPTION_COUNT,
    )
}

fn combo_option_index_at(
    component: LayoutRect,
    is_filtered: bool,
    x: usize,
    y: usize,
) -> Option<usize> {
    option_index_at(
        component,
        x,
        y,
        COMBO_OPTIONS_Y,
        COMBO_OPTION_HEIGHT,
        if is_filtered { 1 } else { COMBO_OPTION_COUNT },
    )
}

fn selection_list_index_at(component: LayoutRect, x: usize, y: usize) -> Option<usize> {
    option_index_at(
        component,
        x,
        y,
        SELECTION_LIST_Y,
        SELECTION_LIST_ROW_HEIGHT,
        SELECTION_LIST_ROW_COUNT,
    )
}

fn option_index_at(
    component: LayoutRect,
    x: usize,
    y: usize,
    options_y: usize,
    option_height: usize,
    option_count: usize,
) -> Option<usize> {
    let panel = LayoutRect::new(
        component.x + TRIGGER_X,
        component.y + options_y,
        TRIGGER_WIDTH,
        option_height * option_count,
    );
    if !panel.contains(x, y) {
        return None;
    }
    Some((y - panel.y) / option_height)
}
