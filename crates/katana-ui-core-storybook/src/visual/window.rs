use super::canvas::Canvas;
use super::modal;
use super::render;
use super::runtime::{StorybookRuntimeReport, StorybookVisualError, StorybookWindowRun};
use super::types::StorybookVisual;
use super::window_cursor::StorybookCursorPort;
use super::window_frame::{
    apply_hover, present_for_window, render_frame_for_scale, render_frame_for_window_scale,
    storybook_scale_factor, window_height_for_canvas, window_width_for_canvas,
};
use super::window_interaction::StorybookWindowInput;
use super::window_interaction::{StorybookWindowState, apply_mouse_click, apply_scroll};
use super::window_keyboard::{StorybookKeyboardInput, apply_keyboard};
use super::window_modal_plan;
use super::window_mouse_trace::{self, MouseTraceWindow};
use super::window_options::{main_window_options, modal_window_options};
use super::window_pair::run_window_pair;
use super::window_text_caret::{show_active_text_caret, update_active_text_caret};
use minifb::Window;
use std::thread;
use std::time::Duration;
const MAIN_WINDOW_TITLE: &str = "katana-ui-core Storybook";
const MODAL_WINDOW_TITLE: &str = "katana-ui-core Modal";

impl StorybookVisual {
    pub fn open_window(self, frames: usize) -> Result<(), minifb::Error> {
        self.open_window_for_page(frames, crate::DEFAULT_STORYBOOK_PAGE)
    }

    pub fn open_window_for_page(
        self,
        frames: usize,
        selected_page: &'static str,
    ) -> Result<(), minifb::Error> {
        self.open_window_for_page_and_preset(frames, selected_page, None)
    }

    pub fn open_window_for_page_and_preset(
        self,
        frames: usize,
        selected_page: &'static str,
        preset_index: Option<usize>,
    ) -> Result<(), minifb::Error> {
        let state = window_state_for_selected_page_and_preset(selected_page, preset_index);
        let mut renderer = render::StorybookFrameRenderer::new();
        let frame = render_frame_for_scale(&mut renderer, &state, storybook_scale_factor());
        let mut window = create_main_window(MAIN_WINDOW_TITLE, &frame)?;
        run_single_window(&mut window, &mut renderer, frame, frames, state)
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
        let pair_result = run_window_pair(
            &mut main,
            &main_frame,
            &mut modal_window,
            &modal_frame,
            frames,
        );
        finish_modal_window_run(
            pair_result,
            frames,
            plan.same_display,
            plan.frontmost,
            runtime,
        )
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

fn finish_modal_window_run(
    pair_result: Result<(), minifb::Error>,
    frames: usize,
    same_display: bool,
    frontmost: bool,
    runtime: StorybookRuntimeReport,
) -> Result<StorybookWindowRun, StorybookVisualError> {
    pair_result?;
    Ok(StorybookWindowRun {
        frames,
        modal_window_opened: true,
        same_display,
        frontmost,
        state_reflected: runtime.state_reflected,
        overlay_rendered: runtime.overlay_rendered,
    })
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

fn run_single_window<W: StorybookRuntimeWindow>(
    window: &mut W,
    renderer: &mut render::StorybookFrameRenderer,
    mut frame: Canvas,
    frames: usize,
    mut state: StorybookWindowState,
) -> Result<(), W::Error> {
    let mut frame_index = 0;
    let mut left_mouse_was_down = false;
    let mut right_mouse_was_down = false;
    let mut text_caret_epoch_frame = 0;
    let mut presented = present_for_window(window, &frame);
    let mut presented_window_size = window.surface_size();
    while frames == 0 || frame_index < frames {
        if !StorybookRuntimeWindow::is_open(window) {
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
            frame = render_frame_for_window_scale(renderer, &state);
        }
        let window_size = window.surface_size();
        if frame_changed || window_size != presented_window_size {
            presented = present_for_window(window, &frame);
            presented_window_size = window_size;
        }
        window.update_frame(&presented)?;
        window.wait_for_next_frame();
        frame_index += 1;
    }
    Ok(())
}

trait StorybookRuntimeWindow:
    StorybookWindowInput + StorybookKeyboardInput + StorybookCursorPort + MouseTraceWindow
{
    type Error;

    fn is_open(&self) -> bool;
    fn update_frame(&mut self, frame: &Canvas) -> Result<(), Self::Error>;
    fn wait_for_next_frame(&mut self);
}

impl StorybookRuntimeWindow for Window {
    type Error = minifb::Error;

    fn is_open(&self) -> bool {
        self.is_open()
    }

    fn update_frame(&mut self, frame: &Canvas) -> Result<(), Self::Error> {
        self.update_with_buffer(frame.pixels(), frame.width(), frame.height())
    }

    fn wait_for_next_frame(&mut self) {
        thread::sleep(Duration::from_millis(render::FRAME_DELAY_MS));
    }
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

#[cfg(all(test, target_os = "linux"))]
#[path = "window_native_coverage_tests.rs"]
mod native_coverage_tests;

#[cfg(test)]
mod tests {
    use super::StorybookVisual;
    use super::{
        apply_runtime_tick, apply_runtime_tick_for_frame, finish_modal_window_run,
        render_frame_for_scale, window_height_for_canvas, window_state_for_selected_page,
        window_width_for_canvas,
    };
    use crate::test_assert::KucTestExpect;
    use crate::visual::canvas::Canvas;
    use crate::visual::render::StorybookFrameRenderer;
    use crate::visual::window_cursor::StorybookCursorPort;
    use crate::visual::window_frame::parse_storybook_scale_factor;
    use crate::visual::window_interaction::StorybookWindowInput;
    use crate::visual::window_interaction::StorybookWindowState;
    use crate::visual::window_keyboard::StorybookKeyboardInput;
    use crate::visual::window_mouse_trace::MouseTraceWindow;
    use minifb::{CursorStyle, Key, MouseButton};

    #[test]
    fn runtime_report_requires_state_overlay_and_modal_plan() {
        let report = StorybookVisual.runtime_report();

        assert!(report.state_reflected);
        assert!(report.overlay_rendered);
        assert!(report.modal_plan_same_display);
        assert!(report.modal_plan_frontmost);
    }

    #[test]
    fn modal_window_run_reports_success_and_propagates_pair_errors() {
        let runtime = super::StorybookRuntimeReport {
            state_reflected: true,
            overlay_rendered: true,
            modal_plan_same_display: true,
            modal_plan_frontmost: true,
        };
        let report = finish_modal_window_run(Ok(()), 3, true, false, runtime)
            .kuc_expect("successful window pair should produce a run report");
        assert_eq!(3, report.frames);
        assert!(report.modal_window_opened);
        assert!(report.same_display);
        assert!(!report.frontmost);
        assert!(report.state_reflected);
        assert!(report.overlay_rendered);

        assert!(matches!(
            finish_modal_window_run(
                Err(minifb::Error::WindowCreate("test".to_string())),
                3,
                true,
                false,
                runtime,
            ),
            Err(super::StorybookVisualError::Window(_))
        ));
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

    #[derive(Default)]
    struct FakeRuntimeWindow {
        open: bool,
        size: (usize, usize),
        mouse_position: Option<(f32, f32)>,
        mouse_buttons: Vec<MouseButton>,
        keys_down: Vec<Key>,
        keys_pressed: Vec<Key>,
        updates: usize,
        waits: usize,
        fail_update: bool,
        last_frame_size: Option<(usize, usize)>,
        fallback_cursor: Option<CursorStyle>,
        pointing_hand: usize,
    }

    impl StorybookWindowInput for FakeRuntimeWindow {
        fn scroll_wheel(&self) -> Option<(f32, f32)> {
            None
        }

        fn mouse_position(&self) -> Option<(f32, f32)> {
            self.mouse_position
        }

        fn mouse_down(&self, button: MouseButton) -> bool {
            self.mouse_buttons.contains(&button)
        }

        fn surface_size(&self) -> (usize, usize) {
            self.size
        }
    }

    impl StorybookKeyboardInput for FakeRuntimeWindow {
        fn key_down(&self, key: Key) -> bool {
            self.keys_down.contains(&key)
        }

        fn keys_pressed(&self) -> Vec<Key> {
            self.keys_pressed.clone()
        }
    }

    impl StorybookCursorPort for FakeRuntimeWindow {
        fn set_fallback_cursor(&mut self, cursor: CursorStyle) {
            self.fallback_cursor = Some(cursor);
        }

        fn set_pointing_hand_cursor(&mut self) {
            self.pointing_hand += 1;
        }
    }

    impl MouseTraceWindow for FakeRuntimeWindow {
        fn mouse_pos(&self) -> Option<(f32, f32)> {
            self.mouse_position
        }

        fn size(&self) -> (usize, usize) {
            self.size
        }

        fn left_down(&self) -> bool {
            self.mouse_buttons.contains(&MouseButton::Left)
        }
    }

    impl super::StorybookRuntimeWindow for FakeRuntimeWindow {
        type Error = &'static str;

        fn is_open(&self) -> bool {
            self.open
        }

        fn update_frame(&mut self, frame: &Canvas) -> Result<(), Self::Error> {
            if self.fail_update {
                return Err("frame update failed");
            }
            self.updates += 1;
            self.last_frame_size = Some((frame.width(), frame.height()));
            Ok(())
        }

        fn wait_for_next_frame(&mut self) {
            self.waits += 1;
        }
    }

    #[test]
    fn single_window_loop_runs_keyboard_render_present_update_and_wait_headlessly() {
        let mut window = FakeRuntimeWindow {
            open: true,
            size: (720, 460),
            keys_pressed: vec![Key::Tab, Key::Space],
            ..FakeRuntimeWindow::default()
        };
        let mut renderer = StorybookFrameRenderer::new();
        let state = StorybookWindowState {
            selected_page: "checkbox",
            ..StorybookWindowState::default()
        };

        super::run_single_window(
            &mut window,
            &mut renderer,
            Canvas::new(1440, 920, 0),
            1,
            state,
        )
        .kuc_expect("headless port must present one frame");

        assert_eq!(1, window.updates);
        assert_eq!(1, window.waits);
        assert_eq!(Some((720, 460)), window.last_frame_size);
    }

    #[test]
    fn single_window_loop_stops_when_closed_and_propagates_update_errors() {
        let mut closed = FakeRuntimeWindow {
            size: (10, 10),
            ..FakeRuntimeWindow::default()
        };
        let mut renderer = StorybookFrameRenderer::new();
        super::run_single_window(
            &mut closed,
            &mut renderer,
            Canvas::new(10, 10, 0),
            0,
            StorybookWindowState::default(),
        )
        .kuc_expect("closed window is a clean stop");
        assert_eq!(0, closed.updates);

        let mut failing = FakeRuntimeWindow {
            open: true,
            size: (10, 10),
            fail_update: true,
            ..FakeRuntimeWindow::default()
        };
        assert_eq!(
            Err("frame update failed"),
            super::run_single_window(
                &mut failing,
                &mut renderer,
                Canvas::new(10, 10, 0),
                1,
                StorybookWindowState::default(),
            )
        );
    }

    #[test]
    fn fake_runtime_window_ports_report_pointer_buttons_size_and_cursor_updates() {
        let mut window = FakeRuntimeWindow {
            size: (320, 200),
            mouse_position: Some((12.0, 24.0)),
            mouse_buttons: vec![MouseButton::Left],
            ..FakeRuntimeWindow::default()
        };

        assert_eq!(
            Some((12.0, 24.0)),
            StorybookWindowInput::mouse_position(&window)
        );
        assert!(StorybookWindowInput::mouse_down(&window, MouseButton::Left));
        assert_eq!((320, 200), MouseTraceWindow::size(&window));
        assert_eq!(Some((12.0, 24.0)), MouseTraceWindow::mouse_pos(&window));
        assert!(MouseTraceWindow::left_down(&window));

        StorybookCursorPort::set_fallback_cursor(&mut window, CursorStyle::Arrow);
        StorybookCursorPort::set_pointing_hand_cursor(&mut window);
        assert_eq!(Some(CursorStyle::Arrow), window.fallback_cursor);
        assert_eq!(1, window.pointing_hand);
    }

    fn advance_progress_runtime_boundary(state: &mut StorybookWindowState) {
        let reached = (0..16).any(|_| apply_runtime_tick(state));
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
