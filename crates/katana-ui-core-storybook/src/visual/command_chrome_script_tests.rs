use super::command_chrome_script::run_scripted_sequence;
#[path = "command_chrome_script_assertions_tests.rs"]
mod command_chrome_script_assertions_tests;
use command_chrome_script_assertions_tests::{
    FRAME_COUNT, artifact_hashes, assert_dropdown_closed, assert_dropdown_focus,
    assert_dropdown_item_activated, assert_dropdown_opened, assert_fixture_inventory,
    assert_floating_closed, assert_floating_command, assert_floating_event, assert_hover_tooltip,
    assert_initial_floating_surface, assert_no_command, assert_no_toolbar_command,
    assert_search_event, assert_text_event, assert_toolbar_command,
};
use katana_ui_core::atom::TextAreaEvent;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeDropdownCloseReason, CommandChromeSearchEvent, FloatingCommandToolbarCloseReason,
    FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::structured::{
    SearchControlStripEvent, SearchOptionKind, SearchReplaceScope,
};
use std::error::Error;

#[test]
fn command_chrome_script_covers_the_complete_actual_egui_raw_input_contract()
-> Result<(), Box<dyn Error>> {
    let sequence = run_scripted_sequence()?;
    let frames = &sequence.frames;

    assert_eq!(FRAME_COUNT, frames.len());
    assert_fixture_inventory(&frames[0]);
    assert_initial_floating_surface(&frames[0])?;
    assert_hover_tooltip(&frames[1])?;

    assert_floating_event(&frames[2], FloatingCommandToolbarEvent::FocusRetained);
    assert_floating_command(&frames[2], "floating-code");
    assert_floating_closed(&frames[3], FloatingCommandToolbarCloseReason::OutsideClick);
    assert_no_toolbar_command(&frames[3]);

    assert_no_command(frames, "disabled");
    assert_toolbar_command(&frames[5], "inline-bold");

    assert_dropdown_opened(&frames[6])?;
    assert_dropdown_focus(&frames[6], "code-01");
    assert_dropdown_focus(&frames[7], "code-02");
    assert_dropdown_focus(&frames[8], "code-01");
    assert!(
        frames[9].toolbar.events.is_empty(),
        "Home keeps code-01 focused"
    );
    assert_dropdown_focus(&frames[10], "code-17");
    assert_dropdown_item_activated(&frames[11], "code-17");
    assert_dropdown_opened(&frames[12])?;
    assert_dropdown_item_activated(&frames[13], "code-01");
    assert_dropdown_opened(&frames[14])?;
    assert_dropdown_closed(&frames[15], CommandChromeDropdownCloseReason::OutsideClick);
    assert_no_toolbar_command(&frames[15]);
    assert_dropdown_opened(&frames[16])?;
    assert_dropdown_closed(&frames[17], CommandChromeDropdownCloseReason::Escape);

    assert_floating_closed(&frames[19], FloatingCommandToolbarCloseReason::Escape);

    assert_text_event(
        &frames[21],
        |event| matches!(event, TextAreaEvent::TextInput(value) if value == "日本語 ⭐️"),
    );
    assert_text_event(&frames[21], |event| {
        matches!(event, TextAreaEvent::EmojiInput { grapheme_count: 5 })
    });
    assert_search_event(
        &frames[21],
        |event| matches!(event, SearchControlStripEvent::SearchQueryChanged(value) if value.contains("日本語") && value.contains('⭐')),
    );
    assert_text_event(
        &frames[22],
        |event| matches!(event, TextAreaEvent::ImeComposition(state) if state.preedit == "ほし"),
    );
    assert!(
        frames[22].search.events.is_empty(),
        "IME preedit must not publish a committed search mutation"
    );
    assert!(
        frames[23].search.events.is_empty(),
        "controlled presentation must not synthesize a search command"
    );
    assert!(
        frames[23]
            .search
            .record
            .query
            .frame
            .layout_identity
            .contains("同期検索 ⭐️")
    );
    assert_text_event(
        &frames[24],
        |event| matches!(event, TextAreaEvent::ImeCommit(value) if value == "⭐️"),
    );
    assert_search_event(&frames[25], |event| {
        matches!(
            event,
            SearchControlStripEvent::SearchOptionChanged {
                option: SearchOptionKind::MatchCase,
                enabled: true,
            }
        )
    });
    assert_search_event(&frames[26], |event| {
        matches!(
            event,
            SearchControlStripEvent::SearchOptionChanged {
                option: SearchOptionKind::WholeWord,
                enabled: true,
            }
        )
    });
    assert_search_event(
        &frames[28],
        |event| matches!(event, SearchControlStripEvent::ReplaceValueChanged(value) if value.contains("置換") && value.contains('⭐')),
    );
    assert_search_event(&frames[29], |event| {
        matches!(
            event,
            SearchControlStripEvent::ReplaceRequested {
                scope: SearchReplaceScope::One,
                ..
            }
        )
    });
    assert_search_event(&frames[30], |event| {
        matches!(
            event,
            SearchControlStripEvent::ReplaceRequested {
                scope: SearchReplaceScope::All,
                ..
            }
        )
    });
    assert!(
        frames[31].search.events.is_empty(),
        "the unavailable regex capability must not publish an event"
    );
    assert!(
        frames[32]
            .search
            .events
            .contains(&CommandChromeSearchEvent::CloseRequested)
    );
    assert!(frames[33].floating.events.is_empty());
    assert!(frames[33].floating.record.is_some());

    let accesskit_labels = frames
        .iter()
        .flat_map(|frame| frame.accesskit_labels.iter())
        .collect::<Vec<_>>();
    assert!(
        accesskit_labels
            .iter()
            .any(|label| label.as_str() == "検索 ⭐️")
    );
    assert!(
        accesskit_labels
            .iter()
            .any(|label| label.as_str() == "太字 ⭐️")
    );
    assert!(
        accesskit_labels
            .iter()
            .any(|label| label.as_str() == "リンク ⭐️")
    );
    Ok(())
}

#[test]
fn command_chrome_script_is_deterministic_for_every_actual_adapter_artifact()
-> Result<(), Box<dyn Error>> {
    let first = run_scripted_sequence()?;
    let second = run_scripted_sequence()?;

    assert_eq!(FRAME_COUNT, first.frames.len());
    assert_eq!(first.frames.len(), second.frames.len());
    for (index, (left, right)) in first.frames.iter().zip(&second.frames).enumerate() {
        assert_eq!(
            artifact_hashes(left),
            artifact_hashes(right),
            "frame {index} changed across independently constructed actual-egui runs"
        );
    }
    Ok(())
}
