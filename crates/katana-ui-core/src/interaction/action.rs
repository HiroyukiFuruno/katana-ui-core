use super::action_name::value_name;
use crate::interaction::{ColorDragAction, ProgressAction, UiActionSource};
use crate::render_model::{UiRect, UiScrollbarVisibility, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiAction {
    Press {
        target: UiStateId,
        source: UiActionSource,
    },
    SetFocus {
        target: UiStateId,
        focused: bool,
    },
    SetHover {
        target: UiStateId,
        hovered: bool,
    },
    SetActive {
        target: UiStateId,
        active: bool,
    },
    SetDragging {
        target: UiStateId,
        dragging: bool,
    },
    AnimationTick {
        target: UiStateId,
        phase: u16,
    },
    SetReducedMotion {
        target: UiStateId,
        reduced_motion: bool,
    },
    SetCursorSelection {
        target: UiStateId,
        cursor: usize,
        selection_start: usize,
        selection_end: usize,
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
    ScrollTo {
        target: UiStateId,
        x: u32,
        y: u32,
    },
    ScrollBy {
        target: UiStateId,
        dx: i32,
        dy: i32,
    },
    ScrollIntoView {
        target: UiStateId,
        target_rect: UiRect,
    },
    SetScrollbarVisibility {
        target: UiStateId,
        visibility: UiScrollbarVisibility,
    },
}

impl UiAction {
    #[must_use]
    pub fn target(&self) -> &UiStateId {
        match self {
            Self::Press { target, .. }
            | Self::SetFocus { target, .. }
            | Self::SetHover { target, .. }
            | Self::SetActive { target, .. }
            | Self::SetDragging { target, .. }
            | Self::AnimationTick { target, .. }
            | Self::SetReducedMotion { target, .. }
            | Self::SetCursorSelection { target, .. }
            | Self::SetOpen { target, .. }
            | Self::SetSelectedIndex { target, .. }
            | Self::SetValue { target, .. }
            | Self::ClearValue { target }
            | Self::Dismiss { target }
            | Self::ScrollTo { target, .. }
            | Self::ScrollBy { target, .. }
            | Self::ScrollIntoView { target, .. }
            | Self::SetScrollbarVisibility { target, .. } => target,
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Press { source, .. } => source.press_name(),
            Self::SetFocus { focused, .. } => {
                if *focused {
                    "focus"
                } else {
                    "blur"
                }
            }
            Self::SetHover { hovered, .. } => {
                if *hovered {
                    "hover_start"
                } else {
                    "hover_end"
                }
            }
            Self::SetActive { active, .. } => {
                if *active {
                    "active_start"
                } else {
                    "active_end"
                }
            }
            Self::SetDragging { dragging, .. } => {
                if *dragging {
                    "drag_start"
                } else {
                    "drag_end"
                }
            }
            Self::AnimationTick { .. } => "animation_tick",
            Self::SetReducedMotion { .. } => "reduced_motion_toggle",
            Self::SetCursorSelection { .. } => "cursor_selection_changed",
            Self::SetSelectedIndex { source, .. } => source.selection_name(),
            Self::SetValue {
                source,
                progress,
                color_drag,
                ..
            } => value_name(*source, progress, color_drag),
            Self::SetOpen { open, .. } => {
                if *open {
                    "open"
                } else {
                    "close"
                }
            }
            Self::ClearValue { .. } => "clear_value",
            Self::Dismiss { .. } => "dismiss",
            Self::ScrollTo { .. } => "scroll_to",
            Self::ScrollBy { .. } => "scroll_by",
            Self::ScrollIntoView { .. } => "scroll_into_view",
            Self::SetScrollbarVisibility { .. } => "scrollbar_visibility_changed",
        }
    }
}
