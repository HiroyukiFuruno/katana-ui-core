use katana_ui_core_storybook::StoryCatalog;

const REQUIRED_INTERACTIVE_ATOMS: [&str; 14] = [
    "button",
    "text-button",
    "svg-button",
    "icon-text-button",
    "chip",
    "text-input",
    "checkbox",
    "radio",
    "loading-dots",
    "spinner",
    "progress-bar",
    "color-swatch",
    "toggle",
    "slide-control",
];

const REQUIRED_PASSIVE_ATOMS: [&str; 4] = ["text", "icon", "badge", "key-cap"];

#[test]
fn atom_story_pages_expose_action_and_event_history() {
    let examples = StoryCatalog.examples();

    for page in REQUIRED_INTERACTIVE_ATOMS {
        let example = examples.iter().find(|it| it.page == page);
        assert!(example.is_some(), "{page} story is missing");
        let Some(example) = example else {
            continue;
        };

        assert!(!example.callback_logs.is_empty(), "{page} lacks action log");
        assert!(
            example.callback_logs.iter().all(|it| !it.action.is_empty()),
            "{page} contains empty action name"
        );
    }

    for page in REQUIRED_PASSIVE_ATOMS {
        let example = examples.iter().find(|it| it.page == page);
        assert!(example.is_some(), "{page} story is missing");
        let Some(example) = example else {
            continue;
        };

        assert!(example.callback_logs.is_empty(), "{page} should be passive");
    }
}
