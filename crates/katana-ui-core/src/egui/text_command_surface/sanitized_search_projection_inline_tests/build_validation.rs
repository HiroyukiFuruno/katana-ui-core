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
fn build_rejects_enabled_operations_missing_required_targets() {
    use SanitizedSearchOperationSlot as Slot;
    let cases = [
        Slot::Next,
        Slot::Previous,
        Slot::Replace,
        Slot::ReplaceAll,
        Slot::Close,
    ];

    for slot in cases {
        let mut projection =
            SanitizedSearchProjectionBuilder::new().localized_presentation(localized("次へ"));
        projection = match slot {
            Slot::Next => projection.next_enabled(true),
            Slot::Previous => projection.previous_enabled(true),
            Slot::Replace => projection.replace_enabled(true),
            Slot::ReplaceAll => projection.replace_all_enabled(true),
            Slot::Close => projection.close_enabled(true),
            _ => unreachable!(),
        };

        let error = projection
            .build()
            .expect_err("対象未指定の有効化は弾かれるべき");
        assert!(matches!(
            error,
            SanitizedSearchProjectionBuildError::EnabledOperationWithoutTarget {
                operation
            } if operation == slot
        ));
    }
}

#[test]
fn presentation_rejects_empty_text_in_operation_and_unavailable_sections() {
    let invalid_operation = SanitizedSearchOperationPresentation::new(
        text(""),
        text("次へ"),
        text("置換"),
        text("すべて置換"),
        text("閉じる"),
    );
    let invalid_unavailable = SanitizedSearchUnavailablePresentation::new(
        "",
        "置換は利用不可",
        "移動は利用不可",
        "閉じる操作は利用不可",
    );
    let presentation = SanitizedSearchLocalizedPresentation::new(
        localized("次へ").controls,
        invalid_operation,
        localized("次へ").result_summary,
        invalid_unavailable,
    );

    let error = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(presentation)
        .build()
        .expect_err("空文字は拒否されるべき");

    assert_eq!(
        error,
        SanitizedSearchProjectionBuildError::EmptyPresentationText
    );
}
