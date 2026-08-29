use super::{
    FullTextCommandSurfaceScenarioError, FullTextCommandSurfaceScenarioSession,
    ScenarioSessionState, ScenarioSessionUpdate, valid_selection,
};
use crate::text_command_surface::{
    EguiTextCommandSurfaceRootEventTransport, EguiTextCommandSurfaceRootFactory,
    FullTextCommandSurfaceScenarioFactory, FullTextCommandSurfaceScenarioId,
    KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::selection::ContextMenuEvent;
use katana_ui_core::text_surface::TextSurfaceEvent;
use std::convert::Infallible;

struct SessionDispatcher;

impl KucRootEventBatchDispatcher for SessionDispatcher {
    type Error = Infallible;

    fn dispatch_text_events(&mut self, _events: Vec<TextSurfaceEvent>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_search_events(
        &mut self,
        _events: Vec<CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct SessionForwarder;

impl KucRootEventBatchForwarder for SessionForwarder {
    type Error = String;

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        transport
            .dispatch_once(&mut SessionDispatcher)
            .map(|_| ())
            .map_err(|error| format!("session event dispatch failed: {error:?}"))
    }
}

fn render_and_forward(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    lease: crate::text_command_surface::EguiTextCommandSurfaceHostProjectionLease,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
    root.synchronize_with_lease(lease)
        .expect("current scenario lease synchronizes");
    let mut output = None;
    let mut platform_output = context.run_ui(input, |ui| {
        output = Some(root.show_output_for_test(ui));
    });
    platform_output.textures_delta.clear();
    let output = output
        .expect("root renders")
        .expect("root output is available");
    output
        .events()
        .forward_once(&mut SessionForwarder)
        .expect("one-shot scenario event transport forwards");
    output
}

#[test]
fn state_accepts_exact_japanese_vs16_and_zwj_text() {
    let value = String::from("日本語 ⭐️ family: 👩‍💻");
    let mut state = ScenarioSessionState::default();
    state.apply(ScenarioSessionUpdate {
        text: Some(value.clone()),
        selection: Some((0, value.len())),
        search_query: None,
        replace_value: None,
    });

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
    state.apply(ScenarioSessionUpdate {
        text: Some(value.clone()),
        selection: Some((0, value.len())),
        search_query: None,
        replace_value: None,
    });
    state.apply(ScenarioSessionUpdate {
        text: None,
        selection: Some((1, value.len())),
        search_query: None,
        replace_value: None,
    });

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
    state.apply(ScenarioSessionUpdate {
        text: Some(original.clone()),
        selection: None,
        search_query: None,
        replace_value: Some(String::from("replacement")),
    });

    let presentation = state.presentation(super::FullTextCommandSurfaceScenarioId::WorkspaceTabs);
    assert_eq!(presentation.text.value, original);
    assert_eq!(
        presentation
            .search
            .expect("search is present")
            .value
            .replace_value,
        "replacement"
    );
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
