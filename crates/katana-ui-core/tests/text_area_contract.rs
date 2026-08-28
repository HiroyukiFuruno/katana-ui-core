use katana_ui_core::atom::{
    TextArea, TextAreaAction, TextAreaCaretMove, TextAreaCompositionPhase, TextAreaEvent,
    TextAreaKey, TextAreaKeyChord, TextAreaNewlineKey, TextAreaSelection, TextAreaSubmitKey,
    TextAreaTabBehavior, TextAreaValidationError, TextAreaWrapPolicy,
};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::{
    UiCommonProps, UiNode, UiNodeKind, UiSlotPlacement, UiVisualRole,
};
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
fn unconfigured_key_chord_is_ignored_without_mutating_state() {
    let mut text_area = TextArea::new("Composer");
    let before = text_area.state().clone();

    let outcome = text_area.handle_key(TextAreaKeyChord {
        key: TextAreaKey::Enter,
        shift: true,
        primary_modifier: true,
    });
    assert!(outcome.is_ok(), "valid text area");
    let Ok(outcome) = outcome else {
        return;
    };

    assert!(!outcome.handled);
    assert!(outcome.events.is_empty());
    assert_eq!(&before, text_area.state());
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
fn editable_clear_common_props_and_fixed_row_measurement_are_explicit() {
    let mut editable = TextArea::new("Editable").value("draft");
    let cleared = editable.apply_text_area_action(TextAreaAction::Clear);
    assert!(cleared.handled);
    assert_eq!("", editable.state().value);
    assert_eq!(
        [TextAreaEvent::Change(String::new())],
        cleared.events.as_slice()
    );

    let common = UiCommonProps {
        disabled: true,
        ..UiCommonProps::default()
    };
    let fixed = TextArea::new("Fixed")
        .value("one\ntwo")
        .min_rows(0)
        .max_rows(0)
        .auto_grow(false)
        .common(common);
    assert_eq!(1, fixed.state().measured_rows);
    assert!(fixed.state().internal_scroll);
    let node = UiNode::from(fixed);
    assert!(node.props().common.disabled);
    assert!(node.props().disabled);
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

#[test]
fn text_area_grapheme_navigation_selection_and_disabled_ime_cover_boundaries() {
    let value = "e\u{301}👍🏻✈️";
    let mut text_area = TextArea::new("Unicode").value(value);

    assert!(
        text_area
            .apply_text_area_action(TextAreaAction::MoveCaret(TextAreaCaretMove::Start))
            .handled
    );
    assert!(
        text_area
            .apply_text_area_action(TextAreaAction::MoveCaret(TextAreaCaretMove::NextGrapheme))
            .handled
    );
    assert_eq!("e\u{301}".len(), text_area.state().caret);
    assert!(
        text_area
            .apply_text_area_action(TextAreaAction::MoveCaret(TextAreaCaretMove::End))
            .handled
    );
    assert_eq!(value.len(), text_area.state().caret);
    assert!(
        text_area
            .apply_text_area_action(TextAreaAction::MoveCaret(TextAreaCaretMove::To(2)))
            .handled
    );
    assert_eq!(1, text_area.state().caret);
    let _ = text_area.apply_text_area_action(TextAreaAction::MoveCaret(TextAreaCaretMove::Start));
    assert!(
        !text_area
            .apply_text_area_action(TextAreaAction::DeleteBackward)
            .handled
    );

    let selected = text_area.apply_text_area_action(TextAreaAction::Select(TextAreaSelection {
        start: value.len(),
        end: 0,
    }));
    assert!(selected.handled);
    assert_eq!(0, selected.state.caret);
    let replaced = text_area.apply_text_area_action(TextAreaAction::Type("🙂".to_string()));
    assert_eq!("🙂", replaced.state.value);
    assert!(
        replaced
            .events
            .iter()
            .any(|event| matches!(event, TextAreaEvent::EmojiInput { grapheme_count: 1 }))
    );

    let mut no_ime = TextArea::new("No IME").ime_enabled(false);
    assert!(
        !no_ime
            .apply_text_area_action(TextAreaAction::composition(
                TextAreaCompositionPhase::Start,
                "に",
                3
            ))
            .handled
    );
    assert!(
        !no_ime
            .apply_text_area_action(TextAreaAction::ime_commit("日本語"))
            .handled
    );
}

#[test]
fn text_area_component_actions_options_and_key_validation_cover_remaining_paths() {
    let mut text_area = TextArea::new("Composer").value("draft");
    assert_eq!("draft", text_area.options().value);

    assert!(
        text_area
            .apply_action(&UiAction::copy_selection(text_area.state_id().clone()))
            .handled
    );
    assert!(
        text_area
            .apply_action(&UiAction::input_submitted(text_area.state_id().clone()))
            .handled
    );
    assert!(
        text_area
            .apply_action(&UiAction::clear_value(text_area.state_id().clone()))
            .handled
    );
    assert_eq!("", text_area.state().value);
    assert!(
        text_area
            .apply_action(&UiAction::blur(text_area.state_id().clone()))
            .handled
    );
    assert!(text_area.events().contains(&TextAreaEvent::Blur));

    assert_eq!(
        Err(TextAreaValidationError::MinRowsMustBePositive),
        TextArea::new("Rows").min_rows(0).validate()
    );
    assert_eq!(
        Err(TextAreaValidationError::MaxRowsBelowMinRows),
        TextArea::new("Rows").min_rows(4).max_rows(3).validate()
    );

    let mut mod_enter = TextArea::new("Submit")
        .submit_key(TextAreaSubmitKey::ModEnter)
        .newline_key(TextAreaNewlineKey::Disabled);
    assert!(
        mod_enter
            .handle_key(TextAreaKeyChord::mod_enter())
            .is_ok_and(|outcome| outcome.handled)
    );
}

#[test]
fn text_area_component_action_replaces_selected_graphemes_on_paste() {
    let mut text_area = TextArea::new("Composer").value("a日本語z");
    let target = text_area.state_id().clone();

    assert!(
        text_area
            .apply_action(&UiAction::cursor_selection(target.clone(), 4, 1, 4))
            .handled
    );
    let pasted = text_area.apply_action(&UiAction::paste_text(target, "KUC"));

    assert!(pasted.handled);
    assert_eq!("aKUCz", text_area.state().value);
    assert_eq!(4, text_area.state().caret);
    assert_eq!(TextAreaSelection::collapsed(4), text_area.state().selection);
}
