use crate::molecule::command_chrome::{
    CommandChromeCapability, CommandChromeSearchPresentation, CommandChromeText,
    SearchControlCapabilities, SearchControlIcons, SearchControlStrings,
    SearchResultSummaryTemplate,
};
use crate::molecule::structured::{ReplaceMode, SearchOptions};

use super::sanitized_search_projection::{
    SanitizedSearchProjection, SanitizedSearchTextPresentation,
};

pub(super) struct SanitizedSearchPresentation {
    pub(super) label: String,
    pub(super) value: CommandChromeSearchPresentation,
}

impl From<&SanitizedSearchProjection> for SanitizedSearchPresentation {
    fn from(projection: &SanitizedSearchProjection) -> Self {
        let replace_visible = projection.replace.enabled || projection.replace_all.enabled;
        let navigation_enabled = projection.next.enabled || projection.previous.enabled;
        let regex_enabled = projection.regex.enabled;
        let presentation = &projection.presentation;

        Self {
            label: presentation.controls.strip.visible.clone(),
            value: CommandChromeSearchPresentation {
                query: String::new(),
                options: SearchOptions {
                    match_case: projection.match_case.enabled && projection.match_case.current,
                    whole_word: projection.whole_word.enabled && projection.whole_word.current,
                    use_regex: projection.regex.enabled && projection.regex.current,
                },
                result_count: None,
                active_index: None,
                replace_mode: if replace_visible {
                    ReplaceMode::Visible
                } else {
                    ReplaceMode::Hidden
                },
                replace_value: String::new(),
                strings: SearchControlStrings {
                    strip: text(&presentation.controls.strip),
                    query: text(&presentation.controls.query),
                    replace: text(&presentation.controls.replacement),
                    match_case: text(&presentation.controls.match_case),
                    whole_word: text(&presentation.controls.whole_word),
                    use_regex: text(&presentation.controls.regex),
                    previous: text(&presentation.operations.previous),
                    next: text(&presentation.operations.next),
                    replace_one: text(&presentation.operations.replace),
                    replace_all: text(&presentation.operations.replace_all),
                    close: text(&presentation.operations.close),
                    result_summary: SearchResultSummaryTemplate {
                        empty: presentation.result_summary.empty.clone(),
                        zero_results: presentation.result_summary.zero_results.clone(),
                        single_result: presentation.result_summary.single_result.clone(),
                        indexed_result: presentation.result_summary.indexed_result.clone(),
                        count_results: presentation.result_summary.count_results.clone(),
                    },
                },
                capabilities: SearchControlCapabilities {
                    regex: capability(regex_enabled, presentation.unavailable.regex.clone()),
                    replace: capability(replace_visible, presentation.unavailable.replace.clone()),
                    navigation: capability(
                        navigation_enabled,
                        presentation.unavailable.navigation.clone(),
                    ),
                    close: capability(
                        projection.close.enabled,
                        presentation.unavailable.close.clone(),
                    ),
                },
                icons: SearchControlIcons::default(),
            },
        }
    }
}

fn text(value: &SanitizedSearchTextPresentation) -> CommandChromeText {
    CommandChromeText::new(
        value.visible.clone(),
        value.tooltip.clone(),
        value.accessibility_label.clone(),
    )
}

fn capability(enabled: bool, unavailable: String) -> CommandChromeCapability {
    if enabled {
        CommandChromeCapability::available()
    } else {
        CommandChromeCapability::unavailable(unavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::super::sanitized_search_projection::{
        SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
        SanitizedSearchOperationPresentation, SanitizedSearchProjectionBuilder,
        SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget,
        SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
    };
    use super::SanitizedSearchPresentation;
    use crate::molecule::command_chrome::CommandChromeCapability;
    use crate::molecule::structured::ReplaceMode;

    fn text(value: &str) -> SanitizedSearchTextPresentation {
        SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
    }

    fn localized() -> SanitizedSearchLocalizedPresentation {
        SanitizedSearchLocalizedPresentation::new(
            SanitizedSearchControlPresentation::new(
                text("検索"),
                text("検索語"),
                text("置換"),
                text("大文字小文字"),
                text("単語"),
                text("正規表現"),
            ),
            SanitizedSearchOperationPresentation::new(
                text("前へ"),
                text("次へ"),
                text("置換"),
                text("すべて置換"),
                text("閉じる"),
            ),
            SanitizedSearchResultSummaryPresentation::new(
                "検索待機 ⭐️",
                "一致なし ⭐️",
                "一件 ⭐️",
                "位置 ⭐️",
                "件数 ⭐️",
            ),
            SanitizedSearchUnavailablePresentation::new(
                "正規表現は利用不可 ⭐️",
                "置換は利用不可 ⭐️",
                "移動は利用不可 ⭐️",
                "閉じる操作は利用不可 ⭐️",
            ),
        )
    }

    #[test]
    fn maps_required_localized_presentation_without_generated_fallbacks() {
        let projection = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized())
            .close_enabled(true)
            .close_target(SanitizedSearchTarget::from_opaque_bytes([1]))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([2]))
            .replace_all_enabled(true)
            .replace_all_target(SanitizedSearchTarget::from_opaque_bytes([3]))
            .build()
            .expect("検証済み");

        let presentation = SanitizedSearchPresentation::from(&projection);

        assert_eq!(presentation.label, "検索");
        assert_eq!(presentation.value.strings.strip.visible, "検索");
        assert_eq!(presentation.value.strings.query.visible, "検索語");
        assert_eq!(presentation.value.strings.replace.visible, "置換");
        assert_eq!(presentation.value.strings.next.visible, "次へ");
        assert_eq!(presentation.value.strings.next.tooltip, "次へ ⭐️");
        assert_eq!(
            presentation.value.strings.next.accessibility_label,
            "次へ ⭐️"
        );
        assert_eq!(
            presentation.value.strings.result_summary.empty,
            "検索待機 ⭐️"
        );
        assert_eq!(presentation.value.replace_mode, ReplaceMode::Visible);
        assert_eq!(presentation.value.query, "");
        assert_eq!(presentation.value.replace_value, "");
        assert_eq!(presentation.value.result_count, None);
        assert_eq!(presentation.value.active_index, None);
        assert!(!presentation.value.options.match_case);
        assert!(!presentation.value.options.whole_word);
        assert!(!presentation.value.options.use_regex);
        assert!(matches!(
            presentation.value.capabilities.navigation,
            CommandChromeCapability::Available
        ));
        assert!(matches!(
            presentation.value.capabilities.replace,
            CommandChromeCapability::Available
        ));
        assert!(matches!(
            presentation.value.capabilities.close,
            CommandChromeCapability::Available
        ));
        assert_eq!(
            presentation.value.capabilities.regex.disabled_reason(),
            Some("正規表現は利用不可 ⭐️")
        );
    }

    #[test]
    fn disabled_capabilities_consume_localized_unavailable_reasons() {
        let projection = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized())
            .build()
            .expect("検証済み");

        let presentation = SanitizedSearchPresentation::from(&projection).value;

        assert_eq!(
            presentation.capabilities.navigation.disabled_reason(),
            Some("移動は利用不可 ⭐️")
        );
        assert_eq!(
            presentation.capabilities.replace.disabled_reason(),
            Some("置換は利用不可 ⭐️")
        );
        assert_eq!(
            presentation.capabilities.close.disabled_reason(),
            Some("閉じる操作は利用不可 ⭐️")
        );
        assert_eq!(presentation.replace_mode, ReplaceMode::Hidden);
    }

    #[test]
    fn regex_projection_enables_regex_presentation_from_generic_projection() {
        let projection = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized())
            .regex_target(SanitizedSearchTarget::from_opaque_bytes([9]))
            .build()
            .expect("検証済み");

        let presentation = SanitizedSearchPresentation::from(&projection).value;

        assert!(!presentation.options.use_regex);
        assert!(matches!(
            presentation.capabilities.regex,
            CommandChromeCapability::Available
        ));
    }

    #[test]
    fn maps_host_projected_option_state_without_inference_from_targets() {
        let projection = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized())
            .match_case_state(true)
            .match_case_target(SanitizedSearchTarget::from_opaque_bytes([1]))
            .whole_word_state(false)
            .whole_word_target(SanitizedSearchTarget::from_opaque_bytes([2]))
            .regex_state(true)
            .regex_target(SanitizedSearchTarget::from_opaque_bytes([3]))
            .build()
            .expect("検証済み");

        let options = SanitizedSearchPresentation::from(&projection).value.options;
        assert!(options.match_case);
        assert!(!options.whole_word);
        assert!(options.use_regex);
    }

    #[test]
    fn target_absence_cannot_make_an_option_checked() {
        let projection = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized())
            .match_case_state(true)
            .whole_word_state(true)
            .regex_state(true)
            .build()
            .expect("検証済み");

        let options = SanitizedSearchPresentation::from(&projection).value.options;
        assert!(!options.match_case);
        assert!(!options.whole_word);
        assert!(!options.use_regex);
    }
}
