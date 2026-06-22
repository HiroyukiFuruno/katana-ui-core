use crate::interaction::{UiAction, UiActionSource};
use crate::layout::SplitPaneResizeSource;
use crate::render_model::{UiRect, UiScrollbarVisibility, UiStateId};

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
    pub fn accordion_icon_toggle(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::AccordionIcon,
        }
    }

    #[must_use]
    pub fn accordion_text_toggle(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::AccordionText,
        }
    }

    #[must_use]
    pub fn accordion_row_toggle(target: UiStateId) -> Self {
        Self::Press {
            target,
            source: UiActionSource::AccordionRow,
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
    pub fn split_pane_set_ratio(target: UiStateId, ratio_percent: u8) -> Self {
        Self::SplitPaneSetRatio {
            target,
            ratio_percent,
        }
    }

    #[must_use]
    pub fn split_pane_resize_by(
        target: UiStateId,
        delta_percent: i8,
        source: SplitPaneResizeSource,
    ) -> Self {
        Self::SplitPaneResizeBy {
            target,
            delta_percent,
            source,
        }
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
    pub fn split_pane_reset_ratio(target: UiStateId) -> Self {
        Self::SplitPaneResetRatio { target }
    }

    #[must_use]
    pub fn split_pane_start_resize(target: UiStateId) -> Self {
        Self::SplitPaneStartResize { target }
    }

    #[must_use]
    pub fn split_pane_end_resize(target: UiStateId) -> Self {
        Self::SplitPaneEndResize { target }
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
    pub fn code_diff_language(target: UiStateId, value: impl Into<String>) -> Self {
        Self::SetValue {
            target,
            value: value.into(),
            source: UiActionSource::CodeDiffLanguage,
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

    #[must_use]
    pub fn scroll_to(target: UiStateId, x: u32, y: u32) -> Self {
        Self::ScrollTo { target, x, y }
    }

    #[must_use]
    pub fn scroll_by(target: UiStateId, dx: i32, dy: i32) -> Self {
        Self::ScrollBy { target, dx, dy }
    }

    #[must_use]
    pub fn scroll_into_view(target: UiStateId, target_rect: UiRect) -> Self {
        Self::ScrollIntoView {
            target,
            target_rect,
        }
    }

    #[must_use]
    pub fn scrollbar_visibility(target: UiStateId, visibility: UiScrollbarVisibility) -> Self {
        Self::SetScrollbarVisibility { target, visibility }
    }
}
