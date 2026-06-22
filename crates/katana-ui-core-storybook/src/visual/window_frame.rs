use super::canvas::Canvas;
use super::render;
use super::text_selection_overlay::draw_text_selection_highlight;
use super::window_interaction::StorybookWindowState;
use katana_ui_core::theme::ThemeSnapshot;
use minifb::Window;
use std::env;

const STORYBOOK_SCALE_ENV: &str = "KUC_STORYBOOK_SCALE";

pub(in crate::visual) fn present_for_window(window: &Window, frame: &Canvas) -> Canvas {
    let (width, height) = window.get_size();
    let fill = frame.pixels().first().copied().unwrap_or_default();
    super::presentation::StorybookPresentation::present_frame_for_window(frame, width, height, fill)
}

pub(in crate::visual) fn apply_hover(
    window: &mut Window,
    state: &mut StorybookWindowState,
) -> bool {
    let Some((x, y)) = window.get_unscaled_mouse_pos(minifb::MouseMode::Discard) else {
        return clear_hover(state);
    };
    let (width, height) = window.get_size();
    let Some(point) = super::window_coordinates::window_point_to_canvas_point(
        super::window_coordinates::WindowPoint::new(x, y),
        super::window_coordinates::SurfaceSize::new(width, height),
        super::window_coordinates::SurfaceSize::new(render::WIDTH, render::HEIGHT),
    ) else {
        return clear_hover(state);
    };
    super::window_cursor::apply_cursor_style(
        window,
        super::window_interaction::cursor_style_at(state, point.x, point.y),
    );
    super::window_interaction::apply_hover_at(state, point.x, point.y)
}

pub(in crate::visual) fn clear_hover(state: &mut StorybookWindowState) -> bool {
    let tooltip_changed = state.screen_state.register_tooltip_hover_close();
    let preview_changed = state.screen_state.set_preview_hovered(false);
    let icon_button_changed = state
        .screen_state
        .set_hovered_text_input_icon_button_index(None);
    let input_clear_changed = state
        .screen_state
        .set_hovered_text_input_clear_action(false);
    let text_area_icon_changed = state
        .screen_state
        .set_hovered_text_area_icon_button_index(None);
    let text_area_clear_changed = state.screen_state.set_hovered_text_area_clear_action(false);
    let toolbar_action_changed = state.screen_state.set_hovered_toolbar_action_index(None);
    tooltip_changed
        || preview_changed
        || icon_button_changed
        || input_clear_changed
        || text_area_icon_changed
        || text_area_clear_changed
        || toolbar_action_changed
}

pub(in crate::visual) fn render_frame_for_window_scale(
    renderer: &mut render::StorybookFrameRenderer,
    state: &StorybookWindowState,
    _window: &Window,
) -> Canvas {
    render_frame_for_scale(renderer, state, storybook_scale_factor())
}

pub(in crate::visual) fn render_frame_for_scale(
    renderer: &mut render::StorybookFrameRenderer,
    state: &StorybookWindowState,
    scale_factor: f32,
) -> Canvas {
    let mut frame = renderer.render_for_scale(
        render::StorybookRenderOptions {
            theme_id: state.theme_id,
            selected_page: state.selected_page,
            selected_instance_id: state.selected_instance_id,
            preset_index: state.preset_index,
            preset_tab_scroll_x: state.preset_tab_scroll_x,
            scroll_y: state.scroll_y,
            scrollbar_visible: state.scrollbar_visible,
            panel_scroll: state.panel_scroll,
            tree_expansion: state.tree_expansion,
            show_navigation_lines: state.show_navigation_lines,
            show_navigation_text_connectors: state.show_navigation_text_connectors,
            screen_state: state.screen_state.clone(),
        },
        scale_factor,
    );
    draw_text_selection_highlight(
        &mut frame,
        state.text_selection_start,
        state.text_selection_end,
        selection_color(state.theme_id),
    );
    frame
}

pub(in crate::visual) fn storybook_scale_factor() -> f32 {
    env::var(STORYBOOK_SCALE_ENV)
        .ok()
        .and_then(|value| parse_storybook_scale_factor(value.as_str()))
        .unwrap_or_else(default_storybook_scale_factor)
}

pub(in crate::visual) fn parse_storybook_scale_factor(value: &str) -> Option<f32> {
    let scale = value.parse::<u32>().ok()?;
    match scale {
        1 | 2 => Some(scale as f32),
        _ => None,
    }
}

pub(in crate::visual) fn window_width_for_canvas(frame: &Canvas) -> usize {
    frame.logical_width()
}

pub(in crate::visual) fn window_height_for_canvas(frame: &Canvas) -> usize {
    frame.logical_height()
}

fn default_storybook_scale_factor() -> f32 {
    if cfg!(target_os = "macos") { 2.0 } else { 1.0 }
}

fn selection_color(theme_id: &str) -> u32 {
    let theme = if theme_id == "light" {
        ThemeSnapshot::light()
    } else {
        ThemeSnapshot::dark()
    };
    super::palette::VisualPalette::from_theme(&theme).selection
}
