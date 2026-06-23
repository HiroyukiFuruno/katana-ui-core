use super::canvas::Canvas;
use super::modal;
use super::render;
use katana_ui_core::window::{
    DisplayBounds, ModalWindowPlacement, ModalWindowPlacementError, ModalWindowPlan, WindowId,
    WindowPoint, WindowRect, WindowSize,
};
use minifb::Window;

const DISPLAY_PADDING: f32 = 256.0;
const SAMPLE_PARENT_X: f32 = 80.0;
const SAMPLE_PARENT_Y: f32 = 80.0;
const SAMPLE_DISPLAY_WIDTH: f32 = 1920.0;
const SAMPLE_DISPLAY_HEIGHT: f32 = 1200.0;
const SAMPLE_DISPLAY_SCALE: f32 = 2.0;

pub(super) fn from_main_window(
    main: &Window,
    main_frame: &Canvas,
    modal_frame: &Canvas,
) -> Result<ModalWindowPlan, ModalWindowPlacementError> {
    let (x, y) = main.get_position();
    let parent_rect = WindowRect::new(
        WindowPoint::new(x as f32, y as f32),
        WindowSize::new(main_frame.width() as f32, main_frame.height() as f32),
    );
    let modal_size = WindowSize::new(modal_frame.width() as f32, modal_frame.height() as f32);
    ModalWindowPlacement::same_display(
        WindowId::new("storybook-main"),
        WindowId::new("storybook-modal"),
        parent_rect,
        modal_size,
        display_around_parent(parent_rect),
    )
    .resolve()
}

pub(super) fn sample() -> Result<ModalWindowPlan, ModalWindowPlacementError> {
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
