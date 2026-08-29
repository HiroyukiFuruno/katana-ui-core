use super::{TextArea, TextAreaAction, TextAreaCompositionPhase, TextAreaSelection};

#[test]
fn controlled_identity_sync_changes_only_the_text_area_identity() {
    let value = "\u{65e5}\u{672c}\u{8a9e} \u{2b50}\u{fe0f}";
    let mut area = TextArea::new("editor")
        .stable_state_id("editor.before")
        .value(value);
    let _ = area.apply_text_area_action(TextAreaAction::Select(TextAreaSelection {
        start: 3,
        end: value.len(),
    }));
    let _ = area.apply_text_area_action(TextAreaAction::composition(
        TextAreaCompositionPhase::Update,
        "\u{5165}\u{529b}\u{4e2d} \u{2b50}\u{fe0f}",
        "\u{5165}".len(),
    ));
    let before_state = area.state().clone();
    let before_events = area.events().to_vec();

    assert!(area.synchronize_state_id("editor.after"));
    assert_eq!(area.state_id().as_str(), "editor.after");
    assert_eq!(area.state().value, value);
    assert_eq!(area.state().selection, before_state.selection);
    assert_eq!(area.state().caret, before_state.caret);
    assert_eq!(area.state().composition, before_state.composition);
    assert_eq!(area.events(), before_events);
}

#[test]
fn controlled_identity_sync_returns_false_for_the_existing_text_area_identity() {
    let mut area = TextArea::new("editor").stable_state_id("editor.identity");

    assert!(!area.synchronize_state_id("editor.identity"));
    assert_eq!(area.state_id().as_str(), "editor.identity");
    assert!(area.events().is_empty());
}
