use super::{
    CollapsiblePanelStoryAction, CommandPaletteStoryAction, DiagnosticsListStoryAction,
    DragAndDropAction, LayoutStoryAction, ScrollAreaStoryAction, StorybookWindowState,
    VirtualizationStoryAction, preview_detail,
};

pub(super) fn focus_command_palette(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("command-palette").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_command_palette_action(CommandPaletteStoryAction::Focus);
    true
}

pub(super) fn focus_collapsible_panel(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if !preview_detail::component_action_hit_rect("collapsible-panel").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_collapsible_panel_action(CollapsiblePanelStoryAction::Focus);
    true
}

pub(super) fn focus_virtualization(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("virtualization").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_virtualization_action(VirtualizationStoryAction::Focus);
    true
}

pub(super) fn focus_diagnostics_list(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("diagnostics-list").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_diagnostics_list_action(DiagnosticsListStoryAction::FocusList);
    true
}

pub(super) fn focus_empty_state(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("empty-state").contains(x, y) {
        return false;
    }
    state.screen_state.register_empty_state_focus();
    true
}

pub(super) fn focus_tree_view(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("tree-view").contains(x, y) {
        return false;
    }
    state.screen_state.register_tree_view_focus();
    true
}

pub(super) fn focus_drag_and_drop(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("drag-and-drop").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_drag_and_drop_action(DragAndDropAction::FocusSource);
    true
}

pub(super) fn focus_panel(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("panel").contains(x, y) {
        return false;
    }
    state.screen_state.register_panel_focus();
    true
}

pub(super) fn focus_row(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("row").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(LayoutStoryAction::RowFocus);
    true
}

pub(super) fn focus_column(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("column").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(LayoutStoryAction::ColumnFocus);
    true
}

pub(super) fn focus_stack(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("stack").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(LayoutStoryAction::StackFocus);
    true
}

pub(super) fn focus_grid(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("grid").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(LayoutStoryAction::GridFocus);
    true
}

pub(super) fn focus_align_center(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("align-center").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(LayoutStoryAction::AlignCenterFocus);
    true
}

pub(super) fn focus_scroll_area(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("scroll-area").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_scroll_area_action(ScrollAreaStoryAction::Focus);
    true
}

#[cfg(test)]
mod tests {
    use super::super::focus_dispatch;
    use super::*;

    #[test]
    fn primary_focus_surfaces_reject_points_outside_the_component() {
        for page in [
            "command-palette",
            "collapsible-panel",
            "virtualization",
            "diagnostics-list",
            "empty-state",
            "tree-view",
            "drag-and-drop",
            "panel",
            "row",
            "column",
            "stack",
            "grid",
            "align-center",
            "scroll-area",
        ] {
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };
            assert!(!focus_dispatch::focus_at(&mut state, 0, 0), "{page}");
        }
    }
}
