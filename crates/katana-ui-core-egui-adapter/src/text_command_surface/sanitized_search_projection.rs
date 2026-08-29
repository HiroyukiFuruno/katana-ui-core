#[path = "sanitized_search_projection/builder.rs"]
mod builder;
#[path = "sanitized_search_projection/builder_types.rs"]
mod builder_types;
#[path = "sanitized_search_projection/operation.rs"]
mod operation;
#[path = "sanitized_search_projection/presentation.rs"]
mod presentation;
#[path = "sanitized_search_projection/projection.rs"]
mod projection;
#[path = "sanitized_search_projection/types.rs"]
mod types;

pub use builder_types::SanitizedSearchProjectionBuilder;
pub use presentation::{
    SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
    SanitizedSearchOperationPresentation, SanitizedSearchResultSummaryPresentation,
    SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
};
pub use projection::SanitizedSearchProjection;
pub(crate) use types::{SanitizedSearchCapability, TextCapability, UnitCapability};
pub use types::{
    SanitizedSearchCapabilityRejection, SanitizedSearchOperationSlot,
    SanitizedSearchProjectionBuildError, SanitizedSearchTarget, SanitizedSearchTextOperation,
    SanitizedSearchUnitOperation,
};

#[cfg(test)]
mod tests {
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
                "正規表現は利用不可",
                "置換は利用不可",
                "移動は利用不可",
                "閉じる操作は利用不可",
            ),
        )
    }

    #[test]
    fn build_requires_localized_presentation() {
        let error = SanitizedSearchProjectionBuilder::new()
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1]))
            .build()
            .expect_err("表示未指定");

        assert_eq!(
            error,
            SanitizedSearchProjectionBuildError::MissingPresentation
        );
    }

    #[test]
    fn build_rejects_empty_presentation_text() {
        let invalid = SanitizedSearchLocalizedPresentation::new(
            SanitizedSearchControlPresentation::new(
                SanitizedSearchTextPresentation::new("", "検索 ⭐️", "検索 ⭐️"),
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
                "正規表現は利用不可",
                "置換は利用不可",
                "移動は利用不可",
                "閉じる操作は利用不可",
            ),
        );

        let error = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(invalid)
            .build()
            .expect_err("空表示拒否");

        assert_eq!(
            error,
            SanitizedSearchProjectionBuildError::EmptyPresentationText
        );
    }

    #[test]
    fn build_rejects_enabled_operation_without_target() {
        let error = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次へ"))
            .next_enabled(true)
            .build()
            .expect_err("対象未指定");

        assert_eq!(
            error,
            SanitizedSearchProjectionBuildError::EnabledOperationWithoutTarget {
                operation: SanitizedSearchOperationSlot::Next
            }
        );
    }

    #[test]
    fn fingerprint_tracks_target_and_presentation_without_debug_leaks() {
        let base = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次へ"))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1, 2, 3]))
            .build()
            .expect("検証済み");
        let same = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次へ"))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1, 2, 3]))
            .build()
            .expect("検証済み");
        let changed_presentation = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次の一致"))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1, 2, 3]))
            .build()
            .expect("検証済み");
        let changed_target = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次へ"))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1, 2, 4]))
            .build()
            .expect("検証済み");

        assert!(base.same_as(&same));
        assert!(!base.same_as(&changed_presentation));
        assert!(!base.same_as(&changed_target));

        let debug = format!("{base:?}");
        assert!(!debug.contains("検索"));
        assert!(!debug.contains("置換"));
        assert!(!debug.contains("次へ"));
        assert!(!debug.contains("⭐️"));
        assert!(!debug.contains("1, 2, 3"));
    }
}
