use super::canvas::Canvas;
use super::modal;
use super::render;
use super::runtime::{StorybookRuntimeReport, StorybookVisualError, StorybookWindowRun};
use super::types::StorybookVisual;
use katana_ui_core::window::{
    DisplayBounds, ModalWindowPlacement, ModalWindowPlacementError, WindowId, WindowPoint,
    WindowRect, WindowSize,
};
use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;
const DISPLAY_PADDING: f32 = 256.0;
const SAMPLE_PARENT_X: f32 = 80.0;
const SAMPLE_PARENT_Y: f32 = 80.0;
const SAMPLE_DISPLAY_WIDTH: f32 = 1920.0;
const SAMPLE_DISPLAY_HEIGHT: f32 = 1200.0;
const SAMPLE_DISPLAY_SCALE: f32 = 2.0;
const MAIN_WINDOW_TITLE: &str = "katana-ui-core Storybook";
const MODAL_WINDOW_TITLE: &str = "katana-ui-core Modal";
impl StorybookVisual {
    pub fn open_window(self, frames: usize) -> Result<(), minifb::Error> {
        let frame = render::render_storybook_canvas();
        let mut window = Window::new(
            MAIN_WINDOW_TITLE,
            frame.width(),
            frame.height(),
            WindowOptions::default(),
        )?;
        run_single_window(&mut window, &frame, frames)
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
        let plan = modal_plan_from_main_window(&main, &main_frame, &modal_frame)?;
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
        let plan = sample_modal_plan();
        StorybookRuntimeReport {
            state_reflected: modal::state_reflected_after_operation(),
            overlay_rendered: modal::overlay_rendered(),
            modal_plan_same_display: plan.as_ref().map(|it| it.same_display).unwrap_or(false),
            modal_plan_frontmost: plan.as_ref().map(|it| it.frontmost).unwrap_or(false),
        }
    }
}

fn create_window(title: &str, frame: &Canvas) -> Result<Window, minifb::Error> {
    Window::new(
        title,
        frame.width(),
        frame.height(),
        WindowOptions::default(),
    )
}

fn run_single_window(
    window: &mut Window,
    frame: &Canvas,
    frames: usize,
) -> Result<(), minifb::Error> {
    let mut frame_index = 0;
    while frames == 0 || frame_index < frames {
        if !window.is_open() || window.is_key_down(Key::Escape) {
            break;
        }
        window.update_with_buffer(frame.pixels(), frame.width(), frame.height())?;
        thread::sleep(Duration::from_millis(render::FRAME_DELAY_MS));
        frame_index += 1;
    }
    Ok(())
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

fn modal_plan_from_main_window(
    main: &Window,
    main_frame: &Canvas,
    modal_frame: &Canvas,
) -> Result<katana_ui_core::window::ModalWindowPlan, ModalWindowPlacementError> {
    let (x, y) = main.get_position();
    let parent_rect = WindowRect::new(
        WindowPoint::new(x as f32, y as f32),
        WindowSize::new(main_frame.width() as f32, main_frame.height() as f32),
    );
    let modal_size = WindowSize::new(modal_frame.width() as f32, modal_frame.height() as f32);
    let display = display_around_parent(parent_rect);
    ModalWindowPlacement::same_display(
        WindowId::new("storybook-main"),
        WindowId::new("storybook-modal"),
        parent_rect,
        modal_size,
        display,
    )
    .resolve()
}

fn display_around_parent(parent_rect: WindowRect) -> DisplayBounds {
    DisplayBounds::new(
        "parent-display",
        WindowRect::new(
            WindowPoint::new(
                parent_rect.origin.x - DISPLAY_PADDING,
                parent_rect.origin.y - DISPLAY_PADDING,
            ),
            WindowSize::new(
                parent_rect.size.width + DISPLAY_PADDING * 2.0,
                parent_rect.size.height + DISPLAY_PADDING * 2.0,
            ),
        ),
        1.0,
    )
}

fn sample_modal_plan() -> Result<katana_ui_core::window::ModalWindowPlan, ModalWindowPlacementError>
{
    ModalWindowPlacement::same_display(
        WindowId::new("storybook-main"),
        WindowId::new("storybook-modal"),
        WindowRect::new(
            WindowPoint::new(SAMPLE_PARENT_X, SAMPLE_PARENT_Y),
            WindowSize::new(render::WIDTH as f32, render::HEIGHT as f32),
        ),
        WindowSize::new(modal::MODAL_WIDTH as f32, modal::MODAL_HEIGHT as f32),
        DisplayBounds::new(
            "main",
            WindowRect::new(
                WindowPoint::new(0.0, 0.0),
                WindowSize::new(SAMPLE_DISPLAY_WIDTH, SAMPLE_DISPLAY_HEIGHT),
            ),
            SAMPLE_DISPLAY_SCALE,
        ),
    )
    .resolve()
}

#[cfg(test)]
mod tests {
    use super::StorybookVisual;

    #[test]
    fn runtime_report_requires_state_overlay_and_modal_plan() {
        let report = StorybookVisual.runtime_report();

        assert!(report.state_reflected);
        assert!(report.overlay_rendered);
        assert!(report.modal_plan_same_display);
        assert!(report.modal_plan_frontmost);
    }
}
