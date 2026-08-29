use katana_ui_core::molecule::command_chrome::{
    CommandChromeCapability, CommandChromeSearchPresentation, CommandChromeSearchStrip,
    CommandChromeText, SearchControlCapabilities, SearchControlIconSlot, SearchControlIcons,
    SearchControlStrings, SearchResultSummaryParameters, SearchResultSummaryTemplate,
};
use katana_ui_core::molecule::structured::{ReplaceMode, SearchControlStrip, SearchOptions};
use katana_ui_core::render_model::UiIconProps;

#[test]
fn search_result_summary_template_formats_all_contract_branches_without_locale_state() {
    let template = template();

    assert_eq!(
        "未入力",
        template.format(SearchResultSummaryParameters::new(None, None))
    );
    assert_eq!(
        "0件",
        template.format(SearchResultSummaryParameters::new(Some(0), None))
    );
    assert_eq!(
        "1件",
        template.format(SearchResultSummaryParameters::new(Some(1), None))
    );
    assert_eq!(
        "2件 (集計中)",
        template.format(SearchResultSummaryParameters::new(Some(2), None))
    );
    assert_eq!(
        "3件中 / 2件目",
        template.format(SearchResultSummaryParameters::new(Some(2), Some(1)))
    );
    assert_eq!(
        "3件中 / 3件目",
        template.format(SearchResultSummaryParameters::new(Some(3), Some(99)))
    );
}

#[test]
fn search_control_icons_are_slot_addressable_and_replace_previous_slot_when_reassigned() {
    let replace = icon("replace");
    let next = icon("next");

    let icons = SearchControlIcons::default()
        .icon(SearchControlIconSlot::ReplaceOne, icon("old"))
        .icon(SearchControlIconSlot::ReplaceOne, replace.clone())
        .icon(SearchControlIconSlot::Next, next.clone());

    assert_eq!(
        Some(&replace),
        icons.icon_for(SearchControlIconSlot::ReplaceOne)
    );
    assert_eq!(Some(&next), icons.icon_for(SearchControlIconSlot::Next));
    assert!(icons.icon_for(SearchControlIconSlot::Close).is_none());
}

#[test]
fn search_strip_synchronization_contract_keeps_noop_and_capability_reasons_explicit() {
    let mut strip = CommandChromeSearchStrip::new(
        SearchControlStrip::new("search-contract"),
        japanese_strings(),
    );
    let presentation = strip_presentation();

    assert!(strip.synchronize_presentation(presentation.clone()));
    assert_eq!(Some(3), strip.result_count_model());
    assert_eq!(
        Some(2),
        strip
            .active_index_model()
            .map(|active| active.saturating_add(1))
    );
    assert!(!strip.synchronize_presentation(presentation));

    let mut unavailable = strip_presentation();
    unavailable.capabilities = SearchControlCapabilities {
        regex: CommandChromeCapability::unavailable("ok regex"),
        replace: CommandChromeCapability::available(),
        navigation: CommandChromeCapability::available(),
        close: CommandChromeCapability::available(),
    };
    unavailable.query = "updated".to_string();
    assert!(strip.synchronize_presentation(unavailable));
    let mut repeated = strip_presentation();
    repeated.query = "updated".to_string();
    repeated.capabilities = SearchControlCapabilities {
        regex: CommandChromeCapability::unavailable("ok regex"),
        replace: CommandChromeCapability::available(),
        navigation: CommandChromeCapability::available(),
        close: CommandChromeCapability::available(),
    };
    assert!(!strip.synchronize_presentation(repeated));

    assert_eq!(strip.query_model(), "updated");
    assert_eq!(
        Some("ok regex"),
        strip.capabilities_model().regex.disabled_reason()
    );
}

fn template() -> SearchResultSummaryTemplate {
    SearchResultSummaryTemplate {
        empty: "未入力".to_string(),
        zero_results: "0件".to_string(),
        single_result: "1件".to_string(),
        indexed_result: "3件中 / {active}件目".to_string(),
        count_results: "{count}件 (集計中)".to_string(),
    }
}

fn icon(label: &str) -> UiIconProps {
    UiIconProps::new(format!(
        "<svg viewBox=\"0 0 16 16\"><title>{label}</title><path d=\"M1 1h14v14H1z\"/></svg>"
    ))
}

fn strip_presentation() -> CommandChromeSearchPresentation {
    CommandChromeSearchPresentation {
        query: "foo".to_string(),
        options: SearchOptions::default(),
        result_count: Some(3),
        active_index: Some(1),
        replace_mode: ReplaceMode::Visible,
        replace_value: String::new(),
        strings: japanese_strings(),
        capabilities: SearchControlCapabilities::all_available(),
        icons: SearchControlIcons::default()
            .icon(SearchControlIconSlot::Close, icon("close"))
            .icon(SearchControlIconSlot::ReplaceOne, icon("replace")),
    }
}

fn japanese_strings() -> SearchControlStrings {
    SearchControlStrings {
        strip: text("検索"),
        query: text("検索対象"),
        replace: text("置換"),
        match_case: text("大文字小文字"),
        whole_word: text("単語一致"),
        use_regex: text("正規表現"),
        previous: text("戻る"),
        next: text("進む"),
        replace_one: text("置換1件"),
        replace_all: text("すべて置換"),
        close: text("閉じる"),
        result_summary: template(),
    }
}

fn text(value: &str) -> CommandChromeText {
    CommandChromeText::new(value, value, value)
}
