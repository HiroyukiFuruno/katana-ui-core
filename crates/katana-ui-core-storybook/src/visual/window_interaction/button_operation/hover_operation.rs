use super::{
    CollapsiblePanelStoryAction, ColorPickerAction, CommandPaletteStoryAction,
    DiagnosticsListStoryAction, DragAndDropAction, DynamicArrayEditorAction, LayoutStoryAction,
    ScrollAreaStoryAction, SearchControlScreenAction, SegmentedToggleScreenAction,
    SelectionScreenAction, SettingsListStoryAction, SideMenuScreenAction, SplitPaneStoryAction,
    StorybookWindowState, ThemeTokensStoryAction, VirtualizationStoryAction, dedicated_breadcrumb,
    preview, preview_detail, status_bar_operation, text_area_operation, text_input_operation,
    toolbar_operation,
};

#[path = "hover_operation_primary.rs"]
mod primary;
#[path = "hover_operation_secondary.rs"]
mod secondary;
#[path = "hover_operation_state.rs"]
mod state_hover;

pub(in crate::visual) fn apply_hover_at(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    let signature = (state.selected_page, x, y);
    if state.last_hover_signature == Some(signature) {
        return true;
    }
    let handled = primary::apply(state, x, y)
        || secondary::apply(state, x, y)
        || state_hover::apply(state, x, y);
    if handled {
        state.last_hover_signature = Some((state.selected_page, x, y));
    } else {
        state.last_hover_signature = None;
    }
    handled
}
