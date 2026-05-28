use minifb::{MouseButton, MouseMode, Window};

mod button_operation;
mod content_position;
mod cursor_operation;
mod panel_scroll_drag;
mod preset_selection;
mod scroll_operation;
mod state_store;
mod text_area_keyboard;
mod text_area_resize;
mod text_input_keyboard;

use super::navigation_tree::{NavigationRow, TreeExpansionState, row_from_click};
use super::panel_scroll_state::PanelScrollOffsets;
use super::preview_detail;
use super::render::{HEIGHT, WIDTH};
use super::screen_state::StorybookScreenState;
use super::window_coordinates::{
    CanvasPoint, SurfaceSize, WindowPoint, window_point_to_canvas_point,
};
use crate::DEFAULT_STORYBOOK_PAGE;
pub(super) use button_operation::apply_hover_at;
use button_operation::button_operation_at;
pub(super) use content_position::click_content_y;
pub(super) use cursor_operation::{StorybookCursorStyle, cursor_style_at};
use panel_scroll_drag::PanelScrollDragTarget;
use scroll_operation::{
    apply_horizontal_scrollbar_drag, apply_scroll_delta, apply_scroll_delta_at,
    apply_scroll_delta_x_at, apply_scrollbar_drag, apply_scrollbar_drag_target,
};
use state_store::StorybookScreenStateStore;
use std::collections::BTreeMap;
pub(super) use text_area_keyboard::{TextAreaKey, apply_text_area_key};
pub(super) use text_input_keyboard::{TextInputKey, apply_text_input_key};

const DEFAULT_THEME_ID: &str = "dark";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StorybookWindowState {
    pub(super) selected_page: &'static str,
    pub(super) theme_id: &'static str,
    pub(super) preset_index: usize,
    pub(super) preset_tab_scroll_x: usize,
    pub(super) selected_component_presets: BTreeMap<&'static str, usize>,
    pub(super) scroll_y: usize,
    pub(super) panel_scroll: PanelScrollOffsets,
    pub(super) scrollbar_visible: bool,
    pub(super) tree_expansion: TreeExpansionState,
    pub(super) show_navigation_lines: bool,
    pub(super) show_navigation_text_connectors: bool,
    pub(super) screen_state: StorybookScreenState,
    pub(super) screen_states: StorybookScreenStateStore,
    pub(super) drag_scroll_target: Option<PanelScrollDragTarget>,
    pub(super) text_area_resize_dragging: bool,
}

impl Default for StorybookWindowState {
    fn default() -> Self {
        Self {
            selected_page: DEFAULT_STORYBOOK_PAGE,
            theme_id: DEFAULT_THEME_ID,
            preset_index: 0,
            preset_tab_scroll_x: 0,
            selected_component_presets: BTreeMap::new(),
            scroll_y: 0,
            panel_scroll: PanelScrollOffsets::default(),
            scrollbar_visible: true,
            tree_expansion: TreeExpansionState::default(),
            show_navigation_lines: true,
            show_navigation_text_connectors: false,
            screen_state: StorybookScreenState::default(),
            screen_states: StorybookScreenStateStore::default(),
            drag_scroll_target: None,
            text_area_resize_dragging: false,
        }
    }
}

pub(super) fn apply_scroll(window: &Window, state: &mut StorybookWindowState) -> bool {
    let Some((delta_x, delta_y)) = window.get_scroll_wheel() else {
        return false;
    };
    if delta_x == 0.0 && delta_y == 0.0 {
        return false;
    }
    let Some((x, y)) = window.get_unscaled_mouse_pos(MouseMode::Discard) else {
        return apply_scroll_delta(state, delta_y);
    };
    let Some(point) = normalize_mouse_point(window, x, y) else {
        return false;
    };
    let vertical_changed = apply_scroll_delta_at(state, point.x, point.y, delta_y);
    let horizontal_changed = apply_scroll_delta_x_at(state, point.x, point.y, delta_x);
    vertical_changed || horizontal_changed
}

pub(super) fn apply_mouse_click(
    window: &Window,
    state: &mut StorybookWindowState,
    left_mouse_was_down: &mut bool,
    right_mouse_was_down: &mut bool,
) -> bool {
    let left_started = click_started(window, MouseButton::Left, left_mouse_was_down);
    let right_started = click_started(window, MouseButton::Right, right_mouse_was_down);
    if !window.get_mouse_down(MouseButton::Left) {
        state.drag_scroll_target = None;
        state.text_area_resize_dragging = false;
        if state.screen_state.release_button_press() {
            return true;
        }
    }
    let Some((mouse_x, mouse_y)) = window.get_unscaled_mouse_pos(MouseMode::Discard) else {
        return false;
    };
    let Some(point) = normalize_mouse_point(window, mouse_x, mouse_y) else {
        return false;
    };
    let x = point.x;
    let raw_y = point.y;
    if window.get_mouse_down(MouseButton::Left)
        && let Some(target) = state.drag_scroll_target
    {
        return apply_scrollbar_drag_target(state, target, x, raw_y);
    }
    let y = click_content_y(state, x, raw_y);
    if window.get_mouse_down(MouseButton::Left) && state.text_area_resize_dragging {
        return text_area_resize::apply_drag_at(state, x, y);
    }
    if !left_started && !right_started {
        return false;
    }
    if left_started
        && let Some(region) = panel_scroll_drag::vertical_region_at(
            x,
            raw_y,
            state.panel_scroll,
            state.selected_page,
            state.tree_expansion,
            state.scrollbar_visible,
        )
    {
        state.drag_scroll_target = Some(PanelScrollDragTarget::Vertical(region));
        return apply_scrollbar_drag(state, region, raw_y);
    }
    if left_started
        && let Some(region) = panel_scroll_drag::horizontal_region_at(
            x,
            raw_y,
            state.panel_scroll,
            state.selected_page,
            state.tree_expansion,
            state.scrollbar_visible,
        )
    {
        state.drag_scroll_target = Some(PanelScrollDragTarget::Horizontal(region));
        return apply_horizontal_scrollbar_drag(state, region, x);
    }
    if left_started && text_area_resize::handle_at(state, x, y) {
        state.text_area_resize_dragging = true;
        return text_area_resize::apply_drag_at(state, x, y);
    }
    if right_started {
        return apply_context_click(state, x, y);
    }
    apply_click(state, x, y)
}

pub(super) fn apply_click(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if let Some(operation) = button_operation_at(state, x, y) {
        return operation.apply(state);
    }
    let logical_navigation_y = state.panel_scroll.offset_with_max(
        super::panel_scroll_state::PanelScrollRegion::Navigation,
        super::panel_scroll_state::PanelScrollOverflowModel::max_scroll_y_for(
            super::panel_scroll_state::PanelScrollRegion::Navigation,
            state.selected_page,
            state.tree_expansion,
        ),
    );
    if let Some(row) = row_from_click(x, y + logical_navigation_y, state.tree_expansion) {
        match row {
            NavigationRow::Group(group) => state.tree_expansion.toggle(group),
            NavigationRow::Section { group, section } => {
                state.tree_expansion.toggle_section(group, section)
            }
            NavigationRow::Page { page, .. } => state.select_page(page),
            NavigationRow::PageWithoutSection { page, .. } => state.select_page(page),
        }
        return true;
    }
    false
}

fn apply_context_click(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if matches!(state.selected_page, "tree-view" | "context-menu")
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state
            .screen_state
            .register_context_menu(state.selected_page);
        return true;
    }
    false
}

fn click_started(window: &Window, button: MouseButton, mouse_was_down: &mut bool) -> bool {
    let mouse_down = window.get_mouse_down(button);
    let started = mouse_down && !*mouse_was_down;
    *mouse_was_down = mouse_down;
    started
}

fn normalize_mouse_point(window: &Window, x: f32, y: f32) -> Option<CanvasPoint> {
    let (width, height) = window.get_size();
    window_point_to_canvas_point(
        WindowPoint::new(x, y),
        SurfaceSize::new(width, height),
        SurfaceSize::new(WIDTH, HEIGHT),
    )
}

#[cfg(test)]
pub(super) fn apply_scroll_delta_at_for_test(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
    delta_y: f32,
) -> bool {
    scroll_operation::apply_scroll_delta_at(state, x, y, delta_y)
}

#[cfg(test)]
pub(super) fn apply_scroll_delta_x_at_for_test(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
    delta_x: f32,
) -> bool {
    scroll_operation::apply_scroll_delta_x_at(state, x, y, delta_x)
}

#[cfg(test)]
pub(super) fn apply_text_area_resize_drag_at_for_test(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    text_area_resize::apply_drag_at(state, x, y)
}

#[cfg(test)]
pub(super) fn cursor_style_at_for_test(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> StorybookCursorStyle {
    cursor_operation::cursor_style_at(state, x, y)
}

#[cfg(test)]
mod tests;
