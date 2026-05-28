use super::StorybookWindowState;
use super::button_operation::{self, StorybookButtonOperation, button_operation_at};
use super::text_area_resize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum StorybookCursorStyle {
    Arrow,
    Ibeam,
    ResizeAll,
    PointingHand,
}

pub(in crate::visual) fn cursor_style_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> StorybookCursorStyle {
    if text_area_resize::handle_at(state, x, y) {
        return StorybookCursorStyle::ResizeAll;
    }
    let Some(operation) = button_operation_at(state, x, y) else {
        return StorybookCursorStyle::Arrow;
    };
    match operation {
        StorybookButtonOperation::TextInputFocus { .. }
        | StorybookButtonOperation::TextAreaFocus => StorybookCursorStyle::Ibeam,
        StorybookButtonOperation::PreviewComponent
            if !button_operation::is_button_preview_page(state.selected_page) =>
        {
            StorybookCursorStyle::Arrow
        }
        _ => StorybookCursorStyle::PointingHand,
    }
}
