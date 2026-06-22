use super::window_interaction::{StorybookWindowState, apply_text_paste_shortcut_for_audit};

#[test]
fn storybook_window_pastes_clipboard_text_into_focused_text_input_selection() {
    let mut state = StorybookWindowState {
        selected_page: "text-input",
        clipboard_text: "ZZ".to_string(),
        ..StorybookWindowState::default()
    };
    state
        .screen_state
        .register_text_input_focus_for("text-input.preview", "abcdef", false);
    state
        .screen_state
        .set_text_input_selection_for_test("text-input.preview", 1, 4);

    assert!(apply_text_paste_shortcut_for_audit(&mut state));
    assert_eq!("aZZef", state.screen_state.text_input_value());
    assert_eq!("text_input_paste", state.screen_state.last_action);
    assert_eq!("clipboard_paste", state.screen_state.last_event);
}

#[test]
fn storybook_window_pastes_clipboard_text_into_focused_text_area_selection() {
    let mut state = StorybookWindowState {
        selected_page: "text-area",
        clipboard_text: "文".to_string(),
        ..StorybookWindowState::default()
    };
    state
        .screen_state
        .register_text_area_focus_for("text-area.preview", false, false);
    state
        .screen_state
        .set_text_area_value_for_test("text-area.preview", "A日🔷b");
    state
        .screen_state
        .set_text_area_selection_for_test("text-area.preview", 1, 3);

    assert!(apply_text_paste_shortcut_for_audit(&mut state));
    assert_eq!("A文b", state.screen_state.text_area_value());
    assert_eq!("text_area_paste", state.screen_state.last_action);
    assert_eq!("clipboard_paste", state.screen_state.last_event);
}
