use super::support::*;
use super::*;

const IME_QUERY_TARGET: [u8; 2] = [9, 1];
const IME_REPLACEMENT_TARGET: [u8; 2] = [9, 2];

pub(super) fn input_with_search(revision: u64) -> Result<SanitizedDocumentRootInput, String> {
    Ok(
        input(revision, b"document", "本文 ⭐️").with_search_projection(
            super::super::super::sanitized_document_root_process::SearchProjectionForIme::build(
                IME_QUERY_TARGET,
                IME_REPLACEMENT_TARGET,
            )?,
        ),
    )
}

pub(super) fn search_text(value: &str) -> SanitizedSearchTextPresentation {
    SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
}

pub(super) fn search_localized() -> SanitizedSearchLocalizedPresentation {
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

pub(super) fn input_with_recorders(
    revision: u64,
    text_events: Rc<RefCell<Vec<(SanitizedSearchTextOperation, String)>>>,
    unit_events: Rc<RefCell<Vec<SanitizedSearchUnitOperation>>>,
) -> SanitizedDocumentRootInput {
    let text_target = |operation| {
        let events = text_events.clone();
        let _ = operation;
        SanitizedSearchTarget::from_opaque_bytes([0]).with_text_capability(move |actual, value| {
            events.borrow_mut().push((actual, value));
            Ok::<(), ()>(())
        })
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

pub(super) fn input_with_rejecting_recorders(
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

pub(super) fn run_search_root_frame(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, SanitizedDocumentRootFrame) {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    output.textures_delta.clear();
    (output, frame.expect("frame exists"))
}

pub(super) fn accesskit_bounds(
    output: &egui::FullOutput,
    role: egui::accesskit::Role,
    label: &str,
) -> egui::Rect {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.role() == role && node.label() == Some(label)).then(|| node.bounds())
            })
        })
        .flatten()
        .map(|bounds| {
            egui::Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            )
        })
        .expect("current output contains the requested control bounds")
}

pub(super) fn accesskit_button(
    output: &egui::FullOutput,
    label: &str,
) -> (egui::accesskit::NodeId, egui::Rect) {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(node_id, node)| {
                (node.role() == egui::accesskit::Role::Button && node.label() == Some(label))
                    .then(|| {
                        node.bounds().map(|bounds| {
                            (
                                *node_id,
                                egui::Rect::from_min_max(
                                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                                ),
                            )
                        })
                    })
                    .flatten()
            })
        })
        .expect("current output contains the requested button node")
}
