pub(crate) struct SearchProjectionForIme;

impl SearchProjectionForIme {
    pub(crate) fn build(
        query_target: impl Into<Vec<u8>>,
        replacement_target: impl Into<Vec<u8>>,
    ) -> Result<super::super::SanitizedSearchProjection, String> {
        let query_record = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
        let replacement_record = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
        fn text(value: &str) -> super::super::SanitizedSearchTextPresentation {
            super::super::SanitizedSearchTextPresentation::new(
                value,
                format!("{value} ⭐️"),
                format!("{value} ⭐️"),
            )
        }

        fn localized() -> super::super::SanitizedSearchLocalizedPresentation {
            super::super::SanitizedSearchLocalizedPresentation::new(
                super::super::SanitizedSearchControlPresentation::new(
                    text("検索"),
                    text("検索語"),
                    text("置換語"),
                    text("大文字小文字"),
                    text("単語一致"),
                    text("正規表現"),
                ),
                super::super::SanitizedSearchOperationPresentation::new(
                    text("前へ"),
                    text("次へ"),
                    text("置換"),
                    text("すべて置換"),
                    text("閉じる"),
                ),
                super::super::SanitizedSearchResultSummaryPresentation::new(
                    "検索待機 ⭐️",
                    "一致なし",
                    "1件",
                    "{active} / {count}",
                    "{count}件",
                ),
                super::super::SanitizedSearchUnavailablePresentation::new(
                    "正規表現は利用不可",
                    "置換は利用不可",
                    "移動は利用不可",
                    "閉じる操作は利用不可",
                ),
            )
        }

        super::super::SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized())
            .query_target(
                super::super::SanitizedSearchTarget::from_opaque_bytes(query_target)
                    .with_text_capability({
                        let record = query_record;
                        move |_, value| {
                            record.borrow_mut().push(value);
                            Ok::<(), ()>(())
                        }
                    }),
            )
            .replacement_target(
                super::super::SanitizedSearchTarget::from_opaque_bytes(replacement_target)
                    .with_text_capability({
                        let record = replacement_record;
                        move |_, value| {
                            record.borrow_mut().push(value);
                            Ok::<(), ()>(())
                        }
                    }),
            )
            .build()
            .map_err(|error| format!("{error:?}"))
    }
}
