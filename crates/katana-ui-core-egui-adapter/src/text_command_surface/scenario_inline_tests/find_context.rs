#[test]
fn find_renders_visible_disabled_replace_controls() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Find);
    let output = render(&egui::Context::default(), &mut root, None);
    let record = output.search_record.as_ref().expect("find search record");
    assert!(record.replace.is_some(), "replace input must be visible");
    for id in ["replace-one", "replace-all"] {
        let control = record
            .controls
            .iter()
            .find(|control| control.control_id.ends_with(id))
            .expect("replace control record");
        assert!(control.disabled, "{id} must be disabled");
    }
}

#[test]
fn workspace_tabs_default_workbench_keeps_find_and_replace_blocker_in_the_same_root() {
    let context = egui::Context::default();
    let mut root = retained(FullTextCommandSurfaceScenarioId::WorkspaceTabs);
    let output = render(&context, &mut root, None);
    let search = output
        .search_record
        .as_ref()
        .expect("workspace-tabs search record");
    assert!(
        search.replace.is_some(),
        "replace input must remain visible"
    );
    for id in ["replace-one", "replace-all"] {
        let control = search
            .controls
            .iter()
            .find(|control| control.control_id.ends_with(id))
            .expect("replace blocker control");
        assert!(
            control.disabled,
            "{id} must remain disabled without a host route"
        );
    }
    let annotations = &output.evidence_text.record.frame.annotations;
    assert_eq!(
        annotations.len(),
        generic_find_annotations(FIXTURE_TEXT).len()
    );
    assert_eq!(annotations[0].visual_role, GENERIC_SEARCH_CURRENT_ROLE);
    assert!(annotations[1..]
        .iter()
        .all(|annotation| annotation.visual_role == GENERIC_SEARCH_MATCH_ROLE));
}

#[test]
fn find_fixture_paints_generic_match_and_current_annotations() -> Result<(), String> {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Find);
    let output = render(&egui::Context::default(), &mut root, None);
    let annotations = &output.evidence_text.record.frame.annotations;
    let expected = generic_find_annotations(FIXTURE_TEXT);
    assert_eq!(annotations.len(), expected.len());
    assert_eq!(annotations[0].visual_role, GENERIC_SEARCH_CURRENT_ROLE);
    assert_eq!(annotations[0].style, TextSurfaceAnnotationStyle::Fill);
    assert!(annotations[1..].iter().all(|annotation| {
        annotation.visual_role == GENERIC_SEARCH_MATCH_ROLE
            && annotation.style == TextSurfaceAnnotationStyle::Outline
    }));

    let style = scenario_style().map_err(|error| format!("scenario style: {error:?}"))?;
    let annotation_colors = output
        .evidence_text
        .artifact
        .paint_plan
        .operations
        .iter()
        .filter(|operation| operation.layer == EguiTextSurfaceDrawLayer::Annotation)
        .filter_map(|operation| match operation.kind {
            TextSurfacePaintOperationKind::Fill { color_rgba, .. } => Some(color_rgba),
            TextSurfacePaintOperationKind::Texture { .. } => None,
        })
        .collect::<Vec<_>>();
    assert!(annotation_colors.contains(&style.text_paint.preedit_rgba));
    assert!(annotation_colors.contains(&style.text_paint.selection_rgba));
    Ok(())
}

#[test]
fn find_fixture_annotations_change_the_final_composite_rgba() -> Result<(), String> {
    let context = egui::Context::default();
    let mut resting_root = retained(FullTextCommandSurfaceScenarioId::Resting);
    let resting = render_input(&context, &mut resting_root, public_motion_input());
    let mut find_root = retained(FullTextCommandSurfaceScenarioId::Find);
    let find = render_input(&context, &mut find_root, public_motion_input());
    let style = scenario_style().map_err(|error| format!("scenario style: {error:?}"))?;

    for (role, color) in [
        (GENERIC_SEARCH_CURRENT_ROLE, style.text_paint.preedit_rgba),
        (GENERIC_SEARCH_MATCH_ROLE, style.text_paint.selection_rgba),
    ] {
        let resting_count = rgba_color_count(&resting.evidence_composite.rgba_pixels, color);
        let find_count = rgba_color_count(&find.evidence_composite.rgba_pixels, color);
        assert!(
            find_count > resting_count,
            "{role} must increase final composite pixels: resting={resting_count}, find={find_count}"
        );
    }
    Ok(())
}

#[test]
fn context_stage_opens_then_escape_closes_and_restores_text_focus() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Context);
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Context)
        .expect("scenario issues");
    let stages = scenario.stages().to_vec();
    assert_eq!(stages.len(), 4, "context trace has a complete close cycle");
    let context = egui::Context::default();
    let _ = render(&context, &mut root, Some(&stages[0]));
    let opened = render(&context, &mut root, Some(&stages[1]));
    assert!(
        opened.context_menu_record.is_some(),
        "context menu record exists"
    );
    assert!(
        opened
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_none(),
        "floating toolbar is selection-only"
    );
    assert!(
        !opened.evidence_composite.rgba_pixels.is_empty(),
        "opened menu contributes an actual composite frame"
    );

    let closed = render(&context, &mut root, Some(&stages[2]));
    assert!(
        closed.context_menu_record.is_none(),
        "Escape removes the retained context menu from the close frame"
    );
    assert!(
        closed
            .events()
            .current_context()
            .context_menu_events()
            .iter()
            .any(|event| {
                matches!(
                    event,
                    katana_ui_core::molecule::selection::ContextMenuEvent::Closed { .. }
                )
            }),
        "Escape produces a KUC context-menu Closed event"
    );

    let restored = render(&context, &mut root, Some(&stages[3]));
    assert!(
        restored
            .evidence_text
            .record
            .frame
            .accessibility
            .root
            .focused,
        "the retained text surface regains focus after context-menu dismissal"
    );
}

#[test]
fn readonly_stage_has_no_mutation_or_events() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Readonly);
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Readonly)
        .expect("scenario issues");
    let stages = scenario.stages().to_vec();
    let output = render(&egui::Context::default(), &mut root, Some(&stages[1]));
    assert!(output.evidence_text.events.is_empty());
    assert_eq!(
        output.evidence_text.record.frame.caret,
        output.evidence_text.record.frame.selection_start
    );
}
