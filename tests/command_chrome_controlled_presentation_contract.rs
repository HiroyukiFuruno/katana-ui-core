use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeSearchPresentation,
    CommandChromeSearchStrip, CommandChromeToolbar, CommandChromeToolbarPresentation,
    SearchControlCapabilities, SearchControlIcons, SearchControlStrings,
};
use katana_ui_core::molecule::structured::{ReplaceMode, SearchControlStrip, SearchOptions};

fn strings() -> SearchControlStrings {
    let text = |value| katana_ui_core::molecule::command_chrome::CommandChromeText::new(value, value, value);
    SearchControlStrings {
        strip: text("search"), query: text("query"), replace: text("replace"),
        match_case: text("case"), whole_word: text("word"), use_regex: text("regex"),
        previous: text("previous"), next: text("next"), replace_one: text("replace"),
        replace_all: text("replace all"), close: text("close"),
        result_summary: katana_ui_core::molecule::command_chrome::SearchResultSummaryTemplate {
            empty: String::new(), zero_results: "0".to_string(), single_result: "1".to_string(),
            indexed_result: "{active}/{count}".to_string(), count_results: "{count}".to_string(),
        },
    }
}

#[test]
fn controlled_command_chrome_presentation_changes_do_not_emit_events() {
    let mut toolbar = CommandChromeToolbar::new().action(CommandChromeAction::new("one", "One"));
    assert!(toolbar.synchronize_presentation(CommandChromeToolbarPresentation {
        actions: vec![CommandChromeAction::new("one", "Uno")], groups: Vec::new(),
        display_mode: CommandChromeDisplayMode::IconLeading,
        density: Default::default(), overflow_strategy: Default::default(),
    }));
    assert_eq!(toolbar.actions()[0].label_model(), "Uno");

    let mut search = CommandChromeSearchStrip::new(SearchControlStrip::new("search"), strings());
    assert!(search.synchronize_presentation(CommandChromeSearchPresentation {
        query: "\u{65e5}\u{672c}\u{8a9e} \u{2b50}\u{fe0f}".to_string(), options: SearchOptions::default(),
        result_count: Some(2), active_index: Some(0), replace_mode: ReplaceMode::Visible,
        replace_value: "replace".to_string(), strings: strings(),
        capabilities: SearchControlCapabilities::all_available(), icons: SearchControlIcons::default(),
    }));
    assert_eq!(search.query_model(), "\u{65e5}\u{672c}\u{8a9e} \u{2b50}\u{fe0f}");
    assert_eq!(search.result_count_model(), Some(2));
}
