#[test]
fn state_accepts_exact_japanese_vs16_and_zwj_text() {
    let value = String::from("日本語 ⭐️ family: 👩‍💻");
    let mut state = ScenarioSessionState::default();
    state.apply(
        ScenarioSessionUpdate {
            text: Some(value.clone()),
            selection: Some((0, value.len())),
            search_query: None,
            replace_value: None,
        },
        &value,
    );

    let presentation = state.presentation(super::FullTextCommandSurfaceScenarioId::WorkspaceTabs);
    assert_eq!(presentation.text.value, value);
    assert_eq!(presentation.text.selection_start, 0);
    assert_eq!(
        presentation.text.selection_end,
        presentation.text.value.len()
    );
}

#[test]
fn invalid_utf8_selection_does_not_replace_the_last_valid_selection() {
    let value = String::from("⭐️");
    let mut state = ScenarioSessionState::default();
    state.apply(
        ScenarioSessionUpdate {
            text: Some(value.clone()),
            selection: Some((0, value.len())),
            search_query: None,
            replace_value: None,
        },
        &value,
    );
    state.apply(
        ScenarioSessionUpdate {
            text: None,
            selection: Some((1, value.len())),
            search_query: None,
            replace_value: None,
        },
        &value,
    );

    let presentation = state.presentation(super::FullTextCommandSurfaceScenarioId::WorkspaceTabs);
    assert_eq!(
        (
            presentation.text.selection_start,
            presentation.text.selection_end
        ),
        (0, value.len())
    );
    assert!(!valid_selection(&value, 1, value.len()));
}

#[test]
fn replacement_input_never_changes_generic_text() {
    let original = String::from("original ⭐️");
    let mut state = ScenarioSessionState::default();
    state.apply(
        ScenarioSessionUpdate {
            text: Some(original.clone()),
            selection: None,
            search_query: Some(String::from("query")),
            replace_value: Some(String::from("replacement")),
        },
        &original,
    );

    let presentation = state.presentation(super::FullTextCommandSurfaceScenarioId::WorkspaceTabs);
    assert_eq!(presentation.text.value, original);
    let search = presentation.search.expect("search is present");
    assert_eq!(search.value.query, "query");
    assert_eq!(search.value.replace_value, "replacement");
}

#[test]
fn physical_ime_commit_updates_only_the_next_kuc_scenario_projection() {
    let session = FullTextCommandSurfaceScenarioSession::new(
        FullTextCommandSurfaceScenarioId::ResizeScrollIme,
    );
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::ResizeScrollIme)
        .expect("scenario stages issue");
    let stages = scenario.stages().to_vec();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(session.retain_lease().expect("initial session lease"))
        .expect("session root retains");
    let context = egui::Context::default();

    for stage in stages.iter().skip(1) {
        let mut input = egui::RawInput::default();
        stage.apply_to(&mut input);
        let _ = render_and_forward(
            &context,
            &mut root,
            session.synchronize_lease().expect("current session lease"),
            input,
        );
    }

    let updated = render_and_forward(
        &context,
        &mut root,
        session.synchronize_lease().expect("updated session lease"),
        egui::RawInput::default(),
    );
    assert!(
        updated
            .evidence_text
            .record
            .frame
            .layout_identity
            .contains("入力"),
        "the KUC-owned next frame retains the physical IME commit"
    );
    assert!(
        updated
            .evidence_text
            .record
            .frame
            .layout_identity
            .contains("⭐️"),
        "the session preserves the existing VS16 fixture while accepting IME input"
    );
    assert!(
        updated.evidence_composite.non_transparent_pixel_count > 0,
        "the updated value remains part of the KUC-owned composite"
    );
}

#[test]
fn next_revision_exhausts_before_wraparound() {
    let session =
        FullTextCommandSurfaceScenarioSession::new(FullTextCommandSurfaceScenarioId::Resting);
    session.next_revision.set(u64::MAX);
    assert!(matches!(
        session.next_revision(),
        Err(FullTextCommandSurfaceScenarioError::RevisionExhausted)
    ));
}

#[test]
fn scenario_session_update_is_empty_by_default_and_not_empty_after_changes() {
    let mut update = ScenarioSessionUpdate::default();
    assert!(update.is_empty());

    update.text = Some(String::from("kuc-motion"));
    assert!(!update.is_empty());
}

#[test]
fn utf8_selection_boundaries_are_respected_before_replacing_previous_selection() {
    let mut state = ScenarioSessionState::default();
    let value = String::from("ab😀cd");
    state.apply(
        ScenarioSessionUpdate {
            text: Some(value.clone()),
            selection: Some((0, 2)),
            search_query: None,
            replace_value: None,
        },
        &value,
    );

    state.apply(
        ScenarioSessionUpdate {
            text: None,
            selection: Some((1, 3)),
            search_query: None,
            replace_value: None,
        },
        &value,
    );
    let middle = state.presentation(super::FullTextCommandSurfaceScenarioId::WorkspaceTabs);
    assert_eq!(
        (middle.text.selection_start, middle.text.selection_end),
        (0, 2),
        "selection must stay on last valid UTF-8 boundary pair"
    );

    state.apply(
        ScenarioSessionUpdate {
            text: None,
            selection: Some((0, value.len() + 1)),
            search_query: None,
            replace_value: None,
        },
        &value,
    );
    let next = state.presentation(super::FullTextCommandSurfaceScenarioId::WorkspaceTabs);
    assert_eq!(
        (next.text.selection_start, next.text.selection_end),
        (0, 2),
        "selection exceeding UTF-8 text length must not replace last valid range"
    );
}
