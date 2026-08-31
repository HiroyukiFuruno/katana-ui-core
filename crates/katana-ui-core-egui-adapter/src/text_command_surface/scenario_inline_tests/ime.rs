#[test]
fn selection_scenario_keeps_the_kuc_owned_physical_lifecycle() {
    let stages = stages(FullTextCommandSurfaceScenarioId::Selection);
    assert_eq!(stages.len(), 1);
    assert!(stages[0].input.events.is_empty());
    let plan = FullTextCommandSurfaceMotionPlan::issue(
        FullTextCommandSurfaceMotionPlan::minimum_frame_count(),
    )
    .expect("complete motion plan");
    assert_eq!(
        plan.frames()
            .iter()
            .filter(|frame| frame.scenario_id() == FullTextCommandSurfaceScenarioId::Selection)
            .count(),
        6
    );
}

#[test]
fn resize_scroll_ime_stages_reconfigure_viewport_and_keep_preedit_and_commit() {
    let stages = stages(FullTextCommandSurfaceScenarioId::ResizeScrollIme);
    assert_eq!(stages.len(), 7);
    assert_eq!(
        stages[4].input.screen_rect,
        Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(900.0, 520.0),
        ))
    );
    assert!(matches!(
        stages[5].input.events.as_slice(),
        [egui::Event::Ime(egui::ImeEvent::Preedit { text: value, .. })] if value == "かな"
    ));
    assert!(matches!(
        stages[6].input.events.as_slice(),
        [egui::Event::Ime(egui::ImeEvent::Commit(value))] if value == "入力"
    ));
}

#[test]
fn resize_scroll_ime_trace_focuses_then_records_actual_preedit_and_commit() {
    let stages = stages(FullTextCommandSurfaceScenarioId::ResizeScrollIme);
    let context = egui::Context::default();
    let mut root = retained(FullTextCommandSurfaceScenarioId::ResizeScrollIme);
    let outputs = stages
        .iter()
        .map(|stage| render(&context, &mut root, Some(stage)))
        .collect::<Vec<_>>();

    assert!(
        outputs[3].evidence_text.record.frame.viewport.scroll_y > 0,
        "wheel input after physical focus must change the retained viewport"
    );
    assert!(
        outputs[4].evidence_text.record.frame.viewport.width
            < outputs[0].evidence_text.record.frame.viewport.width,
        "the resized screen must reduce the measured viewport"
    );
    assert!(matches!(
        outputs[5].evidence_text.record.frame.preedit.as_ref(),
        Some(preedit) if preedit.text == "かな"
    ));
    assert!(outputs[5].evidence_text.events.iter().any(|event| {
        matches!(
            event,
            katana_ui_core::text_surface::TextSurfaceEvent::TextArea(
                katana_ui_core::atom::TextAreaEvent::ImeComposition(value)
            ) if value.preedit == "かな"
        )
    }));
    assert!(outputs[6].evidence_text.record.frame.preedit.is_none());
    assert!(outputs[6].evidence_text.events.iter().any(|event| {
        matches!(
            event,
            katana_ui_core::text_surface::TextSurfaceEvent::TextArea(
                katana_ui_core::atom::TextAreaEvent::ImeCommit(value)
            ) if value == "入力"
        )
    }));
}
