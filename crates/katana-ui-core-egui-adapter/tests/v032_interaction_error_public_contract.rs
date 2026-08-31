#[path = "text_command_surface/fixtures.rs"]
pub mod fixtures;

use katana_ui_core::atom::TextArea;
use katana_ui_core::molecule::command_chrome::{CommandChromeAction, CommandChromeToolbar};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use katana_ui_core_egui_adapter::status_bar::EguiStatusBarAdapter;
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRoot,
    EguiTextCommandSurfaceRootOutput, KucInteractionActionClass, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueClickContinuationError,
    KucSearchTraceContinuationError, KucTextSelectionContinuationError, TextCommandSurfaceStyle,
};

fn style() -> TextCommandSurfaceStyle {
    TextCommandSurfaceStyle {
        text_raster: fixtures::text_raster(),
        text_paint: fixtures::text_paint(),
        chrome_raster: fixtures::raster_style(),
        chrome_paint: fixtures::paint_style(),
        search: fixtures::search_style(),
    }
}

fn render(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    events: Vec<egui::Event>,
) -> EguiTextCommandSurfaceRootOutput {
    let mut output = None;
    let mut full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| output = Some(root.show(ui, &style())),
    );
    full.textures_delta.clear();
    output
        .expect("root did not run")
        .expect("root frame failed")
}

fn text_surface_fixture() -> TextSurface {
    let value = "選択範囲 ⭐️";
    let mut props = TextSurfaceProps::new(
        TextArea::new("v032.text-selection")
            .stable_state_id("v032.text-selection")
            .value(value),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 640, 96),
    )
    .adapter_measured_viewport();
    props.accessibility_label = "v032 selection".to_owned();
    props.context_target_label = Some("v032 context target".to_owned());
    let mut presentation = TextSurfacePresentation::from_props(&props);
    presentation.selection_start = 0;
    presentation.selection_end = value.len();
    let mut surface = TextSurface::new(props);
    assert!(surface.synchronize_presentation(presentation));
    surface
}

#[test]
fn interaction_locator_debug_and_revision_are_public_and_stable() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "v032.interaction-debug",
        EguiTextCommandSurface::new(fixtures::text_surface_fixture())
            .with_toolbar(fixtures::toolbar_fixture())
            .with_search_strip(fixtures::search_fixture(false)),
    )
    .expect("root construction");

    let initial = render(&context, &mut root, Vec::new());
    let locator = initial.interaction_locator();
    let request = locator
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("toolbar action");

    assert_eq!(format!("{locator:?}"), "KucInteractionLocator(..)");
    assert_eq!(format!("{request:?}"), "KucOpaqueInteractionRequest(..)");
    assert_eq!(locator.state_revision(), initial.frame().state_revision());
}

#[test]
fn actual_duplicate_toolbar_targets_fail_closed_before_raw_input() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let toolbar = CommandChromeToolbar::new()
        .action(CommandChromeAction::new("v032.duplicate", "first"))
        .action(CommandChromeAction::new("v032.duplicate", "second"));
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "v032.duplicate-target",
        EguiTextCommandSurface::new(fixtures::text_surface_fixture()).with_toolbar(toolbar),
    )
    .expect("root construction");

    let output = render(&context, &mut root, Vec::new());
    let input = egui::RawInput::default();
    assert!(matches!(
        output
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "v032.duplicate",
                KucInteractionActionClass::Toolbar,
            )),
        Err(KucInteractionLocatorError::Ambiguous)
    ));
    assert!(input.events.is_empty());
}

#[test]
fn interaction_error_debug_masking() {
    assert_eq!(
        format!("{}", KucInteractionRequestError::AlreadyQueued),
        "interaction request is already queued"
    );
    assert_eq!(
        format!("{}", KucInteractionLocatorError::Duplicate),
        "interaction action is duplicated"
    );
    assert_eq!(
        format!("{}", KucOpaqueClickContinuationError::AlreadyApplied),
        "click continuation step was already applied"
    );
    assert_eq!(
        format!("{}", KucSearchTraceContinuationError::Unavailable),
        "search trace is unavailable"
    );
    assert_eq!(
        format!("{}", KucSearchTraceContinuationError::FocusNotEstablished),
        "search query focus was not established"
    );
    assert_eq!(
        format!("{}", KucTextSelectionContinuationError::FloatingNotVisible),
        "text-selection continuation did not open floating output"
    );
}

#[test]
fn search_trace_fails_closed_without_text_input_target() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "v032.search-missing",
        EguiTextCommandSurface::new(fixtures::text_surface_fixture()),
    )
    .expect("root construction");

    let output = render(&context, &mut root, Vec::new());
    let locator = output.interaction_locator();
    assert_eq!(
        locator.begin_search_trace().expect_err("missing query"),
        KucSearchTraceContinuationError::Unavailable
    );
    assert_eq!(
        format!("{}", KucSearchTraceContinuationError::Unavailable),
        "search trace is unavailable"
    );
}

#[test]
fn search_trace_one_shot_and_fail_closed_when_not_applied() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "v032.search-linear",
        EguiTextCommandSurface::new(fixtures::text_surface_fixture())
            .with_search_strip(fixtures::search_fixture(false)),
    )
    .expect("root construction");
    let mut other = EguiTextCommandSurfaceRoot::with_identity(
        "v032.search-linear-other",
        EguiTextCommandSurface::new(fixtures::text_surface_fixture())
            .with_search_strip(fixtures::search_fixture(false)),
    )
    .expect("other root construction");

    let initial = render(&context, &mut root, Vec::new());
    let locator = initial.interaction_locator();
    let trace = locator
        .begin_search_trace()
        .expect("search trace starts from query");
    assert_eq!(format!("{trace:?}"), "KucOpaqueSearchTraceContinuation(..)");

    assert!(matches!(
        trace.advance(&locator),
        Err(KucSearchTraceContinuationError::NotApplied)
    ));

    let fresh_output = render(&context, &mut root, Vec::new());
    let fresh_locator = fresh_output.interaction_locator();
    let mut trace = fresh_locator
        .begin_search_trace()
        .expect("search trace starts from query");
    let mut input = egui::RawInput::default();
    assert_eq!(trace.apply_to_raw_input_once(&mut input), Ok(()));
    assert_eq!(
        trace.apply_to_raw_input_once(&mut input),
        Err(KucSearchTraceContinuationError::AlreadyApplied)
    );
    let focused_output = render(&context, &mut root, input.events);
    let _focused = focused_output.interaction_locator();
    let other_output = render(&context, &mut other, Vec::new());
    let other_locator = other_output.interaction_locator();
    assert!(matches!(
        trace.advance(&other_locator),
        Err(KucSearchTraceContinuationError::RootMismatch),
    ));
}

#[test]
fn click_continuation_is_one_shot_and_fail_closed_for_unapplied_or_wrong_root() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "v032.click-failclosed",
        EguiTextCommandSurface::new(fixtures::text_surface_fixture())
            .with_toolbar(fixtures::toolbar_fixture()),
    )
    .expect("root construction");
    let mut other = EguiTextCommandSurfaceRoot::with_identity(
        "v032.click-other",
        EguiTextCommandSurface::new(fixtures::text_surface_fixture())
            .with_toolbar(fixtures::toolbar_fixture()),
    )
    .expect("other root construction");

    let locator_output = render(&context, &mut root, Vec::new());
    let locator = locator_output.interaction_locator();
    let continuation = locator
        .begin_click(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("click continuation");

    assert!(matches!(
        continuation.advance(&locator),
        Err(KucOpaqueClickContinuationError::NotApplied)
    ));

    let other_locator_output = render(&context, &mut other, Vec::new());
    let other_locator = other_locator_output.interaction_locator();
    let mismatch_output = render(&context, &mut root, Vec::new());
    let mut mismatch = mismatch_output
        .interaction_locator()
        .begin_click(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("click continuation");
    let mut mismatch_raw = egui::RawInput::default();
    assert_eq!(mismatch.apply_to_raw_input_once(&mut mismatch_raw), Ok(()));
    assert!(matches!(
        mismatch.advance(&other_locator),
        Err(KucOpaqueClickContinuationError::RootMismatch)
    ));

    let mut applied = render(&context, &mut root, Vec::new())
        .interaction_locator()
        .begin_click(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("click continuation");
    let mut raw = egui::RawInput::default();
    assert_eq!(applied.apply_to_raw_input_once(&mut raw), Ok(()));
    let pressed_output = render(&context, &mut root, raw.events.clone());
    let pressed_locator = pressed_output.interaction_locator();
    let mut pressed = applied
        .advance(&pressed_locator)
        .expect("click press phase")
        .expect("click continuation advanced");
    assert_eq!(format!("{pressed:?}"), "KucOpaqueClickContinuation(..)");
    let mut press_raw = raw;
    assert_eq!(pressed.apply_to_raw_input_once(&mut press_raw), Ok(()));

    let release_output = render(&context, &mut root, press_raw.events);
    let release_locator = release_output.interaction_locator();
    let mut release = pressed
        .advance(&release_locator)
        .expect("click continuation progresses to release")
        .expect("click continuation in release phase");
    assert_eq!(format!("{release:?}"), "KucOpaqueClickContinuation(..)");
    let mut release_raw = egui::RawInput::default();
    assert_eq!(release.apply_to_raw_input_once(&mut release_raw), Ok(()));

    let done_output = render(&context, &mut root, release_raw.events);
    let done_locator = done_output.interaction_locator();
    assert_eq!(
        release
            .advance(&done_locator)
            .expect("click continuation completed")
            .map(|_| ()),
        None
    );
}

#[test]
fn text_selection_continuation_is_fail_closed_without_floating_output() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "v032.selection-no-floating",
        EguiTextCommandSurface::new(text_surface_fixture()),
    )
    .expect("root construction");

    let initial = render(&context, &mut root, Vec::new());
    let locator = initial.interaction_locator();
    let mut continuation = locator
        .begin_text_selection()
        .expect("text-selection continuation");

    assert_eq!(
        format!("{continuation:?}"),
        "KucOpaqueTextSelectionContinuation(..)"
    );

    let mut aim_input = egui::RawInput::default();
    assert_eq!(continuation.apply_to_raw_input_once(&mut aim_input), Ok(()));
    let mut continuation = continuation
        .advance(&render(&context, &mut root, aim_input.events).interaction_locator())
        .expect("selection press phase")
        .expect("selection press continuation");

    let mut press_input = egui::RawInput::default();
    assert_eq!(
        continuation.apply_to_raw_input_once(&mut press_input),
        Ok(())
    );
    let mut continuation = continuation
        .advance(&render(&context, &mut root, press_input.events).interaction_locator())
        .expect("selection midpoint phase")
        .expect("selection midpoint continuation");

    let mut midpoint_input = egui::RawInput::default();
    assert_eq!(
        continuation.apply_to_raw_input_once(&mut midpoint_input),
        Ok(())
    );
    let mut continuation = continuation
        .advance(&render(&context, &mut root, midpoint_input.events).interaction_locator())
        .expect("selection end phase")
        .expect("selection end continuation");

    let mut end_input = egui::RawInput::default();
    assert_eq!(continuation.apply_to_raw_input_once(&mut end_input), Ok(()));
    let mut continuation = continuation
        .advance(&render(&context, &mut root, end_input.events).interaction_locator())
        .expect("selection release phase")
        .expect("selection release continuation");

    let mut release_input = egui::RawInput::default();
    assert_eq!(
        continuation.apply_to_raw_input_once(&mut release_input),
        Ok(())
    );
    let final_error = continuation
        .advance(&render(&context, &mut root, release_input.events).interaction_locator())
        .expect_err("floating not visible");
    assert_eq!(
        final_error,
        KucTextSelectionContinuationError::FloatingNotVisible
    );
    assert_eq!(
        format!("{final_error}"),
        "text-selection continuation did not open floating output"
    );
}

#[test]
fn interaction_requests_and_continuations_do_not_leak_payload() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "v032.secret-debug",
        EguiTextCommandSurface::new(fixtures::text_surface_fixture())
            .with_toolbar(fixtures::toolbar_fixture()),
    )
    .expect("root construction");

    let initial = render(&context, &mut root, Vec::new());
    let locator = initial.interaction_locator();
    let mut request = locator
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("toolbar request");

    let mut input = egui::RawInput::default();
    let before = format!("{:?}", request);
    request
        .apply_to_raw_input_once(&mut input)
        .expect("request application");
    let after = format!("{:?}", request);
    assert_eq!(before, "KucOpaqueInteractionRequest(..)");
    assert_eq!(after, "KucOpaqueInteractionRequest(..)");
    assert_eq!(input.events.len(), 3);
    assert_eq!(
        request.apply_to_raw_input_once(&mut input),
        Err(KucInteractionRequestError::AlreadyQueued)
    );
    assert!(format!("{locator:?}").contains("KucInteractionLocator(..)"));

    assert!(matches!(
        locator.request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        )),
        Err(KucInteractionLocatorError::Duplicate)
    ));
    let mut search = render(&context, &mut root, Vec::new())
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("second request");
    let mut secondary_input = egui::RawInput::default();
    assert_eq!(search.apply_to_raw_input_once(&mut secondary_input), Ok(()));
    assert_eq!(
        search.apply_to_raw_input_once(&mut secondary_input),
        Err(KucInteractionRequestError::AlreadyQueued)
    );
}

#[test]
fn interaction_error_types_for_hidden_and_stale_paths_are_stable() {
    let missing_error = KucInteractionLocatorError::Missing;
    assert_eq!(format!("{missing_error}"), "interaction action is missing");
    let disabled_error = KucInteractionLocatorError::Disabled;
    assert_eq!(
        format!("{disabled_error}"),
        "interaction action is disabled"
    );
    let hidden_error = KucInteractionLocatorError::Hidden;
    assert_eq!(format!("{hidden_error}"), "interaction action is hidden");
    let ambiguous_error = KucInteractionLocatorError::Ambiguous;
    assert_eq!(
        format!("{ambiguous_error}"),
        "interaction action is ambiguous"
    );
}

#[test]
fn request_with_matching_root_and_revision_but_different_event_batch_is_stale() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let identity = "v032.correlation-stale";
    let mut event_root = EguiTextCommandSurfaceRoot::with_identity(
        identity,
        EguiTextCommandSurface::new(fixtures::text_surface_fixture())
            .with_toolbar(fixtures::toolbar_fixture()),
    )
    .expect("event root construction");

    let initial = render(&context, &mut event_root, Vec::new());
    let mut activation_input = egui::RawInput::default();
    initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("toolbar activation request")
        .apply_to_raw_input_once(&mut activation_input)
        .expect("toolbar activation input");
    let event_frame = render(&context, &mut event_root, activation_input.events);
    let request = event_frame
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("request bound to the event frame");

    let props = TextSurfaceProps::new(
        TextArea::new("v032.correlation-controlled")
            .stable_state_id("v032.correlation-controlled")
            .value("controlled"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 640, 96),
    )
    .adapter_measured_viewport();
    let mut presentation = TextSurfacePresentation::from_props(&props);
    presentation.value = "controlled revision".to_owned();
    let mut controlled_root = EguiTextCommandSurfaceRoot::with_identity(
        identity,
        EguiTextCommandSurface::new(TextSurface::new(props)),
    )
    .expect("controlled root construction");
    assert!(
        controlled_root.synchronize_presentation(EguiTextCommandSurfacePresentation {
            text_state_id: None,
            text: presentation,
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        })
    );
    let current_frame = render(&context, &mut controlled_root, Vec::new());
    assert_eq!(
        event_frame.interaction_locator().state_revision(),
        current_frame.interaction_locator().state_revision()
    );

    let mut input = egui::RawInput {
        events: vec![egui::Event::Copy],
        ..egui::RawInput::default()
    };
    assert_eq!(
        current_frame
            .interaction_locator()
            .queue_request(request, &mut input),
        Err(KucInteractionRequestError::Stale)
    );
    assert_eq!(input.events, vec![egui::Event::Copy]);
}

#[test]
fn status_bar_public_evidence_accessors_start_empty() {
    let adapter = EguiStatusBarAdapter::new("v032.status-evidence")
        .expect("status adapter construction should use its matching catalog policy");

    assert!(adapter.artifact_paint_plan().is_none());
    assert!(adapter.raster_evidence().is_empty());
}
