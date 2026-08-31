#[test]
fn resting_has_no_floating_output() {
    let mut root = retained(FullTextCommandSurfaceScenarioId::Resting);
    let output = render(&egui::Context::default(), &mut root, None);
    assert!(output
        .floating
        .as_ref()
        .and_then(|value| value.record.as_ref())
        .is_none());
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
    assert!(readonly
        .actions
        .iter()
        .all(|action| action.disabled_model()));
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
