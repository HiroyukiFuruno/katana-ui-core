use super::canvas::Canvas;
use super::command_chrome_runtime;
use super::modal;
use super::render;
use super::runtime::{StorybookRuntimeReport, StorybookVisualError, StorybookWindowRun};
use super::text_command_root_storybook;
use super::text_surface_runtime;
use super::types::StorybookVisual;
use super::window_frame::{
    apply_hover, present_for_window, render_frame_for_scale, render_frame_for_window_scale,
    storybook_scale_factor, window_height_for_canvas, window_width_for_canvas,
};
use super::window_interaction::{StorybookWindowState, apply_mouse_click, apply_scroll};
use super::window_keyboard::apply_keyboard;
use super::window_modal_plan;
use super::window_mouse_trace;
use super::window_options::{main_window_options, modal_window_options};
use super::window_pair::run_window_pair;
use super::window_text_caret::{show_active_text_caret, update_active_text_caret};
use minifb::Window;
use std::thread;
use std::time::Duration;
const MAIN_WINDOW_TITLE: &str = "katana-ui-core Storybook";
const MODAL_WINDOW_TITLE: &str = "katana-ui-core Modal";

impl StorybookVisual {
    pub fn open_window(self, frames: usize) -> Result<(), StorybookVisualError> {
        self.open_window_for_page(frames, crate::DEFAULT_STORYBOOK_PAGE)
    }

    pub fn open_window_for_page(
        self,
        frames: usize,
        selected_page: &'static str,
    ) -> Result<(), StorybookVisualError> {
        self.open_window_for_page_and_preset(frames, selected_page, None)
    }

    pub fn open_window_for_page_and_preset(
        self,
        frames: usize,
        selected_page: &'static str,
        preset_index: Option<usize>,
    ) -> Result<(), StorybookVisualError> {
        match window_runtime_for_page(selected_page) {
            StorybookWindowRuntime::TextCommandRoot => {
                return text_command_root_storybook::open_window(frames).map_err(Into::into);
            }
            StorybookWindowRuntime::CommandChrome => {
                return command_chrome_runtime::open_window(frames).map_err(Into::into);
            }
            StorybookWindowRuntime::TextSurface => {
                return text_surface_runtime::open_window(frames).map_err(Into::into);
            }
            StorybookWindowRuntime::Canvas => {}
        }
        let state = window_state_for_selected_page_and_preset(selected_page, preset_index);
        let mut renderer = render::StorybookFrameRenderer::new();
        let frame = render_frame_for_scale(&mut renderer, &state, storybook_scale_factor());
        let mut window = create_main_window(MAIN_WINDOW_TITLE, &frame)?;
        run_single_window(&mut window, &mut renderer, frame, frames, state)?;
        Ok(())
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StorybookWindowRuntime {
    Canvas,
    TextCommandRoot,
    TextSurface,
    CommandChrome,
}

fn window_runtime_for_page(selected_page: &str) -> StorybookWindowRuntime {
    if text_command_root_storybook::handles_page(selected_page) {
        return StorybookWindowRuntime::TextCommandRoot;
    }
    if command_chrome_runtime::handles_page(selected_page) {
        return StorybookWindowRuntime::CommandChrome;
    }
    if text_surface_runtime::handles_page(selected_page) {
        return StorybookWindowRuntime::TextSurface;
    }
    StorybookWindowRuntime::Canvas
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
    mut state: StorybookWindowState,
) -> Result<(), minifb::Error> {
    let mut frame_index = 0;
    let mut left_mouse_was_down = false;
    let mut right_mouse_was_down = false;
    let mut text_caret_epoch_frame = 0;
    let mut presented = present_for_window(window, &frame);
    let mut presented_window_size = window.get_size();
    while frames == 0 || frame_index < frames {
        if !window.is_open() {
            break;
        }
        window_mouse_trace::record(window, &state, frame_index);
        let scrolled = apply_scroll(window, &mut state);
        let hovered = apply_hover(window, &mut state);
        let clicked = apply_mouse_click(
            window,
            &mut state,
            &frame,
            &mut left_mouse_was_down,
            &mut right_mouse_was_down,
        );
        let keyed = apply_keyboard(window, &mut state, &frame);
        if clicked || keyed {
            text_caret_epoch_frame = frame_index;
            show_active_text_caret(&mut state);
        }
        let runtime_tick = apply_runtime_tick_for_frame(&mut state, clicked || keyed);
        let caret_changed =
            update_active_text_caret(&mut state, frame_index, text_caret_epoch_frame);
        let frame_changed =
            scrolled || hovered || clicked || keyed || runtime_tick || caret_changed;
        if frame_changed {
            frame = render_frame_for_window_scale(renderer, &state, window);
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

#[cfg(test)]
fn window_state_for_selected_page(selected_page: &'static str) -> StorybookWindowState {
    window_state_for_selected_page_and_preset(selected_page, None)
}

fn window_state_for_selected_page_and_preset(
    selected_page: &'static str,
    preset_index: Option<usize>,
) -> StorybookWindowState {
    StorybookWindowState {
        selected_page,
        preset_index: preset_index.unwrap_or_default(),
        ..StorybookWindowState::default()
    }
}

fn apply_runtime_tick(state: &mut StorybookWindowState) -> bool {
    if state.selected_page != "progress-bar" {
        return false;
    }
    let before = state.screen_state.progress_percent();
    state
        .screen_state
        .register_progress_bar_timed_tick(render::FRAME_DELAY_MS as u16);
    state.screen_state.progress_percent() != before
}

fn apply_runtime_tick_for_frame(
    state: &mut StorybookWindowState,
    user_activation_changed: bool,
) -> bool {
    if user_activation_changed {
        return false;
    }
    apply_runtime_tick(state)
}

fn clear_hover(state: &mut StorybookWindowState) -> bool {
    super::window_frame::clear_hover(state)
}

pub(in crate::visual) fn clear_hover_for_audit(state: &mut StorybookWindowState) -> bool {
    clear_hover(state)
}

#[cfg(test)]
mod tests {
    use super::StorybookVisual;
    use super::{
        StorybookWindowRuntime, apply_runtime_tick, apply_runtime_tick_for_frame,
        render_frame_for_scale, window_height_for_canvas, window_runtime_for_page,
        window_state_for_selected_page, window_width_for_canvas,
    };
    use crate::test_assert::KucTestExpect;
    use crate::visual::canvas::Canvas;
    use crate::visual::render::StorybookFrameRenderer;
    use crate::visual::window_frame::parse_storybook_scale_factor;
    use crate::visual::window_interaction::StorybookWindowState;

    #[test]
    fn runtime_report_requires_state_overlay_and_modal_plan() {
        let report = StorybookVisual.runtime_report();

        assert!(report.state_reflected);
        assert!(report.overlay_rendered);
        assert!(report.modal_plan_same_display);
        assert!(report.modal_plan_frontmost);
    }

    #[test]
    fn eframe_pages_dispatch_to_their_actual_runtimes() {
        assert_eq!(
            StorybookWindowRuntime::TextSurface,
            window_runtime_for_page("text-area")
        );
        assert_eq!(
            StorybookWindowRuntime::CommandChrome,
            window_runtime_for_page("command-chrome")
        );
        assert_eq!(
            StorybookWindowRuntime::Canvas,
            window_runtime_for_page("text-input")
        );
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
    fn render_frame_draws_text_selection_highlight() {
        let state = StorybookWindowState {
            selected_page: "text",
            ..StorybookWindowState::default()
        };
        let mut renderer = StorybookFrameRenderer::new();
        let base = render_frame_for_scale(&mut renderer, &state, 1.0);
        let run = base
            .text_runs()
            .iter()
            .find(|run| run.text().contains("Heading"))
            .kuc_expect("text story must expose Heading text run");
        let rect = run.rect();
        let selected_state = StorybookWindowState {
            text_selection_start: Some((rect.x, rect.y)),
            text_selection_end: Some((rect.right(), rect.bottom())),
            ..state
        };
        let selected = render_frame_for_scale(&mut renderer, &selected_state, 1.0);

        assert!(rect_pixel_diff(&base, &selected, rect.x, rect.y, rect.width, rect.height) > 0);
    }

    #[test]
    fn progress_bar_window_runtime_tick_repaints_meter_body() {
        let mut state = window_state_for_selected_page("progress-bar");
        let mut renderer = StorybookFrameRenderer::new();
        let before = render_frame_for_scale(&mut renderer, &state, 1.0);

        for _ in 0..15 {
            assert!(!apply_runtime_tick(&mut state));
        }
        assert!(apply_runtime_tick(&mut state));

        let after = render_frame_for_scale(&mut renderer, &state, 1.0);
        assert_eq!("progress_tick", state.screen_state.last_action);
        assert_eq!("progress_changed", state.screen_state.last_event);
        assert!(rect_pixel_diff(&before, &after, 620, 330, 860, 300) > 0);
    }

    #[test]
    fn progress_bar_window_runtime_tick_cycles_after_maximum() {
        let mut state = window_state_for_selected_page("progress-bar");
        let mut renderer = StorybookFrameRenderer::new();

        advance_progress_runtime_boundary(&mut state);
        advance_progress_runtime_boundary(&mut state);
        let at_max = render_frame_for_scale(&mut renderer, &state, 1.0);

        advance_progress_runtime_boundary(&mut state);
        let cycled = render_frame_for_scale(&mut renderer, &state, 1.0);

        assert_eq!(0, state.screen_state.progress_percent());
        assert_eq!("percent=0", state.screen_state.state_label);
        assert_eq!("progress_tick", state.screen_state.last_action);
        assert_eq!("progress_changed", state.screen_state.last_event);
        assert!(rect_pixel_diff(&at_max, &cycled, 620, 330, 860, 300) > 0);
    }

    #[test]
    fn runtime_tick_does_not_mutate_non_progress_pages() {
        let mut state = window_state_for_selected_page("button");

        assert!(!apply_runtime_tick(&mut state));
        assert_eq!(65, state.screen_state.progress_percent());
        assert_eq!("none", state.screen_state.last_action);
    }

    #[test]
    fn progress_runtime_tick_does_not_overwrite_user_activation_frame() {
        let mut state = window_state_for_selected_page("progress-bar");
        state.screen_state.register_progress_bar_change();
        let percent_after_click = state.screen_state.progress_percent();

        assert!(!apply_runtime_tick_for_frame(&mut state, true));
        assert_eq!(percent_after_click, state.screen_state.progress_percent());
        assert_eq!("progress_change", state.screen_state.last_action);
        assert_eq!("progress_changed", state.screen_state.last_event);
    }

    #[test]
    fn open_window_page_state_drives_progress_bar_runtime_tick() {
        let mut state = window_state_for_selected_page("progress-bar");

        assert_eq!("progress-bar", state.selected_page);
        advance_progress_runtime_boundary(&mut state);

        assert_eq!(82, state.screen_state.progress_percent());
        assert_eq!("progress_tick", state.screen_state.last_action);
        assert_eq!("progress_changed", state.screen_state.last_event);
    }

    #[test]
    fn clear_hover_closes_open_tooltip_when_pointer_leaves_window() {
        let mut state = window_state_for_selected_page("tooltip");
        state.screen_state.register_tooltip_hover_open();

        assert!(state.screen_state.is_tooltip_open());
        assert!(super::clear_hover(&mut state));

        assert!(!state.screen_state.is_tooltip_open());
        assert_eq!("tooltip_hover", state.screen_state.last_action);
        assert_eq!("tooltip_closed", state.screen_state.last_event);
        assert_eq!("hover=false focus=false", state.screen_state.state_label);
    }

    #[test]
    fn progress_bar_runtime_tick_starts_after_navigation_selection() {
        let mut state = StorybookWindowState::default();

        state.select_page("progress-bar");
        advance_progress_runtime_boundary(&mut state);

        assert_eq!("progress-bar", state.selected_page);
        assert_eq!(82, state.screen_state.progress_percent());
        assert_eq!("progress_tick", state.screen_state.last_action);
        assert_eq!("progress_changed", state.screen_state.last_event);
    }

    fn advance_progress_runtime_boundary(state: &mut StorybookWindowState) {
        for _ in 0..16 {
            if apply_runtime_tick(state) {
                return;
            }
        }
        let reached = false;
        assert!(reached, "progress runtime tick boundary was not reached");
    }

    fn rect_pixel_diff(
        before: &Canvas,
        after: &Canvas,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> usize {
        let mut diff = 0;
        for current_y in y..y.saturating_add(height) {
            for current_x in x..x.saturating_add(width) {
                let index = current_y * before.width() + current_x;
                if before.pixels()[index] != after.pixels()[index] {
                    diff += 1;
                }
            }
        }
        diff
    }
}
