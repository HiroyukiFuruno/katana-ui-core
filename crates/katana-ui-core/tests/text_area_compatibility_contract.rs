use katana_ui_core::atom::{
    TextArea, TextAreaAction, TextAreaCaretMove, TextAreaCompositionPhase,
    TextAreaCompositionState, TextAreaEvent, TextAreaKeyChord, TextAreaNewlineKey, TextAreaOptions,
    TextAreaResizeDelta, TextAreaResizeEvent, TextAreaSelection, TextAreaState, TextAreaSubmitKey,
    TextAreaTabBehavior, TextAreaValidationError, TextAreaWrapPolicy,
};
use katana_ui_core::render_model::UiStateId;

#[test]
fn legacy_text_area_public_dtos_keep_their_serialized_shape() -> serde_json::Result<()> {
    let composition = TextAreaCompositionState {
        phase: TextAreaCompositionPhase::Update,
        preedit: "⭐️".to_string(),
        caret: "⭐️".len(),
    };
    let options = TextAreaOptions {
        value: "日本語 ⭐️".to_string(),
        placeholder: "入力".to_string(),
        font_role: "body".to_string(),
        disabled: false,
        readonly: true,
        invalid: false,
        min_rows: 1,
        max_rows: 8,
        auto_grow: true,
        wrap_policy: TextAreaWrapPolicy::Soft,
        submit_key: TextAreaSubmitKey::ModEnter,
        newline_key: TextAreaNewlineKey::Enter,
        tab_behavior: TextAreaTabBehavior::InsertTab,
        ime_enabled: true,
        resize_enabled: true,
        vertical_scroll_enabled: true,
        horizontal_scroll_enabled: false,
        vertical_scrollbar_visible: true,
        horizontal_scrollbar_visible: false,
        leading_slot: None,
        trailing_slot: None,
        trailing_icon_buttons: Vec::new(),
        clear_action: None,
    };
    let state = TextAreaState {
        state_id: UiStateId::new("legacy.text-area"),
        value: options.value.clone(),
        caret: options.value.len(),
        selection: TextAreaSelection { start: 0, end: 3 },
        composition: Some(composition.clone()),
        focused: true,
        disabled: false,
        readonly: true,
        invalid: false,
        measured_rows: 2,
        internal_scroll: false,
        resize_width_delta: 12,
        resize_height_delta: 4,
    };
    let text_area = TextArea::new("既存の入力")
        .stable_state_id("legacy.builder")
        .value("日本語 ⭐️")
        .placeholder("入力")
        .font_role("body")
        .readonly(true)
        .min_rows(1)
        .max_rows(8)
        .auto_grow(true)
        .wrap_policy(TextAreaWrapPolicy::Soft)
        .submit_key(TextAreaSubmitKey::ModEnter)
        .newline_key(TextAreaNewlineKey::Enter)
        .tab_behavior(TextAreaTabBehavior::InsertTab)
        .ime_enabled(true)
        .resize_enabled(true)
        .vertical_scroll_enabled(true)
        .vertical_scrollbar_visible(true);

    assert_eq!(
        options,
        serde_json::from_value(serde_json::to_value(&options)?)?
    );
    assert_eq!(
        state,
        serde_json::from_value(serde_json::to_value(&state)?)?
    );
    assert_eq!(
        text_area,
        serde_json::from_value(serde_json::to_value(&text_area)?)?
    );
    let value = serde_json::to_value(&text_area)?;
    assert_eq!(Some("既存の入力"), value["label"].as_str());
    assert_eq!(Some("日本語 ⭐️"), value["options"]["value"].as_str());
    assert_eq!(Some("legacy.builder"), value["state"]["state_id"].as_str());
    Ok(())
}

#[test]
fn legacy_text_area_action_and_event_consumers_remain_exhaustive() {
    let composition = TextAreaCompositionState::new(TextAreaCompositionPhase::Start, "に", 3);
    let actions = vec![
        TextAreaAction::Type("日本語".to_string()),
        TextAreaAction::Submit,
        TextAreaAction::InsertNewline,
        TextAreaAction::Clear,
        TextAreaAction::MoveCaret(TextAreaCaretMove::PreviousGrapheme),
        TextAreaAction::MoveCaret(TextAreaCaretMove::NextGrapheme),
        TextAreaAction::MoveCaret(TextAreaCaretMove::Start),
        TextAreaAction::MoveCaret(TextAreaCaretMove::End),
        TextAreaAction::MoveCaret(TextAreaCaretMove::To(3)),
        TextAreaAction::Select(TextAreaSelection { start: 0, end: 3 }),
        TextAreaAction::ImeComposition(composition.clone()),
        TextAreaAction::ImeCommit("⭐️".to_string()),
        TextAreaAction::DeleteBackward,
        TextAreaAction::Resize(TextAreaResizeDelta::new(8, 4)),
    ];
    let events = vec![
        TextAreaEvent::KeyInput(TextAreaKeyChord::enter()),
        TextAreaEvent::TextInput("日本語".to_string()),
        TextAreaEvent::ImeComposition(composition),
        TextAreaEvent::ImeCommit("⭐️".to_string()),
        TextAreaEvent::EmojiInput { grapheme_count: 1 },
        TextAreaEvent::Submit("submitted".to_string()),
        TextAreaEvent::Change("changed".to_string()),
        TextAreaEvent::Focus,
        TextAreaEvent::Blur,
        TextAreaEvent::Resize(TextAreaResizeEvent {
            rows: 3,
            internal_scroll: true,
            width_delta: 8,
            height_delta: 4,
        }),
        TextAreaEvent::InsertNewline,
        TextAreaEvent::FocusMove,
    ];

    assert_eq!(
        vec![
            "type",
            "submit",
            "newline",
            "clear",
            "move",
            "move",
            "move",
            "move",
            "move",
            "select",
            "composition",
            "commit",
            "delete",
            "resize",
        ],
        actions
            .into_iter()
            .map(legacy_action_name)
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        vec![
            "key",
            "text",
            "composition",
            "commit",
            "emoji",
            "submit",
            "change",
            "focus",
            "blur",
            "resize",
            "newline",
            "focus-move",
        ],
        events
            .into_iter()
            .map(legacy_event_name)
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        "conflict",
        legacy_validation_error_name(TextAreaValidationError::ConflictingKeyBindings)
    );
}

fn legacy_action_name(action: TextAreaAction) -> &'static str {
    match action {
        TextAreaAction::Type(_) => "type",
        TextAreaAction::Submit => "submit",
        TextAreaAction::InsertNewline => "newline",
        TextAreaAction::Clear => "clear",
        TextAreaAction::MoveCaret(_) => "move",
        TextAreaAction::Select(_) => "select",
        TextAreaAction::ImeComposition(_) => "composition",
        TextAreaAction::ImeCommit(_) => "commit",
        TextAreaAction::DeleteBackward => "delete",
        TextAreaAction::Resize(_) => "resize",
    }
}

fn legacy_event_name(event: TextAreaEvent) -> &'static str {
    match event {
        TextAreaEvent::KeyInput(_) => "key",
        TextAreaEvent::TextInput(_) => "text",
        TextAreaEvent::ImeComposition(_) => "composition",
        TextAreaEvent::ImeCommit(_) => "commit",
        TextAreaEvent::EmojiInput { .. } => "emoji",
        TextAreaEvent::Submit(_) => "submit",
        TextAreaEvent::Change(_) => "change",
        TextAreaEvent::Focus => "focus",
        TextAreaEvent::Blur => "blur",
        TextAreaEvent::Resize(_) => "resize",
        TextAreaEvent::InsertNewline => "newline",
        TextAreaEvent::FocusMove => "focus-move",
    }
}

fn legacy_validation_error_name(error: TextAreaValidationError) -> &'static str {
    match error {
        TextAreaValidationError::ConflictingKeyBindings => "conflict",
        TextAreaValidationError::MinRowsMustBePositive => "min-rows",
        TextAreaValidationError::MaxRowsBelowMinRows => "max-rows",
        TextAreaValidationError::VerticalScrollbarRequiresVerticalScroll => "vertical-scroll",
        TextAreaValidationError::HorizontalScrollbarRequiresHorizontalScroll => "horizontal-scroll",
    }
}
