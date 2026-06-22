use crate::interaction::{ColorDragAction, ProgressAction, UiActionSource};
use crate::layout::SplitPaneResizeSource;
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
    CopySelection {
        target: UiStateId,
    },
    PasteText {
        target: UiStateId,
        text: String,
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
    InvokeCallback {
        target: UiStateId,
        callback: String,
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
    SplitPaneSetRatio {
        target: UiStateId,
        ratio_percent: u8,
    },
    SplitPaneResizeBy {
        target: UiStateId,
        delta_percent: i8,
        source: SplitPaneResizeSource,
    },
    SplitPaneResetRatio {
        target: UiStateId,
    },
    SplitPaneStartResize {
        target: UiStateId,
    },
    SplitPaneEndResize {
        target: UiStateId,
    },
    TabSelect {
        target: UiStateId,
        tab_id: String,
    },
    TabAdd {
        target: UiStateId,
        tab_id: String,
        label: String,
        activate: bool,
    },
    TabClose {
        target: UiStateId,
        tab_id: String,
    },
    TabCloseOthers {
        target: UiStateId,
        tab_id: String,
    },
    TabCloseToRight {
        target: UiStateId,
        tab_id: String,
    },
    TabCloseToLeft {
        target: UiStateId,
        tab_id: String,
    },
    TabCloseAll {
        target: UiStateId,
    },
    TabRestoreClosed {
        target: UiStateId,
    },
    TabPin {
        target: UiStateId,
        tab_id: String,
        pinned: bool,
    },
    TabMove {
        target: UiStateId,
        tab_id: String,
        to_visual_index: usize,
    },
    TabMoveToGroup {
        target: UiStateId,
        tab_id: String,
        group_id: Option<String>,
    },
    TabMoveToNewGroup {
        target: UiStateId,
        tab_id: String,
        group_id: String,
        group_label: String,
    },
    TabMoveGroup {
        target: UiStateId,
        group_id: String,
        to_index: usize,
    },
    TabRenameGroup {
        target: UiStateId,
        group_id: String,
        label: String,
    },
    TabSetGroupColor {
        target: UiStateId,
        group_id: String,
        color: String,
    },
    TabUngroup {
        target: UiStateId,
        group_id: String,
    },
    TabCloseGroup {
        target: UiStateId,
        group_id: String,
    },
    TabToggleGroupCollapse {
        target: UiStateId,
        group_id: String,
    },
}
