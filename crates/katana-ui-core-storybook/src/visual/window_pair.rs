use super::canvas::Canvas;
use super::render;
use minifb::{Key, Window};
use std::thread;
use std::time::Duration;

pub(super) fn run_window_pair(
    main: &mut Window,
    main_frame: &Canvas,
    modal_window: &mut Window,
    modal_frame: &Canvas,
    frames: usize,
) -> Result<(), minifb::Error> {
    let mut sleep = thread::sleep;
    run_window_pair_with(
        main,
        main_frame,
        modal_window,
        modal_frame,
        frames,
        &mut sleep,
    )
}

trait WindowPairPort {
    fn is_open(&self) -> bool;
    fn escape_down(&self) -> bool;
    fn update(&mut self, frame: &Canvas) -> Result<(), minifb::Error>;
}

impl WindowPairPort for Window {
    fn is_open(&self) -> bool {
        self.is_open()
    }

    fn escape_down(&self) -> bool {
        self.is_key_down(Key::Escape)
    }

    fn update(&mut self, frame: &Canvas) -> Result<(), minifb::Error> {
        self.update_with_buffer(frame.pixels(), frame.width(), frame.height())
    }
}

fn run_window_pair_with(
    main: &mut dyn WindowPairPort,
    main_frame: &Canvas,
    modal_window: &mut dyn WindowPairPort,
    modal_frame: &Canvas,
    frames: usize,
    sleep: &mut dyn FnMut(Duration),
) -> Result<(), minifb::Error> {
    let mut frame_index = 0;
    while frames == 0 || frame_index < frames {
        if should_close_window_pair(
            main.is_open(),
            modal_window.is_open(),
            main.escape_down(),
            modal_window.escape_down(),
        ) {
            break;
        }
        main.update(main_frame)?;
        modal_window.update(modal_frame)?;
        sleep(Duration::from_millis(render::FRAME_DELAY_MS));
        frame_index += 1;
    }
    Ok(())
}

fn should_close_window_pair(
    main_open: bool,
    modal_open: bool,
    main_escape_down: bool,
    modal_escape_down: bool,
) -> bool {
    !main_open || !modal_open || main_escape_down || modal_escape_down
}

#[cfg(test)]
mod tests {
    use super::{Canvas, WindowPairPort, run_window_pair_with, should_close_window_pair};
    use crate::test_assert::KucTestExpect;

    #[derive(Default)]
    struct FakeWindow {
        open: bool,
        escape: bool,
        updates: usize,
        fail_at: Option<usize>,
    }

    impl WindowPairPort for FakeWindow {
        fn is_open(&self) -> bool {
            self.open
        }

        fn escape_down(&self) -> bool {
            self.escape
        }

        fn update(&mut self, _frame: &Canvas) -> Result<(), minifb::Error> {
            self.updates += 1;
            if self.fail_at == Some(self.updates) {
                return Err(minifb::Error::WindowCreate("update failed".to_string()));
            }
            Ok(())
        }
    }

    #[test]
    fn closes_when_modal_window_receives_escape() {
        assert!(should_close_window_pair(true, true, false, true));
    }

    #[test]
    fn stays_open_while_both_windows_are_open_without_escape() {
        assert!(!should_close_window_pair(true, true, false, false));
    }

    #[test]
    fn pair_loop_updates_both_windows_for_the_requested_frames() {
        let frame = Canvas::new(2, 2, 0);
        let mut main = FakeWindow {
            open: true,
            ..FakeWindow::default()
        };
        let mut modal = FakeWindow {
            open: true,
            ..FakeWindow::default()
        };
        let mut sleeps = 0;
        let mut sleep = |_| sleeps += 1;

        run_window_pair_with(&mut main, &frame, &mut modal, &frame, 3, &mut sleep)
            .kuc_expect("fake windows must update");

        assert_eq!(3, main.updates);
        assert_eq!(3, modal.updates);
        assert_eq!(3, sleeps);
    }

    #[test]
    fn pair_loop_stops_on_closed_window_and_propagates_update_error() {
        let frame = Canvas::new(2, 2, 0);
        let mut closed = FakeWindow::default();
        let mut modal = FakeWindow {
            open: true,
            ..FakeWindow::default()
        };
        let mut sleep = std::thread::sleep;
        run_window_pair_with(&mut closed, &frame, &mut modal, &frame, 0, &mut sleep)
            .kuc_expect("closed window must be a clean stop");
        assert_eq!(0, closed.updates);

        let mut main = FakeWindow {
            open: true,
            fail_at: Some(1),
            ..FakeWindow::default()
        };
        assert!(
            run_window_pair_with(&mut main, &frame, &mut modal, &frame, 1, &mut sleep).is_err()
        );
    }
}
