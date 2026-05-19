use katana_ui_core_storybook::StoryCatalog;

const REQUIRED_INTERACTIVE_MOLECULES: [(&str, &str); 15] = [
    ("card", "click"),
    ("tooltip", "hover_start"),
    ("modal", "modal_escape"),
    ("accordion", "accordion_toggle"),
    ("combo-box", "select_box_selected"),
    ("menu-button", "select_box_selected"),
    ("notification-toast", "dismiss"),
    ("popover", "modal_escape"),
    ("search-box", "search_submitted"),
    ("segmented-toggle", "segmented_toggle_selected"),
    ("select-box", "select_box_selected"),
    ("modal-overlay", "modal_escape"),
    ("code-diff", "code_diff_mode_changed"),
    ("color-picker-rgba", "color_drag"),
    ("tree-view", "click"),
];

#[test]
fn molecule_story_pages_expose_component_specific_action_history() {
    let examples = StoryCatalog.examples();

    for (page, action) in REQUIRED_INTERACTIVE_MOLECULES {
        let example = examples.iter().find(|it| it.page == page);
        assert!(example.is_some(), "{page} story is missing");
        let Some(example) = example else {
            continue;
        };

        assert!(
            example.callback_logs.iter().any(|it| it.action == action),
            "{page} lacks {action} action"
        );
    }
}
