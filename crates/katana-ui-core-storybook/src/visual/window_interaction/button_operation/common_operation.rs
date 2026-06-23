use super::{StorybookButtonOperation, StorybookWindowState};
use crate::visual::dedicated_dod_molecule_tree_parts as tree_parts;
use crate::visual::layout_metrics::{dark_theme_rect, light_theme_rect};
use crate::visual::panel_options;
use crate::visual::preset_tab_scroll;
use crate::visual::preview_detail;

const RESIZE_HIT_SIZE: usize = 24;

pub(super) fn theme_operation_at(x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if light_theme_rect().contains(x, y) {
        return Some(StorybookButtonOperation::LightTheme);
    }
    if dark_theme_rect().contains(x, y) {
        return Some(StorybookButtonOperation::DarkTheme);
    }
    None
}

pub(super) fn preset_operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    preset_tab_scroll::hit_index_at(state.selected_page, x, y, state.preset_tab_scroll_x)
        .map(StorybookButtonOperation::Preset)
}

pub(super) fn preview_operation_at(
    page: &str,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if preview_detail::button_action_hit_rect(page).contains(x, y) {
        return Some(StorybookButtonOperation::PreviewButton);
    }
    let component = preview_detail::component_action_hit_rect(page);
    if page == "tree-view" && component.contains(x, y) {
        return Some(StorybookButtonOperation::TreeViewPointer {
            pointer_x: x.saturating_sub(component.x),
            pointer_y: y.saturating_sub(component.y + tree_parts::TREE_PANEL_Y),
        });
    }
    if component.contains(x, y) {
        return Some(StorybookButtonOperation::PreviewComponent);
    }
    None
}

pub(super) fn panel_operation_at(
    page: &str,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if page != "panel" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(page);
    if resize_hit(origin, x, y) {
        return Some(StorybookButtonOperation::PanelResize);
    }
    if let Some(control) = panel_options::control_at(x, y) {
        return Some(StorybookButtonOperation::PanelOption(control));
    }
    crate::visual::dedicated_foundation_panel::panel_at(origin.x, origin.y, x, y)
        .map(StorybookButtonOperation::PanelChild)
}

fn resize_hit(component: crate::visual::layout_metrics::LayoutRect, x: usize, y: usize) -> bool {
    x + RESIZE_HIT_SIZE >= component.right() && y + RESIZE_HIT_SIZE >= component.bottom()
}
