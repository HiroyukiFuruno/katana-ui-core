use super::canvas::Canvas;
use super::render;
use super::text_selection_overlay::draw_text_selection_highlight;
use super::window_cursor::StorybookCursorPort;
use super::window_interaction::StorybookWindowInput;
use super::window_interaction::StorybookWindowState;
use katana_ui_core::theme::ThemeSnapshot;
use std::env;

const STORYBOOK_SCALE_ENV: &str = "KUC_STORYBOOK_SCALE";

pub(in crate::visual) fn present_for_window(
    window: &impl StorybookWindowInput,
    frame: &Canvas,
) -> Canvas {
    let (width, height) = window.surface_size();
    let fill = frame.pixels().first().copied().unwrap_or_default();
    super::presentation::StorybookPresentation::present_frame_for_window(frame, width, height, fill)
}

pub(in crate::visual) fn apply_hover(
    window: &mut (impl StorybookWindowInput + StorybookCursorPort),
    state: &mut StorybookWindowState,
) -> bool {
    let Some((x, y)) = window.mouse_position() else {
        return clear_hover(state);
    };
    let (width, height) = window.surface_size();
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
    let value = env::var(STORYBOOK_SCALE_ENV).ok();
    resolve_storybook_scale_factor(value.as_deref(), default_storybook_scale_factor())
}

fn resolve_storybook_scale_factor(value: Option<&str>, default: f32) -> f32 {
    value
        .and_then(parse_storybook_scale_factor)
        .unwrap_or(default)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::preview_detail;
    use crate::visual::window_interaction::StorybookCursorStyle;
    use minifb::{CursorStyle, MouseButton};

    struct FakeHoverWindow {
        position: Option<(f32, f32)>,
        size: (usize, usize),
        cursor: Option<CursorStyle>,
        pointing_hand: usize,
    }

    impl StorybookWindowInput for FakeHoverWindow {
        fn scroll_wheel(&self) -> Option<(f32, f32)> {
            None
        }

        fn mouse_position(&self) -> Option<(f32, f32)> {
            self.position
        }

        fn mouse_down(&self, _button: MouseButton) -> bool {
            false
        }

        fn surface_size(&self) -> (usize, usize) {
            self.size
        }
    }

    impl StorybookCursorPort for FakeHoverWindow {
        fn set_fallback_cursor(&mut self, cursor: CursorStyle) {
            self.cursor = Some(cursor);
        }

        fn set_pointing_hand_cursor(&mut self) {
            self.pointing_hand += 1;
        }
    }

    #[test]
    fn presentation_and_scale_resolution_are_headless_and_deterministic() {
        let window = FakeHoverWindow {
            position: None,
            size: (20, 10),
            cursor: None,
            pointing_hand: 0,
        };
        let frame = Canvas::new(40, 20, 0x112233);

        let presented = present_for_window(&window, &frame);

        assert_eq!((20, 10), (presented.width(), presented.height()));
        assert_eq!(None, StorybookWindowInput::scroll_wheel(&window));
        assert!(!StorybookWindowInput::mouse_down(
            &window,
            MouseButton::Left
        ));
        assert_eq!(2.0, resolve_storybook_scale_factor(Some("2"), 1.0));
        assert_eq!(1.5, resolve_storybook_scale_factor(Some("invalid"), 1.5));
        assert_eq!(1.5, resolve_storybook_scale_factor(None, 1.5));
        assert_ne!(selection_color("light"), selection_color("dark"));
    }

    #[test]
    fn hover_port_clears_missing_pointer_and_maps_component_pointer() {
        let mut state = StorybookWindowState {
            selected_page: "button",
            ..StorybookWindowState::default()
        };
        state.screen_state.set_preview_hovered(true);
        let mut missing = FakeHoverWindow {
            position: None,
            size: (render::WIDTH, render::HEIGHT),
            cursor: None,
            pointing_hand: 0,
        };
        assert!(apply_hover(&mut missing, &mut state));

        let target = preview_detail::component_action_hit_rect("button");
        let mut present = FakeHoverWindow {
            position: Some(((target.x + 4) as f32, (target.y + 4) as f32)),
            size: (render::WIDTH, render::HEIGHT),
            cursor: None,
            pointing_hand: 0,
        };
        assert!(apply_hover(&mut present, &mut state));
        assert!(matches!(
            present.cursor,
            Some(CursorStyle::Arrow | CursorStyle::OpenHand)
        ));
        assert_eq!(
            usize::from(
                super::super::window_interaction::cursor_style_at(
                    &state,
                    target.x + 4,
                    target.y + 4
                ) == StorybookCursorStyle::PointingHand
            ),
            present.pointing_hand
        );

        state.screen_state.set_preview_hovered(true);
        let mut outside = FakeHoverWindow {
            position: Some((-1.0, -1.0)),
            size: (render::WIDTH, render::HEIGHT),
            cursor: None,
            pointing_hand: 0,
        };
        assert!(apply_hover(&mut outside, &mut state));
    }
}
