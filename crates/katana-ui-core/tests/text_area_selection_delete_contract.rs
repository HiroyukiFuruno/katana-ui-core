use katana_ui_core::atom::{TextArea, TextAreaAction, TextAreaEvent, TextAreaSelection};

#[test]
fn delete_backward_removes_selected_unicode_range_once_in_both_directions() {
    let value = "前日本⭐️後";
    let start = "前".len();
    let end = "前日本⭐️".len();
    for (start, end) in [(start, end), (end, start)] {
        let mut area = TextArea::new("selected-delete").value(value);
        let selected =
            area.apply_text_area_action(TextAreaAction::Select(TextAreaSelection { start, end }));
        assert!(selected.handled);
        let result = area.apply_text_area_action(TextAreaAction::DeleteBackward);
        assert!(result.handled);
        assert_eq!(result.state.value, "前後");
        assert_eq!(result.state.caret, "前".len());
        assert_eq!(
            result.state.selection,
            TextAreaSelection::collapsed("前".len())
        );
        assert_eq!(result.events, vec![TextAreaEvent::Change("前後".into())]);
    }
}

#[test]
fn select_all_delete_reaches_empty_and_repeated_delete_is_noop() {
    let value = "日本語⭐️";
    let mut area = TextArea::new("empty-name").value(value);
    let selected = area.apply_text_area_action(TextAreaAction::Select(TextAreaSelection {
        start: 0,
        end: value.len(),
    }));
    assert!(selected.handled);
    let cleared = area.apply_text_area_action(TextAreaAction::DeleteBackward);
    assert_eq!(cleared.state.value, "");
    assert_eq!(cleared.events, vec![TextAreaEvent::Change(String::new())]);
    let repeated = area.apply_text_area_action(TextAreaAction::DeleteBackward);
    assert!(!repeated.handled);
    assert!(repeated.events.is_empty());
}
