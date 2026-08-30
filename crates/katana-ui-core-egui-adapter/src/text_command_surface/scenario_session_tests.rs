use super::{
    FullTextCommandSurfaceScenarioError, FullTextCommandSurfaceScenarioSession,
    ScenarioSessionState, ScenarioSessionUpdate, valid_selection,
};
use crate::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceRootEventTransport,
    EguiTextCommandSurfaceRootFactory, FullTextCommandSurfaceScenarioFactory,
    FullTextCommandSurfaceScenarioId, KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
    TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeCapability, CommandChromeSearchEvent, CommandChromeToolbarEvent,
    FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::selection::ContextMenuEvent;
use katana_ui_core::molecule::structured::ReplaceMode;
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
    render_current_and_forward(context, root, input)
}

fn render_current_and_forward(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
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

fn render_current(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
    let mut output = None;
    crate::run_ui_discard(context, input, |ui| {
        output = Some(root.show_output_for_test(ui));
    });
    output
        .expect("root renders")
        .expect("root output is available")
}

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

#[test]
fn text_change_forwarding_updates_the_next_session_projection() {
    let session = FullTextCommandSurfaceScenarioSession::new(
        FullTextCommandSurfaceScenarioId::ResizeScrollIme,
    );
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::ResizeScrollIme)
        .expect("scenario stages issue");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(session.retain_lease().expect("initial session lease"))
        .expect("session root retains");
    let context = egui::Context::default();

    for stage in scenario.stages().iter().take(2) {
        let mut input = egui::RawInput::default();
        stage.apply_to(&mut input);
        let _ = render_and_forward(
            &context,
            &mut root,
            session.synchronize_lease().expect("current session lease"),
            input,
        );
    }

    let mut input = egui::RawInput::default();
    input
        .events
        .push(egui::Event::Text(String::from("session text ⭐️")));
    let _ = render_and_forward(
        &context,
        &mut root,
        session
            .synchronize_lease()
            .expect("session lease synchronizes"),
        input,
    );

    let updated = render_and_forward(
        &context,
        &mut root,
        session
            .synchronize_lease()
            .expect("updated session lease synchronizes"),
        egui::RawInput::default(),
    );
    assert!(
        updated
            .evidence_text
            .record
            .frame
            .layout_identity
            .contains("session text ⭐️"),
        "a forwarded text change must be retained by the next opaque projection"
    );
}

#[test]
fn physical_selection_is_retained_by_the_next_scenario_projection() {
    let session =
        FullTextCommandSurfaceScenarioSession::new(FullTextCommandSurfaceScenarioId::Selection);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(session.retain_lease().expect("initial session lease"))
        .expect("session root retains");
    let context = egui::Context::default();
    let motion_input = || {
        let mut input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..egui::RawInput::default()
        };
        input.viewports.insert(
            egui::ViewportId::ROOT,
            egui::ViewportInfo {
                native_pixels_per_point: Some(1.0),
                ..egui::ViewportInfo::default()
            },
        );
        input
    };
    let initial = render_and_forward(
        &context,
        &mut root,
        session.synchronize_lease().expect("initial frame lease"),
        motion_input(),
    );
    let mut continuation = initial
        .interaction_locator()
        .begin_text_selection()
        .expect("selection continuation");

    for _ in 0..5 {
        let mut input = motion_input();
        continuation
            .apply_to_raw_input_once(&mut input)
            .expect("selection step applies");
        let output = render_current_and_forward(&context, &mut root, input);
        match continuation
            .advance(output.interaction_locator())
            .expect("selection step advances")
        {
            Some(next) => continuation = next,
            None => break,
        }
    }

    let selection = session
        .state
        .borrow()
        .selection
        .expect("physical selection is retained by the session router");
    assert!(selection.0 < selection.1);
}

#[test]
fn physical_search_trace_routes_query_non_value_and_close_events() {
    let session =
        FullTextCommandSurfaceScenarioSession::new(FullTextCommandSurfaceScenarioId::Find);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(session.retain_lease().expect("initial session lease"))
        .expect("session root retains");
    let context = egui::Context::default();
    let initial = render_and_forward(
        &context,
        &mut root,
        session.synchronize_lease().expect("initial frame lease"),
        egui::RawInput::default(),
    );
    let mut continuation = initial
        .interaction_locator()
        .begin_search_trace()
        .expect("search continuation");

    for step in 0..7 {
        let mut input = egui::RawInput::default();
        continuation
            .apply_to_raw_input_once(&mut input)
            .expect("search step applies");
        let output = if step == 6 {
            render_current_and_forward(&context, &mut root, input)
        } else {
            render_and_forward(
                &context,
                &mut root,
                session.synchronize_lease().expect("search frame lease"),
                input,
            )
        };
        match continuation
            .advance(output.interaction_locator())
            .expect("search step advances")
        {
            Some(next) => continuation = next,
            None => break,
        }
    }

    assert!(
        session
            .state
            .borrow()
            .search_query
            .as_deref()
            .is_some_and(|query| query.contains("入力 ⭐️"))
    );
}

#[test]
fn physical_replace_input_is_extracted_from_the_actual_root_context() {
    let mut presentation = super::scenario::presentation(FullTextCommandSurfaceScenarioId::Find);
    let search = presentation.search.as_mut().expect("find search");
    search.value.replace_mode = ReplaceMode::Visible;
    search.value.capabilities.replace = CommandChromeCapability::available();
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"scenario-replace-target",
        presentation,
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("replace token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("replace root retains");
    let context = egui::Context::default();
    let initial = render_current(&context, &mut root, egui::RawInput::default());
    let bounds = initial
        .search_record
        .as_ref()
        .and_then(|record| record.replace.as_ref())
        .expect("visible replace input")
        .frame
        .content_bounds;
    let pointer = egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    );
    let pointer_event = |pressed| egui::Event::PointerButton {
        pos: pointer,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    let _ = render_current(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![egui::Event::PointerMoved(pointer), pointer_event(true)],
            ..egui::RawInput::default()
        },
    );
    let _ = render_current(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_event(false)],
            ..egui::RawInput::default()
        },
    );
    let changed = render_current(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![egui::Event::Text(String::from("replacement ⭐️"))],
            ..egui::RawInput::default()
        },
    );
    let update = ScenarioSessionUpdate::from_context(&changed.events().current_context());

    assert!(
        update
            .replace_value
            .as_deref()
            .is_some_and(|value| value.contains("replacement ⭐️"))
    );
}
