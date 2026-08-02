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
    from_geometry(
        WindowPoint::new(x as f32, y as f32),
        WindowSize::new(main_frame.width() as f32, main_frame.height() as f32),
        WindowSize::new(modal_frame.width() as f32, modal_frame.height() as f32),
    )
}

fn from_geometry(
    parent_origin: WindowPoint,
    parent_size: WindowSize,
    modal_size: WindowSize,
) -> Result<ModalWindowPlan, ModalWindowPlacementError> {
    let parent_rect = WindowRect::new(parent_origin, parent_size);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_assert::KucTestExpect;

    #[test]
    fn geometry_plan_centers_modal_on_the_parent_display() {
        let plan = from_geometry(
            WindowPoint::new(320.0, 180.0),
            WindowSize::new(1280.0, 720.0),
            WindowSize::new(480.0, 320.0),
        )
        .kuc_expect("valid geometry must resolve");

        assert_eq!(WindowId::new("storybook-main"), plan.parent_window_id);
        assert_eq!(WindowId::new("storybook-modal"), plan.modal_window_id);
        assert!(plan.same_display);
        assert!(plan.frontmost);
        assert_eq!(720.0, plan.position.x);
        assert_eq!(380.0, plan.position.y);
    }

    #[test]
    fn display_bounds_include_parent_with_padding() {
        let parent = WindowRect::new(
            WindowPoint::new(100.0, 200.0),
            WindowSize::new(800.0, 600.0),
        );
        let display = display_around_parent(parent);

        assert_eq!("parent-display", display.name);
        assert_eq!(-156.0, display.rect.origin.x);
        assert_eq!(-56.0, display.rect.origin.y);
        assert_eq!(1312.0, display.rect.size.width);
        assert_eq!(1112.0, display.rect.size.height);
        assert_eq!(1.0, display.scale_factor);
    }
}
