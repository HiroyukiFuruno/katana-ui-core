use std::collections::BTreeMap;

use katana_ui_core::render_model::UiNode;
use katana_ui_core_storybook::{
    Canvas, StoryCatalog, StoryExample, StorybookPanel, StorybookVisual,
};

const MIN_PAGE_PIXELS: usize = 10_000;
const MIN_PRESET_DIFF_PIXELS: usize = 1_000;

const LEGACY_PAGE_COVERAGE: [(&str, &[&str]); 24] = [
    ("01-theme-tokens", &["theme-tokens"]),
    ("02-text", &["text"]),
    ("03-icon", &["icon"]),
    ("04-loading", &["loading-dots", "spinner", "progress-bar"]),
    ("05-svg-button", &["svg-button"]),
    ("06-text-button", &["text-button"]),
    ("07-icon-text-button", &["icon-text-button"]),
    ("08-toggle", &["toggle"]),
    ("09-segmented-toggle", &["segmented-toggle"]),
    ("10-select-box", &["select-box"]),
    ("11-color-swatch", &["color-swatch"]),
    ("12-text-input", &["text-input"]),
    ("13-search-box", &["search-box"]),
    ("14-tooltip", &["tooltip"]),
    ("15-badge", &["badge"]),
    ("16-key-cap", &["key-cap"]),
    ("17-card", &["card"]),
    ("18-accordion", &["accordion"]),
    ("19-split-pane", &["split-pane"]),
    ("20-modal-overlay", &["modal", "modal-overlay"]),
    ("21-popover", &["popover"]),
    ("22-rgba-color-picker", &["color-picker-rgba"]),
    ("23-color-picker-parity", &["color-picker-rgba"]),
    ("24-code-diff", &["code-diff"]),
];

const INTERACTIVE_LEGACY_PAGES: &[&str] = &[
    "theme-tokens",
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
fn legacy_01_to_24_requirements_are_mapped_to_real_story_pages() {
    let examples = StoryCatalog.examples();
    let by_page = examples_by_page(&examples);

    assert_eq!(24, LEGACY_PAGE_COVERAGE.len());

    for (legacy_id, pages) in LEGACY_PAGE_COVERAGE {
        for page in pages {
            let example = by_page.get(page);
            assert!(
                example.is_some(),
                "{legacy_id} maps to missing story page {page}"
            );
            let Some(example) = example else {
                continue;
            };
            assert!(
                example.contract.is_complete(),
                "{legacy_id} page {page} lacks preview/settings/state/event/action/preset/status"
            );
            assert!(
                node_count(example.tree.root()) >= example.minimum_nodes,
                "{legacy_id} page {page} is below its minimum preview node count"
            );
        }
    }
}

#[test]
fn legacy_interactive_pages_expose_action_logs_in_storybook_panel() {
    let examples = StoryCatalog.examples();
    let by_page = examples_by_page(&examples);
    let panel_report = StorybookPanel::interaction_report(&examples);

    assert!(!panel_report.operation_sequence.is_empty());
    assert!(!panel_report.selector_operations.is_empty());
    assert!(!panel_report.overlay_dismissals.is_empty());
    assert!(!panel_report.color_picker_updates.is_empty());

    for page in INTERACTIVE_LEGACY_PAGES {
        let example = by_page.get(page);
        assert!(example.is_some(), "{page} story is missing");
        let Some(example) = example else {
            continue;
        };
        assert!(
            !example.callback_logs.is_empty(),
            "{page} must expose user-visible action history"
        );
        assert!(
            example.callback_logs.iter().all(|it| !it.after.is_empty()),
            "{page} contains an action log without state transition summary"
        );
    }
}

#[test]
fn legacy_01_to_24_pages_have_visual_cases_and_preset_differences() {
    for (_, pages) in LEGACY_PAGE_COVERAGE {
        for page in pages {
            let default = StorybookVisual.render_preset("dark", page, 0, 0);
            let interactive = StorybookVisual.render_preset("dark", page, 1, 0);

            assert!(
                default.non_background_pixels(0x1f1f1f) > MIN_PAGE_PIXELS,
                "{page} visual case is too sparse"
            );
            assert!(
                pixel_diff(&default, &interactive) > MIN_PRESET_DIFF_PIXELS,
                "{page} preset visual difference is too small"
            );
        }
    }
}

#[test]
fn legacy_01_to_24_each_have_option_action_event_state_preset_and_visual_evidence() {
    let examples = StoryCatalog.examples();
    let by_page = examples_by_page(&examples);
    let panel_report = StorybookPanel::interaction_report(&examples);

    for (legacy_id, pages) in LEGACY_PAGE_COVERAGE {
        for page in pages {
            let example = by_page.get(page);
            assert!(
                example.is_some(),
                "{legacy_id} maps to missing story page {page}"
            );
            let Some(example) = example else {
                continue;
            };
            let setting = panel_report
                .settings_mutations
                .iter()
                .find(|it| it.page == *page);

            assert!(
                setting.is_some_and(|it| {
                    !it.option.name.is_empty()
                        && !it.option.after_value.is_empty()
                        && it.preview.after.contains(&it.option.after_value)
                }),
                "{legacy_id} page {page} lacks typed option settings evidence"
            );
            assert!(
                !example.tree.root().props().state_id.as_str().is_empty(),
                "{legacy_id} page {page} lacks component state evidence"
            );
            assert!(
                has_action_or_passive_evidence(example),
                "{legacy_id} page {page} lacks action/event evidence"
            );
            assert!(
                preset_visual_changes(page),
                "{legacy_id} page {page} lacks preset visual evidence"
            );
        }
    }
}

fn examples_by_page(examples: &[StoryExample]) -> BTreeMap<&'static str, &StoryExample> {
    examples.iter().map(|it| (it.page, it)).collect()
}

fn node_count(node: &UiNode) -> usize {
    1 + node.children().iter().map(node_count).sum::<usize>()
}

fn pixel_diff(before: &Canvas, after: &Canvas) -> usize {
    before
        .pixels()
        .iter()
        .zip(after.pixels().iter())
        .filter(|(left, right)| left != right)
        .count()
}

fn has_action_or_passive_evidence(example: &StoryExample) -> bool {
    if INTERACTIVE_LEGACY_PAGES.contains(&example.page) {
        return example
            .callback_logs
            .iter()
            .any(|it| !it.action.is_empty() && !it.after.is_empty());
    }
    example.callback_logs.is_empty()
}

fn preset_visual_changes(page: &str) -> bool {
    let default = StorybookVisual.render_preset("dark", page, 0, 0);
    let interactive = StorybookVisual.render_preset("dark", page, 1, 0);
    default.non_background_pixels(0x1f1f1f) > MIN_PAGE_PIXELS
        && pixel_diff(&default, &interactive) > MIN_PRESET_DIFF_PIXELS
}
