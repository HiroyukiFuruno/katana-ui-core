use katana_ui_core::molecule::structured::{
    ReplaceMode, SearchControlStrip, SearchControlStripAction, SearchControlStripEvent,
    SearchNavigationDirection, SearchOptionKind, SearchOptions, SearchReplaceScope,
};
use std::error::Error;

const COMMAND_CHROME_SEARCH_MODEL: &str =
    include_str!("../src/molecule/command_chrome/search_model.rs");
const COMMAND_CHROME_SEARCH_LOGIC: &str =
    include_str!("../src/molecule/command_chrome/search_logic.rs");

#[test]
fn legacy_search_strip_keeps_its_serialized_public_shape() {
    let strip = SearchControlStrip::new("Search controls")
        .query("heading")
        .options(SearchOptions {
            match_case: true,
            whole_word: false,
            use_regex: true,
        })
        .result_position(4, Some(1))
        .replace_mode(ReplaceMode::Visible)
        .replace_value("title");

    let strip_json = serde_json::to_value(&strip);

    assert!(strip_json.is_ok());
    let Ok(strip_json) = strip_json else {
        return;
    };
    assert_eq!(Some("Search controls"), strip_json["label"].as_str());
    assert_eq!(Some("heading"), strip_json["query"].as_str());
    assert_eq!(Some(true), strip_json["options"]["use_regex"].as_bool());
    assert_eq!(Some("Visible"), strip_json["replace_mode"].as_str());
}

#[test]
fn stable_state_id_is_additive_and_preserves_default_and_serde_contracts()
-> Result<(), Box<dyn Error>> {
    let default_strip = SearchControlStrip::new("Search controls");
    assert!(!default_strip.state_id().as_str().is_empty());

    let stable_strip =
        SearchControlStrip::new("Search controls").stable_state_id("consumer.search.stable");
    let serialized = serde_json::to_value(&stable_strip)?;
    let restored: SearchControlStrip = serde_json::from_value(serialized)?;

    assert_eq!("consumer.search.stable", restored.state_id().as_str());
    Ok(())
}

#[test]
fn legacy_search_action_and_event_consumers_remain_exhaustive() {
    let actions = vec![
        SearchControlStripAction::SetSearchQuery("needle".to_string()),
        SearchControlStripAction::ToggleSearchOption(SearchOptionKind::MatchCase),
        SearchControlStripAction::Navigate(SearchNavigationDirection::Previous),
        SearchControlStripAction::SetReplaceMode(ReplaceMode::Visible),
        SearchControlStripAction::SetReplaceValue("replacement".to_string()),
        SearchControlStripAction::Replace(SearchReplaceScope::All),
        SearchControlStripAction::SetResultPosition {
            result_count: 3,
            active_index: Some(0),
        },
    ];

    let names = actions
        .into_iter()
        .map(legacy_action_name)
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            "query",
            "option",
            "navigate",
            "replace-mode",
            "replace-value",
            "replace",
            "position",
        ],
        names
    );
    assert_eq!(
        "query",
        legacy_event_name(SearchControlStripEvent::SearchQueryChanged("q".to_string()))
    );
}

#[test]
fn command_chrome_search_never_calls_the_legacy_fixed_english_render_path() {
    let source = format!("{COMMAND_CHROME_SEARCH_MODEL}\n{COMMAND_CHROME_SEARCH_LOGIC}");

    for fixed_literal in [
        "Match case",
        "Whole word",
        "Use regex",
        "Previous result",
        "Next result",
        "Replace all",
        "Search controls",
    ] {
        assert!(
            !source.contains(fixed_literal),
            "fixed literal: {fixed_literal}"
        );
    }
    assert!(!source.contains("UiNode"));
    assert!(!source.contains("render::render"));
}

fn legacy_action_name(action: SearchControlStripAction) -> &'static str {
    match action {
        SearchControlStripAction::SetSearchQuery(_) => "query",
        SearchControlStripAction::ToggleSearchOption(_) => "option",
        SearchControlStripAction::Navigate(_) => "navigate",
        SearchControlStripAction::SetReplaceMode(_) => "replace-mode",
        SearchControlStripAction::SetReplaceValue(_) => "replace-value",
        SearchControlStripAction::Replace(_) => "replace",
        SearchControlStripAction::SetResultPosition { .. } => "position",
    }
}

fn legacy_event_name(event: SearchControlStripEvent) -> &'static str {
    match event {
        SearchControlStripEvent::SearchQueryChanged(_) => "query",
        SearchControlStripEvent::SearchOptionChanged { .. } => "option",
        SearchControlStripEvent::SearchNavigationRequested { .. } => "navigate",
        SearchControlStripEvent::ReplaceModeChanged(_) => "replace-mode",
        SearchControlStripEvent::ReplaceValueChanged(_) => "replace-value",
        SearchControlStripEvent::ReplaceRequested { .. } => "replace",
        SearchControlStripEvent::SearchResultPositionChanged { .. } => "position",
    }
}
