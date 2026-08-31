fn input(revision: u64, identity: &[u8], snapshot: &str) -> SanitizedDocumentRootInput {
    SanitizedDocumentRootInput::new(
        revision,
        SanitizedDocumentRootIdentity::from_opaque_bytes(identity.to_vec()),
        snapshot,
        SanitizedDocumentRootStyleKey::Default,
    )
}

fn input_with_tabs(revision: u64) -> SanitizedDocumentRootInput {
    input(revision, b"document", "本文 ⭐️").with_tab_projection(SanitizedTabProjection::new([
        SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "ドキュメント",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "最初")
                .with_capabilities(
                    SanitizedTabCapabilities::new()
                        .active_state(true)
                        .close_state(true),
                ),
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([2]), 1, "次の文書")
                .with_capabilities(SanitizedTabCapabilities::new().close_state(true))
                .with_close_presentation(SanitizedTabClosePresentation::new(
                    "×",
                    "閉じる",
                    "次の文書を閉じる",
                )),
        ),
    ]))
}

fn input_with_search(revision: u64) -> SanitizedDocumentRootInput {
    input(revision, b"document", "本文 ⭐️").with_search_projection(
        super::super::sanitized_document_root_process::SearchProjectionForIme::build(
            SEARCH_IME_KEYBOARD_TARGET,
            SEARCH_IME_COMMAND_TARGET,
        )
        .expect("search projection fixture is valid"),
    )
}

fn search_text(value: &str) -> SanitizedSearchTextPresentation {
    SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
}

fn search_localized() -> SanitizedSearchLocalizedPresentation {
    SanitizedSearchLocalizedPresentation::new(
        SanitizedSearchControlPresentation::new(
            search_text("検索"),
            search_text("検索語"),
            search_text("置換"),
            search_text("大文字小文字"),
            search_text("単語"),
            search_text("正規表現"),
        ),
        SanitizedSearchOperationPresentation::new(
            search_text("前へ"),
            search_text("次へ"),
            search_text("置換"),
            search_text("すべて置換"),
            search_text("閉じる"),
        ),
        SanitizedSearchResultSummaryPresentation::new(
            "検索待機 ⭐️",
            "一致なし",
            "1件",
            "{active} / {count}",
            "{count}件",
        ),
        SanitizedSearchUnavailablePresentation::new(
            "正規表現は利用不可",
            "置換は利用不可",
            "移動は利用不可",
            "閉じる操作は利用不可",
        ),
    )
}

fn input_with_recorders(
    revision: u64,
    text_events: Rc<RefCell<Vec<(SanitizedSearchTextOperation, String)>>>,
    unit_events: Rc<RefCell<Vec<SanitizedSearchUnitOperation>>>,
) -> SanitizedDocumentRootInput {
    let text_target = |operation| {
        let events = text_events.clone();
        let callback_events = events.clone();
        let callback = move |actual, value| {
            callback_events.borrow_mut().push((actual, value));
            Ok::<(), ()>(())
        };
        callback(operation, String::new()).expect("text callback fixture");
        events.borrow_mut().clear();
        SanitizedSearchTarget::from_opaque_bytes([0]).with_text_capability(callback)
    };
    let unit_target = |operation| {
        let events = unit_events.clone();
        let _ = operation;
        SanitizedSearchTarget::from_opaque_bytes([0]).with_unit_capability(move |actual| {
            events.borrow_mut().push(actual);
            Ok::<(), ()>(())
        })
    };
    let projection = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(search_localized())
        .query_target(text_target(SanitizedSearchTextOperation::Query))
        .replacement_target(text_target(SanitizedSearchTextOperation::Replacement))
        .match_case_target(unit_target(SanitizedSearchUnitOperation::MatchCase(false)))
        .whole_word_target(unit_target(SanitizedSearchUnitOperation::WholeWord(false)))
        .regex_target(unit_target(SanitizedSearchUnitOperation::Regex(false)))
        .close_enabled(true)
        .close_target(unit_target(SanitizedSearchUnitOperation::Close))
        .next_enabled(true)
        .next_target(unit_target(SanitizedSearchUnitOperation::Next))
        .previous_enabled(true)
        .previous_target(unit_target(SanitizedSearchUnitOperation::Previous))
        .replace_enabled(true)
        .replace_target(text_target(SanitizedSearchTextOperation::Replace))
        .replace_all_enabled(true)
        .replace_all_target(text_target(SanitizedSearchTextOperation::ReplaceAll))
        .build()
        .expect("complete search projection is valid");
    input(revision, b"document", "本文 ⭐️").with_search_projection(projection)
}

fn input_with_rejecting_recorders(
    revision: u64,
    text_calls: Rc<RefCell<usize>>,
    unit_calls: Rc<RefCell<usize>>,
) -> SanitizedDocumentRootInput {
    let text_target = || {
        let calls = text_calls.clone();
        SanitizedSearchTarget::from_opaque_bytes([0]).with_text_capability(
            move |_operation, _value| {
                *calls.borrow_mut() += 1;
                Err::<(), ()>(())
            },
        )
    };
    let unit_target = || {
        let calls = unit_calls.clone();
        SanitizedSearchTarget::from_opaque_bytes([0]).with_unit_capability(move |_operation| {
            *calls.borrow_mut() += 1;
            Err::<(), ()>(())
        })
    };
    let projection = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(search_localized())
        .query_target(text_target())
        .replacement_target(text_target())
        .match_case_target(unit_target())
        .whole_word_target(unit_target())
        .regex_target(unit_target())
        .close_enabled(true)
        .close_target(unit_target())
        .next_enabled(true)
        .next_target(unit_target())
        .previous_enabled(true)
        .previous_target(unit_target())
        .replace_enabled(true)
        .replace_target(text_target())
        .replace_all_enabled(true)
        .replace_all_target(text_target())
        .build()
        .expect("complete rejecting search projection is valid");
    input(revision, b"document", "本文 ⭐️").with_search_projection(projection)
}
