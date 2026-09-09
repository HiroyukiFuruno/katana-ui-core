use super::super::{ScenarioSessionState, ScenarioSessionUpdate};
use crate::egui::text_command_surface::FullTextCommandSurfaceScenarioId;
use crate::molecule::command_chrome::CommandChromeSearchEvent;
use crate::molecule::structured::{
    ReplaceMode, SearchControlStripEvent, SearchNavigationDirection, SearchOptionKind,
    SearchOptions, SearchReplaceScope,
};

const RESULT_COUNT: usize = 4;
const ACTIVE_RESULT_INDEX: usize = 2;

#[test]
fn search_event_projection_retains_all_supported_search_fields() {
    let mut update = ScenarioSessionUpdate::default();
    let events = [
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchQueryChanged(String::from("日本語 ⭐️")),
        },
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchOptionChanged {
                option: SearchOptionKind::MatchCase,
                enabled: true,
            },
        },
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchOptionChanged {
                option: SearchOptionKind::WholeWord,
                enabled: true,
            },
        },
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchOptionChanged {
                option: SearchOptionKind::UseRegex,
                enabled: true,
            },
        },
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::ReplaceModeChanged(ReplaceMode::Visible),
        },
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::ReplaceValueChanged(String::from("replace ⭐️")),
        },
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchResultPositionChanged {
                result_count: RESULT_COUNT,
                active_index: Some(ACTIVE_RESULT_INDEX),
            },
        },
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchNavigationRequested {
                direction: SearchNavigationDirection::Next,
            },
        },
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::ReplaceRequested {
                scope: SearchReplaceScope::One,
                value: String::from("ignored by retained session"),
            },
        },
        CommandChromeSearchEvent::CloseRequested,
    ];
    for event in &events {
        update.apply_search_event(event);
    }

    let text = String::from("original text");
    let mut state = ScenarioSessionState::default();
    state.apply(update, &text);
    assert_eq!(state.search_visible, Some(false));
    assert_eq!(state.search_query.as_deref(), Some("日本語 ⭐️"));
    assert_eq!(
        state.search_options,
        Some(SearchOptions {
            match_case: true,
            whole_word: true,
            use_regex: true,
        })
    );
    assert_eq!(state.replace_mode, Some(ReplaceMode::Visible));
    assert_eq!(state.replace_value.as_deref(), Some("replace ⭐️"));
    assert_eq!(
        state.result_position,
        Some((RESULT_COUNT, Some(ACTIVE_RESULT_INDEX)))
    );

    state.apply(
        ScenarioSessionUpdate {
            search_visible: Some(true),
            ..ScenarioSessionUpdate::default()
        },
        &text,
    );
    let search = state
        .presentation(FullTextCommandSurfaceScenarioId::Find)
        .search
        .expect("reopened search retains every supported event value");
    assert_eq!(search.value.query, "日本語 ⭐️");
    assert!(search.value.options.match_case);
    assert!(search.value.options.whole_word);
    assert!(search.value.options.use_regex);
    assert_eq!(search.value.replace_mode, ReplaceMode::Visible);
    assert_eq!(search.value.replace_value, "replace ⭐️");
    assert_eq!(search.value.result_count, Some(RESULT_COUNT));
    assert_eq!(search.value.active_index, Some(ACTIVE_RESULT_INDEX));
}
