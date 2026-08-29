use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurfaceRootEventTransport, EguiTextCommandSurfaceRootFactory,
    FullTextCommandSurfaceScenarioFactory, FullTextCommandSurfaceScenarioId,
    KucInteractionActionClass, KucInteractionSelector, KucRootEventBatchContext,
    KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
};
use std::cell::RefCell;
use std::rc::Rc;

const IDS: [FullTextCommandSurfaceScenarioId; 6] = [
    FullTextCommandSurfaceScenarioId::Resting,
    FullTextCommandSurfaceScenarioId::Selection,
    FullTextCommandSurfaceScenarioId::Find,
    FullTextCommandSurfaceScenarioId::Context,
    FullTextCommandSurfaceScenarioId::Readonly,
    FullTextCommandSurfaceScenarioId::ResizeScrollIme,
];

const RICH_AUTHORING_ACTION_IDS: [&str; 12] = [
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
];

const FULL_SURFACE_WIDTH: u32 = 1280;
const FULL_SURFACE_HEIGHT: u32 = 720;

fn render(
    context: &egui::Context,
    root: &mut katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostRoot,
    stage: Option<
        &katana_ui_core_egui_adapter::text_command_surface::FullTextCommandSurfaceRawInputStage,
    >,
) -> String {
    let mut input = full_surface_input();
    if let Some(stage) = stage {
        stage.apply_to(&mut input);
    }
    let mut record = None;
    let _ = context.run_ui(input, |ui| {
        record = Some(root.show(ui).expect("scenario root renders"));
    });
    record
        .expect("scenario frame exists")
        .record()
        .record_hash()
        .to_owned()
}

struct EventForwarder {
    transport: Option<EguiTextCommandSurfaceRootEventTransport>,
}

#[derive(Default)]
struct EventDispatcher {
    toolbar: Vec<katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent>,
    search: Vec<katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent>,
    context_menu: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
}

impl KucRootEventBatchDispatcher for EventDispatcher {
    type Error = ();

    fn dispatch_text_events(
        &mut self,
        _events: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_toolbar_events(
        &mut self,
        events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        self.toolbar = events;
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
        events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        self.search.extend(events);
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        self.context_menu.extend(events);
        Ok(())
    }
}

fn render_frame(
    context: &egui::Context,
    root: &mut katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostRoot,
    mut input: egui::RawInput,
) -> katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostRootFrame {
    if input.screen_rect.is_none() {
        input.screen_rect = Some(full_surface_rect());
    }
    let mut frame = None;
    let _ = context.run_ui(input, |ui| {
        frame = Some(root.show(ui));
    });
    frame
        .expect("scenario frame exists")
        .expect("scenario root renders")
}

fn full_surface_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(full_surface_rect()),
        ..egui::RawInput::default()
    }
}

fn full_surface_rect() -> egui::Rect {
    egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(FULL_SURFACE_WIDTH as f32, FULL_SURFACE_HEIGHT as f32),
    )
}

fn key_input(key: egui::Key) -> egui::RawInput {
    let mut input = full_surface_input();
    input.events.push(egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    });
    input
}

fn dispatch_events(
    frame: &katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
) -> EventDispatcher {
    let mut forwarder = EventForwarder { transport: None };
    frame
        .forward_events_once(&mut forwarder)
        .expect("opaque event forwarding succeeds");
    let mut dispatcher = EventDispatcher::default();
    forwarder
        .transport
        .expect("forwarder retains opaque root transport")
        .dispatch_once(&mut dispatcher)
        .expect("opaque root transport dispatches once");
    dispatcher
}

impl KucRootEventBatchForwarder for EventForwarder {
    type Error = ();

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.transport = Some(transport);
        Ok(())
    }
}

fn render_event_count(
    context: &egui::Context,
    root: &mut katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostRoot,
    stage: Option<
        &katana_ui_core_egui_adapter::text_command_surface::FullTextCommandSurfaceRawInputStage,
    >,
) -> usize {
    let mut input = egui::RawInput::default();
    if let Some(stage) = stage {
        stage.apply_to(&mut input);
    }
    let mut frame = None;
    let _ = context.run_ui(input, |ui| {
        frame = Some(root.show(ui).expect("scenario root renders"));
    });
    let mut forwarder = EventForwarder { transport: None };
    frame
        .expect("scenario frame exists")
        .forward_events_once(&mut forwarder)
        .expect("opaque event forwarding succeeds")
        .event_cardinality()
}

#[test]
fn readonly_stage_does_not_change_the_closed_root_record() {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Readonly)
        .expect("readonly scenario issues");
    let stages = scenario.stages().to_vec();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("readonly lease"))
        .expect("readonly lease retains");
    let context = egui::Context::default();
    let before = render_event_count(&context, &mut root, None);
    let after = render_event_count(&context, &mut root, Some(&stages[1]));
    assert_eq!(before, 0);
    assert_eq!(after, 0);
}

#[test]
fn every_public_scenario_issues_opaque_lease_and_deterministic_stages() {
    let factory = FullTextCommandSurfaceScenarioFactory::new();
    for id in IDS {
        let scenario = factory.issue(id).expect("scenario issues");
        assert_eq!(scenario.id(), id);
        assert!(!scenario.stages().is_empty());
        let stage_counts = scenario
            .stages()
            .iter()
            .map(|stage| stage.event_count())
            .collect::<Vec<_>>();
        let second = factory.issue(id).expect("same scenario issues");
        assert_eq!(
            stage_counts,
            second
                .stages()
                .iter()
                .map(|stage| stage.event_count())
                .collect::<Vec<_>>()
        );

        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(scenario.into_lease().expect("opaque lease"))
            .expect("opaque lease retains");
        let context = egui::Context::default();
        let first_hash = render(&context, &mut root, None);
        let repeat_hash = render(&context, &mut root, None);
        assert_eq!(
            first_hash, repeat_hash,
            "stage 0 must be deterministic for {id:?}"
        );
    }
}

#[test]
fn every_public_scenario_exposes_closed_rgba_paint_and_accesskit_records() {
    let factory = FullTextCommandSurfaceScenarioFactory::new();
    for id in IDS {
        let scenario = factory.issue(id).expect("scenario issues");
        let stage = scenario
            .stages()
            .first()
            .expect("scenario has first stage")
            .clone();
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(scenario.into_lease().expect("opaque lease"))
            .expect("opaque root retains");
        let context = egui::Context::default();
        let mut input = egui::RawInput::default();
        stage.apply_to(&mut input);
        let frame = render_frame(&context, &mut root, input);
        let record = frame.record();
        let dimensions = record.dimensions();

        assert_eq!(dimensions.width(), FULL_SURFACE_WIDTH, "{id:?} width");
        assert_eq!(dimensions.height(), FULL_SURFACE_HEIGHT, "{id:?} height");
        for (name, value) in [
            ("RGBA", record.rgba_hash()),
            ("paint plan", record.paint_plan_hash()),
            ("root", record.record_hash()),
            ("AccessKit", record.accessibility_snapshot_hash()),
        ] {
            assert!(!value.is_empty(), "{id:?} {name} record must be non-empty");
        }
    }
}

#[test]
fn every_rich_authoring_action_uses_opaque_hit_testing_and_one_event_dispatch() {
    for action_id in RICH_AUTHORING_ACTION_IDS {
        if action_id == "kuc.rich.block-code" {
            continue;
        }
        let scenario = FullTextCommandSurfaceScenarioFactory::new()
            .issue(FullTextCommandSurfaceScenarioId::Resting)
            .expect("resting scenario issues");
        let stage = scenario.stages()[0].clone();
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(scenario.into_lease().expect("resting lease"))
            .expect("resting root retains");
        let context = egui::Context::default();
        let mut initial_input = egui::RawInput::default();
        stage.apply_to(&mut initial_input);
        let initial = render_frame(&context, &mut root, initial_input);
        let request = initial
            .interaction_locator()
            .request(KucInteractionSelector::new(
                action_id,
                KucInteractionActionClass::Toolbar,
            ))
            .expect("visible action issues an opaque request");
        let mut triggered_input = egui::RawInput::default();
        stage.apply_to(&mut triggered_input);
        initial
            .interaction_locator()
            .queue_request(request, &mut triggered_input)
            .expect("current locator queues its request once");
        let triggered = render_frame(&context, &mut root, triggered_input);
        let mut forwarder = EventForwarder { transport: None };
        let forwarding = triggered
            .forward_events_once(&mut forwarder)
            .expect("triggered root frame forwards once");
        assert!(forwarding.consumed_once());
        assert_eq!(forwarding.event_cardinality(), 1);

        let mut dispatcher = EventDispatcher::default();
        let receipt = forwarder
            .transport
            .expect("forwarder retains the opaque root transport")
            .dispatch_once(&mut dispatcher)
            .expect("opaque transport dispatches once");
        assert_eq!(receipt.toolbar_count(), 1);
        assert_eq!(
            dispatcher.toolbar,
            [katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent::CommandActivated {
                action_id: action_id.into(),
            }],
            "{action_id} must dispatch only its own generic action"
        );
    }
}

#[test]
fn find_query_navigation_close_and_disabled_replace_use_only_kuc_opaque_targets() {
    use katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent;
    use katana_ui_core::molecule::structured::{
        SearchControlStripEvent, SearchNavigationDirection,
    };

    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Find)
        .expect("find scenario issues");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("find lease"))
        .expect("find root retains");
    let context = egui::Context::default();

    let initial = render_frame(&context, &mut root, egui::RawInput::default());
    let query_request = initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc-scenario-search:query",
            KucInteractionActionClass::TextInput,
        ))
        .expect("query input has a generic AccessKit-backed opaque target");
    let mut focus_input = egui::RawInput::default();
    initial
        .interaction_locator()
        .queue_request(query_request, &mut focus_input)
        .expect("query focus request queues once");
    let focused = render_frame(&context, &mut root, focus_input);
    assert_eq!(dispatch_events(&focused).search.len(), 0);

    let committed = render_frame(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![egui::Event::Ime(egui::ImeEvent::Commit(String::from(
                "追加入力 ⭐️",
            )))],
            ..egui::RawInput::default()
        },
    );
    let committed_events = dispatch_events(&committed).search;
    assert!(committed_events.iter().any(|event| matches!(
        event,
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchQueryChanged(value)
        } if value.contains("日本語") && value.contains("追加入力 ⭐️")
    )));
    assert!(matches!(
        committed
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "kuc-scenario-search:replace-one",
                KucInteractionActionClass::SearchControl,
            )),
        Err(
            katana_ui_core_egui_adapter::text_command_surface::KucInteractionLocatorError::Disabled
        )
    ));
    assert!(matches!(
        committed
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "kuc-scenario-search:replace-all",
                KucInteractionActionClass::SearchControl,
            )),
        Err(
            katana_ui_core_egui_adapter::text_command_surface::KucInteractionLocatorError::Disabled
        )
    ));

    let next_request = committed
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc-scenario-search:next",
            KucInteractionActionClass::SearchControl,
        ))
        .expect("next control has an opaque target");
    let mut next_input = egui::RawInput::default();
    committed
        .interaction_locator()
        .queue_request(next_request, &mut next_input)
        .expect("next request queues once");
    let next = render_frame(&context, &mut root, next_input);
    assert!(dispatch_events(&next).search.iter().any(|event| matches!(
        event,
        CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchNavigationRequested {
                direction: SearchNavigationDirection::Next
            }
        }
    )));

    let previous_request = next
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc-scenario-search:previous",
            KucInteractionActionClass::SearchControl,
        ))
        .expect("previous control has an opaque target");
    let mut previous_input = egui::RawInput::default();
    next.interaction_locator()
        .queue_request(previous_request, &mut previous_input)
        .expect("previous request queues once");
    let previous = render_frame(&context, &mut root, previous_input);
    assert!(
        dispatch_events(&previous)
            .search
            .iter()
            .any(|event| matches!(
                event,
                CommandChromeSearchEvent::Strip {
                    event: SearchControlStripEvent::SearchNavigationRequested {
                        direction: SearchNavigationDirection::Previous
                    }
                }
            ))
    );

    let close_request = previous
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc-scenario-search:close",
            KucInteractionActionClass::SearchControl,
        ))
        .expect("close control has an opaque target");
    let mut close_input = egui::RawInput::default();
    previous
        .interaction_locator()
        .queue_request(close_request, &mut close_input)
        .expect("close request queues once");
    let closed = render_frame(&context, &mut root, close_input);
    assert_eq!(
        dispatch_events(&closed).search,
        [CommandChromeSearchEvent::CloseRequested]
    );
    let after_close = render_frame(&context, &mut root, egui::RawInput::default());
    assert!(matches!(
        after_close
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "kuc-scenario-search:close",
                KucInteractionActionClass::SearchControl,
            )),
        Err(katana_ui_core_egui_adapter::text_command_surface::KucInteractionLocatorError::Missing)
    ));
}

#[test]
fn selection_has_a_distinct_actual_root_and_context_stage_is_consumed() {
    let factory = FullTextCommandSurfaceScenarioFactory::new();
    let resting = factory
        .issue(FullTextCommandSurfaceScenarioId::Resting)
        .unwrap();
    let selection = factory
        .issue(FullTextCommandSurfaceScenarioId::Selection)
        .unwrap();
    let context_scenario = factory
        .issue(FullTextCommandSurfaceScenarioId::Context)
        .unwrap();

    let mut resting_root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(resting.into_lease().unwrap())
        .unwrap();
    let mut selection_root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(selection.into_lease().unwrap())
        .unwrap();
    let context_stages = context_scenario.stages().to_vec();
    let mut context_root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(context_scenario.into_lease().unwrap())
        .unwrap();
    let context = egui::Context::default();

    let resting_hash = render(&context, &mut resting_root, None);
    let selection_hash = render(&context, &mut selection_root, None);
    assert_ne!(resting_hash, selection_hash);
    let before_context = render(&context, &mut context_root, None);
    let after_context = render(&context, &mut context_root, Some(&context_stages[1]));
    assert_ne!(before_context, after_context);
}

#[test]
fn context_scenario_forwards_escape_close_only_through_the_opaque_event_transport() {
    use katana_ui_core::molecule::selection::ContextMenuEvent;

    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Context)
        .expect("context scenario issues");
    let stages = scenario.stages().to_vec();
    assert_eq!(stages.len(), 4, "context trace has a complete close cycle");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("context lease"))
        .expect("context root retains");
    let context = egui::Context::default();

    for stage in &stages[..2] {
        let mut input = egui::RawInput::default();
        stage.apply_to(&mut input);
        let _ = render_frame(&context, &mut root, input);
    }
    let mut close_input = egui::RawInput::default();
    stages[2].apply_to(&mut close_input);
    let closed = render_frame(&context, &mut root, close_input);
    let events = dispatch_events(&closed).context_menu;
    assert!(
        events
            .iter()
            .any(|event| matches!(event, ContextMenuEvent::Closed { .. }))
    );

    let mut restored_input = egui::RawInput::default();
    stages[3].apply_to(&mut restored_input);
    let restored = render_frame(&context, &mut root, restored_input);
    assert!(
        !restored.record().rgba_hash().is_empty(),
        "the closed root remains consumable without KLE reading child state"
    );
}

#[test]
fn generic_language_dropdown_opens_all_choices_and_forwards_a_physical_selection() {
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeDropdownCloseReason, CommandChromeToolbarEvent,
    };

    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Resting)
        .expect("resting scenario issues");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("resting lease"))
        .expect("resting root retains");
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = render_frame(&context, &mut root, egui::RawInput::default());

    assert!(matches!(
        initial
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "kuc.generic-language-00",
                KucInteractionActionClass::DropdownItem,
            )),
        Err(katana_ui_core_egui_adapter::text_command_surface::KucInteractionLocatorError::Hidden)
    ));
    let open_request = initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc.rich.block-code",
            KucInteractionActionClass::DropdownTrigger,
        ))
        .expect("closed generic language dropdown has an opaque trigger");
    let mut open_input = egui::RawInput::default();
    initial
        .interaction_locator()
        .queue_request(open_request, &mut open_input)
        .expect("open request queues once");
    let opened = render_frame(&context, &mut root, open_input);

    for index in 1..17 {
        opened
            .interaction_locator()
            .request(KucInteractionSelector::new(
                format!("kuc.generic-language-{index:02}"),
                KucInteractionActionClass::DropdownItem,
            ))
            .expect("all generic language choices are current-frame targets");
    }

    let mut selection = opened
        .interaction_locator()
        .begin_click(KucInteractionSelector::new(
            "kuc.generic-language-00",
            KucInteractionActionClass::DropdownItem,
        ))
        .expect("the visible first generic language choice begins an opaque click trace");
    let mut aim_input = egui::RawInput::default();
    selection
        .apply_to_raw_input_once(&mut aim_input)
        .expect("the opaque selection aim applies exactly once");
    let aimed = render_frame(&context, &mut root, aim_input);
    let mut selection = selection
        .advance(aimed.interaction_locator())
        .expect("the opaque selection aim consumes the immediately following frame")
        .expect("the opaque selection advances to press");

    let mut press_input = egui::RawInput::default();
    selection
        .apply_to_raw_input_once(&mut press_input)
        .expect("the opaque selection press applies exactly once");
    let pressed = render_frame(&context, &mut root, press_input);
    let mut selection = selection
        .advance(pressed.interaction_locator())
        .expect("the opaque selection press consumes the immediately following frame")
        .expect("the opaque selection advances to release");

    let mut release_input = egui::RawInput::default();
    selection
        .apply_to_raw_input_once(&mut release_input)
        .expect("the opaque selection release applies exactly once");
    let selected = render_frame(&context, &mut root, release_input);
    assert!(
        selection
            .advance(selected.interaction_locator())
            .expect("the opaque selection release consumes the immediately following frame")
            .is_none()
    );
    let events = dispatch_events(&selected).toolbar;
    assert!(
        events.contains(&CommandChromeToolbarEvent::DropdownItemActivated {
            action_id: "kuc.rich.block-code".into(),
            item_id: "kuc.generic-language-00".into(),
        }),
        "physical generic-language selection did not emit an item activation: {events:?}"
    );
    assert!(events.contains(&CommandChromeToolbarEvent::DropdownClosed {
        action_id: "kuc.rich.block-code".into(),
        reason: CommandChromeDropdownCloseReason::ItemActivated,
    }));
    let settled = render_frame(&context, &mut root, egui::RawInput::default());
    assert!(matches!(
        settled
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "kuc.generic-language-00",
                KucInteractionActionClass::DropdownItem,
            )),
        Err(katana_ui_core_egui_adapter::text_command_surface::KucInteractionLocatorError::Hidden)
    ));
}

#[test]
fn generic_language_dropdown_reaches_the_final_choice_with_actual_keyboard_input() {
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeDropdownCloseReason, CommandChromeToolbarEvent,
    };

    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Resting)
        .expect("resting scenario issues");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("resting lease"))
        .expect("resting root retains");
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = render_frame(&context, &mut root, full_surface_input());
    let open_request = initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc.rich.block-code",
            KucInteractionActionClass::DropdownTrigger,
        ))
        .expect("closed generic language dropdown has an opaque trigger");
    let mut open_input = full_surface_input();
    initial
        .interaction_locator()
        .queue_request(open_request, &mut open_input)
        .expect("open request queues once");
    let mut opened = render_frame(&context, &mut root, open_input);
    opened
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc.generic-language-00",
            KucInteractionActionClass::DropdownItem,
        ))
        .expect("opened dropdown exposes its initial keyboard focus target");

    for index in 1..17 {
        opened = render_frame(&context, &mut root, key_input(egui::Key::ArrowDown));
        let events = dispatch_events(&opened).toolbar;
        assert!(
            events.contains(&CommandChromeToolbarEvent::DropdownFocusChanged {
                action_id: "kuc.rich.block-code".into(),
                item_id: format!("kuc.generic-language-{index:02}").into(),
            })
        );
    }

    let selected = render_frame(&context, &mut root, key_input(egui::Key::Enter));
    let events = dispatch_events(&selected).toolbar;
    assert!(
        events.contains(&CommandChromeToolbarEvent::DropdownItemActivated {
            action_id: "kuc.rich.block-code".into(),
            item_id: "kuc.generic-language-16".into(),
        }),
        "final generic language keyboard selection did not activate: {events:?}"
    );
    assert!(events.contains(&CommandChromeToolbarEvent::DropdownClosed {
        action_id: "kuc.rich.block-code".into(),
        reason: CommandChromeDropdownCloseReason::ItemActivated,
    }));

    let settled = render_frame(&context, &mut root, full_surface_input());
    assert!(matches!(
        settled
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "kuc.generic-language-16",
                KucInteractionActionClass::DropdownItem,
            )),
        Err(katana_ui_core_egui_adapter::text_command_surface::KucInteractionLocatorError::Hidden)
    ));
}

#[test]
fn generic_language_dropdown_escape_returns_keyboard_focus_to_its_trigger() {
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeDropdownCloseReason, CommandChromeToolbarEvent,
    };

    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Resting)
        .expect("resting scenario issues");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("resting lease"))
        .expect("resting root retains");
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = render_frame(&context, &mut root, full_surface_input());
    let open_request = initial
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc.rich.block-code",
            KucInteractionActionClass::DropdownTrigger,
        ))
        .expect("closed generic language dropdown has an opaque trigger");
    let mut open_input = full_surface_input();
    initial
        .interaction_locator()
        .queue_request(open_request, &mut open_input)
        .expect("open request queues once");
    let opened = render_frame(&context, &mut root, open_input);
    opened
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc.generic-language-00",
            KucInteractionActionClass::DropdownItem,
        ))
        .expect("generic language dropdown opens before Escape");

    let escaped = render_frame(&context, &mut root, key_input(egui::Key::Escape));
    assert!(dispatch_events(&escaped).toolbar.contains(
        &CommandChromeToolbarEvent::DropdownClosed {
            action_id: "kuc.rich.block-code".into(),
            reason: CommandChromeDropdownCloseReason::Escape,
        }
    ));

    let reopened = render_frame(&context, &mut root, key_input(egui::Key::Enter));
    assert!(dispatch_events(&reopened).toolbar.iter().any(|event| {
        matches!(
            event,
            CommandChromeToolbarEvent::DropdownOpened { action_id, .. }
                if action_id.as_str() == "kuc.rich.block-code"
        )
    }));
    reopened
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "kuc.generic-language-00",
            KucInteractionActionClass::DropdownItem,
        ))
        .expect("Escape leaves keyboard focus on the trigger so Enter reopens it");
}

#[test]
fn custom_router_receives_one_actual_scenario_context_without_projection_readback() {
    let contexts = Rc::new(RefCell::new(Vec::<KucRootEventBatchContext>::new()));
    let router_contexts = Rc::clone(&contexts);
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue_with_router(FullTextCommandSurfaceScenarioId::Context, move |context| {
            router_contexts.borrow_mut().push(context);
            Ok(None)
        })
        .expect("custom router scenario issues");
    let stage = scenario.stages()[1].clone();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("custom router lease"))
        .expect("custom router lease retains");
    let context = egui::Context::default();

    let mut input = egui::RawInput::default();
    stage.apply_to(&mut input);
    let _ = context.run_ui(input, |ui| {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            root.show(ui).expect("scenario root renders");
        });
    });

    let contexts = contexts.borrow();
    assert_eq!(contexts.len(), 1);
    assert_eq!(contexts[0].state_revision(), 1);
    let debug = format!("{:?}", contexts[0]);
    assert!(!debug.contains("Generic text surface"));
    assert!(!debug.contains("kuc-scenario"));
    assert!(!debug.contains("payload"));
}

#[test]
fn issue_remains_backward_compatible_and_lease_is_one_shot() {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Resting)
        .expect("legacy scenario issues");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(scenario.into_lease().expect("legacy lease"))
        .expect("legacy lease retains");
    let context = egui::Context::default();
    let _ = context.run_ui(Default::default(), |ui| {
        root.show(ui).expect("legacy scenario root renders");
    });

    let replay = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Resting)
        .expect("replay fixture issues")
        .into_lease()
        .expect("replay lease issues");
    let mut replay_root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(replay)
        .expect("replay root retains independently");
    let duplicate = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::Resting)
        .expect("duplicate fixture issues")
        .into_lease()
        .expect("duplicate lease issues");
    let error = replay_root
        .synchronize_with_lease(duplicate)
        .expect_err("a retained revision must reject lease replay");
    assert!(matches!(
        error,
        katana_ui_core_egui_adapter::text_command_surface::
            EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 1 }
    ));
}
