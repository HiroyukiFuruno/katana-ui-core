use super::*;
use crate::text_command_surface::EguiTextCommandSurfaceRootFactory;
use crate::text_command_surface::TabStripProposalOperation;
use crate::text_surface::{EguiTextSurfaceDrawLayer, TextSurfacePaintOperationKind};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeDropdownCloseReason, CommandChromeToolbarEvent,
};
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressAction, SourceAddressEvent, SourceAddressPresentation, SourceAddressStrip,
};

fn render(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    stage: Option<&FullTextCommandSurfaceRawInputStage>,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
    let mut input = egui::RawInput::default();
    if let Some(stage) = stage {
        stage.apply_to(&mut input);
    }
    let mut output = None;
    let mut frame = context.run_ui(input, |ui| {
        output = Some(root.show_output_for_test(ui));
    });
    frame.textures_delta.clear();
    output.expect("root frame").expect("root render")
}

fn render_input(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
    let mut output = None;
    let mut frame = context.run_ui(input, |ui| {
        output = Some(root.show_output_for_test(ui));
    });
    frame.textures_delta.clear();
    output.expect("root frame").expect("root render")
}

fn render_public(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceHostRootFrame {
    let mut frame = None;
    let mut output = context.run_ui(input, |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            frame = Some(root.show(ui));
        });
    });
    output.textures_delta.clear();
    frame
        .expect("public root frame")
        .expect("public root render")
}

fn public_motion_input() -> egui::RawInput {
    let mut input = egui::RawInput::default();
    stage(Vec::new(), 1.0).apply_to(&mut input);
    input
}

fn retained(
    id: FullTextCommandSurfaceScenarioId,
) -> crate::text_command_surface::EguiTextCommandSurfaceHostRoot {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(id)
        .expect("scenario issues");
    EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("scenario lease"))
        .expect("scenario root retains")
}

#[test]
fn resting_has_no_floating_output() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Resting);
    let output = render(&egui::Context::default(), &mut root, None);
    assert!(
        output
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_none()
    );
}

#[test]
fn selection_continuation_uses_only_the_public_root_facade() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Selection);
    let context = egui::Context::default();
    let initial = render_public(&context, &mut root, public_motion_input());
    let mut continuation = initial
        .interaction_locator()
        .begin_text_selection()
        .expect("current public root frame issues a selection continuation");
    for step in 0..5 {
        let mut input = public_motion_input();
        continuation
            .apply_to_raw_input_once(&mut input)
            .expect("KUC continuation applies once");
        let frame = render_public(&context, &mut root, input);
        match continuation
            .advance(frame.interaction_locator())
            .expect("KUC continuation advances through the next public root frame")
        {
            Some(next) => continuation = next,
            None => {
                assert_eq!(
                    step, 4,
                    "selection must require aim, press, move, move, release"
                );
                return;
            }
        }
    }
    panic!("selection continuation did not close after its release frame");
}

#[test]
fn selection_continuation_retains_actual_text_selection_between_steps() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Selection);
    let context = egui::Context::default();
    let initial = render_input(&context, &mut root, public_motion_input());
    let mut continuation = initial
        .interaction_locator()
        .begin_text_selection()
        .expect("current root issues a selection continuation");
    for step in 0..5 {
        let mut input = public_motion_input();
        continuation
            .apply_to_raw_input_once(&mut input)
            .expect("KUC continuation applies once");
        let output = render_input(&context, &mut root, input);
        if step == 4 {
            assert!(
                !output
                    .evidence_text
                    .record
                    .frame
                    .selection
                    .range
                    .is_collapsed(),
                "release must retain an actual text range; events={:?}",
                output.evidence_text.events
            );
        }
        match continuation
            .advance(output.interaction_locator())
            .expect("KUC continuation advances through the next root frame")
        {
            Some(next) => continuation = next,
            None => {
                assert_eq!(step, 4);
                return;
            }
        }
    }
    panic!("selection continuation did not close after its release frame");
}

#[test]
fn primary_toolbar_uses_generic_rich_authoring_icons_and_accessible_names() {
    let resting = presentation(FullTextCommandSurfaceScenarioId::Resting);
    let toolbar = resting.toolbar.expect("primary toolbar");
    assert_eq!(toolbar.actions.len(), RICH_AUTHORING_AFFORDANCES.len());
    assert_eq!(toolbar.display_mode, CommandChromeDisplayMode::IconOnly);
    assert!(toolbar.actions.iter().all(|action| {
        action.icon_model().is_some()
            && action.tooltip_model().is_some()
            && action.accessibility_label_model().is_some()
            && action.id().as_str().starts_with("kuc.rich.")
    }));
    let ids = toolbar
        .actions
        .iter()
        .map(|action| action.id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        [
            "kuc.rich.inline-strong",
            "kuc.rich.inline-italic",
            "kuc.rich.inline-strike",
            "kuc.rich.inline-code",
            "kuc.rich.heading-one",
            "kuc.rich.heading-two",
            "kuc.rich.heading-three",
            "kuc.rich.list-unordered",
            "kuc.rich.list-ordered",
            "kuc.rich.blockquote",
            "kuc.rich.block-code",
            "kuc.rich.media-image",
        ]
    );
    let code_block = toolbar
        .actions
        .iter()
        .find(|action| action.id().as_str() == "kuc.rich.block-code")
        .expect("generic language-choice action exists");
    let dropdown = code_block.dropdown_model().expect("dropdown is configured");
    assert_eq!(
        dropdown
            .items()
            .iter()
            .map(|item| item.label_model())
            .collect::<Vec<_>>(),
        GENERIC_LANGUAGE_CHOICE_LABELS
    );
    let readonly = super::presentation(FullTextCommandSurfaceScenarioId::Readonly)
        .toolbar
        .expect("readonly toolbar");
    assert!(
        readonly
            .actions
            .iter()
            .all(|action| action.disabled_model())
    );
}

#[test]
fn automatic_gutter_keeps_40_line_content_separate_from_painted_labels() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Resting);
    let output = render(&egui::Context::default(), &mut root, None);
    let frame = &output.evidence_text.record.frame;
    assert!(
        frame
            .viewport_bounds
            .x
            .saturating_sub(frame.surface_bounds.x)
            >= 52,
        "automatic gutter must provide the generic minimum hit column"
    );
    let (gutter, text) = output
        .evidence_text
        .artifact
        .paint_plan
        .operations
        .iter()
        .filter_map(|operation| match &operation.kind {
            TextSurfacePaintOperationKind::Texture { texture, bounds }
                if operation.layer == EguiTextSurfaceDrawLayer::Gutter
                    && texture.identity.starts_with("gutter:") =>
            {
                Some((Some(*bounds), None))
            }
            TextSurfacePaintOperationKind::Texture { bounds, .. }
                if operation.layer == EguiTextSurfaceDrawLayer::TextTexture =>
            {
                Some((None, Some(*bounds)))
            }
            _ => None,
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut gutters, mut texts), value| {
                if let Some(bounds) = value.0 {
                    gutters.push(bounds);
                }
                if let Some(bounds) = value.1 {
                    texts.push(bounds);
                }
                (gutters, texts)
            },
        );
    assert!(!gutter.is_empty());
    assert!(!text.is_empty());
    assert!(gutter.iter().all(|gutter| {
        text.iter()
            .all(|text| gutter.x.saturating_add_unsigned(gutter.width) <= text.x)
    }));
}

#[test]
fn full_fixture_redraw_is_byte_stable_for_layout_and_composite_evidence() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Resting);
    let context = egui::Context::default();
    let first = render_input(&context, &mut root, public_motion_input());
    let second = render_input(&context, &mut root, public_motion_input());

    assert_eq!(
        first.evidence_text.record.frame, second.evidence_text.record.frame,
        "an unchanged root frame must preserve layout, rows, gutter, caret, selection, preedit, and accessibility evidence"
    );
    assert_eq!(
        first.evidence_text.artifact, second.evidence_text.artifact,
        "an unchanged root frame must preserve its text paint plan"
    );
    assert_eq!(
        first.evidence_composite, second.evidence_composite,
        "the KUC-owned final composite must be byte-stable across an unchanged redraw"
    );
}

#[test]
fn full_surface_fixture_has_more_than_katana_initial_visible_rows_and_vs16() {
    assert!(
        FIXTURE_TEXT.lines().count() > 40,
        "fixture must exceed the fixed host's initial visible row baseline"
    );
    assert!(FIXTURE_TEXT.contains("⭐️"));
    assert!(FIXTURE_TEXT.contains('☆'));
}

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
    assert!(
        annotations[1..]
            .iter()
            .all(|annotation| annotation.visual_role == GENERIC_SEARCH_MATCH_ROLE)
    );
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
    assert!(
        annotations[1..]
            .iter()
            .all(
                |annotation| annotation.visual_role == GENERIC_SEARCH_MATCH_ROLE
                    && annotation.style == TextSurfaceAnnotationStyle::Outline
            )
    );

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

#[test]
fn navigation_input_scenario_keeps_source_submission_inside_the_opaque_root() {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::NavigationInput)
        .expect("navigation scenario issues");
    let stages = scenario.stages().to_vec();
    assert_eq!(
        stages.len(),
        4,
        "navigation trace has focus, text, and submit"
    );
    assert!(stages[1].event_count() > 0);
    assert!(stages[2].event_count() > 0);
    assert!(stages[3].event_count() > 0);
    assert!(
        !format!("{stages:?}").contains(NAVIGATION_INPUT_FIXTURE),
        "opaque stages must not expose their raw navigation text through Debug"
    );

    let mut root = retained(FullTextCommandSurfaceScenarioId::NavigationInput);
    let context = egui::Context::default();
    context.enable_accesskit();
    let _ = render(&context, &mut root, Some(&stages[0]));
    let _ = render(&context, &mut root, Some(&stages[1]));
    let typed = render(&context, &mut root, Some(&stages[2]));
    assert_eq!(
        typed
            .events()
            .current_context()
            .source_address_submission_count(),
        0,
        "typing changes retained input without emitting a host submission"
    );
    let submitted = render(&context, &mut root, Some(&stages[3]));
    let context = submitted.events().current_context();
    assert_eq!(context.source_address_submission_count(), 1);
    assert!(
        !format!("{context:?}").contains(NAVIGATION_INPUT_FIXTURE),
        "public root context must not reveal the submitted navigation text"
    );
    assert!(
        !submitted.evidence_composite.rgba_pixels.is_empty(),
        "the same retained KUC root emits a nonempty composite"
    );
}

#[test]
fn workspace_tabs_scenario_keeps_drag_and_artifact_inside_the_same_opaque_root() {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::WorkspaceTabs)
        .expect("workspace tabs scenario issues");
    let stages = scenario.stages().to_vec();
    assert_eq!(
        stages.len(),
        4,
        "workspace tabs trace has start, drag, and release"
    );
    assert!(stages[1].event_count() > 0);
    assert!(stages[2].event_count() > 0);
    assert!(stages[3].event_count() > 0);

    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = retained(FullTextCommandSurfaceScenarioId::WorkspaceTabs);
    let initial = render(&context, &mut root, Some(&stages[0]));
    let _ = render(&context, &mut root, Some(&stages[1]));
    let dragging = render(&context, &mut root, Some(&stages[2]));
    let released = render(&context, &mut root, Some(&stages[3]));
    assert!(
        released
            .artifact_order()
            .contains(&super::super::EguiTextCommandSurfaceChild::StatusBar),
        "workspace tabs consumes the status projection through the opaque lease"
    );
    assert!(
        released
            .artifact_order()
            .contains(&super::super::EguiTextCommandSurfaceChild::DiagnosticsList),
        "workspace tabs consumes the diagnostics projection through the opaque lease"
    );
    assert!(
        released
            .artifact_order()
            .contains(&super::super::EguiTextCommandSurfaceChild::Preview),
        "workspace tabs consumes the generic preview through the opaque lease"
    );
    assert_ne!(
        initial.evidence_composite.pixel_hash, dragging.evidence_composite.pixel_hash,
        "the KUC-owned drag ghost must alter the final root composite"
    );
    assert!(
        released.evidence_composite.non_transparent_pixel_count > 0,
        "tab strip and text surface remain in one nonempty root composite"
    );
    assert!(
        released
            .events()
            .current_context()
            .source_address_submission_count()
            == 0,
        "a generic tab drag cannot be decoded as an unrelated root event"
    );
}

fn rgba_color_count(pixels: &[u8], color: [u8; 4]) -> usize {
    pixels
        .chunks_exact(4)
        .filter(|pixel| *pixel == color)
        .count()
}

#[test]
fn raw_stage_applies_owned_events_and_debug_keeps_payload_opaque() {
    let stage = super::stage(vec![egui::Event::Text("opaque 日本語".to_string())], 1.5);
    let mut input = egui::RawInput::default();
    stage.apply_to(&mut input);
    assert_eq!(stage.event_count(), 1);
    assert_eq!(input.events.len(), 1);
    assert!(format!("{stage:?}").contains("event_count: 1"));
    assert!(!format!("{stage:?}").contains("opaque"));
}

#[test]
fn motion_frame_apply_to_fails_closed_without_required_continuation() {
    let frame = FullTextCommandSurfaceMotionFrame {
        scenario_id: FullTextCommandSurfaceScenarioId::Selection,
        stage: stage(Vec::new(), 1.0),
        provenance_id: String::from("kuc-motion-fake"),
        selection_transition: SelectionMotionTransition::Advance,
        find_transition: FindMotionTransition::None,
        dropdown_transition: DropdownMotionTransition::None,
    };

    let mut input = egui::RawInput::default();
    let mut continuation = None;
    assert!(matches!(
        frame.apply_to(&mut input, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::MissingContinuation)
    ));
}

#[test]
fn motion_frame_capture_continuation_fails_closed_for_invalid_transition_combo() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Selection);
    let output = render(&egui::Context::default(), &mut root, None);
    let locator = output.interaction_locator();

    let mut frame = FullTextCommandSurfaceMotionFrame {
        scenario_id: FullTextCommandSurfaceScenarioId::Selection,
        stage: stage(Vec::new(), 1.0),
        provenance_id: String::from("kuc-motion-fail"),
        selection_transition: SelectionMotionTransition::Begin,
        find_transition: FindMotionTransition::Begin,
        dropdown_transition: DropdownMotionTransition::None,
    };

    let mut no_continuation = None;
    assert!(matches!(
        frame.capture_continuation(&locator, &mut no_continuation),
        Err(FullTextCommandSurfaceMotionPlanError::InvalidTransition)
    ));

    let trigger_frame = FullTextCommandSurfaceMotionFrame {
        scenario_id: FullTextCommandSurfaceScenarioId::Selection,
        stage: stage(Vec::new(), 1.0),
        provenance_id: String::from("kuc-motion-trigger"),
        selection_transition: SelectionMotionTransition::None,
        find_transition: FindMotionTransition::None,
        dropdown_transition: DropdownMotionTransition::BeginTrigger,
    };
    let mut continuation = None;
    trigger_frame
        .capture_continuation(&locator, &mut continuation)
        .expect("dropdown continuation opens through capture");
    assert_eq!(
        format!(
            "{:?}",
            continuation.as_ref().expect("continuation retained")
        ),
        "KucOpaqueMotionContinuation(..)"
    );

    assert!(matches!(
        trigger_frame.capture_continuation(&locator, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation)
    ));

    frame.selection_transition = SelectionMotionTransition::Begin;
    frame.find_transition = FindMotionTransition::None;
    frame.dropdown_transition = DropdownMotionTransition::None;
    assert!(matches!(
        frame.capture_continuation(&locator, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation)
    ));

    frame.selection_transition = SelectionMotionTransition::None;
    frame.find_transition = FindMotionTransition::Begin;
    assert!(matches!(
        frame.capture_continuation(&locator, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation)
    ));

    frame.selection_transition = SelectionMotionTransition::None;
    frame.find_transition = FindMotionTransition::None;
    frame.dropdown_transition = DropdownMotionTransition::None;
    assert!(matches!(
        frame.capture_continuation(&locator, &mut continuation),
        Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation)
    ));
}

#[test]
fn scenario_and_motion_continuation_errors_keep_typed_context() {
    for (error, expected) in [
        (
            FullTextCommandSurfaceScenarioError::LeaseConsumed,
            "scenario lease was already consumed",
        ),
        (
            FullTextCommandSurfaceScenarioError::InvalidProjection,
            "scenario projection is invalid",
        ),
        (
            FullTextCommandSurfaceScenarioError::RevisionExhausted,
            "scenario session revision is exhausted",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }

    for (error, expected) in [
        (
            KucOpaqueMotionContinuationError::Selection(
                KucTextSelectionContinuationError::Unavailable,
            ),
            "selection continuation failed: current root frame has no selectable text area",
        ),
        (
            KucOpaqueMotionContinuationError::Search(KucSearchTraceContinuationError::Unavailable),
            "search continuation failed: search trace is unavailable",
        ),
        (
            KucOpaqueMotionContinuationError::Click(KucOpaqueClickContinuationError::NotApplied),
            "click continuation failed: click continuation step was not applied",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn motion_frame_debug_and_error_display_keep_opaque_context() {
    let frame = FullTextCommandSurfaceMotionPlan::issue(
        FullTextCommandSurfaceMotionPlan::minimum_frame_count(),
    )
    .expect("complete motion plan")
    .frames()[0]
        .clone();
    assert_eq!(frame.event_count(), frame.stage.event_count());
    let debug = format!("{frame:?}");
    assert!(debug.contains("FullTextCommandSurfaceMotionFrame"));
    assert!(debug.contains("event_count:"));
    assert!(!debug.contains("日本語"));

    let errors = [
        FullTextCommandSurfaceMotionPlanError::IncompleteCatalogue {
            requested: 1,
            minimum: 2,
        },
        FullTextCommandSurfaceMotionPlanError::MissingContinuation,
        FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation,
        FullTextCommandSurfaceMotionPlanError::InvalidTransition,
        FullTextCommandSurfaceMotionPlanError::Selection(
            KucTextSelectionContinuationError::Unavailable,
        ),
        FullTextCommandSurfaceMotionPlanError::Search(KucSearchTraceContinuationError::Unavailable),
        FullTextCommandSurfaceMotionPlanError::Dropdown(KucInteractionLocatorError::Missing),
        FullTextCommandSurfaceMotionPlanError::Continuation(
            KucOpaqueMotionContinuationError::Click(KucOpaqueClickContinuationError::NotApplied),
        ),
    ];
    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn scenario_terminal_sinks_accept_opaque_submission_and_proposal() {
    let mut strip = SourceAddressStrip::new(SourceAddressPresentation::new(
        "表示",
        "ツールチップ",
        "アクセシビリティ",
    ));
    let _ = strip.apply_action(SourceAddressAction::SetDraft(String::from("opaque")));
    let submission = match strip.apply_action(SourceAddressAction::Submit) {
        Some(SourceAddressEvent::Submitted(submission)) => submission,
        _ => panic!("enabled source address should submit"),
    };
    NavigationInputAcknowledgementPort
        .forward_submission(submission)
        .expect("navigation sink accepts the one-shot submission");

    WorkspaceTabsAcknowledgementPort
        .forward_proposal(TabStripProposal::new(
            1,
            TabStripCorrelation::from_opaque_bytes([1]),
            TabStripProposalOperation::SelectPrevious,
        ))
        .expect("workspace-tabs sink consumes the opaque proposal");
}
