use katana_ui_core_storybook::StorybookVisual;

#[test]
fn headless_keyboard_runtime_covers_focus_activation_modal_and_clipboard_routes() {
    assert!(StorybookVisual.dependency_runtime_report().passed());
    assert!(StorybookVisual.keyboard_runtime_report().passed());
    assert!(StorybookVisual.mouse_trace_runtime_report().passed());
}
