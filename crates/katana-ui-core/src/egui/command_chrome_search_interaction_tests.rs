use super::query_key_events;
use crate::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeSearchStrip, CommandChromeText, SearchControlStrings,
    SearchResultSummaryTemplate,
};
use crate::molecule::structured::SearchControlStrip;

fn text(value: &str) -> CommandChromeText {
    CommandChromeText::new(value, value, value)
}

fn strings() -> SearchControlStrings {
    SearchControlStrings {
        strip: text("Search"),
        query: text("Query"),
        replace: text("Replace"),
        match_case: text("Match case"),
        whole_word: text("Whole word"),
        use_regex: text("Regex"),
        previous: text("Previous"),
        next: text("Next"),
        replace_one: text("Replace"),
        replace_all: text("Replace all"),
        close: text("Close"),
        result_summary: SearchResultSummaryTemplate {
            empty: String::new(),
            zero_results: "0".to_owned(),
            single_result: "1".to_owned(),
            indexed_result: "{active}/{count}".to_owned(),
            count_results: "{count}".to_owned(),
        },
    }
}

#[test]
fn focused_query_ignores_a_pressed_non_search_key_before_routing_escape() {
    let context = egui::Context::default();
    let mut strip = CommandChromeSearchStrip::new(SearchControlStrip::new("Search"), strings());
    let mut events = None;
    let mut key_was_visible = false;

    let mut output = context.run_ui(
        egui::RawInput {
            events: vec![
                egui::Event::Copy,
                egui::Event::Key {
                    key: egui::Key::A,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::Key {
                    key: egui::Key::Escape,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::default(),
                },
            ],
            ..egui::RawInput::default()
        },
        |ui| {
            key_was_visible = ui.input(|input| {
                input.events.iter().any(|event| {
                    matches!(
                        event,
                        egui::Event::Key {
                            key: egui::Key::A,
                            pressed: true,
                            ..
                        }
                    )
                })
            });
            events = Some(query_key_events(ui, &mut strip, true));
        },
    );
    output.textures_delta.clear();

    assert!(key_was_visible);
    assert_eq!(
        events.expect("query key route ran"),
        [CommandChromeSearchEvent::CloseRequested]
    );
}
