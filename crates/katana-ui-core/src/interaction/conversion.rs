use crate::interaction::{
    ButtonAction, CheckboxAction, ClickAction, ColorDragAction, InputAction, ProgressAction,
    RadioAction, ToggleAction, UiAction, UiActionSource,
};

impl From<ButtonAction> for UiAction {
    fn from(value: ButtonAction) -> Self {
        Self::Press {
            target: value.target,
            source: UiActionSource::Button,
        }
    }
}

impl From<ClickAction> for UiAction {
    fn from(value: ClickAction) -> Self {
        Self::Press {
            target: value.target,
            source: UiActionSource::Click,
        }
    }
}

impl From<InputAction> for UiAction {
    fn from(value: InputAction) -> Self {
        Self::SetValue {
            target: value.target,
            value: value.value,
            source: UiActionSource::Input,
            progress: None,
            color_drag: None,
        }
    }
}

impl From<CheckboxAction> for UiAction {
    fn from(value: CheckboxAction) -> Self {
        Self::SetSelectedIndex {
            target: value.target,
            selected_index: usize::from(value.checked),
            selected: value.checked,
            source: UiActionSource::Checkbox,
        }
    }
}

impl From<RadioAction> for UiAction {
    fn from(value: RadioAction) -> Self {
        Self::SetSelectedIndex {
            target: value.target,
            selected_index: 1,
            selected: true,
            source: UiActionSource::Radio,
        }
    }
}

impl From<ToggleAction> for UiAction {
    fn from(value: ToggleAction) -> Self {
        Self::SetSelectedIndex {
            target: value.target,
            selected_index: usize::from(value.checked),
            selected: value.checked,
            source: UiActionSource::Toggle,
        }
    }
}

impl From<ProgressAction> for UiAction {
    fn from(value: ProgressAction) -> Self {
        Self::SetValue {
            target: value.target.clone(),
            value: value.percent.to_string(),
            source: UiActionSource::Progress,
            progress: Some(value),
            color_drag: None,
        }
    }
}

impl From<ColorDragAction> for UiAction {
    fn from(value: ColorDragAction) -> Self {
        Self::SetValue {
            target: value.target.clone(),
            value: value.value.css_rgba(),
            source: UiActionSource::ColorPicker,
            progress: None,
            color_drag: Some(value),
        }
    }
}
