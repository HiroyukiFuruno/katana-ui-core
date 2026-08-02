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
        | StorybookButtonOperation::TextAreaFocus { .. } => StorybookCursorStyle::Ibeam,
        StorybookButtonOperation::PreviewComponent
            if !button_operation::uses_clickable_preview_cursor(state.selected_page) =>
        {
            StorybookCursorStyle::Arrow
        }
        _ => StorybookCursorStyle::PointingHand,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_assert::KucTestExpect;
    use crate::visual::{dedicated_dod_form_input_live, preview_detail};

    #[test]
    fn cursor_styles_cover_resize_text_passive_and_clickable_surfaces() {
        let resize_state = StorybookWindowState {
            selected_page: "text-area",
            preset_index: 3,
            ..StorybookWindowState::default()
        };
        let origin = preview_detail::component_action_hit_rect("text-area");
        let resize = dedicated_dod_form_input_live::text_area_resize_grip_rect_for_instance(
            origin.x,
            origin.y,
            resize_state.preset_index,
            &resize_state.screen_state,
            super::super::component_instance_id_for_page(
                resize_state.selected_page,
                resize_state.selected_instance_id,
            ),
        )
        .kuc_expect("resize preset must expose its grip");
        assert_eq!(
            StorybookCursorStyle::ResizeAll,
            cursor_style_at(&resize_state, resize.x, resize.y)
        );

        let text_state = StorybookWindowState {
            selected_page: "text-input",
            ..StorybookWindowState::default()
        };
        let text_origin = preview_detail::component_action_hit_rect("text-input");
        let field = dedicated_dod_form_input_live::search_field_rect(text_origin.x, text_origin.y);
        assert_eq!(
            StorybookCursorStyle::Ibeam,
            cursor_style_at(&text_state, field.x + 1, field.y + 1)
        );

        let passive_state = StorybookWindowState {
            selected_page: "badge",
            ..StorybookWindowState::default()
        };
        let passive = preview_detail::component_action_hit_rect("badge");
        assert_eq!(
            StorybookCursorStyle::Arrow,
            cursor_style_at(&passive_state, passive.x + 1, passive.y + 1)
        );
        assert_eq!(
            StorybookCursorStyle::Arrow,
            cursor_style_at(&passive_state, usize::MAX, usize::MAX)
        );

        let button_state = StorybookWindowState {
            selected_page: "button",
            ..StorybookWindowState::default()
        };
        let button = preview_detail::component_action_hit_rect("button");
        assert_eq!(
            StorybookCursorStyle::PointingHand,
            cursor_style_at(&button_state, button.x + 1, button.y + 1)
        );
    }
}
