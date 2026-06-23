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
    let mut frame_index = 0;
    while frames == 0 || frame_index < frames {
        if should_close_window_pair(
            main.is_open(),
            modal_window.is_open(),
            main.is_key_down(Key::Escape),
            modal_window.is_key_down(Key::Escape),
        ) {
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
    use super::should_close_window_pair;

    #[test]
    fn closes_when_modal_window_receives_escape() {
        assert!(should_close_window_pair(true, true, false, true));
    }

    #[test]
    fn stays_open_while_both_windows_are_open_without_escape() {
        assert!(!should_close_window_pair(true, true, false, false));
    }
}
