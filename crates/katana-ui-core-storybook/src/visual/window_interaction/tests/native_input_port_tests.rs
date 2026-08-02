use super::super::{
    PanelScrollDragTarget, StorybookWindowInput, StorybookWindowState, TabsDragTarget,
    apply_mouse_click, apply_scroll, apply_text_paste_shortcut_for_audit,
    copy_selected_text_to_clipboard_for_frame, write_clipboard_payload,
};
use crate::test_assert::KucTestExpect;
use crate::visual::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use crate::visual::{
    Canvas, dedicated_dod_form_input_live, dedicated_tabs, panel_scrollbars, preview_detail,
};
use minifb::MouseButton;

const SURFACE_SIZE: (usize, usize) = (1440, 920);

#[derive(Debug, Clone)]
struct FakeWindowInput {
    scroll: Option<(f32, f32)>,
    mouse: Option<(f32, f32)>,
    left_down: bool,
    right_down: bool,
    size: (usize, usize),
}

impl Default for FakeWindowInput {
    fn default() -> Self {
        Self {
            scroll: None,
            mouse: Some((10.0, 10.0)),
            left_down: false,
            right_down: false,
            size: SURFACE_SIZE,
        }
    }
}

impl StorybookWindowInput for FakeWindowInput {
    fn scroll_wheel(&self) -> Option<(f32, f32)> {
        self.scroll
    }

    fn mouse_position(&self) -> Option<(f32, f32)> {
        self.mouse
    }

    fn mouse_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.left_down,
            MouseButton::Right => self.right_down,
            MouseButton::Middle => false,
        }
    }

    fn surface_size(&self) -> (usize, usize) {
        self.size
    }
}

#[test]
fn scroll_input_port_covers_absent_zero_pointerless_invalid_and_valid_input() {
    let mut state = StorybookWindowState::default();
    assert!(!apply_scroll(&FakeWindowInput::default(), &mut state));
    assert!(!apply_scroll(
        &FakeWindowInput {
            scroll: Some((0.0, 0.0)),
            ..FakeWindowInput::default()
        },
        &mut state,
    ));
    let _ = apply_scroll(
        &FakeWindowInput {
            scroll: Some((0.0, -1.0)),
            mouse: None,
            ..FakeWindowInput::default()
        },
        &mut state,
    );
    assert!(!apply_scroll(
        &FakeWindowInput {
            scroll: Some((1.0, -1.0)),
            mouse: Some((-1.0, -1.0)),
            ..FakeWindowInput::default()
        },
        &mut state,
    ));
    let _ = apply_scroll(
        &FakeWindowInput {
            scroll: Some((1.0, -1.0)),
            ..FakeWindowInput::default()
        },
        &mut state,
    );
}

#[test]
fn mouse_input_port_covers_missing_invalid_idle_and_release_paths() {
    let frame = Canvas::new(SURFACE_SIZE.0, SURFACE_SIZE.1, 0);
    let mut left_was_down = false;
    let mut right_was_down = false;
    let mut state = StorybookWindowState::default();
    assert!(!apply_mouse_click(
        &FakeWindowInput {
            mouse: None,
            ..FakeWindowInput::default()
        },
        &mut state,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    ));
    assert!(!apply_mouse_click(
        &FakeWindowInput {
            mouse: Some((-1.0, -1.0)),
            ..FakeWindowInput::default()
        },
        &mut state,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    ));
    assert!(!apply_mouse_click(
        &FakeWindowInput::default(),
        &mut state,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    ));

    state.tabs_drag_target = Some(TabsDragTarget {
        tab_id: "tab-a".to_string(),
        committed: false,
    });
    assert!(apply_mouse_click(
        &FakeWindowInput::default(),
        &mut state,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    ));

    state.screen_state.button_pressed = true;
    assert!(apply_mouse_click(
        &FakeWindowInput::default(),
        &mut state,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    ));
}

#[test]
fn mouse_input_port_covers_active_drag_resize_context_and_click_paths() {
    let frame = Canvas::new(SURFACE_SIZE.0, SURFACE_SIZE.1, 0);
    let held = FakeWindowInput {
        left_down: true,
        ..FakeWindowInput::default()
    };

    {
        let target = PanelScrollDragTarget::Vertical(PanelScrollRegion::Navigation);
        let mut state = StorybookWindowState {
            drag_scroll_target: Some(target),
            ..StorybookWindowState::default()
        };
        let mut left_was_down = true;
        let mut right_was_down = false;
        let _ = apply_mouse_click(
            &held,
            &mut state,
            &frame,
            &mut left_was_down,
            &mut right_was_down,
        );
    }

    let mut tabs = StorybookWindowState {
        selected_page: "tabs",
        tabs_drag_target: Some(TabsDragTarget {
            tab_id: "tab-a".to_string(),
            committed: false,
        }),
        ..StorybookWindowState::default()
    };
    let mut left_was_down = true;
    let mut right_was_down = false;
    let _ = apply_mouse_click(
        &held,
        &mut tabs,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    );

    let mut resizing = StorybookWindowState {
        selected_page: "text-area",
        text_area_resize_dragging: true,
        ..StorybookWindowState::default()
    };
    let _ = apply_mouse_click(
        &held,
        &mut resizing,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    );

    let mut context = StorybookWindowState::default();
    let mut left_was_down = false;
    let mut right_was_down = false;
    let _ = apply_mouse_click(
        &FakeWindowInput {
            right_down: true,
            ..FakeWindowInput::default()
        },
        &mut context,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    );

    let mut click = StorybookWindowState::default();
    let _ = apply_mouse_click(
        &held,
        &mut click,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    );
}

#[test]
fn clipboard_copy_rejects_missing_and_empty_rendered_selections() {
    let frame = Canvas::new(40, 20, 0);
    let mut state = StorybookWindowState::default();
    assert!(!copy_selected_text_to_clipboard_for_frame(
        &mut state, &frame
    ));

    state.text_selection_start = Some((0, 0));
    state.text_selection_end = Some((20, 10));
    assert!(!copy_selected_text_to_clipboard_for_frame(
        &mut state, &frame
    ));
    assert!(!apply_text_paste_shortcut_for_audit(&mut state));

    fn failing_writer(_payload: &str) -> Result<(), std::io::Error> {
        Err(std::io::Error::other("clipboard unavailable"))
    }
    write_clipboard_payload("selected", Some(failing_writer));
}

#[test]
fn mouse_input_port_starts_vertical_drag_resize_tabs_and_text_selection() {
    let frame = Canvas::new(SURFACE_SIZE.0, SURFACE_SIZE.1, 0);
    let vertical_thumb = panel_scrollbars::thumb_rect_for(
        PanelScrollRegion::Navigation,
        PanelScrollOffsets::default(),
    );
    let vertical = held_at(
        vertical_thumb.x + vertical_thumb.width / 2,
        vertical_thumb.y + vertical_thumb.height / 2,
    );
    let mut state = StorybookWindowState::default();
    assert!(press(&vertical, &mut state, &frame));
    assert_eq!(
        Some(PanelScrollDragTarget::Vertical(
            PanelScrollRegion::Navigation
        )),
        state.drag_scroll_target
    );

    let mut resize_state = StorybookWindowState {
        selected_page: "text-area",
        preset_index: 3,
        ..StorybookWindowState::default()
    };
    let resize_origin = preview_detail::component_action_hit_rect("text-area");
    let resize_grip = dedicated_dod_form_input_live::text_area_resize_grip_rect_for_instance(
        resize_origin.x,
        resize_origin.y,
        resize_state.preset_index,
        &resize_state.screen_state,
        super::super::component_instance_id_for_page(
            resize_state.selected_page,
            resize_state.selected_instance_id,
        ),
    )
    .kuc_expect("resize preset must expose a grip");
    let _ = press(
        &held_at(resize_grip.x + 1, resize_grip.y + 1),
        &mut resize_state,
        &frame,
    );
    assert!(resize_state.text_area_resize_dragging);

    let mut tabs_state = StorybookWindowState {
        selected_page: "tabs",
        ..StorybookWindowState::default()
    };
    let tabs_origin = preview_detail::component_action_hit_rect("tabs");
    let tab = dedicated_tabs::tab_rect_for_test(&tabs_state.screen_state.tabs, "readme.md")
        .kuc_expect("default tabs must include readme");
    assert!(press(
        &held_at(
            tabs_origin.x + tab.x + tab.width / 2,
            tabs_origin.y + tab.y + tab.height / 2,
        ),
        &mut tabs_state,
        &frame,
    ));
    assert!(tabs_state.tabs_drag_target.is_some());

    let mut text_frame = Canvas::new(SURFACE_SIZE.0, SURFACE_SIZE.1, 0);
    text_frame.record_text_run("selectable", 20, 20, 120, 24);
    let mut text_state = StorybookWindowState {
        selected_page: "text",
        ..StorybookWindowState::default()
    };
    assert!(press(&held_at(24, 24), &mut text_state, &text_frame,));
    assert_eq!(Some((24, 24)), text_state.text_selection_start);
}

#[test]
fn mouse_input_port_covers_text_selection_release_and_empty_drag_boundaries() {
    let frame = Canvas::new(SURFACE_SIZE.0, SURFACE_SIZE.1, 0);
    let mut released = StorybookWindowState {
        selected_page: "text",
        text_selection_start: Some((10, 10)),
        ..StorybookWindowState::default()
    };
    let mut left_was_down = true;
    let mut right_was_down = false;
    assert!(!apply_mouse_click(
        &FakeWindowInput::default(),
        &mut released,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    ));
    assert_eq!(None, released.text_selection_start);

    let mut empty_drag = StorybookWindowState {
        selected_page: "text",
        ..StorybookWindowState::default()
    };
    left_was_down = true;
    assert!(!apply_mouse_click(
        &held_at(30, 30),
        &mut empty_drag,
        &frame,
        &mut left_was_down,
        &mut right_was_down,
    ));
}

fn held_at(x: usize, y: usize) -> FakeWindowInput {
    FakeWindowInput {
        mouse: Some((x as f32, y as f32)),
        left_down: true,
        ..FakeWindowInput::default()
    }
}

fn press(window: &FakeWindowInput, state: &mut StorybookWindowState, frame: &Canvas) -> bool {
    let mut left_was_down = false;
    let mut right_was_down = false;
    apply_mouse_click(
        window,
        state,
        frame,
        &mut left_was_down,
        &mut right_was_down,
    )
}
