use super::*;

#[test]
fn available_capability_reports_no_disabled_reason() {
    let capability = CommandChromeCapability::available();
    assert!(capability.is_available());
    assert_eq!(capability.disabled_reason(), None);
}

#[test]
fn unavailable_capability_exposes_disabled_reason() {
    let capability = CommandChromeCapability::unavailable("disabled for test");
    assert!(!capability.is_available());
    assert_eq!(capability.disabled_reason(), Some("disabled for test"));
}

#[test]
fn controlled_presentation_synchronizes_changed_host_icons() {
    let strings = test_strings();
    let mut strip =
        CommandChromeSearchStrip::new(SearchControlStrip::new("legacy"), strings.clone());
    let icon = UiIconProps {
        svg_source: "<svg data-opaque=\"1\"/>".to_string(),
        ..UiIconProps::default()
    };

    assert!(
        strip.synchronize_presentation(CommandChromeSearchPresentation {
            query: String::new(),
            options: SearchOptions::default(),
            result_count: None,
            active_index: None,
            replace_mode: ReplaceMode::Hidden,
            replace_value: String::new(),
            strings,
            capabilities: SearchControlCapabilities::default(),
            icons: SearchControlIcons::default().icon(SearchControlIconSlot::Next, icon.clone()),
        })
    );
    assert_eq!(
        strip.icons_model().icon_for(SearchControlIconSlot::Next),
        Some(&icon)
    );
}

fn test_strings() -> SearchControlStrings {
    SearchControlStrings {
        strip: test_text("opaque-0"),
        query: test_text("opaque-1"),
        replace: test_text("opaque-2"),
        match_case: test_text("opaque-3"),
        whole_word: test_text("opaque-4"),
        use_regex: test_text("opaque-5"),
        previous: test_text("opaque-6"),
        next: test_text("opaque-7"),
        replace_one: test_text("opaque-8"),
        replace_all: test_text("opaque-9"),
        close: test_text("opaque-10"),
        result_summary: SearchResultSummaryTemplate {
            empty: String::new(),
            zero_results: "0".to_string(),
            single_result: "1".to_string(),
            indexed_result: "{active}/{count}".to_string(),
            count_results: "{count}".to_string(),
        },
    }
}

fn test_text(value: &str) -> CommandChromeText {
    CommandChromeText::new(value, value, value)
}
