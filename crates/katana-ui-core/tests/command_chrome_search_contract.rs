use katana_ui_core::molecule::command_chrome::{
    CommandChromeCapability, CommandChromeSearchAction, CommandChromeSearchEvent,
    CommandChromeSearchPresentation, CommandChromeSearchStrip, CommandChromeText,
    SearchControlCapabilities, SearchControlIconSlot, SearchControlIcons, SearchControlStrings,
    SearchResultSummaryTemplate,
};
use katana_ui_core::molecule::structured::{
    ReplaceMode, SearchControlStrip, SearchControlStripAction, SearchControlStripEvent,
    SearchNavigationDirection, SearchOptionKind, SearchReplaceScope,
};
use katana_ui_core::render_model::UiIconProps;

#[test]
fn host_injected_japanese_and_variation_selector_strings_drive_presentation() {
    let strings = japanese_strings();
    let strip = CommandChromeSearchStrip::new(SearchControlStrip::new("legacy"), strings.clone());

    assert_eq!("検索 ⭐️", strip.strings_model().strip.visible);
    assert_eq!("検索語", strip.strings_model().query.accessibility_label);
    assert_eq!("置換", strip.strings_model().replace.visible);
    assert_eq!("正規表現", strip.strings_model().use_regex.tooltip);
    assert_eq!("閉じる", strip.strings_model().close.accessibility_label);
    assert_eq!("", strip.result_summary_model());
}

#[test]
fn renderer_reuses_the_generic_search_state_identity() {
    let strip =
        CommandChromeSearchStrip::new(SearchControlStrip::new("legacy"), japanese_strings());

    let first = strip.state_id_model().as_str().to_string();
    let second = strip.state_id_model().as_str().to_string();

    assert!(!first.is_empty());
    assert_eq!(first, second);
}

#[test]
fn consumer_stable_state_id_propagates_to_command_chrome_search_strip() {
    let state_id = "storybook.command-chrome.search";
    let strip = CommandChromeSearchStrip::new(
        SearchControlStrip::new("legacy").stable_state_id(state_id),
        japanese_strings(),
    );

    assert_eq!(state_id, strip.state_id_model().as_str());
}

#[test]
fn controlled_search_presentation_updates_without_synthesizing_an_interaction_event() {
    let mut strip =
        CommandChromeSearchStrip::new(SearchControlStrip::new("legacy"), japanese_strings());
    let mut strings = japanese_strings();
    strings.strip = text("同期後の検索");
    assert!(
        strip.synchronize_presentation(CommandChromeSearchPresentation {
            query: "日本語 ⭐️".to_string(),
            options: Default::default(),
            result_count: Some(2),
            active_index: Some(0),
            replace_mode: ReplaceMode::Visible,
            replace_value: "置換".to_string(),
            strings,
            capabilities: SearchControlCapabilities::all_available(),
            icons: SearchControlIcons::default(),
        })
    );
    assert_eq!(strip.query_model(), "日本語 ⭐️");
    assert_eq!(strip.replace_value_model(), "置換");
    assert_eq!(strip.result_count_model(), Some(2));
    assert_eq!(strip.strings_model().strip.visible, "同期後の検索");
}

#[test]
fn equal_consumer_state_ids_remain_equal_and_distinct_ids_remain_distinct() {
    let first = SearchControlStrip::new("first").stable_state_id("consumer.search.shared");
    let second = SearchControlStrip::new("second").stable_state_id("consumer.search.shared");
    let other = SearchControlStrip::new("other").stable_state_id("consumer.search.other");

    assert_eq!(first.state_id(), second.state_id());
    assert_ne!(first.state_id(), other.state_id());
}

#[test]
fn icon_presentation_and_result_accessors_are_additive_to_the_search_contract() {
    let icon = UiIconProps::new("<svg viewBox=\"0 0 16 16\"><path d=\"M1 8h14\"/></svg>");
    let strip = CommandChromeSearchStrip::new(
        SearchControlStrip::new("legacy").result_position(3, Some(1)),
        japanese_strings(),
    )
    .icons(SearchControlIcons::default().icon(SearchControlIconSlot::Next, icon.clone()));

    assert_eq!(Some(3), strip.result_count_model());
    assert_eq!(Some(1), strip.active_index_model());
    assert_eq!(
        Some(&icon),
        strip.icons_model().icon_for(SearchControlIconSlot::Next)
    );
    assert!(
        strip
            .icons_model()
            .icon_for(SearchControlIconSlot::Close)
            .is_none()
    );
}

#[test]
fn result_summary_template_is_serializable_host_presentation_without_a_locale_enum() {
    let mut strip =
        CommandChromeSearchStrip::new(SearchControlStrip::new("legacy"), japanese_strings());
    let _ = strip.apply_action(CommandChromeSearchAction::Strip {
        action: SearchControlStripAction::SetResultPosition {
            result_count: 12,
            active_index: Some(2),
        },
    });

    assert_eq!("3 件目 / 12 件", strip.result_summary_model());
}

#[test]
fn result_summary_keeps_existing_result_position_clamping_semantics() {
    let mut strip =
        CommandChromeSearchStrip::new(SearchControlStrip::new("legacy"), japanese_strings());
    let _ = strip.apply_action(CommandChromeSearchAction::Strip {
        action: SearchControlStripAction::SetResultPosition {
            result_count: 2,
            active_index: Some(9),
        },
    });

    assert_eq!("2 件目 / 2 件", strip.result_summary_model());
}

#[test]
fn query_and_replace_stay_typed_consumer_requests_without_search_execution() {
    let mut strip = CommandChromeSearchStrip::new(
        SearchControlStrip::new("legacy")
            .replace_mode(ReplaceMode::Visible)
            .replace_value("新しい値"),
        japanese_strings(),
    );

    assert_eq!(
        vec![CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchQueryChanged("見出し".to_string()),
        }],
        strip.apply_action(CommandChromeSearchAction::Strip {
            action: SearchControlStripAction::SetSearchQuery("見出し".to_string()),
        })
    );
    assert_eq!(
        vec![CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::ReplaceRequested {
                scope: SearchReplaceScope::All,
                value: "新しい値".to_string(),
            },
        }],
        strip.apply_action(CommandChromeSearchAction::Strip {
            action: SearchControlStripAction::Replace(SearchReplaceScope::All),
        })
    );
}

#[test]
fn unavailable_controls_expose_reasons_and_emit_no_operation_request() {
    let capabilities = SearchControlCapabilities {
        regex: CommandChromeCapability::unavailable("この画面では使えません"),
        replace: CommandChromeCapability::unavailable("置換は使えません"),
        navigation: CommandChromeCapability::unavailable("移動先がありません"),
        close: CommandChromeCapability::unavailable("閉じられません"),
    };
    let mut strip = CommandChromeSearchStrip::new(
        SearchControlStrip::new("legacy").replace_mode(ReplaceMode::Visible),
        japanese_strings(),
    )
    .capabilities(capabilities);

    assert_eq!(
        Some("この画面では使えません"),
        strip.capabilities_model().regex.disabled_reason()
    );
    assert!(
        strip
            .apply_action(CommandChromeSearchAction::Strip {
                action: SearchControlStripAction::ToggleSearchOption(SearchOptionKind::UseRegex),
            })
            .is_empty()
    );
    assert!(
        strip
            .apply_action(CommandChromeSearchAction::Strip {
                action: SearchControlStripAction::Navigate(SearchNavigationDirection::Next),
            })
            .is_empty()
    );
    assert!(
        strip
            .apply_action(CommandChromeSearchAction::Strip {
                action: SearchControlStripAction::Replace(SearchReplaceScope::One),
            })
            .is_empty()
    );
    assert!(
        strip
            .apply_action(CommandChromeSearchAction::RequestClose)
            .is_empty()
    );
    assert!(!strip.options_model().use_regex);
}

#[test]
fn close_event_is_additive_and_leaves_query_and_replace_state_unchanged() {
    let mut strip = CommandChromeSearchStrip::new(
        SearchControlStrip::new("legacy")
            .query("検索対象")
            .replace_mode(ReplaceMode::Visible)
            .replace_value("置換対象"),
        japanese_strings(),
    );

    assert_eq!(
        vec![CommandChromeSearchEvent::CloseRequested],
        strip.apply_action(CommandChromeSearchAction::RequestClose)
    );
    assert_eq!("検索対象", strip.query_model());
    assert_eq!("置換対象", strip.replace_value_model());
}

fn japanese_strings() -> SearchControlStrings {
    SearchControlStrings {
        strip: text("検索 ⭐️"),
        query: text("検索語"),
        replace: text("置換"),
        match_case: text("大文字小文字を区別"),
        whole_word: text("単語単位"),
        use_regex: text("正規表現"),
        previous: text("前へ"),
        next: text("次へ"),
        replace_one: text("置換"),
        replace_all: text("すべて置換"),
        close: text("閉じる"),
        result_summary: SearchResultSummaryTemplate {
            empty: String::new(),
            zero_results: "0 件".to_string(),
            single_result: "1 / 1".to_string(),
            indexed_result: "{active} 件目 / {count} 件".to_string(),
            count_results: "{count} 件".to_string(),
        },
    }
}

fn text(value: &str) -> CommandChromeText {
    CommandChromeText::new(value, value, value)
}
