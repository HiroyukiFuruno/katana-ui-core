use minifb::{MouseButton, MouseMode, Window};

mod button_operation;
mod state_store;

use super::layout_metrics::MAX_SCROLL_Y;
use super::navigation_tree::{NavigationRow, TreeExpansionState, row_from_click};
use super::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion, region_at};
use super::panel_scrollbars;
use super::preview_detail;
use super::render::{HEIGHT, WIDTH};
use super::screen_state::StorybookScreenState;
use super::window_coordinates::{
    CanvasPoint, SurfaceSize, WindowPoint, window_point_to_canvas_point,
};
pub(super) use button_operation::apply_hover_at;
use button_operation::button_operation_at;
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
    pub(super) drag_scroll_region: Option<PanelScrollRegion>,
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
            drag_scroll_region: None,
        }
    }
}

impl StorybookWindowState {
    fn select_page(&mut self, page: &'static str) {
        let preset_index = self
            .selected_component_presets
            .get(page)
            .copied()
            .unwrap_or_default();
        self.switch_screen_state(page, preset_index);
    }

    fn select_preset(&mut self, preset_index: usize) {
        self.switch_screen_state(self.selected_page, preset_index);
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
    let Some((_, delta_y)) = window.get_scroll_wheel() else {
        return false;
    };
    if delta_y == 0.0 {
        return false;
    }
    let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) else {
        return apply_scroll_delta(state, delta_y);
    };
    let Some(point) = normalize_mouse_point(window, x, y) else {
        return false;
    };
    apply_scroll_delta_at(state, point.x, point.y, delta_y)
}

pub(super) fn apply_mouse_click(
    window: &Window,
    state: &mut StorybookWindowState,
    left_mouse_was_down: &mut bool,
    right_mouse_was_down: &mut bool,
) -> bool {
    let left_started = click_started(window, MouseButton::Left, left_mouse_was_down);
    let right_started = click_started(window, MouseButton::Right, right_mouse_was_down);
    let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) else {
        return false;
    };
    let Some(point) = normalize_mouse_point(window, x, y) else {
        return false;
    };
    let x = point.x;
    let raw_y = point.y;
    if !window.get_mouse_down(MouseButton::Left) {
        state.drag_scroll_region = None;
        if state.screen_state.release_button_press() {
            return true;
        }
    }
    if window.get_mouse_down(MouseButton::Left)
        && let Some(region) = state.drag_scroll_region
    {
        return apply_scrollbar_drag(state, region, raw_y);
    }
    if !left_started && !right_started {
        return false;
    }
    if left_started
        && let Some(region) = panel_scrollbars::region_from_thumb(x, raw_y, state.panel_scroll)
    {
        state.drag_scroll_region = Some(region);
        return apply_scrollbar_drag(state, region, raw_y);
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

fn apply_scroll_delta(state: &mut StorybookWindowState, delta_y: f32) -> bool {
    apply_scroll_delta_at_root(state, delta_y)
}

fn apply_scroll_delta_at(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
    delta_y: f32,
) -> bool {
    let region = region_at(x, y + state.panel_scroll.root_y);
    let changed = state.panel_scroll.scroll_delta(region, delta_y);
    if region == PanelScrollRegion::Root {
        state.scroll_y = state.panel_scroll.root_y;
    }
    changed
}

fn apply_scroll_delta_at_root(state: &mut StorybookWindowState, delta_y: f32) -> bool {
    let changed = state
        .panel_scroll
        .scroll_delta(PanelScrollRegion::Root, delta_y);
    state.scroll_y = state.panel_scroll.root_y.min(MAX_SCROLL_Y);
    changed
}

fn apply_scrollbar_drag(
    state: &mut StorybookWindowState,
    region: PanelScrollRegion,
    y: usize,
) -> bool {
    let next = panel_scrollbars::offset_from_drag(region, y);
    let changed = state.panel_scroll.set_drag_offset(region, next);
    if region == PanelScrollRegion::Root {
        state.scroll_y = state.panel_scroll.root_y.min(MAX_SCROLL_Y);
    }
    changed
}

fn click_content_y(state: &StorybookWindowState, x: usize, y: usize) -> usize {
    let content_y = y + state.panel_scroll.root_y;
    match region_at(x, content_y) {
        PanelScrollRegion::Root => content_y,
        PanelScrollRegion::Navigation => content_y,
        PanelScrollRegion::Preview => content_y + state.panel_scroll.preview_y,
        PanelScrollRegion::Inspector => content_y + state.panel_scroll.inspector_y,
    }
}

#[cfg(test)]
mod tests;
