use super::canvas::Canvas;
use super::text_selection::{TextSelection, copy_payload_for_selection, selected_text_run_rects};
use minifb::{MouseButton, MouseMode, Window};

mod button_operation;
mod clickable_operation;
pub(super) mod collapsible_panel_state;
pub(super) mod color_picker_operation;
pub(super) mod color_picker_state;
pub(super) mod color_picker_update;
pub(super) mod command_palette_state;
mod content_position;
mod context_click;
mod cursor_operation;
pub(super) mod diagnostics_list_event_assertions;
pub(super) mod diagnostics_list_fixture;
pub(super) mod diagnostics_list_operation;
pub(super) mod diagnostics_list_option_state;
pub(super) mod diagnostics_list_state;
pub(super) mod diagnostics_list_update;
mod drag_and_drop_contract;
pub(super) mod drag_and_drop_operation;
pub(super) mod dynamic_array_editor_operation;
pub(super) mod layout_operation;
mod panel_scroll_drag;
mod panel_scroll_state_store;
mod preset_selection;
pub(super) mod runtime_structured_state;
pub(super) mod scroll_area_operation;
mod scroll_operation;
pub(super) mod settings_list_operation;
pub(super) mod settings_list_state;
pub(super) mod settings_list_update;
pub(super) mod shortcut_cheatsheet_fixture;
pub(super) mod shortcut_cheatsheet_state;
pub(super) mod split_pane_operation;
mod state_store;
#[cfg(test)]
mod state_store_tests;
mod tabs_drag;
mod tabs_focus;
mod tabs_keyboard;
mod text_area_keyboard;
mod text_area_resize;
mod text_input_keyboard;
pub(super) mod theme_tokens_operation;
pub(super) mod virtualization_state;

use super::dedicated_context_menu_popup;
use super::navigation_tree::{NavigationRow, TreeExpansionState, row_from_click};
use super::panel_scroll_state::PanelScrollOffsets;
use super::preview_detail;
use super::render::{HEIGHT, WIDTH};
use super::screen_state::StorybookScreenState;
use super::text_area_screen_state::DEFAULT_TEXT_AREA_INSTANCE;
use super::text_input_screen_state::DEFAULT_TEXT_INPUT_INSTANCE;
use super::window_coordinates::{
    CanvasPoint, SurfaceSize, WindowPoint, window_point_to_canvas_point,
};
use crate::DEFAULT_STORYBOOK_PAGE;
pub(super) use button_operation::apply_hover_at;
use button_operation::button_operation_at;
pub(super) use content_position::click_content_y;
pub(super) use cursor_operation::{StorybookCursorStyle, cursor_style_at};
use panel_scroll_drag::PanelScrollDragTarget;
use panel_scroll_state_store::StorybookPanelScrollStateStore;
use scroll_operation::{
    apply_horizontal_scrollbar_drag, apply_scroll_delta, apply_scroll_delta_at,
    apply_scroll_delta_x_at, apply_scrollbar_drag, apply_scrollbar_drag_target,
};
pub(in crate::visual) use state_store::DEFAULT_INSTANCE_ID;
use state_store::StorybookScreenStateStore;
use std::collections::BTreeMap;
use tabs_drag::TabsDragTarget;
pub(super) use tabs_keyboard::apply_tabs_keyboard_shortcut;
pub(super) use text_area_keyboard::{TextAreaKey, apply_text_area_key};
pub(super) use text_input_keyboard::{TextInputKey, apply_text_input_key};

const DEFAULT_THEME_ID: &str = "dark";

#[derive(Debug, Clone, PartialEq)]
pub(super) struct StorybookWindowState {
    pub(super) selected_page: &'static str,
    pub(super) theme_id: &'static str,
    pub(super) preset_index: usize,
    pub(super) preset_tab_scroll_x: usize,
    pub(super) selected_instance_id: &'static str,
    pub(super) selected_component_presets: BTreeMap<&'static str, usize>,
    pub(super) selected_component_instances: BTreeMap<&'static str, &'static str>,
    pub(super) scroll_y: usize,
    pub(super) panel_scroll: PanelScrollOffsets,
    pub(super) panel_scroll_states: StorybookPanelScrollStateStore,
    pub(super) scrollbar_visible: bool,
    pub(super) tree_expansion: TreeExpansionState,
    pub(super) show_navigation_lines: bool,
    pub(super) show_navigation_text_connectors: bool,
    pub(super) screen_state: StorybookScreenState,
    pub(super) screen_states: StorybookScreenStateStore,
    pub(super) drag_scroll_target: Option<PanelScrollDragTarget>,
    pub(super) tabs_drag_target: Option<TabsDragTarget>,
    pub(super) text_area_resize_dragging: bool,
    pub(super) text_selection_start: Option<(usize, usize)>,
    pub(super) text_selection_end: Option<(usize, usize)>,
    pub(super) last_hover_signature: Option<(&'static str, usize, usize)>,
    pub(super) clipboard_text: String,
}

impl Default for StorybookWindowState {
    fn default() -> Self {
        Self {
            selected_page: DEFAULT_STORYBOOK_PAGE,
            theme_id: DEFAULT_THEME_ID,
            preset_index: 0,
            preset_tab_scroll_x: 0,
            selected_instance_id: DEFAULT_INSTANCE_ID,
            selected_component_presets: BTreeMap::new(),
            selected_component_instances: BTreeMap::new(),
            scroll_y: 0,
            panel_scroll: PanelScrollOffsets::default(),
            panel_scroll_states: StorybookPanelScrollStateStore::default(),
            scrollbar_visible: true,
            tree_expansion: TreeExpansionState::default(),
            show_navigation_lines: true,
            show_navigation_text_connectors: false,
            screen_state: StorybookScreenState::default(),
            screen_states: StorybookScreenStateStore::default(),
            drag_scroll_target: None,
            tabs_drag_target: None,
            text_area_resize_dragging: false,
            text_selection_start: None,
            text_selection_end: None,
            last_hover_signature: None,
            clipboard_text: String::new(),
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

pub(in crate::visual) fn component_instance_id_for_page(
    page: &str,
    instance_id: &'static str,
) -> &'static str {
    if instance_id != DEFAULT_INSTANCE_ID {
        return instance_id;
    }
    match page {
        "text-input" => DEFAULT_TEXT_INPUT_INSTANCE,
        "text-area" => DEFAULT_TEXT_AREA_INSTANCE,
        _ => instance_id,
    }
}

pub(super) fn apply_mouse_click(
    window: &Window,
    state: &mut StorybookWindowState,
    frame: &Canvas,
    left_mouse_was_down: &mut bool,
    right_mouse_was_down: &mut bool,
) -> bool {
    let left_started = click_started(window, MouseButton::Left, left_mouse_was_down);
    let right_started = click_started(window, MouseButton::Right, right_mouse_was_down);
    if !window.get_mouse_down(MouseButton::Left) {
        state.drag_scroll_target = None;
        state.text_area_resize_dragging = false;
        clear_pending_text_selection(state);
        if tabs_drag::release(state) {
            return true;
        }
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
    let text_selection_changed = apply_text_selection(
        state,
        frame,
        left_started,
        window.get_mouse_down(MouseButton::Left),
        x,
        raw_y,
    );
    if text_selection_changed {
        return true;
    }
    if window.get_mouse_down(MouseButton::Left)
        && let Some(target) = state.drag_scroll_target
    {
        return apply_scrollbar_drag_target(state, target, x, raw_y);
    }
    let y = click_content_y(state, x, raw_y);
    if window.get_mouse_down(MouseButton::Left) && state.tabs_drag_target.is_some() {
        return tabs_drag::apply_drag_at(state, x, y);
    }
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
    if left_started && tabs_drag::start_at(state, x, y) {
        return true;
    }
    if right_started {
        return context_click::apply_context_click(state, x, y);
    }
    apply_click(state, x, y) || text_selection_changed
}

pub(super) fn copy_selected_text_to_clipboard_for_frame(
    state: &mut StorybookWindowState,
    frame: &Canvas,
) -> bool {
    let Some(start) = state.text_selection_start else {
        return false;
    };
    let Some(end) = state.text_selection_end else {
        return false;
    };
    let payload = copy_payload_for_selection(frame.text_runs(), TextSelection::drag(start, end));
    if payload.is_empty() {
        return false;
    }
    state.clipboard_text = payload.clone();
    state.screen_state.action_count += 1;
    state.screen_state.last_action = "copy_selection";
    state.screen_state.last_event = "clipboard_copy";
    state.screen_state.state_label = "clipboard=selected_text";
    #[cfg(not(test))]
    if let Err(error) = write_clipboard_text(&payload) {
        eprintln!("[katana-ui-core-storybook] clipboard write failed: {error}");
    }
    true
}

fn apply_text_selection(
    state: &mut StorybookWindowState,
    frame: &Canvas,
    left_started: bool,
    left_down: bool,
    x: usize,
    y: usize,
) -> bool {
    if !display_text_selection_enabled(state.selected_page) {
        return false;
    }
    if !left_down {
        return false;
    }
    if left_started && text_run_contains_point(frame, x, y) {
        state.text_selection_start = Some((x, y));
        state.text_selection_end = None;
        return true;
    }
    if let Some(start) = state.text_selection_start {
        let selection = TextSelection::drag(start, (x, y));
        if selected_text_run_rects(frame.text_runs(), selection).is_empty() {
            state.text_selection_end = None;
            return false;
        }
        state.text_selection_end = Some((x, y));
        register_text_selection_change(state);
        return true;
    }
    false
}

fn clear_pending_text_selection(state: &mut StorybookWindowState) {
    if state.text_selection_start.is_some() && state.text_selection_end.is_none() {
        state.text_selection_start = None;
    }
}

fn display_text_selection_enabled(page: &str) -> bool {
    page == "text"
}

fn register_text_selection_change(state: &mut StorybookWindowState) {
    state.screen_state.action_count += 1;
    state.screen_state.last_action = "select_text";
    state.screen_state.last_event = "text_selection_changed";
    state.screen_state.state_label = "selection=active";
}

fn text_run_contains_point(frame: &Canvas, x: usize, y: usize) -> bool {
    frame.text_runs().iter().any(|run| {
        let rect = run.rect();
        x >= rect.x && x <= rect.right() && y >= rect.y && y <= rect.bottom()
    })
}

pub(super) fn apply_text_selection_drag_for_audit(
    state: &mut StorybookWindowState,
    frame: &Canvas,
    start: (usize, usize),
    end: (usize, usize),
) -> bool {
    let started = apply_text_selection(state, frame, true, true, start.0, start.1);
    let dragged = apply_text_selection(state, frame, false, true, end.0, end.1);
    started && dragged
}

#[cfg(test)]
pub(super) fn apply_text_selection_press_for_test(
    state: &mut StorybookWindowState,
    frame: &Canvas,
    x: usize,
    y: usize,
) -> bool {
    apply_text_selection(state, frame, true, true, x, y)
}

pub(super) fn apply_text_copy_shortcut_for_audit(
    state: &mut StorybookWindowState,
    frame: &Canvas,
) -> bool {
    copy_selected_text_to_clipboard_for_frame(state, frame)
}

pub(super) fn apply_text_paste_shortcut_for_audit(state: &mut StorybookWindowState) -> bool {
    let text = state.clipboard_text.clone();
    if text.is_empty() {
        return false;
    }
    apply_clipboard_paste_text(state, text.as_str())
}

pub(super) fn apply_clipboard_paste_text(state: &mut StorybookWindowState, text: &str) -> bool {
    match state.selected_page {
        "text-input" => {
            let readonly = state.preset_index == 2;
            let instance =
                component_instance_id_for_page(state.selected_page, state.selected_instance_id);
            state
                .screen_state
                .register_text_input_paste_for(instance, text, readonly)
        }
        "text-area" => {
            let instance =
                component_instance_id_for_page(state.selected_page, state.selected_instance_id);
            state
                .screen_state
                .register_text_area_paste_for(instance, text)
        }
        _ => false,
    }
}

#[cfg(all(not(test), target_os = "macos"))]
fn write_clipboard_text(text: &str) -> Result<(), std::io::Error> {
    crate::system::ProcessCommand::write_stdin("pbcopy", text.as_bytes())
}

#[cfg(all(not(test), not(target_os = "macos")))]
fn write_clipboard_text(_text: &str) -> Result<(), std::io::Error> {
    Ok(())
}

pub(super) fn apply_click(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if let Some(command) = context_click::context_menu_command_at(state, x, y) {
        match command {
            dedicated_context_menu_popup::ContextMenuPreviewCommand::OpenInsertSubmenu => {
                state.screen_state.register_context_menu_submenu();
            }
            dedicated_context_menu_popup::ContextMenuPreviewCommand::SelectLink => {
                state.screen_state.register_context_menu_select_link();
            }
        }
        return true;
    }
    if let Some(command) = context_click::tabs_context_command_at(state, x, y) {
        state.screen_state.register_tabs_context_command(command);
        return true;
    }
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

pub(super) fn apply_scroll_delta_x_at_for_audit(
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
pub(super) fn start_tabs_drag_at_for_test(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    tabs_drag::start_at(state, x, y)
}

pub(super) fn start_tabs_drag_at_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    tabs_drag::start_at(state, x, y)
}

#[cfg(test)]
pub(super) fn apply_tabs_drag_at_for_test(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    tabs_drag::apply_drag_at(state, x, y)
}

pub(super) fn apply_tabs_drag_at_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    tabs_drag::apply_drag_at(state, x, y)
}

#[cfg(test)]
pub(super) fn release_tabs_drag_for_test(state: &mut StorybookWindowState) -> bool {
    tabs_drag::release(state)
}

pub(super) fn release_tabs_drag_for_audit(state: &mut StorybookWindowState) -> bool {
    tabs_drag::release(state)
}

pub(super) fn focus_clickable_at_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    clickable_operation::focus_at(state, x, y)
}

pub(super) fn apply_clickable_keyboard_activation_for_audit(
    state: &mut StorybookWindowState,
) -> bool {
    clickable_operation::keyboard_activate(state)
}

pub(super) fn apply_command_palette_escape_for_audit(state: &mut StorybookWindowState) -> bool {
    if state.selected_page != "command-palette" {
        return false;
    }
    state.screen_state.register_command_palette_action(
        command_palette_state::CommandPaletteStoryAction::KeyboardClose,
    );
    true
}

pub(super) fn apply_slide_drag_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "slide-control" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("slide-control").contains(x, y) {
        return false;
    }
    state.screen_state.register_slide_drag();
    true
}

pub(super) fn apply_drag_and_drop_drag_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "drag-and-drop" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("drag-and-drop").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_drag_and_drop_action(drag_and_drop_operation::DragAndDropAction::StartPointer);
    true
}

pub(super) fn apply_drag_and_drop_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "drag-and-drop" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("drag-and-drop").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_drag_and_drop_action(drag_and_drop_operation::DragAndDropAction::ScrollEdge);
    true
}

pub(super) fn apply_drag_and_drop_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "drag-and-drop" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("drag-and-drop").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_drag_and_drop_action(drag_and_drop_operation::DragAndDropAction::ResizeTarget);
    true
}

pub(super) fn apply_panel_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "panel" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("panel").contains(x, y) {
        return false;
    }
    state.screen_state.register_panel_resize();
    true
}

pub(super) fn apply_row_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "row" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("row").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(layout_operation::LayoutStoryAction::RowResize);
    true
}

pub(super) fn apply_column_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "column" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("column").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(layout_operation::LayoutStoryAction::ColumnResize);
    true
}

pub(super) fn apply_stack_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "stack" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("stack").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(layout_operation::LayoutStoryAction::StackResize);
    true
}

pub(super) fn apply_grid_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "grid" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("grid").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(layout_operation::LayoutStoryAction::GridResize);
    true
}

pub(super) fn apply_align_center_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "align-center" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("align-center").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_layout_action(layout_operation::LayoutStoryAction::AlignCenterResize);
    true
}

pub(super) fn apply_theme_tokens_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "theme-tokens" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("theme-tokens").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_theme_tokens_action(theme_tokens_operation::ThemeTokensStoryAction::Resize);
    true
}

pub(super) fn apply_scroll_area_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "scroll-area" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("scroll-area").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_scroll_area_action(scroll_area_operation::ScrollAreaStoryAction::Scroll);
    true
}

pub(super) fn apply_scroll_area_drag_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "scroll-area" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("scroll-area").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_scroll_area_action(scroll_area_operation::ScrollAreaStoryAction::Drag);
    true
}

pub(super) fn apply_scroll_area_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "scroll-area" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("scroll-area").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_scroll_area_action(scroll_area_operation::ScrollAreaStoryAction::Resize);
    true
}

pub(super) fn apply_split_pane_drag_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "split-pane" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("split-pane").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_split_pane_action(split_pane_operation::SplitPaneStoryAction::Drag);
    true
}

pub(super) fn apply_split_pane_resize_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "split-pane" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("split-pane").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_split_pane_action(split_pane_operation::SplitPaneStoryAction::Resize);
    true
}

pub(super) fn apply_list_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "list" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("list").contains(x, y) {
        return false;
    }
    state.screen_state.register_list_scroll();
    true
}

pub(super) fn apply_select_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "select-box" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("select-box").contains(x, y) {
        return false;
    }
    state.screen_state.register_selection_action(
        crate::visual::selection_screen_state::SelectionScreenAction::SelectScroll,
    );
    true
}

pub(super) fn apply_selection_list_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "selection-list" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("selection-list").contains(x, y) {
        return false;
    }
    state.screen_state.register_selection_action(
        crate::visual::selection_screen_state::SelectionScreenAction::SelectionListScroll,
    );
    true
}

pub(super) fn apply_tree_view_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "tree-view" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("tree-view").contains(x, y) {
        return false;
    }
    scroll_operation::apply_scroll_delta_at(state, x, y, -1.0)
}

pub(super) fn apply_side_menu_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "side-menu" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("side-menu").contains(x, y) {
        return false;
    }
    state.screen_state.register_side_menu_action(
        crate::visual::screen_state_side_menu::SideMenuScreenAction::Scroll,
    );
    true
}

pub(super) fn apply_shortcut_cheatsheet_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "shortcut-cheatsheet" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("shortcut-cheatsheet").contains(x, y) {
        return false;
    }
    state.screen_state.register_shortcut_cheatsheet_scroll();
    true
}

pub(super) fn apply_settings_list_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "settings-list" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("settings-list").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_settings_list_action(settings_list_operation::SettingsListStoryAction::Scroll);
    true
}

pub(super) fn apply_diagnostics_list_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "diagnostics-list" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("diagnostics-list").contains(x, y) {
        return false;
    }
    state.screen_state.register_diagnostics_list_action(
        diagnostics_list_operation::DiagnosticsListStoryAction::ScrollRetention,
    );
    true
}

pub(super) fn apply_virtualization_scroll_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "virtualization" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("virtualization").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_virtualization_action(virtualization_state::VirtualizationStoryAction::Scroll);
    true
}

pub(super) fn apply_code_diff_scroll_sync_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "code-diff" {
        return false;
    }
    if !preview_detail::component_action_hit_rect("code-diff").contains(x, y) {
        return false;
    }
    state.screen_state.register_code_diff_scroll_sync();
    true
}

pub(super) fn focus_tabs_at_for_audit(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    tabs_focus::focus_at(state, x, y)
}

#[cfg(test)]
pub(super) fn apply_tabs_keyboard_shortcut_for_test(
    state: &mut StorybookWindowState,
    shortcut: katana_ui_core::widget::molecules::CloseableTabKeyboardShortcut,
) -> bool {
    tabs_keyboard::apply_tabs_keyboard_shortcut(state, shortcut)
}

#[cfg(test)]
pub(super) fn cursor_style_at_for_test(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> StorybookCursorStyle {
    cursor_operation::cursor_style_at(state, x, y)
}

pub(super) fn apply_context_click(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    context_click::apply_context_click(state, x, y)
}

#[cfg(test)]
pub(super) fn apply_context_click_for_test(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    context_click::apply_context_click(state, x, y)
}

#[cfg(test)]
mod tests;
