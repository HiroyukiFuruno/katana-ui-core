use crate::interaction::{ColorDragAction, ProgressAction, RgbaActionValue, UiActionSource};
use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiAction {
    Press {
        target: UiStateId,
        source: UiActionSource,
    },
    SetOpen {
        target: UiStateId,
        open: bool,
    },
    SetSelectedIndex {
        target: UiStateId,
        selected_index: usize,
        selected: bool,
        source: UiActionSource,
    },
    SetValue {
        target: UiStateId,
        value: String,
        source: UiActionSource,
        progress: Option<ProgressAction>,
        color_drag: Option<ColorDragAction>,
    },
    ClearValue {
        target: UiStateId,
    },
    Dismiss {
        target: UiStateId,
    },
}

impl UiAction {
    #[must_use]
    pub fn press(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::Generic,
        }
    }

    #[must_use]
    pub fn button_press(target: UiStateId) -> Self {
        crate::interaction::ButtonAction::new(target).into()
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
    pub fn progress_changed(target: UiStateId, determinate: bool, percent: u8) -> Self {
        ProgressAction::new(target, determinate, percent).into()
    }

    #[must_use]
    pub fn color_drag(target: UiStateId, value: RgbaActionValue, hue: u16, preview: bool) -> Self {
        ColorDragAction::new(target, value, hue, preview).into()
    }

    #[must_use]
    pub fn clear_value(target: UiStateId) -> Self {
        Self::ClearValue { target }
    }

    #[must_use]
    pub fn dismiss(target: UiStateId) -> Self {
        Self::Dismiss { target }
    }

    #[must_use]
    pub fn target(&self) -> &UiStateId {
        match self {
            Self::Press { target, .. }
            | Self::SetOpen { target, .. }
            | Self::SetSelectedIndex { target, .. }
            | Self::SetValue { target, .. }
            | Self::ClearValue { target }
            | Self::Dismiss { target } => target,
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Press { source, .. } => source.press_name(),
            Self::SetSelectedIndex { source, .. } => source.selection_name(),
            Self::SetValue {
                source,
                progress,
                color_drag,
                ..
            } => value_name(*source, progress, color_drag),
            Self::SetOpen { .. } => "set_open",
            Self::ClearValue { .. } => "clear_value",
            Self::Dismiss { .. } => "dismiss",
        }
    }
}

fn value_name(
    source: UiActionSource,
    progress: &Option<ProgressAction>,
    color_drag: &Option<ColorDragAction>,
) -> &'static str {
    if progress.is_some() {
        return "progress_changed";
    }
    if color_drag.is_some() {
        return "color_drag";
    }
    match source {
        UiActionSource::Input => "input_value",
        _ => "set_value",
    }
}
