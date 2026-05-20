use katana_ui_core::render_model::{UiNode, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::{StoryCatalog, StorybookPanel};

const DETAIL_SECTIONS: &[&str] = &[
    "Preset tabs",
    "Settings",
    "State",
    "Event history",
    "Action history",
    "Requirement status",
];
const GENERIC_PRESET_LABELS: &[&str] = &["default", "interactive", "edge", "theme"];

#[test]
fn storybook_page_contract_sections_are_materialized_per_page() {
    let examples = StoryCatalog.examples();
    let panel = StorybookPanel::new(ThemeSnapshot::dark());

    for example in &examples {
        let tree = panel.build_selected(&examples, example.page);
        let details = panel_child(tree.root(), "Details");
        let preview = panel_child(tree.root(), "Preview");
        assert!(
            details.is_some(),
            "{} details panel is missing",
            example.page
        );
        assert!(
            preview.is_some(),
            "{} preview panel is missing",
            example.page
        );
        let (Some(details), Some(preview)) = (details, preview) else {
            continue;
        };

        assert_eq!(DETAIL_SECTIONS.len(), details.children().len());
        for label in DETAIL_SECTIONS {
            assert!(
                details
                    .children()
                    .iter()
                    .any(|it| it.props().label == *label),
                "{} lacks detail section {}",
                example.page,
                label
            );
        }
        let preset = details
            .children()
            .iter()
            .find(|it| it.props().label == "Preset tabs");
        assert!(
            preset.is_some(),
            "{} preset tabs section is missing",
            example.page
        );
        let Some(preset) = preset else {
            continue;
        };
        assert_eq!(
            expected_preset_count(example.page),
            preset.children().len(),
            "{} preset tabs must expose concrete checks",
            example.page
        );
        assert!(
            preset
                .children()
                .iter()
                .all(|it| !GENERIC_PRESET_LABELS.contains(&it.props().label.as_str())),
            "{} preset tabs still use generic labels",
            example.page
        );
        assert!(
            detail_text(details, "Settings")
                .is_some_and(|it| it.contains("settings:") && it.contains(" -> ")),
            "{} settings section is still placeholder-like",
            example.page
        );
        assert!(
            detail_text(details, "State").is_some_and(|it| it.contains(" state: id=")),
            "{} state section lacks state id",
            example.page
        );
        assert!(
            detail_text(details, "Action history").is_some_and(|it| it.contains(" action: ")),
            "{} action section lacks action evidence",
            example.page
        );
        assert!(
            preview
                .children()
                .iter()
                .any(|it| it.props().label == example.tree.root().props().label),
            "{} preview root is not materialized",
            example.page
        );
        assert_eq!(
            1,
            preview.children().len(),
            "{} preview must show only the selected story",
            example.page
        );
    }
}

fn panel_child<'a>(root: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == label)
}

fn detail_text<'a>(details: &'a UiNode, label: &str) -> Option<&'a str> {
    details
        .children()
        .iter()
        .find(|it| it.props().label == label)
        .and_then(|it| it.children().first())
        .map(|it| it.props().label.as_str())
}

fn expected_preset_count(page: &str) -> usize {
    if matches!(
        page,
        "context-menu"
            | "command-palette"
            | "code-diff"
            | "color-picker-rgba"
            | "drag-and-drop"
            | "hover-card"
            | "search-control-strip"
            | "toolbar"
    ) {
        return 5;
    }
    if page == "text-area" {
        return 7;
    }
    if page == "attachment-chip" {
        return 5;
    }
    if page == "banner" {
        return 5;
    }
    if page == "toast-stack-manager" {
        return 5;
    }
    if page == "status-bar" {
        return 5;
    }
    if page == "empty-state" {
        return 5;
    }
    if page == "closeable-tab-strip" {
        return 6;
    }
    if page == "diagnostics-list" {
        return 6;
    }
    4
}
