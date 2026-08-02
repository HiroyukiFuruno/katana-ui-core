use super::StorybookWindowState;
use crate::visual::preview_detail;
use katana_ui_core::render_model::{UiAlignItems, UiNode};
use katana_ui_core::{atom, layout};

const ROW_PAGE: &str = "row";
const COLUMN_PAGE: &str = "column";
const STACK_PAGE: &str = "stack";
const GRID_PAGE: &str = "grid";
const ALIGN_CENTER_PAGE: &str = "align-center";
const DEFAULT_GAP_PX: f32 = 8.0;
const LARGE_GAP_PX: f32 = 18.0;
const SELECTED_CELL_INDEX: usize = 1;
const STACK_TOP_INDEX: usize = 2;
const RESIZE_HIT_SIZE: usize = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum LayoutStoryAction {
    RowAlign,
    RowHover,
    RowFocus,
    RowKeyboard,
    RowResize,
    ColumnAlign,
    ColumnHover,
    ColumnFocus,
    ColumnKeyboard,
    ColumnResize,
    StackReorder,
    StackHover,
    StackFocus,
    StackKeyboard,
    StackResize,
    GridSelect,
    GridHover,
    GridFocus,
    GridKeyboard,
    GridResize,
    AlignCenterHover,
    AlignCenterFocus,
    AlignCenterKeyboard,
    AlignCenterResize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::visual) struct LayoutStoryState {
    page: &'static str,
    selected_index: usize,
    alignment: &'static str,
    callback: &'static str,
    hovered: bool,
    focused: bool,
    resized: bool,
}

#[path = "layout_operation_state_accessors.rs"]
mod state_accessors;
#[path = "layout_operation_state.rs"]
mod state_impl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct LayoutStoryUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl LayoutStoryUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<LayoutStoryAction> {
    if !is_live_layout_page(state.selected_page) {
        return None;
    }
    let component = preview_detail::component_action_hit_rect(state.selected_page);
    if !component.contains(x, y) {
        return None;
    }
    if resize_hit(component, x, y) {
        let action = match state.selected_page {
            ROW_PAGE => LayoutStoryAction::RowResize,
            COLUMN_PAGE => LayoutStoryAction::ColumnResize,
            STACK_PAGE => LayoutStoryAction::StackResize,
            GRID_PAGE => LayoutStoryAction::GridResize,
            page => {
                debug_assert_eq!(page, ALIGN_CENTER_PAGE);
                LayoutStoryAction::AlignCenterResize
            }
        };
        return Some(action);
    }
    match state.selected_page {
        ROW_PAGE => Some(LayoutStoryAction::RowAlign),
        COLUMN_PAGE => Some(LayoutStoryAction::ColumnAlign),
        STACK_PAGE => Some(LayoutStoryAction::StackReorder),
        GRID_PAGE => Some(LayoutStoryAction::GridSelect),
        _ => None,
    }
}

fn resize_hit(component: crate::visual::layout_metrics::LayoutRect, x: usize, y: usize) -> bool {
    x + RESIZE_HIT_SIZE >= component.right() && y + RESIZE_HIT_SIZE >= component.bottom()
}

pub(in crate::visual) fn is_live_layout_page(page: &str) -> bool {
    matches!(
        page,
        ROW_PAGE | COLUMN_PAGE | STACK_PAGE | GRID_PAGE | ALIGN_CENTER_PAGE
    )
}

fn layout_alignment_for(page: &str) -> &'static str {
    let node: UiNode = match page {
        ROW_PAGE => layout::Row::new()
            .gap(layout::Length::px(LARGE_GAP_PX))
            .align(layout::Alignment::Center)
            .child(atom::Text::new("A"))
            .into(),
        COLUMN_PAGE => layout::Column::new()
            .gap(layout::Length::px(LARGE_GAP_PX))
            .align(layout::Alignment::Center)
            .child(atom::Text::new("A"))
            .into(),
        _ => layout::Row::new()
            .gap(layout::Length::px(DEFAULT_GAP_PX))
            .child(atom::Text::new("A"))
            .into(),
    };
    if node.props().common.align_items == UiAlignItems::Center {
        return "alignment=center";
    }
    "alignment=start"
}

fn layout_page(page: &str) -> &'static str {
    match page {
        ROW_PAGE => ROW_PAGE,
        COLUMN_PAGE => COLUMN_PAGE,
        STACK_PAGE => STACK_PAGE,
        GRID_PAGE => GRID_PAGE,
        ALIGN_CENTER_PAGE => ALIGN_CENTER_PAGE,
        _ => "none",
    }
}

fn align_center_contract() -> &'static str {
    let node: UiNode = layout::AlignCenter::new()
        .align(layout::Alignment::Center)
        .child(atom::Text::new("A"))
        .into();
    assert_eq!(
        UiAlignItems::Center,
        node.props().common.align_items,
        "AlignCenter must project center alignment"
    );
    "alignment=center"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_helpers_cover_align_center_body_and_unknown_fallbacks() {
        let state = StorybookWindowState {
            selected_page: ALIGN_CENTER_PAGE,
            ..StorybookWindowState::default()
        };
        let component = preview_detail::component_action_hit_rect(ALIGN_CENTER_PAGE);
        assert_eq!(
            Some(LayoutStoryAction::AlignCenterResize),
            operation_at(&state, component.right() - 1, component.bottom() - 1)
        );

        assert_eq!("alignment=start", layout_alignment_for("unknown"));
        assert_eq!("none", layout_page("unknown"));
        assert_eq!("alignment=center", align_center_contract());
    }
}
