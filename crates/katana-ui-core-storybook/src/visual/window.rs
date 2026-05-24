use super::canvas::Canvas;
use super::modal;
use super::presentation;
use super::render;
use super::runtime::{StorybookRuntimeReport, StorybookVisualError, StorybookWindowRun};
use super::types::StorybookVisual;
use super::window_interaction::{StorybookWindowState, apply_mouse_click, apply_scroll};
use super::window_modal_plan;
use super::window_options::{main_window_options, modal_window_options};
use minifb::{Key, Window};
use std::env;
use std::thread;
use std::time::Duration;
const MAIN_WINDOW_TITLE: &str = "katana-ui-core Storybook";
const MODAL_WINDOW_TITLE: &str = "katana-ui-core Modal";
const STORYBOOK_SCALE_ENV: &str = "KUC_STORYBOOK_SCALE";

impl StorybookVisual {
    pub fn open_window(self, frames: usize) -> Result<(), minifb::Error> {
        let state = StorybookWindowState::default();
        let mut renderer = render::StorybookFrameRenderer::new();
        let frame = render_frame_for_scale(&mut renderer, &state, storybook_scale_factor());
        let mut window = create_main_window(MAIN_WINDOW_TITLE, &frame)?;
        run_single_window(&mut window, &mut renderer, frame, frames)
    }

    pub fn open_modal_window(
        self,
        frames: usize,
    ) -> Result<StorybookWindowRun, StorybookVisualError> {
        let runtime = self.runtime_report();
        let main_frame = render::render_storybook_canvas();
        let modal_frame = modal::render_modal_canvas();
        let mut main = create_window(MAIN_WINDOW_TITLE, &main_frame)?;
        main.update_with_buffer(main_frame.pixels(), main_frame.width(), main_frame.height())?;
        let plan = window_modal_plan::from_main_window(&main, &main_frame, &modal_frame)?;
        let mut modal_window = create_window(MODAL_WINDOW_TITLE, &modal_frame)?;
        modal_window.set_position(plan.position.x as isize, plan.position.y as isize);
        modal_window.topmost(plan.frontmost);
        run_window_pair(
            &mut main,
            &main_frame,
            &mut modal_window,
            &modal_frame,
            frames,
        )?;
        Ok(StorybookWindowRun {
            frames,
            modal_window_opened: true,
            same_display: plan.same_display,
            frontmost: plan.frontmost,
            state_reflected: runtime.state_reflected,
            overlay_rendered: runtime.overlay_rendered,
        })
    }

    #[must_use]
    pub fn runtime_report(self) -> StorybookRuntimeReport {
        let plan = window_modal_plan::sample();
        StorybookRuntimeReport {
            state_reflected: modal::state_reflected_after_operation(),
            overlay_rendered: modal::overlay_rendered(),
            modal_plan_same_display: plan.as_ref().map(|it| it.same_display).unwrap_or(false),
            modal_plan_frontmost: plan.as_ref().map(|it| it.frontmost).unwrap_or(false),
        }
    }
}

fn create_window(title: &str, frame: &Canvas) -> Result<Window, minifb::Error> {
    let options = if title == MAIN_WINDOW_TITLE {
        main_window_options()
    } else {
        modal_window_options()
    };
    Window::new(title, frame.width(), frame.height(), options)
}

fn create_main_window(title: &str, frame: &Canvas) -> Result<Window, minifb::Error> {
    let options = main_window_options();
    Window::new(
        title,
        window_width_for_canvas(frame),
        window_height_for_canvas(frame),
        options,
    )
}

fn run_single_window(
    window: &mut Window,
    renderer: &mut render::StorybookFrameRenderer,
    mut frame: Canvas,
    frames: usize,
) -> Result<(), minifb::Error> {
    let mut frame_index = 0;
    let mut state = StorybookWindowState::default();
    let mut left_mouse_was_down = false;
    let mut right_mouse_was_down = false;
    let mut presented = present_for_window(window, &frame);
    let mut presented_window_size = window.get_size();
    while frames == 0 || frame_index < frames {
        if !window.is_open() || window.is_key_down(Key::Escape) {
            break;
        }
        let mut frame_changed = false;
        if apply_scroll(window, &mut state)
            || apply_hover(window, &mut state)
            || apply_mouse_click(
                window,
                &mut state,
                &mut left_mouse_was_down,
                &mut right_mouse_was_down,
            )
        {
            frame = render_frame_for_window_scale(renderer, &state, window);
            frame_changed = true;
        }
        let window_size = window.get_size();
        if frame_changed || window_size != presented_window_size {
            presented = present_for_window(window, &frame);
            presented_window_size = window_size;
        }
        window.update_with_buffer(presented.pixels(), presented.width(), presented.height())?;
        thread::sleep(Duration::from_millis(render::FRAME_DELAY_MS));
        frame_index += 1;
    }
    Ok(())
}

fn present_for_window(window: &Window, frame: &Canvas) -> Canvas {
    let window_size = window.get_size();
    if should_present_physical_frame_directly(frame, window_size) {
        return frame.clone();
    }
    let (width, height) = window_size;
    let fill = frame.pixels().first().copied().unwrap_or_default();
    presentation::present_frame(frame, width, height, fill)
}

fn should_present_physical_frame_directly(frame: &Canvas, window_size: (usize, usize)) -> bool {
    frame.scale_factor() > 1.0
        && frame.logical_width() == window_size.0
        && frame.logical_height() == window_size.1
}

fn apply_hover(window: &Window, state: &mut StorybookWindowState) -> bool {
    let Some((x, y)) = window.get_unscaled_mouse_pos(minifb::MouseMode::Discard) else {
        return state.screen_state.set_preview_hovered(false);
    };
    let (width, height) = window.get_size();
    let Some(point) = super::window_coordinates::window_point_to_canvas_point(
        super::window_coordinates::WindowPoint::new(x, y),
        super::window_coordinates::SurfaceSize::new(width, height),
        super::window_coordinates::SurfaceSize::new(render::WIDTH, render::HEIGHT),
    ) else {
        return state.screen_state.set_preview_hovered(false);
    };
    super::window_interaction::apply_hover_at(state, point.x, point.y)
}

fn render_frame_for_window_scale(
    renderer: &mut render::StorybookFrameRenderer,
    state: &StorybookWindowState,
    _window: &Window,
) -> Canvas {
    render_frame_for_scale(renderer, state, storybook_scale_factor())
}

fn render_frame_for_scale(
    renderer: &mut render::StorybookFrameRenderer,
    state: &StorybookWindowState,
    scale_factor: f32,
) -> Canvas {
    renderer.render_for_scale(
        render::StorybookRenderOptions {
            theme_id: state.theme_id,
            selected_page: state.selected_page,
            preset_index: state.preset_index,
            scroll_y: state.scroll_y,
            scrollbar_visible: state.scrollbar_visible,
            panel_scroll: state.panel_scroll,
            tree_expansion: state.tree_expansion,
            show_navigation_lines: state.show_navigation_lines,
            show_navigation_text_connectors: state.show_navigation_text_connectors,
            screen_state: state.screen_state.clone(),
        },
        scale_factor,
    )
}

fn storybook_scale_factor() -> f32 {
    env::var(STORYBOOK_SCALE_ENV)
        .ok()
        .and_then(|value| parse_storybook_scale_factor(value.as_str()))
        .unwrap_or_else(default_storybook_scale_factor)
}

fn parse_storybook_scale_factor(value: &str) -> Option<f32> {
    let scale = value.parse::<u32>().ok()?;
    match scale {
        1 | 2 => Some(scale as f32),
        _ => None,
    }
}

fn default_storybook_scale_factor() -> f32 {
    if cfg!(target_os = "macos") { 2.0 } else { 1.0 }
}

fn window_width_for_canvas(frame: &Canvas) -> usize {
    frame.logical_width()
}

fn window_height_for_canvas(frame: &Canvas) -> usize {
    frame.logical_height()
}

fn run_window_pair(
    main: &mut Window,
    main_frame: &Canvas,
    modal_window: &mut Window,
    modal_frame: &Canvas,
    frames: usize,
) -> Result<(), minifb::Error> {
    let mut frame_index = 0;
    while frames == 0 || frame_index < frames {
        if !main.is_open() || !modal_window.is_open() || main.is_key_down(Key::Escape) {
            break;
        }
        main.update_with_buffer(main_frame.pixels(), main_frame.width(), main_frame.height())?;
        modal_window.update_with_buffer(
            modal_frame.pixels(),
            modal_frame.width(),
            modal_frame.height(),
        )?;
        thread::sleep(Duration::from_millis(render::FRAME_DELAY_MS));
        frame_index += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::StorybookVisual;
    use super::{
        parse_storybook_scale_factor, should_present_physical_frame_directly,
        window_height_for_canvas, window_width_for_canvas,
    };
    use crate::visual::canvas::Canvas;

    #[test]
    fn runtime_report_requires_state_overlay_and_modal_plan() {
        let report = StorybookVisual.runtime_report();

        assert!(report.state_reflected);
        assert!(report.overlay_rendered);
        assert!(report.modal_plan_same_display);
        assert!(report.modal_plan_frontmost);
    }

    #[test]
    fn main_window_is_created_with_logical_canvas_size() {
        let canvas = Canvas::new_scaled(1440, 920, 2.0, 0x111111);
        assert_eq!(1440, window_width_for_canvas(&canvas));
        assert_eq!(920, window_height_for_canvas(&canvas));
    }

    #[test]
    fn parse_storybook_scale_factor_is_strict_for_supported_values() {
        assert_eq!(Some(1.0), parse_storybook_scale_factor("1"));
        assert_eq!(Some(2.0), parse_storybook_scale_factor("2"));
        assert_eq!(None, parse_storybook_scale_factor("0"));
        assert_eq!(None, parse_storybook_scale_factor("3"));
        assert_eq!(None, parse_storybook_scale_factor("invalid"));
    }

    #[test]
    fn should_present_physical_frame_directly_when_scale_2_and_window_matches_logical_size() {
        let frame = Canvas::new_scaled(1440, 920, 2.0, 0x111111);
        assert!(should_present_physical_frame_directly(&frame, (1440, 920)));
    }

    #[test]
    fn should_not_present_physical_frame_directly_when_scale_2_and_window_differs_from_logical_size()
     {
        let frame = Canvas::new_scaled(1440, 920, 2.0, 0x111111);
        assert!(!should_present_physical_frame_directly(&frame, (1440, 800)));
    }

    #[test]
    fn should_not_present_physical_frame_directly_when_scale_is_1_even_if_sizes_match() {
        let frame = Canvas::new_scaled(1440, 920, 1.0, 0x111111);
        assert!(!should_present_physical_frame_directly(&frame, (1440, 920)));
    }
}
