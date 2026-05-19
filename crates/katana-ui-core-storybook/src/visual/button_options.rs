pub(super) use super::button_options_draw::draw_controls;
use super::layout_metrics::{INSPECTOR_WIDTH, INSPECTOR_X, INSPECTOR_Y, LayoutRect};
#[path = "button_options_model.rs"]
mod model;
pub(in crate::visual) use model::{
    StorybookButtonHeightMode, StorybookButtonOptionControl, StorybookButtonOptions,
    StorybookButtonWidthMode, StorybookButtonZIndex,
};

pub(super) const SECTION_X: usize = INSPECTOR_X + 18;
pub(super) const SECTION_WIDTH: usize = INSPECTOR_WIDTH - 36;
pub(super) const ROW_X: usize = SECTION_X + 10;
pub(super) const ROW_WIDTH: usize = SECTION_WIDTH - 20;
pub(super) const ROW_HEIGHT: usize = 30;
pub(super) const ROW_GAP: usize = 8;
pub(super) const FIRST_ROW_Y_OFFSET: usize = 42;
pub(super) const CONTROL_COUNT: usize = 9;
const WIDTH_CONTROL_INDEX: usize = 3;
const HEIGHT_CONTROL_INDEX: usize = 4;
const BORDER_CONTROL_INDEX: usize = 5;
const LABEL_CONTROL_INDEX: usize = 6;
const TAB_INDEX_CONTROL_INDEX: usize = 7;
const Z_INDEX_CONTROL_INDEX: usize = 8;
const INSPECTOR_HEADER_OFFSET: usize = 78;

pub(super) fn is_button_page(page: &str) -> bool {
    matches!(
        page,
        "button" | "text-button" | "svg-button" | "icon-text-button"
    )
}

pub(super) fn control_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOptionControl> {
    if !is_button_page(page) {
        return None;
    }
    StorybookButtonOptionControl::all()
        .into_iter()
        .find(|control| control_rect(*control).contains(x, y))
}

pub(super) fn control_rect(control: StorybookButtonOptionControl) -> LayoutRect {
    let index = control_index(control);
    LayoutRect::new(ROW_X, row_y(index), ROW_WIDTH, ROW_HEIGHT)
}

pub(super) const fn control_index(control: StorybookButtonOptionControl) -> usize {
    match control {
        StorybookButtonOptionControl::Visible => 0,
        StorybookButtonOptionControl::Disabled => 1,
        StorybookButtonOptionControl::Focusable => 2,
        StorybookButtonOptionControl::Width => WIDTH_CONTROL_INDEX,
        StorybookButtonOptionControl::Height => HEIGHT_CONTROL_INDEX,
        StorybookButtonOptionControl::Border => BORDER_CONTROL_INDEX,
        StorybookButtonOptionControl::Label => LABEL_CONTROL_INDEX,
        StorybookButtonOptionControl::TabIndex => TAB_INDEX_CONTROL_INDEX,
        StorybookButtonOptionControl::ZIndex => Z_INDEX_CONTROL_INDEX,
    }
}

const fn row_y(index: usize) -> usize {
    INSPECTOR_Y + INSPECTOR_HEADER_OFFSET + FIRST_ROW_Y_OFFSET + index * (ROW_HEIGHT + ROW_GAP)
}
