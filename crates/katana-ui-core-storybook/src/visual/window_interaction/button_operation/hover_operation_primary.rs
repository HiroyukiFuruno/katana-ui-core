use super::{
    CollapsiblePanelStoryAction, DiagnosticsListStoryAction, DragAndDropAction, LayoutStoryAction,
    ScrollAreaStoryAction, SplitPaneStoryAction, StorybookWindowState, ThemeTokensStoryAction,
    VirtualizationStoryAction, dedicated_breadcrumb, preview_detail,
};

pub(super) fn apply(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page == "breadcrumb" {
        let component = preview_detail::component_action_hit_rect("breadcrumb");
        if dedicated_breadcrumb::file_crumb_rect(component.x, component.y).contains(x, y) {
            state.screen_state.register_breadcrumb_hover(2);
            return true;
        }
    }
    if state.selected_page == "accordion"
        && preview_detail::component_action_hit_rect("accordion").contains(x, y)
    {
        state.screen_state.register_accordion_hover();
        return true;
    }
    if state.selected_page == "code-diff"
        && preview_detail::component_action_hit_rect("code-diff").contains(x, y)
    {
        state.screen_state.register_code_diff_hover();
        return true;
    }
    if state.selected_page == "collapsible-panel"
        && preview_detail::component_action_hit_rect("collapsible-panel").contains(x, y)
    {
        state
            .screen_state
            .register_collapsible_panel_action(CollapsiblePanelStoryAction::Hover);
        return true;
    }
    if state.selected_page == "virtualization"
        && preview_detail::component_action_hit_rect("virtualization").contains(x, y)
    {
        state
            .screen_state
            .register_virtualization_action(VirtualizationStoryAction::Hover);
        return true;
    }
    if state.selected_page == "diagnostics-list"
        && preview_detail::component_action_hit_rect("diagnostics-list").contains(x, y)
    {
        state
            .screen_state
            .register_diagnostics_list_action(DiagnosticsListStoryAction::HoverItem);
        return true;
    }
    if state.selected_page == "empty-state"
        && preview_detail::component_action_hit_rect("empty-state").contains(x, y)
    {
        state.screen_state.register_empty_state_hover();
        return true;
    }
    if state.selected_page == "tree-view"
        && preview_detail::component_action_hit_rect("tree-view").contains(x, y)
    {
        state.screen_state.register_tree_view_hover();
        return true;
    }
    if state.selected_page == "drag-and-drop"
        && preview_detail::component_action_hit_rect("drag-and-drop").contains(x, y)
    {
        state
            .screen_state
            .register_drag_and_drop_action(DragAndDropAction::HoverTarget);
        return true;
    }
    if state.selected_page == "panel"
        && preview_detail::component_action_hit_rect("panel").contains(x, y)
    {
        state.screen_state.register_panel_hover();
        return true;
    }
    if state.selected_page == "row"
        && preview_detail::component_action_hit_rect("row").contains(x, y)
    {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::RowHover);
        return true;
    }
    if state.selected_page == "column"
        && preview_detail::component_action_hit_rect("column").contains(x, y)
    {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::ColumnHover);
        return true;
    }
    if state.selected_page == "stack"
        && preview_detail::component_action_hit_rect("stack").contains(x, y)
    {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::StackHover);
        return true;
    }
    if state.selected_page == "grid"
        && preview_detail::component_action_hit_rect("grid").contains(x, y)
    {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::GridHover);
        return true;
    }
    if state.selected_page == "align-center"
        && preview_detail::component_action_hit_rect("align-center").contains(x, y)
    {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::AlignCenterHover);
        return true;
    }
    if state.selected_page == "scroll-area"
        && preview_detail::component_action_hit_rect("scroll-area").contains(x, y)
    {
        state
            .screen_state
            .register_scroll_area_action(ScrollAreaStoryAction::Hover);
        return true;
    }
    if state.selected_page == "split-pane"
        && preview_detail::component_action_hit_rect("split-pane").contains(x, y)
    {
        state
            .screen_state
            .register_split_pane_action(SplitPaneStoryAction::Hover);
        return true;
    }
    if state.selected_page == "theme-tokens"
        && preview_detail::component_action_hit_rect("theme-tokens").contains(x, y)
    {
        state
            .screen_state
            .register_theme_tokens_action(ThemeTokensStoryAction::Hover);
        return true;
    }
    false
}
