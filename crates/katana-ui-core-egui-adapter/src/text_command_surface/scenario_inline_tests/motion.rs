#[test]
fn motion_plan_rejects_a_request_that_cannot_cover_every_kuc_scenario() {
    let minimum = FullTextCommandSurfaceMotionPlan::minimum_frame_count();
    let error = FullTextCommandSurfaceMotionPlan::issue(minimum - 1)
        .expect_err("partial catalogue must fail closed");
    assert_eq!(
        error,
        FullTextCommandSurfaceMotionPlanError::IncompleteCatalogue {
            requested: minimum - 1,
            minimum,
        }
    );
    assert_eq!(
        FullTextCommandSurfaceMotionPlan::issue(minimum + 1)
            .expect_err("an incomplete repeated catalogue must fail closed"),
        FullTextCommandSurfaceMotionPlanError::IncompleteCatalogue {
            requested: minimum + 1,
            minimum,
        }
    );
}

#[test]
fn motion_catalogue_opens_selects_and_closes_the_generic_language_dropdown() {
    let plan = FullTextCommandSurfaceMotionPlan::issue(
        FullTextCommandSurfaceMotionPlan::minimum_frame_count(),
    )
    .expect("complete motion plan");
    assert_eq!(
        plan.frames()
            .iter()
            .filter(|frame| {
                matches!(
                    frame.dropdown_transition,
                    DropdownMotionTransition::BeginTrigger
                )
            })
            .count(),
        1,
        "the catalogue must include exactly one KUC-owned dropdown opening"
    );
    assert_eq!(
        plan.frames()
            .iter()
            .filter(|frame| {
                matches!(
                    frame.dropdown_transition,
                    DropdownMotionTransition::BeginItem
                )
            })
            .count(),
        1,
        "the catalogue must include exactly one KUC-owned dropdown selection"
    );
    assert_eq!(
        plan.frames()
            .iter()
            .filter(|frame| {
                matches!(frame.dropdown_transition, DropdownMotionTransition::Advance)
            })
            .count(),
        6,
        "the physical click lifecycle must retain its aim, press, and release frames"
    );

    let context = egui::Context::default();
    let mut active_scenario = None;
    let mut root = None;
    let mut continuation = None;
    let mut dropdown_opened = false;
    let mut item_activated = false;
    let mut dropdown_closed = false;
    let mut settled_after_selection = false;
    let mut visible_dropdown_pixel_hash = None;

    for frame in plan.frames() {
        if active_scenario != Some(frame.scenario_id()) {
            assert!(
                continuation.is_none(),
                "a KUC continuation must finish before its retained root changes"
            );
            root = Some(retained(frame.scenario_id()));
            active_scenario = Some(frame.scenario_id());
        }
        let mut input = public_motion_input();
        frame
            .apply_to(&mut input, &mut continuation)
            .expect("KUC-issued motion input applies");
        let output = render_input(&context, root.as_mut().expect("active KUC root"), input);

        if matches!(
            frame.dropdown_transition,
            DropdownMotionTransition::BeginItem
        ) {
            let dropdown = output
                .toolbar_record
                .as_ref()
                .and_then(|toolbar| toolbar.dropdown.as_ref())
                .expect("the KUC foreground dropdown is visible before item selection");
            assert_eq!(dropdown.items.len(), GENERIC_LANGUAGE_CHOICE_LABELS.len());
            visible_dropdown_pixel_hash = Some(output.evidence_composite.pixel_hash.clone());
        }

        let (toolbar_events, _) = output
            .events()
            .detach_command_events()
            .expect("the root event batch is consumed once per motion frame");
        for event in toolbar_events {
            match event {
                CommandChromeToolbarEvent::DropdownOpened { action_id, .. }
                    if action_id.as_str() == "kuc.rich.block-code" =>
                {
                    dropdown_opened = true;
                }
                CommandChromeToolbarEvent::DropdownItemActivated { action_id, item_id }
                    if action_id.as_str() == "kuc.rich.block-code"
                        && item_id.as_str() == "kuc.generic-language-00" =>
                {
                    item_activated = true;
                }
                CommandChromeToolbarEvent::DropdownClosed { action_id, reason }
                    if action_id.as_str() == "kuc.rich.block-code"
                        && reason == CommandChromeDropdownCloseReason::ItemActivated =>
                {
                    dropdown_closed = true;
                }
                _ => {}
            }
        }

        if item_activated
            && matches!(frame.dropdown_transition, DropdownMotionTransition::None)
            && frame.scenario_id() == FullTextCommandSurfaceScenarioId::Resting
        {
            assert!(
                output
                    .toolbar_record
                    .as_ref()
                    .and_then(|toolbar| toolbar.dropdown.as_ref())
                    .is_none(),
                "the next retained frame must not keep the selected dropdown visible"
            );
            assert_ne!(
                visible_dropdown_pixel_hash.as_deref(),
                Some(output.evidence_composite.pixel_hash.as_str()),
                "the KUC composite must retain visible foreground dropdown pixels"
            );
            settled_after_selection = true;
        }

        frame
            .capture_continuation(output.interaction_locator(), &mut continuation)
            .expect("KUC re-resolves each physical target from the current root frame");
    }

    assert!(
        dropdown_opened,
        "trigger release must emit an opening event"
    );
    assert!(
        item_activated,
        "item release must activate the first generic choice"
    );
    assert!(
        dropdown_closed,
        "item activation must emit the KUC-owned ItemActivated close event"
    );
    assert!(
        settled_after_selection,
        "the following frame must prove the foreground dropdown has closed"
    );
    assert!(
        visible_dropdown_pixel_hash.is_some(),
        "the foreground dropdown frame must have KUC-owned raster evidence"
    );
    assert!(
        continuation.is_none(),
        "the complete catalogue must not leave a physical interaction pending"
    );
}

#[test]
fn motion_plan_issues_only_classified_kuc_frames_with_complete_coverage() {
    let minimum = FullTextCommandSurfaceMotionPlan::minimum_frame_count();
    let plan = FullTextCommandSurfaceMotionPlan::issue(minimum).expect("full catalogue");
    assert_eq!(plan.frames().len(), minimum);
    for scenario_id in [
        FullTextCommandSurfaceScenarioId::Resting,
        FullTextCommandSurfaceScenarioId::Selection,
        FullTextCommandSurfaceScenarioId::Find,
        FullTextCommandSurfaceScenarioId::Context,
        FullTextCommandSurfaceScenarioId::Readonly,
        FullTextCommandSurfaceScenarioId::ResizeScrollIme,
    ] {
        assert!(
            plan.frames()
                .iter()
                .any(|frame| frame.scenario_id() == scenario_id),
            "motion catalogue is missing {scenario_id:?}"
        );
    }
    let provenance = plan
        .frames()
        .iter()
        .map(FullTextCommandSurfaceMotionFrame::provenance_id)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(provenance.len(), plan.frames().len());
    assert!(provenance.iter().all(|value| !value.is_empty()));
}

#[test]
fn motion_plan_repeats_the_kuc_catalogue_without_idle_filler() {
    let minimum = FullTextCommandSurfaceMotionPlan::minimum_frame_count();
    let plan = FullTextCommandSurfaceMotionPlan::issue(minimum * 2)
        .expect("complete catalogue can repeat");
    assert_eq!(plan.frames().len(), minimum * 2);
    for index in 0..minimum {
        assert_eq!(
            plan.frames()[index].scenario_id(),
            plan.frames()[index + minimum].scenario_id(),
            "the second cycle must be KUC's ordered catalogue"
        );
        assert_ne!(
            plan.frames()[index].provenance_id(),
            plan.frames()[index + minimum].provenance_id(),
            "each encoded frame needs unique KUC provenance"
        );
    }
}
