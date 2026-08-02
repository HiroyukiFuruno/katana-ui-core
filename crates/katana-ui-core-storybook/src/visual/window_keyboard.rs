use super::canvas::Canvas;
use super::dedicated_breadcrumb;
use super::dedicated_chip;
use super::dedicated_context_menu_popup;
use super::dedicated_dod_form_binary_choice_live;
use super::dedicated_dod_molecule_menu;
use super::dedicated_menu_button;
use super::dedicated_status_bar;
use super::dedicated_tabs_metrics::{STRIP_LEADING_INSET, STRIP_X, TAB_Y};
use super::dedicated_tooltip;
use super::preview_detail;
use super::runtime::StorybookKeyboardRuntimeReport;
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit,
    apply_clipboard_paste_text, apply_tabs_keyboard_shortcut, apply_text_area_key,
    apply_text_input_key, copy_selected_text_to_clipboard_for_frame, focus_clickable_at_for_audit,
    focus_tabs_at_for_audit,
};
use minifb::{Key, KeyRepeat, Window};

#[path = "window_keyboard_keymap.rs"]
mod keymap;

use keymap::{tabs_keyboard_shortcut, text_area_key, text_input_key};

const FOCUS_OFFSET: usize = 4;

pub(super) trait StorybookKeyboardInput {
    fn key_down(&self, key: Key) -> bool;
    fn keys_pressed(&self) -> Vec<Key>;
}

impl StorybookKeyboardInput for Window {
    fn key_down(&self, key: Key) -> bool {
        self.is_key_down(key)
    }

    fn keys_pressed(&self) -> Vec<Key> {
        self.get_keys_pressed(KeyRepeat::Yes)
    }
}

pub(super) fn apply_keyboard(
    window: &dyn StorybookKeyboardInput,
    state: &mut StorybookWindowState,
    frame: &Canvas,
) -> bool {
    let shifted = window.key_down(Key::LeftShift) || window.key_down(Key::RightShift);
    let command_or_control = window.key_down(Key::LeftCtrl)
        || window.key_down(Key::RightCtrl)
        || window.key_down(Key::LeftSuper)
        || window.key_down(Key::RightSuper);
    let mut changed = false;
    for key in window.keys_pressed() {
        changed |= apply_keyboard_key(key, command_or_control, shifted, state, frame);
    }
    changed
}

struct RuntimeKeyboardInput {
    pressed: Key,
}

impl StorybookKeyboardInput for RuntimeKeyboardInput {
    fn key_down(&self, _key: Key) -> bool {
        false
    }

    fn keys_pressed(&self) -> Vec<Key> {
        vec![self.pressed]
    }
}

pub(super) fn runtime_report() -> StorybookKeyboardRuntimeReport {
    let frame = Canvas::new(1, 1, 0);
    let mut checkbox = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };
    let checkbox_focused = apply_keyboard_key(Key::Tab, false, false, &mut checkbox, &frame);
    let checkbox_toggled = apply_keyboard(
        &RuntimeKeyboardInput {
            pressed: Key::Space,
        },
        &mut checkbox,
        &frame,
    );

    let mut modal = StorybookWindowState {
        selected_page: "modal",
        ..StorybookWindowState::default()
    };
    let _ = apply_keyboard_key(Key::Tab, false, false, &mut modal, &frame);
    let modal_closed = apply_keyboard_key(Key::Escape, false, false, &mut modal, &frame);

    let unhandled_key_ignored = !apply_keyboard(
        &RuntimeKeyboardInput { pressed: Key::F12 },
        &mut checkbox,
        &frame,
    );
    let unavailable_clipboard_ignored =
        unhandled_key_ignored && !apply_keyboard_key(Key::V, true, false, &mut checkbox, &frame);

    StorybookKeyboardRuntimeReport {
        checkbox_focused,
        checkbox_toggled,
        modal_closed,
        unavailable_clipboard_ignored,
    }
}

fn apply_keyboard_key(
    key: Key,
    command_or_control: bool,
    shifted: bool,
    state: &mut StorybookWindowState,
    frame: &Canvas,
) -> bool {
    let mut clipboard_text = read_clipboard_text;
    apply_keyboard_key_with_clipboard(
        key,
        command_or_control,
        shifted,
        state,
        frame,
        &mut clipboard_text,
    )
}

fn apply_keyboard_key_with_clipboard(
    key: Key,
    command_or_control: bool,
    shifted: bool,
    state: &mut StorybookWindowState,
    frame: &Canvas,
    clipboard_text: &mut dyn FnMut() -> Option<String>,
) -> bool {
    if let Some(shortcut) = tabs_keyboard_shortcut(key, command_or_control, shifted)
        && apply_tabs_keyboard_shortcut(state, shortcut)
    {
        return true;
    }
    if command_or_control {
        if key == Key::C && copy_selected_text_to_clipboard_for_frame(state, frame) {
            return true;
        }
        if key == Key::V
            && let Some(text) = clipboard_text()
            && apply_clipboard_paste_text(state, text.as_str())
        {
            return true;
        }
        return false;
    }
    if key == Key::Tab {
        return focus_default_clickable(state);
    }
    if state.selected_page == "text-area"
        && let Some(input) = text_area_key(key, shifted)
        && apply_text_area_key(state, input)
    {
        return true;
    }
    if let Some(input) = text_input_key(key, shifted)
        && apply_text_input_key(state, input)
    {
        return true;
    }
    if is_clickable_keyboard_activation_key(key) {
        return apply_clickable_keyboard_activation_for_audit(state);
    }
    false
}

fn focus_default_clickable(state: &mut StorybookWindowState) -> bool {
    let target = preview_detail::component_action_hit_rect(state.selected_page);
    if matches!(state.selected_page, "checkbox" | "radio") {
        let row = dedicated_dod_form_binary_choice_live::row_rect(0, target.x, target.y);
        return focus_clickable_at_for_audit(state, row.x + FOCUS_OFFSET, row.y + FOCUS_OFFSET);
    }
    if state.selected_page == "tooltip" {
        let anchor = dedicated_tooltip::anchor_hit_rect(state.preset_index);
        return focus_clickable_at_for_audit(
            state,
            anchor.x + FOCUS_OFFSET,
            anchor.y + FOCUS_OFFSET,
        );
    }
    if state.selected_page == "chip" {
        return focus_clickable_at_for_audit(
            state,
            target.x + dedicated_chip::CHIP_X + FOCUS_OFFSET,
            target.y + dedicated_chip::CHIP_Y + FOCUS_OFFSET,
        );
    }
    if state.selected_page == "menu" {
        let row = dedicated_dod_molecule_menu::first_row_rect(target);
        return focus_clickable_at_for_audit(state, row.x + FOCUS_OFFSET, row.y + FOCUS_OFFSET);
    }
    if state.selected_page == "context-menu" {
        let row = dedicated_context_menu_popup::insert_row_rect(target.x, target.y);
        return focus_clickable_at_for_audit(state, row.x + FOCUS_OFFSET, row.y + FOCUS_OFFSET);
    }
    if state.selected_page == "breadcrumb" {
        let crumb = dedicated_breadcrumb::file_crumb_rect(target.x, target.y);
        return focus_clickable_at_for_audit(state, crumb.x + FOCUS_OFFSET, crumb.y + FOCUS_OFFSET);
    }
    if state.selected_page == "menu-button" {
        let trigger = dedicated_menu_button::trigger_rect(target);
        return focus_clickable_at_for_audit(
            state,
            trigger.x + FOCUS_OFFSET,
            trigger.y + FOCUS_OFFSET,
        );
    }
    if state.selected_page == "status-bar"
        && let Some(segment) = dedicated_status_bar::segment_rect(0)
    {
        return focus_clickable_at_for_audit(
            state,
            target.x + segment.x + FOCUS_OFFSET,
            target.y + segment.y + FOCUS_OFFSET,
        );
    }
    if state.selected_page == "tabs" || state.selected_page == "closeable-tab-strip" {
        let x = target.x + STRIP_X + STRIP_LEADING_INSET + FOCUS_OFFSET;
        let y = target.y + TAB_Y + FOCUS_OFFSET;
        return focus_tabs_at_for_audit(state, x, y);
    }
    focus_clickable_at_for_audit(state, target.x + FOCUS_OFFSET, target.y + FOCUS_OFFSET)
}

fn is_clickable_keyboard_activation_key(key: Key) -> bool {
    matches!(
        key,
        Key::Space | Key::Enter | Key::NumPadEnter | Key::Escape
    )
}

#[cfg(all(not(test), target_os = "macos"))]
fn read_clipboard_text() -> Option<String> {
    match crate::system::ProcessCommand::read_stdout("pbpaste") {
        Ok(text) => Some(text),
        Err(error) => {
            eprintln!("[katana-ui-core-storybook] clipboard read failed: {error}");
            None
        }
    }
}

#[cfg(any(test, not(target_os = "macos")))]
fn read_clipboard_text() -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::{
        StorybookKeyboardInput, apply_keyboard, apply_keyboard_key,
        apply_keyboard_key_with_clipboard, focus_default_clickable, read_clipboard_text,
        runtime_report,
    };
    use crate::visual::canvas::Canvas;
    use crate::visual::window_interaction::StorybookWindowState;
    use minifb::Key;

    struct FakeKeyboard {
        down: Vec<Key>,
        pressed: Vec<Key>,
    }

    impl StorybookKeyboardInput for FakeKeyboard {
        fn key_down(&self, key: Key) -> bool {
            self.down.contains(&key)
        }

        fn keys_pressed(&self) -> Vec<Key> {
            self.pressed.clone()
        }
    }

    #[test]
    fn headless_keyboard_runtime_report_passes() {
        assert!(runtime_report().passed());
    }

    #[test]
    fn keyboard_input_port_routes_pressed_keys_and_modifiers() {
        let keyboard = FakeKeyboard {
            down: vec![Key::LeftShift],
            pressed: vec![Key::Tab, Key::Space],
        };
        let mut state = StorybookWindowState {
            selected_page: "checkbox",
            ..StorybookWindowState::default()
        };

        assert!(apply_keyboard(&keyboard, &mut state, &Canvas::new(1, 1, 0)));
        assert!(state.screen_state.is_checkbox_checked_at(0));
    }

    #[test]
    fn command_copy_and_injected_paste_route_through_keyboard_contract() {
        let mut frame = Canvas::new(100, 40, 0);
        frame.record_text_run("selected", 0, 0, 80, 20);
        let mut state = StorybookWindowState {
            selected_page: "text",
            text_selection_start: Some((0, 0)),
            text_selection_end: Some((80, 20)),
            ..StorybookWindowState::default()
        };
        assert!(apply_keyboard_key_with_clipboard(
            Key::C,
            true,
            false,
            &mut state,
            &frame,
            &mut || None
        ));
        assert_eq!("selected", state.clipboard_text);

        state.selected_page = "text-input";
        let instance = crate::visual::window_interaction::component_instance_id_for_page(
            state.selected_page,
            state.selected_instance_id,
        );
        state
            .screen_state
            .register_text_input_focus_for(instance, "", false);
        assert!(apply_keyboard_key_with_clipboard(
            Key::V,
            true,
            false,
            &mut state,
            &frame,
            &mut || Some("pasted".to_string())
        ));
        assert!(state.screen_state.text_input_value().contains("pasted"));
    }

    #[test]
    fn keyboard_key_routes_text_area_text_input_tabs_and_unhandled_keys() {
        let frame = Canvas::new(1, 1, 0);
        for page in ["text-area", "text-input", "tabs"] {
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };
            if page == "tabs" {
                assert!(apply_keyboard_key(
                    Key::Tab,
                    false,
                    false,
                    &mut state,
                    &frame
                ));
            } else {
                let instance = crate::visual::window_interaction::component_instance_id_for_page(
                    state.selected_page,
                    state.selected_instance_id,
                );
                if page == "text-area" {
                    state
                        .screen_state
                        .register_text_area_focus_for(instance, false, false);
                } else {
                    state
                        .screen_state
                        .register_text_input_focus_for(instance, "", false);
                }
                assert!(apply_keyboard_key(Key::A, false, false, &mut state, &frame));
            }
        }

        let mut state = StorybookWindowState::default();
        assert!(!apply_keyboard_key(
            Key::F12,
            false,
            false,
            &mut state,
            &frame
        ));
        assert!(!apply_keyboard_key_with_clipboard(
            Key::V,
            true,
            false,
            &mut state,
            &frame,
            &mut || None
        ));
    }

    #[test]
    fn focused_checkbox_space_uses_native_keyboard_activation_path() {
        let mut state = StorybookWindowState {
            selected_page: "checkbox",
            ..StorybookWindowState::default()
        };
        let frame = Canvas::new(1, 1, 0);

        assert!(apply_keyboard_key(
            Key::Tab,
            false,
            false,
            &mut state,
            &frame
        ));

        assert!(apply_keyboard_key(
            Key::Space,
            false,
            false,
            &mut state,
            &frame
        ));

        assert_eq!("checkbox_keyboard_toggle", state.screen_state.last_action);
        assert_eq!("checked_changed", state.screen_state.last_event);
        assert_eq!("before=false after=true", state.screen_state.state_label);
        assert!(state.screen_state.is_checkbox_checked_at(0));
    }

    #[test]
    fn focused_modal_escape_uses_native_keyboard_close_path() {
        let mut state = StorybookWindowState {
            selected_page: "modal",
            ..StorybookWindowState::default()
        };
        let frame = Canvas::new(1, 1, 0);

        assert!(apply_keyboard_key(
            Key::Tab,
            false,
            false,
            &mut state,
            &frame
        ));

        assert!(apply_keyboard_key(
            Key::Escape,
            false,
            false,
            &mut state,
            &frame
        ));

        assert_eq!("modal_escape", state.screen_state.last_action);
        assert_eq!("modal_closed", state.screen_state.last_event);
        assert_eq!("open=false", state.screen_state.state_label);
    }

    #[test]
    fn closeable_tab_strip_tab_key_focuses_visible_tab_hit_target() {
        let mut state = StorybookWindowState {
            selected_page: "closeable-tab-strip",
            ..StorybookWindowState::default()
        };
        let frame = Canvas::new(1, 1, 0);

        assert!(apply_keyboard_key(
            Key::Tab,
            false,
            false,
            &mut state,
            &frame
        ));

        assert_eq!("tab_focus", state.screen_state.last_action);
        assert_eq!("closeable_tab_focused", state.screen_state.last_event);
        assert_eq!("tabs.focus=tab", state.screen_state.state_label);
    }

    #[test]
    fn tab_key_focuses_page_specific_clickable_targets() {
        for (page, expected_action, expected_event, expected_state) in [
            ("chip", "chip_focus", "chip_focused", "focused=true"),
            ("menu", "menu_focus", "menu_focused", "focused=true"),
            (
                "context-menu",
                "context_menu_focus",
                "context_menu_focused",
                "focused=true",
            ),
            (
                "breadcrumb",
                "breadcrumb_focus",
                "breadcrumb_focused",
                "route=2",
            ),
            (
                "menu-button",
                "menu_button_focus",
                "menu_button_focused",
                "focused=true",
            ),
            (
                "status-bar",
                "status_bar_segment_focus",
                "focus",
                "focus=branch",
            ),
        ] {
            let mut state = StorybookWindowState {
                selected_page: page,
                ..StorybookWindowState::default()
            };
            let frame = Canvas::new(1, 1, 0);

            assert!(
                apply_keyboard_key(Key::Tab, false, false, &mut state, &frame),
                "{page} should handle Tab focus"
            );

            assert_eq!(expected_action, state.screen_state.last_action, "{page}");
            assert_eq!(expected_event, state.screen_state.last_event, "{page}");
            assert_eq!(expected_state, state.screen_state.state_label, "{page}");
        }
    }

    #[test]
    fn tabs_shortcut_tooltip_focus_and_test_clipboard_cover_native_boundaries() {
        let frame = Canvas::new(1, 1, 0);
        let mut tabs = StorybookWindowState {
            selected_page: "tabs",
            ..StorybookWindowState::default()
        };
        assert!(apply_keyboard_key(Key::Tab, true, false, &mut tabs, &frame));

        let mut tooltip = StorybookWindowState {
            selected_page: "tooltip",
            ..StorybookWindowState::default()
        };
        assert!(focus_default_clickable(&mut tooltip));
        assert!(read_clipboard_text().is_none());
    }
}
