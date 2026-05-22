use minifb::{MouseButton, MouseMode, Window};

mod button_operation;
mod content_position;
mod panel_scroll_drag;
mod scroll_operation;
mod state_store;

use super::navigation_tree::{NavigationRow, TreeExpansionState, row_from_click};
use super::panel_scroll_state::PanelScrollOffsets;
use super::preview_detail;
use super::render::{HEIGHT, WIDTH};
use super::screen_state::StorybookScreenState;
use super::window_coordinates::{
    CanvasPoint, SurfaceSize, WindowPoint, window_point_to_canvas_point,
};
use crate::catalog::StoryPresetLabels;
pub(super) use button_operation::apply_hover_at;
use button_operation::button_operation_at;
pub(super) use content_position::click_content_y;
use panel_scroll_drag::PanelScrollDragTarget;
use scroll_operation::{
    apply_horizontal_scrollbar_drag, apply_scroll_delta, apply_scroll_delta_at,
    apply_scroll_delta_x_at, apply_scrollbar_drag, apply_scrollbar_drag_target,
};
use state_store::StorybookScreenStateStore;
use std::collections::BTreeMap;

const DEFAULT_SELECTED_PAGE: &str = "button";
const DEFAULT_THEME_ID: &str = "dark";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StorybookWindowState {
    pub(super) selected_page: &'static str,
    pub(super) theme_id: &'static str,
    pub(super) preset_index: usize,
    pub(super) selected_component_presets: BTreeMap<&'static str, usize>,
    pub(super) scroll_y: usize,
    pub(super) panel_scroll: PanelScrollOffsets,
    pub(super) scrollbar_visible: bool,
    pub(super) tree_expansion: TreeExpansionState,
    pub(super) screen_state: StorybookScreenState,
    pub(super) screen_states: StorybookScreenStateStore,
    pub(super) drag_scroll_target: Option<PanelScrollDragTarget>,
}

impl Default for StorybookWindowState {
    fn default() -> Self {
        Self {
            selected_page: DEFAULT_SELECTED_PAGE,
            theme_id: DEFAULT_THEME_ID,
            preset_index: 0,
            selected_component_presets: BTreeMap::new(),
            scroll_y: 0,
            panel_scroll: PanelScrollOffsets::default(),
            scrollbar_visible: true,
            tree_expansion: TreeExpansionState::default(),
            screen_state: StorybookScreenState::default(),
            screen_states: StorybookScreenStateStore::default(),
            drag_scroll_target: None,
        }
    }
}

impl StorybookWindowState {
    pub(super) fn select_page(&mut self, page: &'static str) {
        let preset_index = self
            .selected_component_presets
            .get(page)
            .copied()
            .unwrap_or_default();
        self.switch_screen_state(page, normalized_preset_index(page, preset_index));
    }

    pub(super) fn select_preset(&mut self, preset_index: usize) {
        self.switch_screen_state(
            self.selected_page,
            normalized_preset_index(self.selected_page, preset_index),
        );
    }

    fn switch_screen_state(&mut self, page: &'static str, preset_index: usize) {
        self.screen_states
            .save(self.selected_page, self.preset_index, self.screen_state);
        self.selected_component_presets
            .insert(self.selected_page, self.preset_index);
        self.selected_page = page;
        self.preset_index = preset_index;
        self.selected_component_presets.insert(page, preset_index);
        self.screen_state = self.screen_states.restore(page, preset_index);
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
    if !left_started && !right_started {
        return false;
    }
    if left_started
        && let Some(region) = panel_scroll_drag::vertical_region_at(
            x,
            raw_y,
            state.panel_scroll,
            state.selected_page,
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
            state.scrollbar_visible,
        )
    {
        state.drag_scroll_target = Some(PanelScrollDragTarget::Horizontal(region));
        return apply_horizontal_scrollbar_drag(state, region, x);
    }
    let y = click_content_y(state, x, raw_y);
    if right_started {
        return apply_context_click(state, x, y);
    }
    apply_click(state, x, y)
}

pub(super) fn apply_click(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if let Some(operation) = button_operation_at(state, x, y) {
        return operation.apply(state);
    }
    if let Some(row) = row_from_click(x, y + state.panel_scroll.navigation_y, state.tree_expansion)
    {
        match row {
            NavigationRow::Group(group) => state.tree_expansion.toggle(group),
            NavigationRow::Page { page, .. } => state.select_page(page),
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

fn normalized_preset_index(page: &str, preset_index: usize) -> usize {
    preset_index.min(StoryPresetLabels::for_page(page).len().saturating_sub(1))
}

#[cfg(test)]
mod tests;
