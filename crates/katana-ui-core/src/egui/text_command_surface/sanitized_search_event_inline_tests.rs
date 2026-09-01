use super::super::sanitized_search_projection::{
    SanitizedSearchCapability, SanitizedSearchCapabilityRejection,
    SanitizedSearchLocalizedPresentation, SanitizedSearchProjectionBuilder, SanitizedSearchTarget,
};
use super::{
    SanitizedSearchEventKind, SanitizedSearchEventTransport, SanitizedSearchOneShotText,
    SanitizedSearchRoutedTarget, SanitizedSearchTextOperation, SanitizedSearchUnitOperation,
    route_search_events,
};
use crate::molecule::command_chrome::CommandChromeSearchEvent;
use crate::molecule::structured::{ReplaceMode, SearchControlStripEvent};
use std::cell::RefCell;
use std::rc::Rc;

fn localized_presentation() -> SanitizedSearchLocalizedPresentation {
    use super::super::sanitized_search_projection::SanitizedSearchResultSummaryPresentation;
    use super::super::sanitized_search_projection::SanitizedSearchTextPresentation;
    use super::super::sanitized_search_projection::SanitizedSearchUnavailablePresentation;
    use super::super::sanitized_search_projection::{
        SanitizedSearchControlPresentation, SanitizedSearchOperationPresentation,
    };

    SanitizedSearchLocalizedPresentation::new(
        SanitizedSearchControlPresentation::new(
            SanitizedSearchTextPresentation::new("検索", "検索", "検索"),
            SanitizedSearchTextPresentation::new("検索語", "検索語", "検索語"),
            SanitizedSearchTextPresentation::new("置換", "置換", "置換"),
            SanitizedSearchTextPresentation::new("大文字小文字", "大文字小文字", "大文字小文字"),
            SanitizedSearchTextPresentation::new("単語", "単語", "単語"),
            SanitizedSearchTextPresentation::new("正規表現", "正規表現", "正規表現"),
        ),
        SanitizedSearchOperationPresentation::new(
            SanitizedSearchTextPresentation::new("前へ", "前へ", "前へ"),
            SanitizedSearchTextPresentation::new("次へ", "次へ", "次へ"),
            SanitizedSearchTextPresentation::new("置換", "置換", "置換"),
            SanitizedSearchTextPresentation::new("すべて置換", "すべて置換", "すべて置換"),
            SanitizedSearchTextPresentation::new("閉じる", "閉じる", "閉じる"),
        ),
        SanitizedSearchResultSummaryPresentation::new(
            "検索待機",
            "一致なし",
            "1件",
            "位置",
            "件数",
        ),
        SanitizedSearchUnavailablePresentation::new("未対応", "未対応", "未対応", "未対応"),
    )
}

include!("sanitized_search_event_inline_tests/routing.rs");
include!("sanitized_search_event_inline_tests/capability_edges.rs");
