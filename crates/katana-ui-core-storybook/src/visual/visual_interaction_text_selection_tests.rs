use super::window_interaction::{
    StorybookWindowState, apply_text_copy_shortcut_for_audit, apply_text_selection_drag_for_audit,
    apply_text_selection_press_for_test, copy_selected_text_to_clipboard_for_frame,
};
use super::{StorybookVisual, live_interaction_audit};
use crate::catalog::StoryPresetLabels;
use crate::test_assert::KucTestExpect;
use crate::visual::text_selection::{TextSelection, copy_payload_for_selection};

const DARK_THEME: &str = "dark";
const PAGE: &str = "text";
const ROLE_GRID_PRESET: &str = "role grid";

#[test]
fn rendered_storybook_text_runs_are_selectable_and_copyable() {
    let canvas = StorybookVisual.render_preset(DARK_THEME, PAGE, preset_index(), 0);
    let run = heading_text_run(&canvas);
    let rect = run.rect();
    let payload = copy_payload_for_selection(
        canvas.text_runs(),
        TextSelection::drag((rect.x, rect.y), (rect.right(), rect.bottom())),
    );

    assert!(payload.contains("Heading"));
}

#[test]
fn interactive_page_text_does_not_start_display_text_selection() {
    let canvas = StorybookVisual.render_preset(DARK_THEME, "checkbox", 0, 0);
    let run = canvas
        .text_runs()
        .iter()
        .find(|run| run.text().contains("Checkbox"))
        .kuc_expect("checkbox page renders label text");
    let rect = run.rect();
    let mut state = StorybookWindowState {
        selected_page: "checkbox",
        ..StorybookWindowState::default()
    };

    assert!(!apply_text_selection_drag_for_audit(
        &mut state,
        &canvas,
        (rect.x, rect.y),
        (rect.right(), rect.bottom())
    ));
    assert_eq!(None, state.text_selection_start);
    assert_eq!(None, state.text_selection_end);
    assert_eq!("none", state.screen_state.last_action);
}

#[test]
fn live_audit_covers_text_selection_and_copy_for_text_page_only() {
    let scenarios = live_interaction_audit::text_selection_scenarios(PAGE);
    let has_drag = scenarios.iter().any(|scenario| {
        scenario.page == PAGE
            && scenario.operation == "text_drag_selection"
            && scenario.operation_kind == "drag"
            && scenario.action == "select_text"
            && scenario.event == "text_selection_changed"
            && scenario.passed
    });
    let has_copy = scenarios.iter().any(|scenario| {
        scenario.page == PAGE
            && scenario.operation == "text_keyboard_copy"
            && scenario.operation_kind == "keyboard"
            && scenario.action == "copy_selection"
            && scenario.event == "clipboard_copy"
            && scenario.passed
            && scenario.clipboard_text_len > 0
    });
    let has_zero_distance_noop = scenarios.iter().any(|scenario| {
        scenario.page == PAGE
            && scenario.operation == "text_zero_distance_drag_no_selection"
            && scenario.operation_kind == "drag"
            && scenario.action == "none"
            && scenario.event == "none"
            && scenario.passed
            && scenario.body_pixel_diff == 0
            && scenario.clipboard_text_len == 0
    });

    assert!(has_drag);
    assert!(has_copy);
    assert!(has_zero_distance_noop);
}

#[test]
fn live_audit_covers_text_paste_contract_for_text_surfaces() {
    let text_scenarios = live_interaction_audit::text_selection_scenarios("text");
    let text_input_scenarios = live_interaction_audit::text_selection_scenarios("text-input");
    let text_area_scenarios = live_interaction_audit::text_selection_scenarios("text-area");
    let text_paste_ignored = text_scenarios.iter().any(|scenario| {
        scenario.page == "text"
            && scenario.operation == "text_keyboard_paste"
            && scenario.operation_kind == "keyboard"
            && scenario.action == "none"
            && scenario.event == "none"
            && scenario.passed
    });
    let text_input_paste = text_input_scenarios.iter().any(|scenario| {
        scenario.page == "text-input"
            && scenario.operation == "text_keyboard_paste"
            && scenario.operation_kind == "keyboard"
            && scenario.action == "text_input_paste"
            && scenario.event == "clipboard_paste"
            && scenario.passed
    });
    let text_area_paste = text_area_scenarios.iter().any(|scenario| {
        scenario.page == "text-area"
            && scenario.operation == "text_keyboard_paste"
            && scenario.operation_kind == "keyboard"
            && scenario.action == "text_area_paste"
            && scenario.event == "clipboard_paste"
            && scenario.passed
    });

    assert!(text_paste_ignored, "display text must ignore paste");
    assert!(
        text_input_paste,
        "text-input must replace selection on paste"
    );
    assert!(
        text_area_paste,
        "text-area must replace grapheme selection on paste"
    );
}

#[test]
fn storybook_host_copies_selected_text_runs_to_clipboard_payload() {
    let canvas = StorybookVisual.render_preset(DARK_THEME, PAGE, preset_index(), 0);
    let rect = heading_text_run(&canvas).rect();
    let mut state = StorybookWindowState {
        text_selection_start: Some((rect.x, rect.y)),
        text_selection_end: Some((rect.right(), rect.bottom())),
        ..StorybookWindowState::default()
    };

    assert!(copy_selected_text_to_clipboard_for_frame(
        &mut state, &canvas
    ));
    assert!(state.clipboard_text.contains("Heading"));
}

#[test]
fn storybook_window_drag_selection_updates_state_and_copy_payload() {
    let canvas = StorybookVisual.render_preset(DARK_THEME, PAGE, preset_index(), 0);
    let rect = heading_text_run(&canvas).rect();
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    assert!(apply_text_selection_drag_for_audit(
        &mut state,
        &canvas,
        (rect.x, rect.y),
        (rect.right(), rect.bottom())
    ));
    assert_eq!(Some((rect.x, rect.y)), state.text_selection_start);
    assert_eq!(
        Some((rect.right(), rect.bottom())),
        state.text_selection_end
    );

    assert!(apply_text_copy_shortcut_for_audit(&mut state, &canvas));
    assert!(state.clipboard_text.contains("Heading"));
}

#[test]
fn storybook_window_single_text_press_does_not_create_selection_or_copy_payload() {
    let canvas = StorybookVisual.render_preset(DARK_THEME, PAGE, preset_index(), 0);
    let rect = heading_text_run(&canvas).rect();
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    assert!(apply_text_selection_press_for_test(
        &mut state, &canvas, rect.x, rect.y
    ));

    assert_eq!(Some((rect.x, rect.y)), state.text_selection_start);
    assert_eq!(None, state.text_selection_end);
    assert_eq!("none", state.screen_state.last_action);
    assert!(!copy_selected_text_to_clipboard_for_frame(
        &mut state, &canvas
    ));
}

#[test]
fn storybook_window_zero_distance_drag_does_not_create_selection_or_copy_payload() {
    let canvas = StorybookVisual.render_preset(DARK_THEME, PAGE, preset_index(), 0);
    let rect = heading_text_run(&canvas).rect();
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    assert!(!apply_text_selection_drag_for_audit(
        &mut state,
        &canvas,
        (rect.x, rect.y),
        (rect.x, rect.y)
    ));

    assert_eq!(Some((rect.x, rect.y)), state.text_selection_start);
    assert_eq!(None, state.text_selection_end);
    assert_eq!("none", state.screen_state.last_action);
    assert!(!copy_selected_text_to_clipboard_for_frame(
        &mut state, &canvas
    ));
}

fn heading_text_run(canvas: &super::Canvas) -> &super::text_selection::SelectableTextRun {
    canvas
        .text_runs()
        .iter()
        .find(|run| run.text().contains("Heading"))
        .kuc_expect("rendered Storybook text must expose selectable text runs")
}

fn preset_index() -> usize {
    StoryPresetLabels::for_page(PAGE)
        .iter()
        .position(|it| *it == ROLE_GRID_PRESET)
        .kuc_expect("text preset label must exist")
}
