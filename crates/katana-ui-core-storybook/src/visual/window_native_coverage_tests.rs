use super::{
    StorybookCursorPort, StorybookKeyboardInput, StorybookVisual, StorybookVisualError,
    StorybookWindowInput,
};
use crate::visual::window_mouse_trace::MouseTraceWindow;
use crate::visual::window_options::main_window_options;
use minifb::{CursorStyle, Key, MouseButton, Window};

#[test]
#[ignore = "requires a Linux Xvfb display"]
fn native_xvfb_window_paths_cover_minifb_adapters() -> Result<(), StorybookVisualError> {
    let mut window = Window::new("KUC native coverage", 64, 64, main_window_options())?;

    let _ = StorybookWindowInput::scroll_wheel(&window);
    let _ = StorybookWindowInput::mouse_position(&window);
    let _ = StorybookWindowInput::mouse_down(&window, MouseButton::Left);
    assert_eq!((64, 64), StorybookWindowInput::surface_size(&window));
    let _ = StorybookKeyboardInput::key_down(&window, Key::Escape);
    let _ = StorybookKeyboardInput::keys_pressed(&window);
    let _ = MouseTraceWindow::mouse_pos(&window);
    assert_eq!((64, 64), MouseTraceWindow::size(&window));
    let _ = MouseTraceWindow::left_down(&window);
    StorybookCursorPort::set_fallback_cursor(&mut window, CursorStyle::Arrow);
    StorybookCursorPort::set_pointing_hand_cursor(&mut window);
    drop(window);

    StorybookVisual.open_window(1)?;
    StorybookVisual.open_window_for_page(1, "button")?;
    let modal = StorybookVisual.open_modal_window(1)?;
    assert!(modal.modal_window_opened);
    assert!(modal.same_display);
    assert!(modal.frontmost);
    Ok(())
}
