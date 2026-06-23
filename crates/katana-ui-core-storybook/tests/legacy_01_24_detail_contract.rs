use std::collections::BTreeSet;

use katana_ui_core_storybook::{StoryCatalog, StoryDetailContent};

const MIN_UNIQUE_SETTINGS: usize = 20;

const LEGACY_REAL_PAGES: &[&str] = &[
    "theme-tokens",
    "text",
    "icon",
    "loading-dots",
    "spinner",
    "progress-bar",
    "svg-button",
    "text-button",
    "icon-text-button",
    "toggle",
    "segmented-toggle",
    "select-box",
    "color-swatch",
    "text-input",
    "search-box",
    "tooltip",
    "badge",
    "key-cap",
    "card",
    "accordion",
    "split-pane",
    "modal",
    "modal-overlay",
    "popover",
    "color-picker-rgba",
    "code-diff",
];

#[test]
fn legacy_01_to_24_details_are_selected_component_specific() {
    let examples = StoryCatalog.examples();
    let mut settings = BTreeSet::new();

    for page in LEGACY_REAL_PAGES {
        let example = examples.iter().find(|it| it.page == *page);
        assert!(example.is_some(), "{page} story is missing");
        let Some(example) = example else {
            continue;
        };
        let content = StoryDetailContent::from_example(example);

        assert_eq!(*page, content.page);
        assert_detail_line(page, "settings", &content.settings);
        assert_detail_line(page, "state", &content.state);
        assert_detail_line(page, "event", &content.event);
        assert_detail_line(page, "action", &content.action);
        assert_detail_line(page, "preset", &content.preset);
        assert_detail_line(page, "quality", &content.quality);
        assert!(
            !content.settings.contains("theme_id") || *page == "theme-tokens",
            "{page} settings must expose a component-specific option"
        );
        settings.insert(content.settings);
    }

    assert!(
        settings.len() > MIN_UNIQUE_SETTINGS,
        "legacy details still look like shared placeholder settings"
    );
}

fn assert_detail_line(page: &str, section: &str, line: &str) {
    assert!(line.contains(section), "{page} {section} line is missing");
    assert!(
        line.contains("legacy-") || line.contains("catalog-"),
        "{page} {section} line lacks page marker"
    );
}
