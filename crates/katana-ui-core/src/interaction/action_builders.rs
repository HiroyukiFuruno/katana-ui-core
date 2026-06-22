use crate::interaction::{
    ColorDragAction, ProgressAction, RgbaActionValue, UiAction, UiActionSource,
};
use crate::render_model::UiStateId;

impl UiAction {
    #[must_use]
    pub fn press(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::Generic,
        }
    }

    #[must_use]
    pub fn click(target: UiStateId) -> Self {
        crate::interaction::ClickAction::new(target).into()
    }

    #[must_use]
    pub fn button_press(target: UiStateId) -> Self {
        crate::interaction::ButtonAction::new(target).into()
    }

    #[must_use]
    pub fn search_submitted(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::SearchBox,
        }
    }

    #[must_use]
    pub fn input_submitted(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::InputSubmit,
        }
    }

    #[must_use]
    pub fn focus(target: UiStateId) -> Self {
        Self::SetFocus {
            target,
            focused: true,
        }
    }

    #[must_use]
    pub fn blur(target: UiStateId) -> Self {
        Self::SetFocus {
            target,
            focused: false,
        }
    }

    #[must_use]
    pub fn hover(target: UiStateId, hovered: bool) -> Self {
        Self::SetHover { target, hovered }
    }

    #[must_use]
    pub fn active(target: UiStateId, active: bool) -> Self {
        Self::SetActive { target, active }
    }

    #[must_use]
    pub fn dragging(target: UiStateId, dragging: bool) -> Self {
        Self::SetDragging { target, dragging }
    }

    #[must_use]
    pub fn animation_tick(target: UiStateId, phase: u16) -> Self {
        Self::AnimationTick { target, phase }
    }

    #[must_use]
    pub fn reduced_motion(target: UiStateId, reduced_motion: bool) -> Self {
        Self::SetReducedMotion {
            target,
            reduced_motion,
        }
    }

    #[must_use]
    pub fn cursor_selection(
        target: UiStateId,
        cursor: usize,
        selection_start: usize,
        selection_end: usize,
    ) -> Self {
        Self::SetCursorSelection {
            target,
            cursor,
            selection_start,
            selection_end,
        }
    }

    #[must_use]
    pub fn copy_selection(target: UiStateId) -> Self {
        Self::CopySelection { target }
    }

    #[must_use]
    pub fn paste_text(target: UiStateId, text: impl Into<String>) -> Self {
        Self::PasteText {
            target,
            text: text.into(),
            source: UiActionSource::Input,
        }
    }

    #[must_use]
    pub fn set_open(target: UiStateId, open: bool) -> Self {
        Self::SetOpen { target, open }
    }

    #[must_use]
    pub fn set_selected_index(target: UiStateId, selected_index: usize) -> Self {
        Self::SetSelectedIndex {
            target,
            selected_index,
            selected: true,
            source: UiActionSource::Generic,
        }
    }

    #[must_use]
    pub fn segmented_toggle_selected(target: UiStateId, selected_index: usize) -> Self {
        Self::SetSelectedIndex {
            target,
            selected_index,
            selected: true,
            source: UiActionSource::SegmentedToggle,
        }
    }

    #[must_use]
    pub fn set_value(target: UiStateId, value: impl Into<String>) -> Self {
        Self::SetValue {
            target,
            value: value.into(),
            source: UiActionSource::Generic,
            progress: None,
            color_drag: None,
        }
    }

    #[must_use]
    pub fn input_value(target: UiStateId, value: impl Into<String>) -> Self {
        crate::interaction::InputAction::new(target, value).into()
    }

    #[must_use]
    pub fn slide_changed(target: UiStateId, value: impl Into<String>) -> Self {
        crate::interaction::SlideAction::new(target, value).into()
    }

    #[must_use]
    pub fn checkbox_checked(target: UiStateId, checked: bool) -> Self {
        crate::interaction::CheckboxAction::new(target, checked).into()
    }

    #[must_use]
    pub fn radio_selected(target: UiStateId) -> Self {
        crate::interaction::RadioAction::new(target).into()
    }

    #[must_use]
    pub fn toggle_checked(target: UiStateId, checked: bool) -> Self {
        crate::interaction::ToggleAction::new(target, checked).into()
    }

    #[must_use]
    pub fn progress_changed(target: UiStateId, determinate: bool, percent: u8) -> Self {
        ProgressAction::new(target, determinate, percent).into()
    }

    #[must_use]
    pub fn color_drag(target: UiStateId, value: RgbaActionValue, hue: u16, preview: bool) -> Self {
        ColorDragAction::new(target, value, hue, preview).into()
    }

    #[must_use]
    pub fn color_blending_changed(target: UiStateId, value: impl Into<String>) -> Self {
        Self::SetValue {
            target,
            value: value.into(),
            source: UiActionSource::ColorPickerBlending,
            progress: None,
            color_drag: None,
        }
    }

    #[must_use]
    pub fn clear_value(target: UiStateId) -> Self {
        Self::ClearValue { target }
    }

    #[must_use]
    pub fn invoke_callback(target: UiStateId, callback: impl Into<String>) -> Self {
        Self::InvokeCallback {
            target,
            callback: callback.into(),
        }
    }

    #[must_use]
    pub fn open_uri(target: UiStateId, uri: impl Into<String>) -> Self {
        Self::invoke_callback(target, format!("open-uri:{}", uri.into()))
    }

    #[must_use]
    pub fn dismiss(target: UiStateId) -> Self {
        Self::Dismiss { target }
    }
}
