#[path = "text_command_surface/fixtures.rs"]
mod fixtures;

use katana_ui_core::atom::TextArea;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeToolbar, FloatingCommandToolbarVisibility,
};
use katana_ui_core::molecule::selection::ContextMenuItemKind;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use katana_ui_core_egui_adapter::context_menu::{
    ContextMenuPresentation, ContextMenuPresentationItem,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceRoot,
    EguiTextCommandSurfaceRootEventBatchForwardError,
    EguiTextCommandSurfaceRootEventDispatchReceipt, EguiTextCommandSurfaceRootEventTransport,
    EguiTextCommandSurfaceRootOutput, KucInteractionActionClass, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucRootEventBatchDispatcher,
    KucRootEventBatchForwarder, TextCommandSurfaceStyle,
};

// This test intentionally uses only the public facade, as a separate consumer crate would.
// The event payload remains inaccessible; only the typed dispatch receipt is retained.
struct PublicDispatchConsumer {
    transport: Option<EguiTextCommandSurfaceRootEventTransport>,
}

impl KucRootEventBatchForwarder for PublicDispatchConsumer {
    type Error = ();

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.transport = Some(transport);
        Ok(())
    }
}

struct PublicDispatcher {
    calls: usize,
}

impl KucRootEventBatchDispatcher for PublicDispatcher {
    type Error = ();

    fn dispatch_text_events(
        &mut self,
        _events: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        Ok(())
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        Ok(())
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        Ok(())
    }

    fn dispatch_search_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        Ok(())
    }
}

fn dispatch_receipt_from_public_api(
    output: &EguiTextCommandSurfaceRootOutput,
) -> (EguiTextCommandSurfaceRootEventDispatchReceipt, usize) {
    let mut consumer = PublicDispatchConsumer { transport: None };
    output
        .events()
        .forward_once(&mut consumer)
        .expect("public root event forwarding succeeds");
    let mut dispatcher = PublicDispatcher { calls: 0 };
    let receipt = consumer
        .transport
        .expect("forwarder retained the opaque transport")
        .dispatch_once(&mut dispatcher)
        .expect("public dispatch succeeds");
    (receipt, dispatcher.calls)
}

struct RecordingForwarder {
    calls: usize,
    transport: Option<EguiTextCommandSurfaceRootEventTransport>,
}

impl KucRootEventBatchForwarder for RecordingForwarder {
    type Error = ();

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        self.transport = Some(transport);
        Ok(())
    }
}

fn style() -> TextCommandSurfaceStyle {
    TextCommandSurfaceStyle {
        text_raster: fixtures::text_raster(),
        text_paint: fixtures::text_paint(),
        chrome_raster: fixtures::raster_style(),
        chrome_paint: fixtures::paint_style(),
        search: fixtures::search_style(),
    }
}

fn root() -> EguiTextCommandSurfaceRoot {
    let surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
        .with_toolbar(fixtures::toolbar_fixture())
        .with_search_strip(fixtures::search_fixture(false));
    EguiTextCommandSurfaceRoot::with_identity("contract.text-command-root", surface)
        .expect("root construction")
}

fn root_with_identity(identity: &str) -> EguiTextCommandSurfaceRoot {
    let surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
        .with_toolbar(fixtures::toolbar_fixture())
        .with_search_strip(fixtures::search_fixture(false));
    EguiTextCommandSurfaceRoot::with_identity(identity, surface).expect("root construction")
}

fn use_all_fixture_contracts() {
    let _ = fixtures::floating_toolbar_fixture();
    let _ = fixtures::toolbar_presentation();
    let _ = fixtures::floating_toolbar_presentation();
    let _ = fixtures::search_presentation();
    let _ = fixtures::search_presentation_state_id();
    let _ = fixtures::script_line_height();
}

fn render(root: &mut EguiTextCommandSurfaceRoot) -> EguiTextCommandSurfaceRootOutput {
    use_all_fixture_contracts();
    let context = egui::Context::default();
    let mut result = None;
    let mut full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui, &style())),
    );
    full.textures_delta.clear();
    result
        .expect("root did not run")
        .expect("root frame failed")
}

fn render_actual(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, EguiTextCommandSurfaceRootOutput) {
    let mut result = None;
    let mut full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui, &style())),
    );
    full.textures_delta.clear();
    (
        full,
        result
            .expect("root did not run")
            .expect("root frame failed"),
    )
}

fn render_actual_in_central_panel(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, EguiTextCommandSurfaceRootOutput) {
    let mut result = None;
    let mut full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                result = Some(root.show(ui, &style()));
            });
        },
    );
    full.textures_delta.clear();
    (
        full,
        result
            .expect("root did not run")
            .expect("root frame failed"),
    )
}

fn raw_input_snapshot(input: &egui::RawInput) -> String {
    format!("{input:#?}")
}

fn accesskit_has_label(full: &egui::FullOutput, label: &str) -> bool {
    full.platform_output
        .accesskit_update
        .as_ref()
        .is_some_and(|update| {
            update.nodes.iter().any(|(_, node)| {
                node.label().is_some_and(|actual| actual.contains(label)) && node.bounds().is_some()
            })
        })
}

#[test]
fn central_panel_pointer_state_opens_context_menu_from_fresh_root_frame() {
    let context = egui::Context::default();
    let menu = ContextMenuPresentation {
        visible: true,
        items: vec![ContextMenuPresentationItem::action(
            "context-format",
            "整形 ⭐️",
        )],
    };
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "contract.central-panel-context-root",
        EguiTextCommandSurface::new(selected_text_surface()).with_context_menu(menu),
    )
    .expect("root construction");
    let (_, initial) = render_actual_in_central_panel(&context, &mut root, Vec::new());
    let mut request = initial
        .interaction_locator()
        .request_context_open()
        .expect("current TextSurface response provides context opener");
    let mut input = egui::RawInput::default();
    request
        .apply_to_raw_input_once(&mut input)
        .expect("context opener request");
    let (_, opened) = render_actual_in_central_panel(&context, &mut root, input.events);
    assert!(
        opened
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "context-format",
                KucInteractionActionClass::ContextMenuItem,
            ))
            .is_ok()
    );
}

fn selected_text_surface() -> TextSurface {
    let value = "選択範囲 ⭐️";
    let mut props = TextSurfaceProps::new(
        TextArea::new("root-floating-text")
            .stable_state_id("root-floating-text")
            .value(value),
        Vec::new(),
        TextSurfaceViewport::new(
            0,
            0,
            fixtures::FRAME_WIDTH as u32,
            fixtures::FRAME_HEIGHT as u32,
        ),
    )
    .adapter_measured_viewport();
    props.accessibility_label = "root floating text".to_owned();
    props.context_target_label = Some("root context target".to_owned());
    let mut presentation = TextSurfacePresentation::from_props(&props);
    presentation.selection_start = 0;
    presentation.selection_end = value.len();
    let mut surface = TextSurface::new(props);
    assert!(surface.synchronize_presentation(presentation));
    surface
}

#[test]
fn same_raw_input_trace_produces_deterministic_closed_root_contracts() -> Result<(), String> {
    let first = render(&mut root());
    let second = render(&mut root());
    let mut first_forwarder = RecordingForwarder {
        calls: 0,
        transport: None,
    };
    let mut second_forwarder = RecordingForwarder {
        calls: 0,
        transport: None,
    };
    let first_receipt = first
        .events()
        .forward_once(&mut first_forwarder)
        .map_err(|_| "first root event forwarding failed".to_owned())?;
    let second_receipt = second
        .events()
        .forward_once(&mut second_forwarder)
        .map_err(|_| "second root event forwarding failed".to_owned())?;

    assert_eq!(first.frame().identity(), second.frame().identity());
    assert_eq!(
        first.frame().state_revision(),
        second.frame().state_revision()
    );
    assert_eq!(first.frame().dimensions(), second.frame().dimensions());
    assert_eq!(first.frame().rgba_hash(), second.frame().rgba_hash());
    assert_eq!(
        first.frame().paint_plan_hash(),
        second.frame().paint_plan_hash()
    );
    assert_eq!(first.frame().record_hash(), second.frame().record_hash());
    assert_eq!(
        first.frame().accessibility().snapshot_hash(),
        second.frame().accessibility().snapshot_hash()
    );
    assert_eq!(
        first_receipt.correlation_fingerprint(),
        second_receipt.correlation_fingerprint()
    );
    assert_eq!(
        first_receipt.event_batch_fingerprint(),
        second_receipt.event_batch_fingerprint()
    );
    assert_eq!(
        first_receipt.event_cardinality(),
        second_receipt.event_cardinality()
    );
    Ok(())
}

#[test]
fn actual_root_locator_uses_current_response_accesskit_and_one_shot_raw_input() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = root();
    let (initial_full, initial) = render_actual(&context, &mut root, Vec::new());
    assert!(accesskit_has_label(&initial_full, "太字"));
    let missing_input = egui::RawInput {
        events: vec![egui::Event::Copy],
        ..egui::RawInput::default()
    };
    let missing_before = raw_input_snapshot(&missing_input);
    assert!(matches!(
        initial.interaction_locator().request_context_open(),
        Err(KucInteractionLocatorError::Missing)
    ));
    assert_eq!(raw_input_snapshot(&missing_input), missing_before);
    let mut search_input = egui::RawInput::default();
    initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "storybook.command-chrome.search:use-regex",
            KucInteractionActionClass::SearchControl,
        ))
        .expect("search control from current frame")
        .apply_to_raw_input_once(&mut search_input)
        .expect("search request");
    assert_eq!(search_input.events.len(), 3);

    let mut raw = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
        )),
        ..egui::RawInput::default()
    };
    let mut request = initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("toolbar action from current frame");
    assert!(matches!(
        initial
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "inline-bold",
                KucInteractionActionClass::Toolbar,
            )),
        Err(KucInteractionLocatorError::Duplicate)
    ));
    request
        .apply_to_raw_input_once(&mut raw)
        .expect("one-shot request");
    assert_eq!(
        request.apply_to_raw_input_once(&mut raw),
        Err(KucInteractionRequestError::AlreadyQueued)
    );
    assert_eq!(raw.events.len(), 3);
    let (activated_full, activated) = render_actual(&context, &mut root, raw.events);
    assert!(accesskit_has_label(&activated_full, "太字"));
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport: None,
    };
    let receipt = activated
        .events()
        .forward_once(&mut forwarder)
        .expect("activated root event transport");
    assert!(receipt.event_cardinality() > 0);

    let disabled = initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "disabled",
            KucInteractionActionClass::Toolbar,
        ));
    assert!(matches!(
        disabled,
        Err(KucInteractionLocatorError::Disabled)
    ));

    let unmapped_input = egui::RawInput {
        events: vec![egui::Event::Copy],
        ..egui::RawInput::default()
    };
    let before_unmapped = raw_input_snapshot(&unmapped_input);
    assert!(matches!(
        initial
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "storybook.command-chrome.search:result-summary",
                KucInteractionActionClass::SearchControl,
            )),
        Err(KucInteractionLocatorError::Missing | KucInteractionLocatorError::Hidden)
    ));
    assert_eq!(raw_input_snapshot(&unmapped_input), before_unmapped);
}

#[test]
fn actual_root_locator_rejects_duplicate_action_identities_as_ambiguous() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let toolbar = CommandChromeToolbar::new()
        .action(CommandChromeAction::new("duplicate", "First"))
        .action(CommandChromeAction::new("duplicate", "Second"));
    let surface =
        EguiTextCommandSurface::new(fixtures::text_surface_fixture()).with_toolbar(toolbar);
    let mut root = EguiTextCommandSurfaceRoot::with_identity("contract.ambiguous-root", surface)
        .expect("root construction");
    let (_, output) = render_actual(&context, &mut root, Vec::new());
    assert!(matches!(
        output
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "duplicate",
                KucInteractionActionClass::Toolbar,
            )),
        Err(KucInteractionLocatorError::Ambiguous)
    ));
}

#[test]
fn actual_root_locator_resolves_closed_dropdown_trigger_and_all_seventeen_items() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = root();
    let (_, initial) = render_actual(&context, &mut root, Vec::new());
    let hidden_input = egui::RawInput::default();
    let hidden_before = raw_input_snapshot(&hidden_input);
    let hidden = initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "code-01",
            KucInteractionActionClass::DropdownItem,
        ));
    assert!(matches!(hidden, Err(KucInteractionLocatorError::Hidden)));
    assert_eq!(raw_input_snapshot(&hidden_input), hidden_before);
    let mut trigger_input = egui::RawInput::default();
    initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "code-block",
            KucInteractionActionClass::DropdownTrigger,
        ))
        .expect("closed split trigger")
        .apply_to_raw_input_once(&mut trigger_input)
        .expect("trigger request");
    let (opened_full, opened) = render_actual(&context, &mut root, trigger_input.events);
    assert!(accesskit_has_label(&opened_full, "候補 01 ⭐️"));
    for index in 1..=17 {
        let id = format!("code-{index:02}");
        assert!(
            opened
                .interaction_locator()
                .request(KucInteractionSelector::new(
                    id,
                    KucInteractionActionClass::DropdownItem,
                ))
                .is_ok(),
            "dropdown item {index} was not in the current frame"
        );
    }
}

#[test]
fn actual_root_locator_resolves_floating_and_context_targets_with_accesskit() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let floating_surface = EguiTextCommandSurface::new(selected_text_surface())
        .with_floating_toolbar(
            fixtures::floating_toolbar_fixture(),
            FloatingCommandToolbarVisibility::Visible,
        );
    let mut floating_root =
        EguiTextCommandSurfaceRoot::with_identity("contract.floating-root", floating_surface)
            .expect("root construction");
    let (floating_full, floating) = render_actual(&context, &mut floating_root, Vec::new());
    assert!(accesskit_has_label(&floating_full, "選択ツール ⭐️"));
    let mut floating_input = egui::RawInput::default();
    floating
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "floating-bold",
            KucInteractionActionClass::FloatingToolbar,
        ))
        .expect("floating action from actual frame")
        .apply_to_raw_input_once(&mut floating_input)
        .expect("floating request");
    let (floating_activated_full, floating_activated) =
        render_actual(&context, &mut floating_root, floating_input.events);
    assert!(accesskit_has_label(
        &floating_activated_full,
        "選択ツール ⭐️"
    ));
    assert!(
        floating_activated
            .events()
            .forward_once(&mut RecordingForwarder {
                calls: 0,
                transport: None,
            })
            .is_ok()
    );

    let menu = ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("context-format", "整形 ⭐️"),
            ContextMenuPresentationItem {
                id: "context-disabled".to_owned(),
                label: "利用不可".to_owned(),
                accessibility_label: "利用不可".to_owned(),
                icon: None,
                enabled: false,
                checked: false,
                kind: ContextMenuItemKind::Action,
                children: Vec::new(),
            },
        ],
    };
    let context_surface =
        EguiTextCommandSurface::new(selected_text_surface()).with_context_menu(menu);
    let mut context_root =
        EguiTextCommandSurfaceRoot::with_identity("contract.context-root", context_surface)
            .expect("root construction");
    let (context_initial_full, context_initial) =
        render_actual(&context, &mut context_root, Vec::new());
    let mut context_open_input = egui::RawInput::default();
    let mut context_open_request = context_initial
        .interaction_locator()
        .request_context_open()
        .expect("current TextSurface response provides context opener");
    assert_eq!(
        format!("{context_open_request:?}"),
        "KucOpaqueInteractionRequest(..)"
    );
    context_open_request
        .apply_to_raw_input_once(&mut context_open_input)
        .expect("context opener request");
    assert_eq!(context_open_input.events.len(), 3);
    assert!(matches!(
        context_open_input.events[1],
        egui::Event::PointerButton {
            button: egui::PointerButton::Secondary,
            pressed: true,
            ..
        }
    ));
    assert!(matches!(
        context_open_input.events[2],
        egui::Event::PointerButton {
            button: egui::PointerButton::Secondary,
            pressed: false,
            ..
        }
    ));
    let before_replay = raw_input_snapshot(&context_open_input);
    assert_eq!(
        context_open_request.apply_to_raw_input_once(&mut context_open_input),
        Err(KucInteractionRequestError::AlreadyQueued)
    );
    assert_eq!(raw_input_snapshot(&context_open_input), before_replay);
    let (context_open_full, context_open) =
        render_actual(&context, &mut context_root, context_open_input.events);
    assert!(accesskit_has_label(
        &context_initial_full,
        "root context target"
    ));
    assert!(accesskit_has_label(&context_open_full, "整形 ⭐️"));
    let mut context_input = egui::RawInput::default();
    context_open
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "context-format",
            KucInteractionActionClass::ContextMenuItem,
        ))
        .expect("context action from actual frame")
        .apply_to_raw_input_once(&mut context_input)
        .expect("context request");
    let (_, context_selected) = render_actual(&context, &mut context_root, context_input.events);
    assert!(
        context_selected
            .events()
            .forward_once(&mut RecordingForwarder {
                calls: 0,
                transport: None,
            })
            .is_ok()
    );
}

#[test]
fn current_locator_rejects_cross_root_and_prior_revision_requests_without_mutation() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut first_root = root();
    let mut second_root = root_with_identity("contract.other-root");
    let (_, first_frame) = render_actual(&context, &mut first_root, Vec::new());
    let (_, second_frame) = render_actual(&context, &mut second_root, Vec::new());
    let cross_root_request = first_frame
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("first root request");
    let mut cross_root_input = egui::RawInput {
        events: vec![egui::Event::Copy],
        ..egui::RawInput::default()
    };
    let before_cross_root = raw_input_snapshot(&cross_root_input);
    assert_eq!(
        second_frame
            .interaction_locator()
            .queue_request(cross_root_request, &mut cross_root_input),
        Err(KucInteractionRequestError::RootMismatch)
    );
    assert_eq!(raw_input_snapshot(&cross_root_input), before_cross_root);

    let mut revision_root = root();
    let (_, initial_frame) = render_actual(&context, &mut revision_root, Vec::new());
    let stale_request = initial_frame
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("initial revision request");
    let mut advance_input = egui::RawInput::default();
    initial_frame
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "storybook.command-chrome.search:use-regex",
            KucInteractionActionClass::SearchControl,
        ))
        .expect("revision-advancing request")
        .apply_to_raw_input_once(&mut advance_input)
        .expect("revision-advancing input");
    let (_, current_frame) = render_actual(&context, &mut revision_root, advance_input.events);
    let mut stale_input = egui::RawInput {
        events: vec![egui::Event::Copy],
        ..egui::RawInput::default()
    };
    let before_stale = raw_input_snapshot(&stale_input);
    assert_eq!(
        current_frame
            .interaction_locator()
            .queue_request(stale_request, &mut stale_input),
        Err(KucInteractionRequestError::Stale)
    );
    assert_eq!(raw_input_snapshot(&stale_input), before_stale);
}

#[test]
fn public_locator_debug_does_not_expose_evidence_or_binding_metadata() {
    let output = render(&mut root());
    let locator_debug = format!("{:?}", output.interaction_locator());
    let request = output
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .expect("toolbar request");
    let request_debug = format!("{:?}", request);
    for debug in [locator_debug, request_debug] {
        for forbidden in [
            "egui::Id", "bounds", "label", "Event", "binding", "revision",
        ] {
            assert!(
                !debug.contains(forbidden),
                "public interaction debug leaked `{forbidden}`: {debug}"
            );
        }
    }
}

#[test]
fn root_event_batch_forwards_once_and_returns_a_closed_receipt() -> Result<(), String> {
    let output = render(&mut root());
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport: None,
    };

    let receipt = output
        .events()
        .forward_once(&mut forwarder)
        .map_err(|_| "root event forwarding failed".to_owned())?;

    assert_eq!(forwarder.calls, 1);
    assert!(forwarder.transport.is_some());
    assert_eq!(receipt.root_identity(), "contract.text-command-root");
    assert_eq!(receipt.state_revision(), output.frame().state_revision());
    assert!(!receipt.correlation_fingerprint().is_empty());
    assert!(!receipt.event_batch_fingerprint().is_empty());
    assert!(receipt.consumed_once());
    assert_eq!(receipt.event_cardinality(), 0);

    let second = output.events().forward_once(&mut forwarder);
    assert!(matches!(
        second,
        Err(EguiTextCommandSurfaceRootEventBatchForwardError::AlreadyConsumed)
    ));
    assert_eq!(forwarder.calls, 1);
    Ok(())
}

#[test]
fn external_consumer_can_retain_public_dispatch_receipt() {
    let output = render(&mut root());
    let (receipt, dispatch_calls) = dispatch_receipt_from_public_api(&output);

    assert_eq!(dispatch_calls, 5);
    assert_eq!(receipt.text_count(), 0);
    assert_eq!(receipt.toolbar_count(), 0);
    assert_eq!(receipt.floating_count(), 0);
    assert_eq!(receipt.search_count(), 0);
    assert_eq!(receipt.context_menu_count(), 0);
    assert_eq!(receipt.class_dispatches().len(), 5);
}

#[test]
fn root_frame_public_surface_is_closed_to_child_outputs() {
    let source = include_str!("../src/text_command_surface/root_frame.rs");
    let body = source
        .split_once("pub struct EguiTextCommandSurfaceRootFrame {")
        .and_then(|(_, value)| value.split_once("}\n\nimpl"))
        .map(|(value, _)| value)
        .expect("root frame definition was not found");
    for forbidden in [
        "pub text",
        "pub artifact",
        "pub artifact_order",
        "pub paint_plan",
        "TextureId",
        "egui::Id",
    ] {
        assert!(
            !body.contains(forbidden),
            "closed root frame leaked `{forbidden}`"
        );
    }

    let public_frame_contract = source
        .split_once("pub struct EguiTextCommandSurfaceRootFrame")
        .and_then(|(_, value)| value.split_once("#[derive(Serialize)]"))
        .map(|(value, _)| value)
        .expect("root frame public contract was not found");
    for forbidden in [
        "TextSurfaceEvent",
        "CommandChromeToolbarEvent",
        "FloatingCommandToolbarEvent",
        "CommandChromeSearchEvent",
        "ContextMenuEvent",
        "PaintPlan",
        "TextureId",
        "egui::Id",
        "EguiTextSurfaceOutput",
        "EguiCommandChromeOutput",
    ] {
        assert!(
            !public_frame_contract.contains(forbidden),
            "root frame public contract leaked `{forbidden}`"
        );
    }
}

#[test]
fn public_root_event_forwarding_contract_is_opaque_and_child_free() {
    let transport_source = include_str!("../src/text_command_surface/root_event.rs");
    let public_contract = transport_source
        .split_once("pub struct EguiTextCommandSurfaceRootEventTransport")
        .and_then(|(_, value)| {
            value.split_once("impl std::fmt::Debug for EguiTextCommandSurfaceRootEventTransport")
        })
        .map(|(value, _)| value)
        .expect("public root event forwarding contract was not found");

    for forbidden in [
        "TextSurfaceEvent",
        "CommandChromeToolbarEvent",
        "FloatingCommandToolbarEvent",
        "CommandChromeSearchEvent",
        "ContextMenuEvent",
        "PaintPlan",
        "TextureId",
        "egui::Id",
        "content",
        "query",
        "range",
        "path",
        "Markdown",
    ] {
        assert!(
            !public_contract.contains(forbidden),
            "public root event forwarding contract leaked `{forbidden}`"
        );
    }

    let batch_source = include_str!("../src/text_command_surface/root_event_types.rs");
    let batch_definition = batch_source
        .split_once("pub struct EguiTextCommandSurfaceRootEventBatch")
        .map(|(_, value)| value)
        .expect("root event batch definition was not found");
    assert!(!batch_definition.contains("Clone"));
    assert!(!batch_definition.contains("Serialize"));
    let split_contract = concat!(
        include_str!("../src/text_command_surface/root_event.rs"),
        include_str!("../src/text_command_surface/root_event_contract.rs"),
        include_str!("../src/text_command_surface/root_event_core.rs"),
        include_str!("../src/text_command_surface/root_event_detach.rs"),
        include_str!("../src/text_command_surface/root_event_transport.rs"),
        include_str!("../src/text_command_surface/root_event_types.rs"),
    );
    assert!(!split_contract.contains("pub fn hash"));
    assert!(!split_contract.contains("pub fn len"));
    assert!(!split_contract.contains("pub fn is_empty"));
}

#[test]
fn opaque_host_effect_batch_contract_has_no_semantic_readback_or_value_traits() {
    let source = include_str!("../src/text_command_surface/root_event_contract.rs");
    let batch_contract = source
        .split_once("pub struct KucOpaqueHostEffectBatch")
        .and_then(|(_, value)| value.split_once("/// Generic KUC router"))
        .map(|(value, _)| value);
    let Some(batch_contract) = batch_contract else {
        panic!("opaque host effect batch contract was not found");
    };

    for forbidden in [
        "#[derive(Clone",
        "#[derive(Serialize",
        "impl Clone for KucOpaqueHostEffectBatch",
        "impl Serialize for KucOpaqueHostEffectBatch",
        "impl std::fmt::Display for KucOpaqueHostEffectBatch",
        "pub fn payload",
        "pub fn target",
        "pub fn handler",
        "pub fn readback",
        "pub fn semantic",
    ] {
        assert!(
            !batch_contract.contains(forbidden),
            "opaque host effect batch exposed forbidden public API: {forbidden}"
        );
    }

    assert!(batch_contract.contains("pub fn from_handler"));
    assert!(batch_contract.contains("FnOnce() -> Result<(), KucOpaqueHostEffectError>"));
    assert!(batch_contract.contains("KucOpaqueHostEffectBatch(..)"));
}
