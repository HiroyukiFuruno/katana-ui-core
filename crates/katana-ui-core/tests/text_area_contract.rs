use katana_ui_core::atom::{
    TextArea, TextAreaAction, TextAreaCaretMove, TextAreaCompositionPhase, TextAreaEvent,
    TextAreaKeyChord, TextAreaNewlineKey, TextAreaSubmitKey, TextAreaTabBehavior,
    TextAreaValidationError, TextAreaWrapPolicy,
};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::{UiNode, UiNodeKind, UiSlotPlacement, UiVisualRole};
use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;

#[test]
fn text_area_options_are_typed_and_rendered() {
    let node = UiNode::from(
        TextArea::new("Composer")
            .value("draft")
            .placeholder("Message")
            .font_role("body")
            .disabled(true)
            .readonly(true)
            .invalid(true)
            .min_rows(2)
            .max_rows(8)
            .auto_grow(true)
            .wrap_policy(TextAreaWrapPolicy::Soft)
            .submit_key(TextAreaSubmitKey::Enter)
            .newline_key(TextAreaNewlineKey::ShiftEnter)
            .tab_behavior(TextAreaTabBehavior::MoveFocus)
            .ime_enabled(true)
            .leading_slot("Attach")
            .trailing_slot("Send")
            .visual_role(UiVisualRole::Input),
    );

    assert_eq!(UiNodeKind::TextArea, node.kind());
    assert_eq!("draft", node.props().interaction.value);
    assert_eq!("Message", node.props().placeholder);
    assert_eq!("body", node.props().font_role);
    assert!(node.props().disabled);
    assert!(node.props().readonly);
    assert!(node.props().invalid);
    let text_area = &node.props().text_area;
    assert_eq!(2, text_area.min_rows);
    assert_eq!(8, text_area.max_rows);
    assert!(text_area.auto_grow);
    assert_eq!(TextAreaWrapPolicy::Soft, text_area.wrap_policy);
    assert_eq!(TextAreaSubmitKey::Enter, text_area.submit_key);
    assert_eq!(TextAreaNewlineKey::ShiftEnter, text_area.newline_key);
    assert_eq!(TextAreaTabBehavior::MoveFocus, text_area.tab_behavior);
    assert!(text_area.ime_enabled);
    assert_eq!(UiVisualRole::Input, node.props().visual_role);
    let text_entry = &node.props().text_entry;
    assert_eq!(
        Some(UiSlotPlacement::Leading),
        text_entry.leading_slot.as_ref().map(|slot| slot.placement)
    );
    assert_eq!(
        Some("Send"),
        text_entry
            .trailing_slot
            .as_ref()
            .map(|slot| slot.label.as_str())
    );
}

#[test]
fn enter_submits_and_shift_enter_inserts_newline() {
    let mut text_area = TextArea::new("Composer")
        .submit_key(TextAreaSubmitKey::Enter)
        .newline_key(TextAreaNewlineKey::ShiftEnter);

    let submit = text_area.handle_key(TextAreaKeyChord::enter());
    let newline = text_area.handle_key(TextAreaKeyChord::shift_enter());

    let submit_event = TextAreaEvent::Submit(String::new());
    assert!(
        submit
            .as_ref()
            .is_ok_and(|outcome| outcome.events.contains(&submit_event))
    );
    assert!(
        newline
            .as_ref()
            .is_ok_and(|outcome| outcome.events.contains(&TextAreaEvent::InsertNewline))
    );
    assert_eq!("\n", text_area.state().value);
}

#[test]
fn conflicting_submit_and_newline_keys_fail_contract_validation() {
    let conflict = TextArea::new("Composer")
        .submit_key(TextAreaSubmitKey::Enter)
        .newline_key(TextAreaNewlineKey::Enter);
    let disabled = TextArea::new("Composer")
        .submit_key(TextAreaSubmitKey::Disabled)
        .newline_key(TextAreaNewlineKey::Disabled);

    let conflict_error = Err(TextAreaValidationError::ConflictingKeyBindings);
    assert_eq!(conflict_error, conflict.validate());
    assert_eq!(Ok(()), disabled.validate());
}

#[test]
fn disabled_and_readonly_suppress_text_area_actions() {
    let mut disabled = TextArea::new("Disabled").value("locked").disabled(true);
    let mut readonly = TextArea::new("Readonly").value("locked").readonly(true);

    let disabled_result = disabled.apply_text_area_action(TextAreaAction::Type("x".to_string()));
    let readonly_result = readonly.apply_text_area_action(TextAreaAction::Clear);

    assert!(!disabled_result.handled);
    assert!(!readonly_result.handled);
    assert!(disabled_result.events.is_empty());
    assert!(readonly_result.events.is_empty());
    assert_eq!("locked", disabled.state().value);
    assert_eq!("locked", readonly.state().value);
}

#[test]
fn readonly_text_area_allows_focus_selection_and_submit_without_value_mutation() {
    let mut readonly = TextArea::new("Readonly").value("locked").readonly(true);

    let write = readonly.apply_action(&UiAction::input_value(
        readonly.state_id().clone(),
        "changed",
    ));
    let focus = readonly.apply_action(&UiAction::focus(readonly.state_id().clone()));
    let selection = readonly.apply_action(&UiAction::cursor_selection(
        readonly.state_id().clone(),
        4,
        1,
        4,
    ));
    let submit = readonly.apply_text_area_action(TextAreaAction::Submit);
    let node = UiNode::from(readonly);

    assert!(!write.handled);
    assert!(focus.handled);
    assert!(selection.handled);
    assert!(submit.handled);
    assert_eq!("locked", node.props().interaction.value);
    assert!(node.props().interaction.focused);
    assert_eq!(4, node.props().interaction.cursor);
    assert_eq!(1, node.props().interaction.selection_start);
    assert_eq!(4, node.props().interaction.selection_end);
}

#[test]
fn grapheme_caret_and_delete_treat_joined_emoji_as_one_unit() {
    let emoji = "👨‍👩‍👧‍👦";
    let mut text_area = TextArea::new("Emoji").value(format!("a{emoji}b"));
    let after_a = "a".len();
    let after_emoji = format!("a{emoji}").len();

    let move_after_emoji = TextAreaAction::MoveCaret(TextAreaCaretMove::To(after_emoji));
    let _ = text_area.apply_text_area_action(move_after_emoji.clone());
    let _ = text_area.apply_text_area_action(TextAreaAction::MoveCaret(
        TextAreaCaretMove::PreviousGrapheme,
    ));
    assert_eq!(after_a, text_area.state().caret);

    let _ = text_area.apply_text_area_action(move_after_emoji);
    let _ = text_area.apply_text_area_action(TextAreaAction::DeleteBackward);
    assert_eq!("ab", text_area.state().value);
    assert_eq!(after_a, text_area.state().caret);
}

#[test]
fn ime_composition_lifecycle_commits_once() {
    let mut text_area = TextArea::new("IME").ime_enabled(true);

    let start = TextAreaAction::composition(TextAreaCompositionPhase::Start, "に", 3);
    let _ = text_area.apply_text_area_action(start);
    let _ = text_area.apply_text_area_action(TextAreaAction::composition(
        TextAreaCompositionPhase::Update,
        "日本\n語",
        "日本\n語".len(),
    ));
    let commit = text_area.apply_text_area_action(TextAreaAction::ime_commit("日本語"));

    assert!(text_area.state().composition.is_none());
    assert_eq!("日本語", text_area.state().value);
    let commit_event = TextAreaEvent::ImeCommit("日本語".to_string());
    assert!(commit.events.contains(&commit_event));
    assert_eq!(
        1,
        text_area
            .events()
            .iter()
            .filter(|event| matches!(event, TextAreaEvent::ImeCommit(_)))
            .count()
    );
}

#[test]
fn type_emoji_emits_emoji_input_event_with_typed_value_grapheme_count_for_star_emoji()
-> Result<(), Box<dyn Error>> {
    let mut text_area = TextArea::new("Emoji");

    let result = text_area.apply_text_area_action(TextAreaAction::Type("⭐️".to_string()));
    let grapheme_count = result
        .events
        .iter()
        .find_map(|event| {
            if let TextAreaEvent::EmojiInput { grapheme_count } = event {
                Some(*grapheme_count)
            } else {
                None
            }
        })
        .ok_or_else(|| std::io::Error::other("EmojiInput should be emitted"))?;
    let expected_event = TextAreaEvent::EmojiInput { grapheme_count: 1 };
    let expected_grapheme_count = "⭐️".graphemes(true).count();

    assert!(result.handled);
    assert_eq!("⭐️", text_area.state().value);
    assert!(result.events.contains(&expected_event));
    assert_eq!(expected_grapheme_count, grapheme_count);
    assert_eq!(3, result.events.len());
    Ok(())
}

#[test]
fn ime_commit_emoji_emits_emoji_input_event_with_typed_value_grapheme_count_for_star_emoji()
-> Result<(), Box<dyn Error>> {
    let mut text_area = TextArea::new("Emoji").ime_enabled(true);

    let result = text_area.apply_text_area_action(TextAreaAction::ImeCommit("⭐️".to_string()));
    let grapheme_count = result
        .events
        .iter()
        .find_map(|event| {
            if let TextAreaEvent::EmojiInput { grapheme_count } = event {
                Some(*grapheme_count)
            } else {
                None
            }
        })
        .ok_or_else(|| std::io::Error::other("EmojiInput should be emitted"))?;
    let expected_event = TextAreaEvent::EmojiInput { grapheme_count: 1 };
    let expected_commit = TextAreaEvent::ImeCommit("⭐️".to_string());
    let expected_change = TextAreaEvent::Change("⭐️".to_string());
    let expected_grapheme_count = "⭐️".graphemes(true).count();

    assert!(result.handled);
    assert_eq!("⭐️", text_area.state().value);
    assert!(result.events.contains(&expected_commit));
    assert!(result.events.contains(&expected_event));
    assert!(result.events.contains(&expected_change));
    assert_eq!(expected_grapheme_count, grapheme_count);
    assert_eq!(3, result.events.len());
    Ok(())
}

#[test]
fn type_multiple_emojis_emits_emoji_input_event_with_grapheme_count_of_whole_typed_value()
-> Result<(), Box<dyn Error>> {
    let mut text_area = TextArea::new("Emoji");
    let value = "🙂😄";

    let result = text_area.apply_text_area_action(TextAreaAction::Type(value.to_string()));
    let grapheme_count = result
        .events
        .iter()
        .find_map(|event| {
            if let TextAreaEvent::EmojiInput { grapheme_count } = event {
                Some(*grapheme_count)
            } else {
                None
            }
        })
        .ok_or_else(|| std::io::Error::other("EmojiInput should be emitted"))?;

    assert!(result.handled);
    assert_eq!(value, text_area.state().value);
    assert_eq!(2, grapheme_count);
    assert_eq!(value.graphemes(true).count(), grapheme_count);
    Ok(())
}

#[test]
fn tab_behavior_moves_focus_or_inserts_tab_explicitly() {
    let mut move_focus = TextArea::new("Form").tab_behavior(TextAreaTabBehavior::MoveFocus);
    let mut insert_tab = TextArea::new("Code").tab_behavior(TextAreaTabBehavior::InsertTab);

    let focus = move_focus.handle_key(TextAreaKeyChord::tab());
    let insert = insert_tab.handle_key(TextAreaKeyChord::tab());

    assert!(
        focus
            .as_ref()
            .is_ok_and(|outcome| outcome.events.contains(&TextAreaEvent::FocusMove))
    );
    assert_eq!("", move_focus.state().value);
    let tab_event = TextAreaEvent::Change("\t".to_string());
    assert!(
        insert
            .as_ref()
            .is_ok_and(|outcome| outcome.events.contains(&tab_event))
    );
    assert_eq!("\t", insert_tab.state().value);
}
