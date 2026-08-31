use super::operation::SanitizedSearchOperation;
use super::{
    SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
    SanitizedSearchOperationPresentation, SanitizedSearchOperationSlot,
    SanitizedSearchProjectionBuildError, SanitizedSearchProjectionBuilder,
    SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget,
    SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
};

fn text(value: &str) -> SanitizedSearchTextPresentation {
    SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
}

fn localized(next: &str) -> SanitizedSearchLocalizedPresentation {
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
            text(next),
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

include!("sanitized_search_projection_inline_tests/build_validation.rs");
include!("sanitized_search_projection_inline_tests/fingerprint_debug.rs");
