use crate::interaction::{UiAction, UiActionSource};
use crate::render_model::UiStateId;

impl UiAction {
    #[must_use]
    pub fn modal_escape(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::ModalEscape,
        }
    }

    #[must_use]
    pub fn modal_backdrop_click(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::ModalBackdrop,
        }
    }

    #[must_use]
    pub fn accordion_toggle(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::Accordion,
        }
    }

    #[must_use]
    pub fn tooltip_toggle(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::Tooltip,
        }
    }

    #[must_use]
    pub fn popover_toggle(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::Popover,
        }
    }

    #[must_use]
    pub fn split_pane_resized(target: UiStateId, percent: u8) -> Self {
        crate::interaction::SplitPaneAction::new(target, percent).into()
    }

    #[must_use]
    pub fn split_pane_reset(target: UiStateId) -> Self {
        Self::SetValue {
            target,
            value: String::new(),
            source: UiActionSource::SplitPaneReset,
            progress: None,
            color_drag: None,
        }
    }

    #[must_use]
    pub fn split_pane_keyboard_resize(target: UiStateId, percent: u8) -> Self {
        Self::SetValue {
            target,
            value: percent.to_string(),
            source: UiActionSource::SplitPaneKeyboard,
            progress: None,
            color_drag: None,
        }
    }

    #[must_use]
    pub fn select_box_selected(target: UiStateId, selected_index: usize) -> Self {
        Self::SetSelectedIndex {
            target,
            selected_index,
            selected: true,
            source: UiActionSource::SelectBox,
        }
    }

    #[must_use]
    pub fn code_diff_mode(target: UiStateId, value: impl Into<String>) -> Self {
        Self::SetValue {
            target,
            value: value.into(),
            source: UiActionSource::CodeDiffMode,
            progress: None,
            color_drag: None,
        }
    }

    #[must_use]
    pub fn code_diff_direction(target: UiStateId, value: impl Into<String>) -> Self {
        Self::SetValue {
            target,
            value: value.into(),
            source: UiActionSource::CodeDiffDirection,
            progress: None,
            color_drag: None,
        }
    }

    #[must_use]
    pub fn code_diff_expand(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::CodeDiffExpand,
        }
    }

    #[must_use]
    pub fn code_diff_scroll_sync(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::CodeDiffScrollSync,
        }
    }
}
