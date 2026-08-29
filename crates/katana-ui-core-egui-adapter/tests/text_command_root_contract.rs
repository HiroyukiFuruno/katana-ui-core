#[path = "text_command_surface/fixtures.rs"]
mod fixtures;

use katana_ui_core::atom::TextArea;
use katana_ui_core::molecule::command_chrome::FloatingCommandToolbarVisibility;
use katana_ui_core::molecule::selection::ContextMenuItemKind;
use katana_ui_core::molecule::{
    CodeDiff, CodeDiffLine, CodeDiffLineKind, DiagnosticFixPreview, DiagnosticItem,
    DiagnosticLocation, DiagnosticSeverity, DiagnosticsListEvent, StatusBar, StatusBarMode,
    StatusBarSegment,
};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use katana_ui_core_egui_adapter::context_menu::{
    ContextMenuPresentation, ContextMenuPresentationItem,
};
use katana_ui_core_egui_adapter::diagnostics_list::DiagnosticsTargetIdentity;
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceRoot,
    EguiTextCommandSurfaceRootEventBatchForwardError,
    EguiTextCommandSurfaceRootEventDispatchReceipt, EguiTextCommandSurfaceRootEventTransport,
    EguiTextCommandSurfaceRootOutput, KucInteractionActionClass, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucRootEventBatchDispatcher,
    KucRootEventBatchForwarder, StatusDiagnosticsProjectionLease, TextCommandSurfaceStyle,
};

struct DiagnosticsEventCapture {
    events: Vec<DiagnosticsListEvent>,
}

impl KucRootEventBatchDispatcher for DiagnosticsEventCapture {
    type Error = ();

    fn dispatch_text_events(
        &mut self,
        _events: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_search_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_diagnostics_list_events(
        &mut self,
        events: Vec<DiagnosticsListEvent>,
    ) -> Result<(), Self::Error> {
        self.events.extend(events);
        Ok(())
    }
}

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
) -> Result<(EguiTextCommandSurfaceRootEventDispatchReceipt, usize), String> {
    let mut consumer = PublicDispatchConsumer { transport: None };
    output
        .events()
        .forward_once(&mut consumer)
        .map_err(|_| "public root event forwarding failed".to_owned())?;
    let mut dispatcher = PublicDispatcher { calls: 0 };
    let receipt = consumer
        .transport
        .ok_or_else(|| "forwarder retained no opaque transport".to_owned())?
        .dispatch_once(&mut dispatcher)
        .map_err(|_| "public dispatch failed".to_owned())?;
    Ok((receipt, dispatcher.calls))
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

fn style() -> Result<TextCommandSurfaceStyle, String> {
    Ok(TextCommandSurfaceStyle {
        text_raster: fixtures::text_raster(),
        text_paint: fixtures::text_paint(),
        chrome_raster: fixtures::raster_style(),
        chrome_paint: fixtures::paint_style(),
        search: fixtures::search_style(),
    })
}

fn root() -> Result<EguiTextCommandSurfaceRoot, String> {
    let surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
        .with_toolbar(fixtures::toolbar_fixture())
        .with_search_strip(fixtures::search_fixture(false));
    EguiTextCommandSurfaceRoot::with_identity("contract.text-command-root", surface)
        .map_err(|error| format!("root construction failed: {error}"))
}

fn root_with_identity(identity: &str) -> Result<EguiTextCommandSurfaceRoot, String> {
    let surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
        .with_toolbar(fixtures::toolbar_fixture())
        .with_search_strip(fixtures::search_fixture(false));
    EguiTextCommandSurfaceRoot::with_identity(identity, surface)
        .map_err(|error| format!("root construction failed: {error}"))
}

fn use_all_fixture_contracts() {
    let _ = fixtures::floating_toolbar_fixture();
    let _ = fixtures::toolbar_presentation();
    let _ = fixtures::floating_toolbar_presentation();
    let _ = fixtures::search_presentation();
    let _ = fixtures::search_presentation_state_id();
    let _ = fixtures::script_line_height();
}

fn render(
    root: &mut EguiTextCommandSurfaceRoot,
) -> Result<EguiTextCommandSurfaceRootOutput, String> {
    use_all_fixture_contracts();
    let context = egui::Context::default();
    let surface_style = style()?;
    let mut result = None;
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui, &surface_style)),
    );
    result
        .ok_or_else(|| "root did not run".to_owned())?
        .map_err(|error| format!("root frame failed: {error}"))
}

fn render_actual(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, EguiTextCommandSurfaceRootOutput), String> {
    let mut result = None;
    let surface_style = style()?;
    let full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui, &surface_style)),
    );
    Ok((
        full,
        result
            .ok_or_else(|| "root did not run".to_owned())?
            .map_err(|error| format!("root frame failed: {error}"))?,
    ))
}

fn render_actual_in_central_panel(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, EguiTextCommandSurfaceRootOutput), String> {
    let mut result = None;
    let surface_style = style()?;
    let full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show_inside(ctx, |ui| {
                result = Some(root.show(ui, &surface_style));
            });
        },
    );
    Ok((
        full,
        result
            .ok_or_else(|| "root did not run".to_owned())?
            .map_err(|error| format!("root frame failed: {error}"))?,
    ))
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
fn central_panel_pointer_state_opens_context_menu_from_fresh_root_frame() -> Result<(), String> {
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
    .map_err(|error| format!("root construction failed: {error}"))?;
    let (_, initial) = render_actual_in_central_panel(&context, &mut root, Vec::new())?;
    let mut request = initial
        .interaction_locator()
        .request_context_open()
        .map_err(|_| "context opener unavailable".to_owned())?;
    let mut input = egui::RawInput::default();
    request
        .apply_to_raw_input_once(&mut input)
        .map_err(|_| "context opener request failed".to_owned())?;
    let (_, opened) = render_actual_in_central_panel(&context, &mut root, input.events)?;
    assert!(
        opened
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "context-format",
                KucInteractionActionClass::ContextMenuItem,
            ))
            .is_ok()
    );
    Ok(())
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
    let mut first_root = root()?;
    let mut second_root = root()?;
    let first = render(&mut first_root)?;
    let second = render(&mut second_root)?;
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
fn actual_root_locator_uses_current_response_accesskit_and_one_shot_raw_input() -> Result<(), String>
{
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = root()?;
    let (initial_full, initial) = render_actual(&context, &mut root, Vec::new())?;
    assert!(accesskit_has_label(&initial_full, "太字"));
    let missing_input = egui::RawInput {
        events: vec![egui::Event::Copy],
        ..egui::RawInput::default()
    };
    let missing_before = missing_input.clone();
    assert!(matches!(
        initial.interaction_locator().request_context_open(),
        Err(KucInteractionLocatorError::Missing)
    ));
    assert_eq!(missing_input, missing_before);
    let mut search_input = egui::RawInput::default();
    initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "storybook.command-chrome.search:use-regex",
            KucInteractionActionClass::SearchControl,
        ))
        .map_err(|_| "search control unavailable".to_owned())?
        .apply_to_raw_input_once(&mut search_input)
        .map_err(|_| "search request failed".to_owned())?;
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
        .map_err(|_| "toolbar action unavailable".to_owned())?;
    request
        .apply_to_raw_input_once(&mut raw)
        .map_err(|_| "one-shot request failed".to_owned())?;
    assert_eq!(
        request.apply_to_raw_input_once(&mut raw),
        Err(KucInteractionRequestError::AlreadyQueued)
    );
    assert_eq!(raw.events.len(), 3);
    let (activated_full, activated) = render_actual(&context, &mut root, raw.events)?;
    assert!(accesskit_has_label(&activated_full, "太字"));
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport: None,
    };
    let receipt = activated
        .events()
        .forward_once(&mut forwarder)
        .map_err(|_| "activated root event transport failed".to_owned())?;
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
    let before_unmapped = unmapped_input.clone();
    assert!(matches!(
        initial
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "storybook.command-chrome.search:result-summary",
                KucInteractionActionClass::SearchControl,
            )),
        Err(KucInteractionLocatorError::Missing | KucInteractionLocatorError::Hidden)
    ));
    assert_eq!(unmapped_input, before_unmapped);
    Ok(())
}

#[test]
fn actual_root_locator_resolves_closed_dropdown_trigger_and_all_seventeen_items()
-> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = root()?;
    let (_, initial) = render_actual(&context, &mut root, Vec::new())?;
    let hidden_input = egui::RawInput::default();
    let hidden_before = hidden_input.clone();
    let hidden = initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "code-01",
            KucInteractionActionClass::DropdownItem,
        ));
    assert!(matches!(hidden, Err(KucInteractionLocatorError::Hidden)));
    assert_eq!(hidden_input, hidden_before);
    let mut trigger_input = egui::RawInput::default();
    initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "code-block",
            KucInteractionActionClass::DropdownTrigger,
        ))
        .map_err(|_| "closed split trigger unavailable".to_owned())?
        .apply_to_raw_input_once(&mut trigger_input)
        .map_err(|_| "trigger request failed".to_owned())?;
    let (opened_full, opened) = render_actual(&context, &mut root, trigger_input.events)?;
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
    Ok(())
}

#[test]
fn actual_root_locator_resolves_floating_and_context_targets_with_accesskit() -> Result<(), String>
{
    let context = egui::Context::default();
    context.enable_accesskit();
    let floating_surface = EguiTextCommandSurface::new(selected_text_surface())
        .with_floating_toolbar(
            fixtures::floating_toolbar_fixture(),
            FloatingCommandToolbarVisibility::Visible,
        );
    let mut floating_root =
        EguiTextCommandSurfaceRoot::with_identity("contract.floating-root", floating_surface)
            .map_err(|error| format!("floating root construction failed: {error}"))?;
    let (floating_full, floating) = render_actual(&context, &mut floating_root, Vec::new())?;
    assert!(accesskit_has_label(&floating_full, "選択ツール ⭐️"));
    let mut floating_input = egui::RawInput::default();
    floating
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "floating-bold",
            KucInteractionActionClass::FloatingToolbar,
        ))
        .map_err(|_| "floating action unavailable".to_owned())?
        .apply_to_raw_input_once(&mut floating_input)
        .map_err(|_| "floating request failed".to_owned())?;
    let (floating_activated_full, floating_activated) =
        render_actual(&context, &mut floating_root, floating_input.events)?;
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
            .map_err(|error| format!("context root construction failed: {error}"))?;
    let (context_initial_full, context_initial) =
        render_actual(&context, &mut context_root, Vec::new())?;
    let mut context_open_input = egui::RawInput::default();
    let mut context_open_request = context_initial
        .interaction_locator()
        .request_context_open()
        .map_err(|_| "context opener unavailable".to_owned())?;
    assert_eq!(
        format!("{context_open_request:?}"),
        "KucOpaqueInteractionRequest(..)"
    );
    context_open_request
        .apply_to_raw_input_once(&mut context_open_input)
        .map_err(|_| "context opener request failed".to_owned())?;
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
    let before_replay = context_open_input.clone();
    assert_eq!(
        context_open_request.apply_to_raw_input_once(&mut context_open_input),
        Err(KucInteractionRequestError::AlreadyQueued)
    );
    assert_eq!(context_open_input, before_replay);
    let (context_open_full, context_open) =
        render_actual(&context, &mut context_root, context_open_input.events)?;
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
        .map_err(|_| "context action unavailable".to_owned())?
        .apply_to_raw_input_once(&mut context_input)
        .map_err(|_| "context request failed".to_owned())?;
    let (_, context_selected) = render_actual(&context, &mut context_root, context_input.events)?;
    assert!(
        context_selected
            .events()
            .forward_once(&mut RecordingForwarder {
                calls: 0,
                transport: None,
            })
            .is_ok()
    );
    Ok(())
}

#[test]
fn current_locator_rejects_cross_root_and_prior_revision_requests_without_mutation()
-> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut first_root = root()?;
    let mut second_root = root_with_identity("contract.other-root")?;
    let (_, first_frame) = render_actual(&context, &mut first_root, Vec::new())?;
    let (_, second_frame) = render_actual(&context, &mut second_root, Vec::new())?;
    let cross_root_request = first_frame
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .map_err(|_| "first root request unavailable".to_owned())?;
    let mut cross_root_input = egui::RawInput {
        events: vec![egui::Event::Copy],
        ..egui::RawInput::default()
    };
    let before_cross_root = cross_root_input.clone();
    assert_eq!(
        second_frame
            .interaction_locator()
            .queue_request(cross_root_request, &mut cross_root_input),
        Err(KucInteractionRequestError::RootMismatch)
    );
    assert_eq!(cross_root_input, before_cross_root);

    let mut revision_root = root()?;
    let (_, initial_frame) = render_actual(&context, &mut revision_root, Vec::new())?;
    let stale_request = initial_frame
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .map_err(|_| "initial revision request unavailable".to_owned())?;
    let mut advance_input = egui::RawInput::default();
    initial_frame
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "storybook.command-chrome.search:use-regex",
            KucInteractionActionClass::SearchControl,
        ))
        .map_err(|_| "revision-advancing request unavailable".to_owned())?
        .apply_to_raw_input_once(&mut advance_input)
        .map_err(|_| "revision-advancing input failed".to_owned())?;
    let (_, current_frame) = render_actual(&context, &mut revision_root, advance_input.events)?;
    let mut stale_input = egui::RawInput {
        events: vec![egui::Event::Copy],
        ..egui::RawInput::default()
    };
    let before_stale = stale_input.clone();
    assert_eq!(
        current_frame
            .interaction_locator()
            .queue_request(stale_request, &mut stale_input),
        Err(KucInteractionRequestError::Stale)
    );
    assert_eq!(stale_input, before_stale);
    Ok(())
}

#[test]
fn public_locator_debug_does_not_expose_evidence_or_binding_metadata() -> Result<(), String> {
    let mut root = root()?;
    let output = render(&mut root)?;
    let locator_debug = format!("{:?}", output.interaction_locator());
    let request = output
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "inline-bold",
            KucInteractionActionClass::Toolbar,
        ))
        .map_err(|_| "toolbar request unavailable".to_owned())?;
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
    Ok(())
}

#[test]
fn root_event_batch_forwards_once_and_returns_a_closed_receipt() -> Result<(), String> {
    let mut root = root()?;
    let output = render(&mut root)?;
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
fn external_consumer_can_retain_public_dispatch_receipt() -> Result<(), String> {
    let mut root = root()?;
    let output = render(&mut root)?;
    let (receipt, dispatch_calls) = dispatch_receipt_from_public_api(&output)?;

    assert_eq!(dispatch_calls, 5);
    assert_eq!(receipt.text_count(), 0);
    assert_eq!(receipt.toolbar_count(), 0);
    assert_eq!(receipt.floating_count(), 0);
    assert_eq!(receipt.search_count(), 0);
    assert_eq!(receipt.context_menu_count(), 0);
    assert_eq!(receipt.class_dispatches().len(), 7);
    Ok(())
}

#[test]
fn root_frame_public_surface_is_closed_to_child_outputs() -> Result<(), String> {
    let source = include_str!("../src/text_command_surface/root_frame.rs");
    let body = source
        .split_once("pub struct EguiTextCommandSurfaceRootFrame {")
        .and_then(|(_, value)| value.split_once("}\n\nimpl"))
        .map(|(value, _)| value)
        .ok_or_else(|| "root frame definition was not found".to_owned())?;
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
        .ok_or_else(|| "root frame public contract was not found".to_owned())?;
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
    Ok(())
}

#[test]
fn public_root_event_forwarding_contract_is_opaque_and_child_free() -> Result<(), String> {
    let source = include_str!("../src/text_command_surface/root_event.rs");
    let public_contract = source
        .split_once("pub struct EguiTextCommandSurfaceRootEventTransport")
        .and_then(|(_, value)| value.split_once("/// Deterministic receipt"))
        .map(|(value, _)| value)
        .ok_or_else(|| "public root event forwarding contract was not found".to_owned())?;

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

    let batch_definition = source
        .split_once("pub struct EguiTextCommandSurfaceRootEventBatch")
        .and_then(|(_, value)| value.split_once("/// Generic callback used to dispatch"))
        .map(|(value, _)| value)
        .ok_or_else(|| "root event batch definition was not found".to_owned())?;
    assert!(!batch_definition.contains("Clone"));
    assert!(!batch_definition.contains("Serialize"));
    assert!(!source.contains("pub fn hash"));
    assert!(!source.contains("pub fn len"));
    assert!(!source.contains("pub fn is_empty"));
    Ok(())
}

#[test]
fn opaque_host_effect_batch_contract_has_no_semantic_readback_or_value_traits() -> Result<(), String>
{
    let source = include_str!("../src/text_command_surface/root_event/api.rs");
    let batch_contract = source
        .split_once("impl KucOpaqueHostEffectBatch")
        .and_then(|(_, value)| value.split_once("impl std::fmt::Debug"))
        .map(|(value, _)| value);
    let Some(batch_contract) = batch_contract else {
        return Err("opaque host effect batch contract was not found".to_owned());
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
    assert!(source.contains("KucOpaqueHostEffectBatch(..)"));
    Ok(())
}

#[test]
fn root_composes_status_and_diagnostics_with_input_accesskit_and_opaque_transport()
-> Result<(), String> {
    let mut root = root()?;
    root.attach_status_diagnostics(
        StatusDiagnosticsProjectionLease::new()
            .with_status_bar(
                StatusBar::new("status-root")
                    .mode(StatusBarMode::MultiSegment)
                    .segment(StatusBarSegment::new("status-click", "Status ⭐️").interactive(true)),
            )
            .with_diagnostics_list(
                katana_ui_core::molecule::DiagnosticsList::new("Diagnostics 日本語").item(
                    DiagnosticItem::new(
                        "diagnostic-1",
                        DiagnosticSeverity::Error,
                        "構文エラー",
                        DiagnosticLocation::new("src/lib.rs", 3, 4),
                    ),
                ),
            ),
    );
    let context = egui::Context::default();
    let (_first_full, first) = render_actual(&context, &mut root, Vec::new())?;
    assert!(first.artifact_order().contains(
        &katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceChild::StatusBar
    ));
    assert!(first.artifact_order().contains(&katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceChild::DiagnosticsList));
    assert!(!first.rgba_pixels().is_empty());
    let (published_full, published) = render_actual(&context, &mut root, Vec::new())?;
    assert!(published.frame().state_revision() >= first.frame().state_revision());
    assert!(accesskit_has_label(&published_full, "Status ⭐️"));
    assert!(accesskit_has_label(&published_full, "構文エラー"));

    let pointer = egui::Event::PointerButton {
        pos: egui::pos2(24.0, fixtures::FRAME_HEIGHT - 14.0),
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    };
    let (_, _) = render_actual(&context, &mut root, vec![pointer])?;
    let (_, pointer_frame) = render_actual(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(24.0, fixtures::FRAME_HEIGHT - 14.0),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(pointer_frame.frame().state_revision() > first.frame().state_revision());

    let (_, keyboard_frame) = render_actual(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::ArrowDown,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert_eq!(
        keyboard_frame.frame().dimensions(),
        pointer_frame.frame().dimensions()
    );

    let target = published_full
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(id, node)| {
                node.label()
                    .is_some_and(|label| label == "Status ⭐️")
                    .then_some(*id)
            })
        })
        .ok_or_else(|| "status AccessKit target not found".to_owned())?;
    let (_, accesskit_frame) = render_actual(
        &context,
        &mut root,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: target,
                data: None,
            },
        )],
    )?;
    assert_ne!(
        accesskit_frame.frame().record_hash(),
        keyboard_frame.frame().record_hash()
    );

    let mut consumer = PublicDispatchConsumer { transport: None };
    accesskit_frame
        .events()
        .forward_once(&mut consumer)
        .map_err(|_| "root event transport forwarding failed".to_owned())?;
    let mut dispatcher = PublicDispatcher { calls: 0 };
    let receipt = consumer
        .transport
        .take()
        .ok_or_else(|| "opaque transport was not retained".to_owned())?
        .dispatch_once(&mut dispatcher)
        .map_err(|_| "opaque transport dispatch failed".to_owned())?;
    assert_eq!(receipt.status_bar_count(), 1);
    assert_eq!(receipt.class_dispatches().len(), 7);
    assert_eq!(
        accesskit_frame.interaction_locator().state_revision(),
        accesskit_frame.frame().state_revision()
    );
    Ok(())
}

#[test]
fn diagnostics_targets_are_resolved_opaquely_and_dispatch_once() -> Result<(), String> {
    fn diagnostics_root() -> Result<EguiTextCommandSurfaceRoot, String> {
        let mut root = root()?;
        root.attach_status_diagnostics(
            StatusDiagnosticsProjectionLease::new().with_diagnostics_list(
                katana_ui_core::molecule::DiagnosticsList::new("診断 ⭐️")
                    .scope("scope-opaque-key", "全件 ⭐️", "全件の診断 ⭐️")
                    .scope("scope-secondary-key", "別の範囲", "別の診断範囲")
                    .item(
                        DiagnosticItem::new(
                            "diagnostic-stable-id",
                            DiagnosticSeverity::Error,
                            "表示ラベルは変更され得る ⭐️",
                            DiagnosticLocation::new("opaque-location", 3, 4),
                        )
                        .quickfix(katana_ui_core::molecule::DiagnosticAction::new(
                            "fix-stable-id",
                            "修正を適用",
                        ))
                        .scopes(["scope-opaque-key", "scope-secondary-key"]),
                    ),
            ),
        );
        Ok(root)
    }

    fn dispatch_diagnostics(
        output: &EguiTextCommandSurfaceRootOutput,
    ) -> Result<Vec<DiagnosticsListEvent>, String> {
        let mut consumer = PublicDispatchConsumer { transport: None };
        output
            .events()
            .forward_once(&mut consumer)
            .map_err(|_| "diagnostics event transport forwarding failed".to_owned())?;
        let mut dispatcher = DiagnosticsEventCapture { events: Vec::new() };
        consumer
            .transport
            .ok_or_else(|| "opaque diagnostics transport was not retained".to_owned())?
            .dispatch_once(&mut dispatcher)
            .map_err(|_| "diagnostics transport dispatch failed".to_owned())?;
        Ok(dispatcher.events)
    }

    for (identity, action_class, expected) in [
        (
            DiagnosticsTargetIdentity::scope("scope-secondary-key"),
            KucInteractionActionClass::DiagnosticsScope,
            "scope",
        ),
        (
            DiagnosticsTargetIdentity::severity_filter(DiagnosticSeverity::Error),
            KucInteractionActionClass::DiagnosticsSeverityFilter,
            "filter",
        ),
        (
            DiagnosticsTargetIdentity::item("diagnostic-stable-id"),
            KucInteractionActionClass::DiagnosticsItem,
            "item",
        ),
        (
            DiagnosticsTargetIdentity::fix("diagnostic-stable-id"),
            KucInteractionActionClass::DiagnosticsFix,
            "fix",
        ),
    ] {
        let mut root = diagnostics_root()?;
        let context = egui::Context::default();
        let (_, initial) = render_actual(&context, &mut root, Vec::new())?;
        let mut raw = egui::RawInput::default();
        initial
            .interaction_locator()
            .request(KucInteractionSelector::new(identity, action_class))
            .map_err(|_| "opaque diagnostics target did not resolve".to_owned())?
            .apply_to_raw_input_once(&mut raw)
            .map_err(|_| "opaque diagnostics request was not queued".to_owned())?;
        let (_, activated) = render_actual(&context, &mut root, raw.events)?;
        let events = dispatch_diagnostics(&activated)?;
        assert_eq!(events.len(), 1, "{expected} target must dispatch once");
        match expected {
            "scope" => assert!(matches!(
                events[0],
                DiagnosticsListEvent::ScopeSelected { .. }
            )),
            "filter" => assert!(matches!(events[0], DiagnosticsListEvent::FilterChanged)),
            "item" => assert!(matches!(
                events[0],
                DiagnosticsListEvent::DiagnosticSelected { .. }
            )),
            "fix" => assert!(matches!(
                events[0],
                DiagnosticsListEvent::DiagnosticFixApplied { .. }
            )),
            _ => return Err(format!("unexpected diagnostics target kind: {expected}")),
        }
    }

    for (role, label, expected) in [
        (egui::accesskit::Role::RadioButton, "別の診断範囲", "scope"),
        (egui::accesskit::Role::CheckBox, "Error", "filter"),
        (egui::accesskit::Role::ListItem, "表示ラベル", "item"),
        (egui::accesskit::Role::Button, "修正を適用", "fix"),
    ] {
        let mut root = diagnostics_root()?;
        let context = egui::Context::default();
        context.enable_accesskit();
        let _ = render_actual(&context, &mut root, Vec::new())?;
        let (full, _) = render_actual(&context, &mut root, Vec::new())?;
        let target = full
            .platform_output
            .accesskit_update
            .as_ref()
            .and_then(|update| {
                update.nodes.iter().find_map(|(id, node)| {
                    (node.role() == role && node.label().is_some_and(|value| value.contains(label)))
                        .then_some(*id)
                })
            })
            .ok_or_else(|| "generic diagnostics AccessKit target not found".to_owned())?;
        let (_, activated) = render_actual(
            &context,
            &mut root,
            vec![egui::Event::AccessKitActionRequest(
                egui::accesskit::ActionRequest {
                    action: egui::accesskit::Action::Click,
                    target_tree: egui::accesskit::TreeId::ROOT,
                    target_node: target,
                    data: None,
                },
            )],
        )?;
        let events = dispatch_diagnostics(&activated)?;
        assert_eq!(
            events.len(),
            1,
            "AccessKit {expected} target must dispatch once"
        );
    }
    Ok(())
}

#[test]
fn root_transports_diagnostic_fix_preview_opened_by_accesskit() -> Result<(), String> {
    let mut root = root()?;
    root.attach_status_diagnostics(
        StatusDiagnosticsProjectionLease::new().with_diagnostics_list(
            katana_ui_core::molecule::DiagnosticsList::new("診断 ⭐️").item(
                DiagnosticItem::new(
                    "preview-diagnostic",
                    DiagnosticSeverity::Error,
                    "修正対象",
                    DiagnosticLocation::new("src/lib.rs", 3, 4),
                )
                .fix_preview(DiagnosticFixPreview::new(
                    CodeDiff::new("差分")
                        .line(CodeDiffLine {
                            old_number: Some(3),
                            new_number: Some(3),
                            kind: CodeDiffLineKind::Removed,
                            text: "古い ⭐️".to_string(),
                        })
                        .line(CodeDiffLine {
                            old_number: None,
                            new_number: Some(3),
                            kind: CodeDiffLineKind::Added,
                            text: "新しい ⭐️".to_string(),
                        }),
                )),
            ),
        ),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let (initial_full, _initial) = render_actual(&context, &mut root, Vec::new())?;
    let disclosure = initial_full
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(id, node)| {
                (node.role() == egui::accesskit::Role::Button && node.label() == Some("展開"))
                    .then_some(*id)
            })
        })
        .ok_or_else(|| "root did not expose opaque preview disclosure".to_owned())?;
    let (_, opened) = render_actual(
        &context,
        &mut root,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: disclosure,
                data: None,
            },
        )],
    )?;
    let before = opened.frame().record_hash();
    let (_, rendered) = render_actual(&context, &mut root, Vec::new())?;
    assert_ne!(rendered.frame().record_hash(), before);

    let mut consumer = PublicDispatchConsumer { transport: None };
    opened
        .events()
        .forward_once(&mut consumer)
        .map_err(|_| "preview event transport forwarding failed".to_owned())?;
    let mut dispatcher = DiagnosticsEventCapture { events: Vec::new() };
    consumer
        .transport
        .ok_or_else(|| "preview opaque transport was not retained".to_owned())?
        .dispatch_once(&mut dispatcher)
        .map_err(|_| "preview transport dispatch failed".to_owned())?;
    assert!(dispatcher.events.iter().any(|event| {
        matches!(
            event,
            DiagnosticsListEvent::DiagnosticFixPreviewToggled { id, expanded }
                if id.as_str() == "preview-diagnostic" && *expanded
        )
    }));

    let (_, closed) = render_actual(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    let mut close_consumer = PublicDispatchConsumer { transport: None };
    closed
        .events()
        .forward_once(&mut close_consumer)
        .map_err(|_| "preview close transport forwarding failed".to_owned())?;
    let mut close_dispatcher = DiagnosticsEventCapture { events: Vec::new() };
    close_consumer
        .transport
        .ok_or_else(|| "preview close opaque transport was not retained".to_owned())?
        .dispatch_once(&mut close_dispatcher)
        .map_err(|_| "preview close transport dispatch failed".to_owned())?;
    assert!(close_dispatcher.events.iter().any(|event| {
        matches!(
            event,
            DiagnosticsListEvent::DiagnosticFixPreviewToggled { id, expanded }
                if id.as_str() == "preview-diagnostic" && !expanded
        )
    }));
    Ok(())
}
