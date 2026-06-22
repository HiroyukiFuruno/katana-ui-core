use super::action::UiAction;
use super::action_name::value_name;
use crate::render_model::UiStateId;

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
            | Self::CopySelection { target }
            | Self::PasteText { target, .. }
            | Self::SetOpen { target, .. }
            | Self::SetSelectedIndex { target, .. }
            | Self::SetValue { target, .. }
            | Self::ClearValue { target }
            | Self::InvokeCallback { target, .. }
            | Self::Dismiss { target }
            | Self::ScrollTo { target, .. }
            | Self::ScrollBy { target, .. }
            | Self::ScrollIntoView { target, .. }
            | Self::SetScrollbarVisibility { target, .. }
            | Self::SplitPaneSetRatio { target, .. }
            | Self::SplitPaneResizeBy { target, .. }
            | Self::SplitPaneResetRatio { target }
            | Self::SplitPaneStartResize { target }
            | Self::SplitPaneEndResize { target }
            | Self::TabSelect { target, .. }
            | Self::TabAdd { target, .. }
            | Self::TabClose { target, .. }
            | Self::TabCloseOthers { target, .. }
            | Self::TabCloseToRight { target, .. }
            | Self::TabCloseToLeft { target, .. }
            | Self::TabCloseAll { target }
            | Self::TabRestoreClosed { target }
            | Self::TabPin { target, .. }
            | Self::TabMove { target, .. }
            | Self::TabMoveToGroup { target, .. }
            | Self::TabMoveToNewGroup { target, .. }
            | Self::TabMoveGroup { target, .. }
            | Self::TabRenameGroup { target, .. }
            | Self::TabSetGroupColor { target, .. }
            | Self::TabUngroup { target, .. }
            | Self::TabCloseGroup { target, .. }
            | Self::TabToggleGroupCollapse { target, .. } => target,
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Press { source, .. } => source.press_name(),
            Self::SetFocus { focused, .. } => focus_name(*focused),
            Self::SetHover { hovered, .. } => hover_name(*hovered),
            Self::SetActive { active, .. } => active_name(*active),
            Self::SetDragging { dragging, .. } => dragging_name(*dragging),
            Self::AnimationTick { .. } => "animation_tick",
            Self::SetReducedMotion { .. } => "reduced_motion_toggle",
            Self::SetCursorSelection { .. } => "cursor_selection_changed",
            Self::CopySelection { .. } => "copy_selection",
            Self::PasteText { .. } => "paste_text",
            Self::SetSelectedIndex { source, .. } => source.selection_name(),
            Self::SetValue {
                source,
                progress,
                color_drag,
                ..
            } => value_name(*source, progress, color_drag),
            Self::SetOpen { open, .. } => open_name(*open),
            Self::ClearValue { .. } => "clear_value",
            Self::InvokeCallback { .. } => "callback_invoked",
            Self::Dismiss { .. } => "dismiss",
            Self::ScrollTo { .. } => "scroll_to",
            Self::ScrollBy { .. } => "scroll_by",
            Self::ScrollIntoView { .. } => "scroll_into_view",
            Self::SetScrollbarVisibility { .. } => "scrollbar_visibility_changed",
            Self::SplitPaneSetRatio { .. } => "split_pane_set_ratio",
            Self::SplitPaneResizeBy { .. } => "split_pane_resize_by",
            Self::SplitPaneResetRatio { .. } => "split_pane_reset_ratio",
            Self::SplitPaneStartResize { .. } => "split_pane_start_resize",
            Self::SplitPaneEndResize { .. } => "split_pane_end_resize",
            Self::TabSelect { .. } => "tab_select",
            Self::TabAdd { .. } => "tab_add",
            Self::TabClose { .. } => "tab_close",
            Self::TabCloseOthers { .. } => "tab_close_others",
            Self::TabCloseToRight { .. } => "tab_close_to_right",
            Self::TabCloseToLeft { .. } => "tab_close_to_left",
            Self::TabCloseAll { .. } => "tab_close_all",
            Self::TabRestoreClosed { .. } => "tab_restore_closed",
            Self::TabPin { pinned, .. } => {
                if *pinned {
                    "tab_pin"
                } else {
                    "tab_unpin"
                }
            }
            Self::TabMove { .. } => "tab_move",
            Self::TabMoveToGroup { group_id, .. } => {
                if group_id.is_some() {
                    "tab_move_to_group"
                } else {
                    "tab_move_to_ungrouped"
                }
            }
            Self::TabMoveToNewGroup { .. } => "tab_move_to_new_group",
            Self::TabMoveGroup { .. } => "tab_move_group",
            Self::TabRenameGroup { .. } => "tab_rename_group",
            Self::TabSetGroupColor { .. } => "tab_set_group_color",
            Self::TabUngroup { .. } => "tab_ungroup",
            Self::TabCloseGroup { .. } => "tab_close_group",
            Self::TabToggleGroupCollapse { .. } => "tab_toggle_group_collapse",
        }
    }

    #[must_use]
    pub fn callback_log_action(&self) -> String {
        match self {
            Self::InvokeCallback { callback, .. } => callback.clone(),
            _ => self.name().to_string(),
        }
    }
}

fn focus_name(focused: bool) -> &'static str {
    if focused { "focus" } else { "blur" }
}

fn hover_name(hovered: bool) -> &'static str {
    if hovered { "hover_start" } else { "hover_end" }
}

fn active_name(active: bool) -> &'static str {
    if active { "active_start" } else { "active_end" }
}

fn dragging_name(dragging: bool) -> &'static str {
    if dragging { "drag_start" } else { "drag_end" }
}

fn open_name(open: bool) -> &'static str {
    if open { "open" } else { "close" }
}
