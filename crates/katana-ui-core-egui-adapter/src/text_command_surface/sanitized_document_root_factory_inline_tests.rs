use super::super::sanitized_command_projection::SanitizedCommandCapabilityRejection;
use super::super::sanitized_context_projection::SanitizedContextMenuCapabilityRejection;
use super::super::sanitized_document_root_transport::{
    SanitizedDocumentRootEventDispatchError, SanitizedDocumentRootEventForwardError,
};
use super::super::sanitized_search_projection::{
    SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
    SanitizedSearchOperationPresentation, SanitizedSearchProjectionBuilder,
    SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget, SanitizedSearchTextOperation,
    SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
    SanitizedSearchUnitOperation,
};
use super::super::sanitized_tab_projection::SanitizedTabGroupTarget;
use super::{
    SanitizedDocumentRootFactory, SanitizedDocumentRootFactoryError, SanitizedDocumentRootFrame,
};
use crate::text_command_surface::KucRootEventBatchDispatcher;
use crate::text_command_surface::{
    SanitizedContextMenuItem, SanitizedContextMenuProjection,
    SanitizedContextMenuProjectionBuilder, SanitizedContextMenuTarget,
    SanitizedDocumentRootEventForwarder, SanitizedDocumentRootEventTransport,
    SanitizedDocumentRootIdentity, SanitizedDocumentRootInput, SanitizedDocumentRootStyleKey,
    SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation, SanitizedTabGroup,
    SanitizedTabProjection, SanitizedTabTarget,
};
use std::cell::RefCell;
use std::rc::Rc;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

struct RecordingForwarder {
    calls: usize,
    transport_debug: Option<String>,
    reject_forwarding: bool,
}

struct RetainingForwarder {
    calls: usize,
    transport_debug: Option<String>,
    transport: Option<SanitizedDocumentRootEventTransport>,
}

#[derive(Default)]
struct TestRootDispatcher {
    reject_text: bool,
}

impl KucRootEventBatchDispatcher for TestRootDispatcher {
    type Error = ();

    fn dispatch_text_events(
        &mut self,
        _events: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        if self.reject_text { Err(()) } else { Ok(()) }
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
}

impl SanitizedDocumentRootEventForwarder for RecordingForwarder {
    type Error = ();

    fn forward_sanitized_document_root_event(
        &mut self,
        mut transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        self.transport_debug = Some(format!("{transport:?}"));
        if self.reject_forwarding {
            return Err(());
        }
        transport
            .dispatch_root_once(&mut TestRootDispatcher::default())
            .map_err(|_| ())?;
        Ok(())
    }
}

impl RetainingForwarder {
    fn dispatch_root_once(
        &mut self,
    ) -> Result<
        crate::text_command_surface::EguiTextCommandSurfaceRootEventDispatchReceipt,
        SanitizedDocumentRootEventDispatchError<()>,
    > {
        if let Some(transport) = self.transport.as_mut() {
            transport.dispatch_root_once(&mut TestRootDispatcher::default())
        } else {
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
        }
    }

    fn dispatch_root_once_rejecting_text(
        &mut self,
    ) -> Result<
        crate::text_command_surface::EguiTextCommandSurfaceRootEventDispatchReceipt,
        SanitizedDocumentRootEventDispatchError<()>,
    > {
        if let Some(transport) = self.transport.as_mut() {
            transport.dispatch_root_once(&mut TestRootDispatcher { reject_text: true })
        } else {
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
        }
    }
}

impl SanitizedDocumentRootEventForwarder for RetainingForwarder {
    type Error = ();

    fn forward_sanitized_document_root_event(
        &mut self,
        transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        self.transport_debug = Some(format!("{transport:?}"));
        self.transport = Some(transport);
        Ok(())
    }
}

#[test]
fn retained_transport_propagates_a_real_child_dispatch_rejection() {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input(1, b"child-rejection", "本文"))
        .expect("retain succeeds");
    let context = egui::Context::default();
    let frame = run_root_frame_events(&context, &mut root, Vec::new()).1;
    let mut forwarder = RetainingForwarder {
        calls: 0,
        transport_debug: None,
        transport: None,
    };

    frame
        .forward_events_once(&mut forwarder)
        .expect("root forwarding succeeds");
    assert_eq!(
        forwarder.dispatch_root_once_rejecting_text(),
        Err(SanitizedDocumentRootEventDispatchError::Child(()))
    );
    assert_eq!(
        forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
    );
}

fn input(revision: u64, identity: &[u8], snapshot: &str) -> SanitizedDocumentRootInput {
    SanitizedDocumentRootInput::new(
        revision,
        SanitizedDocumentRootIdentity::from_opaque_bytes(identity.to_vec()),
        snapshot,
        SanitizedDocumentRootStyleKey::Default,
    )
}

fn input_with_tabs(revision: u64) -> SanitizedDocumentRootInput {
    input(revision, b"document", "本文 ⭐️").with_tab_projection(SanitizedTabProjection::new([
        SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "ドキュメント",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "最初")
                .with_capabilities(
                    SanitizedTabCapabilities::new()
                        .active_state(true)
                        .close_state(true),
                ),
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([2]), 1, "次の文書")
                .with_capabilities(SanitizedTabCapabilities::new().close_state(true))
                .with_close_presentation(SanitizedTabClosePresentation::new(
                    "×",
                    "閉じる",
                    "次の文書を閉じる",
                )),
        ),
    ]))
}

fn input_with_search(revision: u64) -> Result<SanitizedDocumentRootInput, String> {
    Ok(
        input(revision, b"document", "本文 ⭐️").with_search_projection(
            super::super::sanitized_document_root_process::search_projection_for_ime(
                [9, 1],
                [9, 2],
            )?,
        ),
    )
}

fn search_text(value: &str) -> SanitizedSearchTextPresentation {
    SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
}

fn search_localized() -> SanitizedSearchLocalizedPresentation {
    SanitizedSearchLocalizedPresentation::new(
        SanitizedSearchControlPresentation::new(
            search_text("検索"),
            search_text("検索語"),
            search_text("置換"),
            search_text("大文字小文字"),
            search_text("単語"),
            search_text("正規表現"),
        ),
        SanitizedSearchOperationPresentation::new(
            search_text("前へ"),
            search_text("次へ"),
            search_text("置換"),
            search_text("すべて置換"),
            search_text("閉じる"),
        ),
        SanitizedSearchResultSummaryPresentation::new(
            "検索待機 ⭐️",
            "一致なし",
            "1件",
            "{active} / {count}",
            "{count}件",
        ),
        SanitizedSearchUnavailablePresentation::new(
            "正規表現は利用不可",
            "置換は利用不可",
            "移動は利用不可",
            "閉じる操作は利用不可",
        ),
    )
}

fn input_with_recorders(
    revision: u64,
    text_events: Rc<RefCell<Vec<(SanitizedSearchTextOperation, String)>>>,
    unit_events: Rc<RefCell<Vec<SanitizedSearchUnitOperation>>>,
) -> SanitizedDocumentRootInput {
    let text_target = |operation| {
        let events = text_events.clone();
        let _ = operation;
        SanitizedSearchTarget::from_opaque_bytes([0]).with_text_capability(move |actual, value| {
            events.borrow_mut().push((actual, value));
            Ok::<(), ()>(())
        })
    };
    let unit_target = |operation| {
        let events = unit_events.clone();
        let _ = operation;
        SanitizedSearchTarget::from_opaque_bytes([0]).with_unit_capability(move |actual| {
            events.borrow_mut().push(actual);
            Ok::<(), ()>(())
        })
    };
    let projection = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(search_localized())
        .query_target(text_target(SanitizedSearchTextOperation::Query))
        .replacement_target(text_target(SanitizedSearchTextOperation::Replacement))
        .match_case_target(unit_target(SanitizedSearchUnitOperation::MatchCase(false)))
        .whole_word_target(unit_target(SanitizedSearchUnitOperation::WholeWord(false)))
        .regex_target(unit_target(SanitizedSearchUnitOperation::Regex(false)))
        .close_enabled(true)
        .close_target(unit_target(SanitizedSearchUnitOperation::Close))
        .next_enabled(true)
        .next_target(unit_target(SanitizedSearchUnitOperation::Next))
        .previous_enabled(true)
        .previous_target(unit_target(SanitizedSearchUnitOperation::Previous))
        .replace_enabled(true)
        .replace_target(text_target(SanitizedSearchTextOperation::Replace))
        .replace_all_enabled(true)
        .replace_all_target(text_target(SanitizedSearchTextOperation::ReplaceAll))
        .build()
        .expect("complete search projection is valid");
    input(revision, b"document", "本文 ⭐️").with_search_projection(projection)
}

fn input_with_rejecting_recorders(
    revision: u64,
    text_calls: Rc<RefCell<usize>>,
    unit_calls: Rc<RefCell<usize>>,
) -> SanitizedDocumentRootInput {
    let text_target = || {
        let calls = text_calls.clone();
        SanitizedSearchTarget::from_opaque_bytes([0]).with_text_capability(
            move |_operation, _value| {
                *calls.borrow_mut() += 1;
                Err::<(), ()>(())
            },
        )
    };
    let unit_target = || {
        let calls = unit_calls.clone();
        SanitizedSearchTarget::from_opaque_bytes([0]).with_unit_capability(move |_operation| {
            *calls.borrow_mut() += 1;
            Err::<(), ()>(())
        })
    };
    let projection = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(search_localized())
        .query_target(text_target())
        .replacement_target(text_target())
        .match_case_target(unit_target())
        .whole_word_target(unit_target())
        .regex_target(unit_target())
        .close_enabled(true)
        .close_target(unit_target())
        .next_enabled(true)
        .next_target(unit_target())
        .previous_enabled(true)
        .previous_target(unit_target())
        .replace_enabled(true)
        .replace_target(text_target())
        .replace_all_enabled(true)
        .replace_all_target(text_target())
        .build()
        .expect("complete rejecting search projection is valid");
    input(revision, b"document", "本文 ⭐️").with_search_projection(projection)
}

fn run_search_root_frame(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, SanitizedDocumentRootFrame) {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    output.textures_delta.clear();
    (output, frame.expect("frame exists"))
}

fn accesskit_bounds(
    output: &egui::FullOutput,
    role: egui::accesskit::Role,
    label: &str,
) -> egui::Rect {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.role() == role && node.label() == Some(label)).then(|| node.bounds())
            })
        })
        .flatten()
        .map(|bounds| {
            egui::Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            )
        })
        .expect("current output contains the requested control bounds")
}

fn accesskit_button(
    output: &egui::FullOutput,
    label: &str,
) -> (egui::accesskit::NodeId, egui::Rect) {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(node_id, node)| {
                (node.role() == egui::accesskit::Role::Button && node.label() == Some(label))
                    .then(|| {
                        node.bounds().map(|bounds| {
                            (
                                *node_id,
                                egui::Rect::from_min_max(
                                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                                ),
                            )
                        })
                    })
                    .flatten()
            })
        })
        .expect("current output contains the requested button node")
}

fn input_with_recorders_and_unit_targets(
    revision: u64,
    text_events: Rc<RefCell<Vec<(SanitizedSearchTextOperation, String)>>>,
    unit_events: Rc<RefCell<Vec<SanitizedSearchUnitOperation>>>,
    unit_targets_enabled: bool,
) -> SanitizedDocumentRootInput {
    input_with_recorders_and_unit_targets_and_state(
        revision,
        text_events,
        unit_events,
        unit_targets_enabled,
        false,
        false,
        false,
    )
}

fn input_with_recorders_and_unit_targets_and_state(
    revision: u64,
    text_events: Rc<RefCell<Vec<(SanitizedSearchTextOperation, String)>>>,
    unit_events: Rc<RefCell<Vec<SanitizedSearchUnitOperation>>>,
    unit_targets_enabled: bool,
    match_case_state: bool,
    whole_word_state: bool,
    regex_state: bool,
) -> SanitizedDocumentRootInput {
    let text_target = |operation| {
        let events = text_events.clone();
        let _ = operation;
        SanitizedSearchTarget::from_opaque_bytes([0]).with_text_capability(move |actual, value| {
            events.borrow_mut().push((actual, value));
            Ok::<(), ()>(())
        })
    };
    let unit_target = |operation| {
        let events = unit_events.clone();
        let _ = operation;
        SanitizedSearchTarget::from_opaque_bytes([0]).with_unit_capability(move |actual| {
            events.borrow_mut().push(actual);
            Ok::<(), ()>(())
        })
    };
    let mut builder = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(search_localized())
        .match_case_state(match_case_state)
        .whole_word_state(whole_word_state)
        .regex_state(regex_state)
        .query_target(text_target(SanitizedSearchTextOperation::Query))
        .replacement_target(text_target(SanitizedSearchTextOperation::Replacement))
        .close_enabled(unit_targets_enabled)
        .close_target(unit_target(SanitizedSearchUnitOperation::Close))
        .next_enabled(unit_targets_enabled)
        .next_target(unit_target(SanitizedSearchUnitOperation::Next))
        .previous_enabled(unit_targets_enabled)
        .previous_target(unit_target(SanitizedSearchUnitOperation::Previous))
        .replace_enabled(true)
        .replace_target(text_target(SanitizedSearchTextOperation::Replace))
        .replace_all_enabled(true)
        .replace_all_target(text_target(SanitizedSearchTextOperation::ReplaceAll));
    if unit_targets_enabled {
        builder = builder
            .match_case_target(unit_target(SanitizedSearchUnitOperation::MatchCase(false)))
            .whole_word_target(unit_target(SanitizedSearchUnitOperation::WholeWord(false)))
            .regex_target(unit_target(SanitizedSearchUnitOperation::Regex(false)));
    }
    input(revision, b"document", "本文 ⭐️")
        .with_search_projection(builder.build().expect("search projection is valid"))
}

fn run_unit_operation_case(
    operation: SanitizedSearchUnitOperation,
    input_events: impl FnOnce(&egui::Context, &mut super::SanitizedDocumentRoot) -> Vec<egui::Event>,
) {
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders_and_unit_targets(
            1,
            Rc::new(RefCell::new(Vec::new())),
            unit_events.clone(),
            true,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let events = input_events(&context, &mut root);
    let (_, frame) = run_search_root_frame(&context, &mut root, events);
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = frame
        .forward_events_once(&mut forwarder)
        .expect("unit operation forwarding succeeds");
    assert_eq!(unit_events.borrow().as_slice(), &[operation]);
    assert_eq!(
        frame.output.events().event_cardinality(),
        0,
        "sanitized search consumes its physical text event exclusively"
    );
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    assert!(!format!("{receipt:?}").contains("payload"));
}

fn assert_search_leaf_forwarded_once(
    leaf: &str,
    frame: &SanitizedDocumentRootFrame,
    forwarder: &mut RecordingForwarder,
) {
    assert_eq!(
        frame.output.events().event_cardinality(),
        0,
        "{leaf}: root batch must not duplicate the sanitized physical search event"
    );
    assert_eq!(
        frame.search_events.borrow().as_ref().map_or(0, Vec::len),
        1,
        "{leaf}: exactly one sanitized search receipt must be retained"
    );
    let receipt = frame
        .forward_events_once(forwarder)
        .unwrap_or_else(|error| panic!("{leaf}: forwarding failed: {error:?}"));
    assert_eq!(
        receipt.event_cardinality(),
        1,
        "{leaf}: receipt cardinality"
    );
    assert_eq!(forwarder.calls, 1, "{leaf}: outer forwarder calls");

    for (name, debug) in [
        ("frame", format!("{frame:?}")),
        (
            "transport",
            forwarder
                .transport_debug
                .clone()
                .expect("forwarder captured transport Debug"),
        ),
        ("receipt", format!("{receipt:?}")),
    ] {
        for forbidden in ["日本語", "置換後", "⭐️", "👩‍💻", "かな", "opaque payload"]
        {
            assert!(
                !debug.contains(forbidden),
                "{leaf}: {name} Debug leaked `{forbidden}`: {debug}"
            );
        }
    }
    assert_eq!(
        frame.forward_events_once(forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed),
        "{leaf}: replay must be AlreadyConsumed"
    );
    assert_eq!(
        forwarder.calls, 1,
        "{leaf}: replay changed outer forwarding"
    );
}

fn accesskit_click(node: egui::accesskit::NodeId) -> egui::Event {
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        action: egui::accesskit::Action::Click,
        target_tree: egui::accesskit::TreeId::ROOT,
        target_node: node,
        data: None,
    })
}

type SearchTextEvents = Rc<RefCell<Vec<(SanitizedSearchTextOperation, String)>>>;
type SearchUnitEvents = Rc<RefCell<Vec<SanitizedSearchUnitOperation>>>;

fn recorded_search_case() -> (
    egui::Context,
    super::SanitizedDocumentRoot,
    SearchTextEvents,
    SearchUnitEvents,
) {
    let text_events = Rc::new(RefCell::new(Vec::new()));
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let root = SanitizedDocumentRootFactory::new()
        .retain(input_with_recorders(
            1,
            text_events.clone(),
            unit_events.clone(),
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    (context, root, text_events, unit_events)
}

#[test]
fn each_enabled_search_leaf_has_individual_raw_input_accesskit_evidence() {
    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let bounds = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️");
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), true)],
        );
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), false)],
        );
        let frame = run_search_root_frame(
            &context,
            &mut root,
            vec![egui::Event::Ime(egui::ImeEvent::Commit(
                "日本語 ⭐️👩‍💻".to_string(),
            ))],
        )
        .1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        assert_search_leaf_forwarded_once("query IME commit", &frame, &mut forwarder);
        assert_eq!(
            text_events.borrow().as_slice(),
            &[(
                SanitizedSearchTextOperation::Query,
                "日本語 ⭐️👩‍💻".to_string()
            )],
            "query IME commit callback"
        );
        assert!(unit_events.borrow().is_empty(), "query unit callbacks");
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let bounds = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "置換 ⭐️");
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), true)],
        );
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), false)],
        );
        let frame = run_search_root_frame(
            &context,
            &mut root,
            vec![egui::Event::Ime(egui::ImeEvent::Commit(
                "日本語 ⭐️👩‍💻".to_string(),
            ))],
        )
        .1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        assert_search_leaf_forwarded_once("replacement IME commit", &frame, &mut forwarder);
        assert_eq!(
            text_events.borrow().as_slice(),
            &[(
                SanitizedSearchTextOperation::Replacement,
                "日本語 ⭐️👩‍💻".to_string(),
            )],
            "replacement IME commit callback"
        );
        assert!(
            unit_events.borrow().is_empty(),
            "replacement unit callbacks"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, "置換 ⭐️");
        let frame = run_search_root_frame(&context, &mut root, vec![accesskit_click(node)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        assert_search_leaf_forwarded_once("replace-one", &frame, &mut forwarder);
        assert_eq!(
            text_events.borrow().as_slice(),
            &[(SanitizedSearchTextOperation::Replace, String::new())],
            "replace-one callback"
        );
        assert!(
            unit_events.borrow().is_empty(),
            "replace-one unit callbacks"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, "すべて置換 ⭐️");
        let frame = run_search_root_frame(&context, &mut root, vec![accesskit_click(node)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        assert_search_leaf_forwarded_once("replace-all", &frame, &mut forwarder);
        assert_eq!(
            text_events.borrow().as_slice(),
            &[(SanitizedSearchTextOperation::ReplaceAll, String::new())],
            "replace-all callback"
        );
        assert!(
            unit_events.borrow().is_empty(),
            "replace-all unit callbacks"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let bounds = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️");
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), true)],
        );
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), false)],
        );
        let frame =
            run_search_root_frame(&context, &mut root, vec![key_press(egui::Key::ArrowDown)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        assert_search_leaf_forwarded_once("next", &frame, &mut forwarder);
        assert!(text_events.borrow().is_empty(), "next text callbacks");
        assert_eq!(
            unit_events.borrow().as_slice(),
            &[SanitizedSearchUnitOperation::Next],
            "next callback"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let bounds = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️");
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), true)],
        );
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), false)],
        );
        let frame =
            run_search_root_frame(&context, &mut root, vec![key_press(egui::Key::ArrowUp)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        assert_search_leaf_forwarded_once("previous", &frame, &mut forwarder);
        assert!(text_events.borrow().is_empty(), "previous text callbacks");
        assert_eq!(
            unit_events.borrow().as_slice(),
            &[SanitizedSearchUnitOperation::Previous],
            "previous callback"
        );
    }

    for (leaf, label, operation) in [
        (
            "match-case",
            "大文字小文字 ⭐️",
            SanitizedSearchUnitOperation::MatchCase(true),
        ),
        (
            "whole-word",
            "単語 ⭐️",
            SanitizedSearchUnitOperation::WholeWord(true),
        ),
        (
            "regex",
            "正規表現 ⭐️",
            SanitizedSearchUnitOperation::Regex(true),
        ),
    ] {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, label);
        let frame = run_search_root_frame(&context, &mut root, vec![accesskit_click(node)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        assert_search_leaf_forwarded_once(leaf, &frame, &mut forwarder);
        assert!(text_events.borrow().is_empty(), "{leaf} text callbacks");
        assert_eq!(
            unit_events.borrow().as_slice(),
            &[operation],
            "{leaf} callback"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, "閉じる ⭐️");
        let frame = run_search_root_frame(&context, &mut root, vec![accesskit_click(node)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        assert_search_leaf_forwarded_once("close", &frame, &mut forwarder);
        assert!(text_events.borrow().is_empty(), "close text callbacks");
        assert_eq!(
            unit_events.borrow().as_slice(),
            &[SanitizedSearchUnitOperation::Close],
            "close callback"
        );
    }
}

#[test]
fn current_search_unit_operations_use_physical_input_one_shot_routing() {
    let pointer_case = |label: &'static str, operation| {
        run_unit_operation_case(operation, move |context, root| {
            let (output, _) = run_search_root_frame(context, root, Vec::new());
            let (_, bounds) = accesskit_button(&output, label);
            let _ =
                run_search_root_frame(context, root, vec![pointer_button(bounds.center(), true)]);
            vec![pointer_button(bounds.center(), false)]
        });
    };
    pointer_case(
        "大文字小文字 ⭐️",
        SanitizedSearchUnitOperation::MatchCase(true),
    );
    pointer_case("単語 ⭐️", SanitizedSearchUnitOperation::WholeWord(true));
    pointer_case("正規表現 ⭐️", SanitizedSearchUnitOperation::Regex(true));
    run_unit_operation_case(SanitizedSearchUnitOperation::Close, |context, root| {
        let (output, _) = run_search_root_frame(context, root, Vec::new());
        let (node, _) = accesskit_button(&output, "閉じる ⭐️");
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: node,
                data: None,
            },
        )]
    });
}

#[test]
fn projected_option_state_renders_and_is_acknowledged_by_a_newer_revision() {
    let text_events = Rc::new(RefCell::new(Vec::new()));
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders_and_unit_targets_and_state(
            1,
            text_events.clone(),
            unit_events.clone(),
            true,
            true,
            false,
            true,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let initial_options = root
        .process
        .search_options()
        .expect("search presentation exists");
    assert!(initial_options.match_case);
    assert!(!initial_options.whole_word);
    assert!(initial_options.use_regex);
    let (initial_output, _) = run_search_root_frame(&context, &mut root, Vec::new());

    let (_, match_case_bounds) = accesskit_button(&initial_output, "大文字小文字 ⭐️");
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(match_case_bounds.center(), true)],
    );
    let (_, toggled_frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(match_case_bounds.center(), false)],
    );
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = toggled_frame
        .forward_events_once(&mut forwarder)
        .expect("toggle forwards");
    assert_eq!(
        unit_events.borrow().as_slice(),
        &[SanitizedSearchUnitOperation::MatchCase(false)]
    );
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);

    root.synchronize(input_with_recorders_and_unit_targets_and_state(
        2,
        text_events,
        unit_events,
        true,
        false,
        false,
        true,
    ))
    .expect("newer host acknowledgement synchronizes");
    let _ = run_search_root_frame(&context, &mut root, Vec::new());
    let acknowledged_options = root
        .process
        .search_options()
        .expect("search presentation exists");
    assert!(!acknowledged_options.match_case);
    assert!(!acknowledged_options.whole_word);
    assert!(acknowledged_options.use_regex);
}

#[test]
fn disabled_current_search_unit_operations_emit_no_callback_or_forwarded_event() {
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders_and_unit_targets(
            1,
            Rc::new(RefCell::new(Vec::new())),
            unit_events.clone(),
            false,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let (_, bounds) = accesskit_button(&output, "正規表現 ⭐️");
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(bounds.center(), true)],
    );
    let (_, frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(bounds.center(), false)],
    );
    assert!(unit_events.borrow().is_empty());
    assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 0);
    assert!(!format!("{frame:?}").contains("payload"));
}

#[test]
fn unsupported_current_search_unit_operations_are_disabled_without_routing() {
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders_and_unit_targets(
            1,
            Rc::new(RefCell::new(Vec::new())),
            unit_events.clone(),
            false,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let (_, regex_bounds) = accesskit_button(&output, "正規表現 ⭐️");
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(regex_bounds.center(), true)],
    );
    let (_, frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(regex_bounds.center(), false)],
    );
    assert!(unit_events.borrow().is_empty());
    assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 0);

    for label in ["前へ ⭐️", "次へ ⭐️"] {
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (_, bounds) = accesskit_button(&output, label);
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), true)],
        );
        let (_, frame) = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), false)],
        );
        assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 0);
    }
    assert!(unit_events.borrow().is_empty());
}

#[test]
fn physical_raw_input_routes_text_replace_and_navigation_operations() {
    let text_events = Rc::new(RefCell::new(Vec::new()));
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders(
            1,
            text_events.clone(),
            unit_events.clone(),
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let query = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️").center();
    let _ = run_search_root_frame(&context, &mut root, vec![pointer_button(query, true)]);
    let _ = run_search_root_frame(&context, &mut root, vec![pointer_button(query, false)]);
    let query_frame = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語 ⭐️👩‍💻".to_string(),
        ))],
    )
    .1;
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    query_frame
        .forward_events_once(&mut forwarder)
        .expect("query forwards");

    let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let replacement =
        accesskit_bounds(&output, egui::accesskit::Role::TextInput, "置換 ⭐️").center();
    let _ = run_search_root_frame(&context, &mut root, vec![pointer_button(replacement, true)]);
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(replacement, false)],
    );
    let replacement_frame = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "置換後 ⭐️👩‍💻".to_string(),
        ))],
    )
    .1;
    replacement_frame
        .forward_events_once(&mut forwarder)
        .expect("replacement forwards");

    for (label, operation) in [
        ("置換 ⭐️", SanitizedSearchTextOperation::Replace),
        ("すべて置換 ⭐️", SanitizedSearchTextOperation::ReplaceAll),
    ] {
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, label);
        let frame = run_search_root_frame(
            &context,
            &mut root,
            vec![egui::Event::AccessKitActionRequest(
                egui::accesskit::ActionRequest {
                    action: egui::accesskit::Action::Click,
                    target_tree: egui::accesskit::TreeId::ROOT,
                    target_node: node,
                    data: None,
                },
            )],
        )
        .1;
        frame
            .forward_events_once(&mut forwarder)
            .expect("replace operation forwards");
        assert!(text_events.borrow().iter().any(|(actual, value)| {
            *actual == operation && value == "置換後 ⭐️👩‍💻"
        }));
    }

    assert_eq!(
        text_events.borrow().as_slice(),
        &[
            (
                SanitizedSearchTextOperation::Query,
                "日本語 ⭐️👩‍💻".to_string()
            ),
            (
                SanitizedSearchTextOperation::Replacement,
                "置換後 ⭐️👩‍💻".to_string(),
            ),
            (
                SanitizedSearchTextOperation::Replace,
                "置換後 ⭐️👩‍💻".to_string(),
            ),
            (
                SanitizedSearchTextOperation::ReplaceAll,
                "置換後 ⭐️👩‍💻".to_string(),
            ),
        ]
    );

    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let mut next_root = factory
        .retain(input_with_recorders(
            1,
            Rc::new(RefCell::new(Vec::new())),
            unit_events.clone(),
        ))
        .expect("retain succeeds");
    let (output, _) = run_search_root_frame(&context, &mut next_root, Vec::new());
    let query = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️").center();
    let _ = run_search_root_frame(&context, &mut next_root, vec![pointer_button(query, true)]);
    let _ = run_search_root_frame(&context, &mut next_root, vec![pointer_button(query, false)]);
    let frame = run_search_root_frame(
        &context,
        &mut next_root,
        vec![key_press(egui::Key::ArrowDown)],
    )
    .1;
    frame
        .forward_events_once(&mut forwarder)
        .expect("next navigation forwards");
    assert_eq!(
        unit_events.borrow().as_slice(),
        &[SanitizedSearchUnitOperation::Next]
    );
    let frame = run_search_root_frame(
        &context,
        &mut next_root,
        vec![key_press(egui::Key::ArrowUp)],
    )
    .1;
    frame
        .forward_events_once(&mut forwarder)
        .expect("previous navigation forwards");
    assert_eq!(
        unit_events.borrow().as_slice(),
        &[
            SanitizedSearchUnitOperation::Next,
            SanitizedSearchUnitOperation::Previous,
        ]
    );
}

#[test]
fn synchronize_maps_process_errors_to_the_public_error_contract() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input(3, b"one", "a"))
        .expect("retain succeeds");

    assert_eq!(
        root.synchronize(input(4, b"two", "b")),
        Err(SanitizedDocumentRootFactoryError::IdentityChanged)
    );
    assert_eq!(
        root.synchronize(input(2, b"one", "b")),
        Err(SanitizedDocumentRootFactoryError::StaleRevision {
            current: 3,
            received: 2,
        })
    );
    assert_eq!(
        root.synchronize(input(3, b"one", "b")),
        Err(SanitizedDocumentRootFactoryError::RevisionConflict { revision: 3 })
    );
}

#[test]
fn readonly_is_revisioned_and_exposed_to_accesskit() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input(1, b"readonly", "本文 ⭐️"))
        .expect("retain succeeds");
    assert_eq!(
        root.synchronize(input(1, b"readonly", "本文 ⭐️").with_readonly(true)),
        Err(SanitizedDocumentRootFactoryError::RevisionConflict { revision: 1 })
    );

    assert!(
        root.synchronize(input(2, b"readonly", "本文 ⭐️").with_readonly(true))
            .expect("new revision synchronizes")
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, frame) = run_root_frame_events(&context, &mut root, Vec::new());
    assert!(
        frame
            .output
            .evidence_text
            .record
            .frame
            .accessibility
            .root
            .readonly
    );
    let node = output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::TextInput
                    || node.role() == egui::accesskit::Role::MultilineTextInput)
                    .then_some(node)
            })
        })
        .expect("readonly text input is exposed to AccessKit");
    assert!(node.is_read_only());
}

#[test]
fn readonly_raw_text_and_ime_do_not_mutate_but_pointer_selection_remains_available() {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input(1, b"readonly-input", "本文 ⭐️").with_readonly(true))
        .expect("retain succeeds");
    let context = egui::Context::default();

    let (_, initial) = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
    );
    let content = initial.output.evidence_text.record.frame.content_bounds;
    let start = egui::pos2(
        content.x as f32 + 8.0,
        content.y as f32 + content.height as f32 / 2.0,
    );
    let end = egui::pos2(content.x as f32 + content.width as f32 - 8.0, start.y);
    let midpoint = egui::pos2(content.x as f32 + content.width as f32 / 2.0, start.y);

    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, true)]);
    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, false)]);
    let (_, focused) = run_root_frame_events(&context, &mut root, Vec::new());
    assert!(
        focused
            .output
            .evidence_text
            .record
            .frame
            .accessibility
            .root
            .focused
    );
    for events in [
        vec![egui::Event::Text("追加入力 ⭐️".to_string())],
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "下書き ⭐️".to_string(),
            active_range_chars: None,
        })],
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "確定 ⭐️".to_string(),
        ))],
    ] {
        let (_, frame) = run_root_frame_events(&context, &mut root, events);
        assert_eq!(root.process.input.snapshot, "本文 ⭐️");
        assert!(!frame.output.evidence_text.events.iter().any(|event| {
            matches!(
                event,
                katana_ui_core::text_surface::TextSurfaceEvent::TextArea(
                    katana_ui_core::atom::TextAreaEvent::TextInput(_)
                        | katana_ui_core::atom::TextAreaEvent::ImeComposition(_)
                        | katana_ui_core::atom::TextAreaEvent::ImeCommit(_)
                        | katana_ui_core::atom::TextAreaEvent::Change(_)
                )
            )
        }));
    }

    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, true)]);
    let _ = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(midpoint)],
    );
    let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(end)]);
    let (_, selected) =
        run_root_frame_events(&context, &mut root, vec![pointer_button(end, false)]);
    let range = selected
        .output
        .evidence_text
        .record
        .frame
        .selection
        .range
        .ordered();
    assert!(range.start < range.end);
    assert_eq!(root.process.input.snapshot, "本文 ⭐️");
}

#[test]
fn readonly_does_not_assign_semantics_to_an_enabled_opaque_command() {
    let calls = Rc::new(RefCell::new(0));
    let projection =
        super::super::SanitizedCommandProjection::new([super::super::SanitizedCommandGroup::new(
            1, "generic",
        )
        .item(
            super::super::SanitizedCommandItem::new(
                super::super::SanitizedCommandTarget::from_opaque_bytes([7]).with_unit_capability(
                    {
                        let calls = calls.clone();
                        move || {
                            *calls.borrow_mut() += 1;
                            Ok::<(), ()>(())
                        }
                    },
                ),
                1,
                "opaque action",
            )
            .with_icon(katana_ui_core::render_model::UiIconProps::new("<svg/>")),
        )]);
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(
            input(1, b"readonly-command", "本文 ⭐️")
                .with_readonly(true)
                .with_command_projection(projection),
        )
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
    let (node, _) = command_node(&output, "opaque action");
    let (_, released) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
    assert_eq!(
        released
            .command_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        1
    );
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    released
        .forward_events_once(&mut forwarder)
        .expect("opaque command forwards");
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(root.process.input.snapshot, "本文 ⭐️");
}

#[test]
fn show_returns_a_closed_record_and_forwards_events_only_once() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input(1, b"document", "日本語 ⭐️"))
        .expect("retain succeeds");
    let context = egui::Context::default();
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    output.textures_delta.clear();
    let frame = frame.expect("frame exists");

    assert_eq!(frame.record().revision(), 1);
    assert!(frame.record().dimensions().width() > 0);
    assert!(frame.record().dimensions().height() > 0);
    assert_eq!(frame.record().rgba_hash().len(), 64);
    assert_eq!(frame.record().accessibility_snapshot_hash().len(), 64);

    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = frame
        .forward_events_once(&mut forwarder)
        .expect("first forwarding succeeds");
    assert_eq!(forwarder.calls, 1);
    assert!(receipt.consumed_once());
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
}

#[test]
fn raw_ime_search_at_current_root_renders_and_forwards_one_opaque_event() -> Result<(), String> {
    let factory = SanitizedDocumentRootFactory::new();
    let search_input = input_with_search(1)?;
    let mut root = factory.retain(search_input).expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (initial_output, initial_frame) = run_search_root_frame(&context, &mut root, Vec::new());
    let query_bounds = initial_output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::TextInput).then(|| node.bounds())
            })
        })
        .flatten()
        .expect("the localized query input has an actual AccessKit bounds");
    assert!(query_bounds.x1 > query_bounds.x0);
    assert!(query_bounds.y1 > query_bounds.y0);

    let no_search_context = egui::Context::default();
    let mut no_search_root = factory
        .retain(input(1, b"document", "本文 ⭐️"))
        .expect("retain without search succeeds");
    let (_, no_search_frame) =
        run_search_root_frame(&no_search_context, &mut no_search_root, Vec::new());
    assert_ne!(
        initial_frame.record().record_hash(),
        no_search_frame.record().record_hash()
    );

    let query = egui::pos2(
        ((query_bounds.x0 + query_bounds.x1) / 2.0) as f32,
        ((query_bounds.y0 + query_bounds.y1) / 2.0) as f32,
    );
    let (_, pressed) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: query,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let (_, focused) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: query,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    assert_eq!(pressed.output.events().event_cardinality(), 0);
    assert_eq!(focused.output.events().event_cardinality(), 0);

    let (_, preedit) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".to_string(),
            active_range_chars: None,
        })],
    );
    assert_eq!(preedit.output.events().event_cardinality(), 0);
    assert_ne!(
        focused.record().record_hash(),
        preedit.record().record_hash()
    );

    let (_, committed) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    );
    assert_eq!(committed.output.events().event_cardinality(), 0);
    assert_eq!(
        committed
            .search_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        1
    );

    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = committed
        .forward_events_once(&mut forwarder)
        .expect("one-shot search forwarding succeeds");
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 1);

    let frame_debug = format!("{committed:?}");
    for forbidden in ["検索語 ⭐️", "かな", "payload"] {
        assert!(!frame_debug.contains(forbidden));
    }
    let transport_debug = forwarder
        .transport_debug
        .as_deref()
        .expect("transport debug exists");
    for forbidden in ["検索語 ⭐️", "かな", "opaque payload"] {
        assert!(!transport_debug.contains(forbidden));
    }
    assert!(transport_debug.contains("<opaque>"));
    let receipt_debug = format!("{receipt:?}");
    for forbidden in ["検索語 ⭐️", "かな", "opaque payload"] {
        assert!(!receipt_debug.contains(forbidden));
    }
    assert_eq!(
        committed.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    Ok(())
}

#[test]
fn physical_ime_commit_routes_exact_text_once_without_debug_leakage() {
    let text_events = Rc::new(RefCell::new(Vec::new()));
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders(1, text_events.clone(), unit_events))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (initial_output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let query_bounds = accesskit_bounds(
        &initial_output,
        egui::accesskit::Role::TextInput,
        "検索語 ⭐️",
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(query_bounds.center(), true)],
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(query_bounds.center(), false)],
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".to_string(),
            active_range_chars: None,
        })],
    );
    let (_, committed) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    );

    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = committed
        .forward_events_once(&mut forwarder)
        .expect("physical IME event forwards");
    assert_eq!(
        text_events.borrow().as_slice(),
        &[(SanitizedSearchTextOperation::Query, "⭐️".to_string(),)]
    );
    assert!(committed.output.events().event_cardinality() == 0);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);
    assert!(!format!("{committed:?}").contains("⭐️"));
    assert!(!format!("{committed:?}").contains("👩‍💻"));
    assert!(!format!("{receipt:?}").contains("⭐️"));
    assert!(!format!("{receipt:?}").contains("👩‍💻"));
    assert!(
        !forwarder
            .transport_debug
            .as_deref()
            .expect("transport debug exists")
            .contains("⭐️")
    );
    assert!(
        !forwarder
            .transport_debug
            .as_deref()
            .expect("transport debug exists")
            .contains("👩‍💻")
    );
    assert_eq!(
        committed.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
}

#[test]
fn sanitized_physical_search_callback_rejection_is_opaque_and_consumed() {
    for operation in ["query", "replacement", "option"] {
        let text_calls = Rc::new(RefCell::new(0));
        let unit_calls = Rc::new(RefCell::new(0));
        let factory = SanitizedDocumentRootFactory::new();
        let mut root = factory
            .retain(input_with_rejecting_recorders(
                1,
                text_calls.clone(),
                unit_calls.clone(),
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        context.enable_accesskit();
        let (initial_output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let event = match operation {
            "query" => {
                let bounds = accesskit_bounds(
                    &initial_output,
                    egui::accesskit::Role::TextInput,
                    "検索語 ⭐️",
                );
                let _ = run_search_root_frame(
                    &context,
                    &mut root,
                    vec![pointer_button(bounds.center(), true)],
                );
                let _ = run_search_root_frame(
                    &context,
                    &mut root,
                    vec![pointer_button(bounds.center(), false)],
                );
                vec![egui::Event::Ime(egui::ImeEvent::Commit(
                    "日本語 ⭐️👩‍💻".to_string(),
                ))]
            }
            "replacement" => {
                let bounds =
                    accesskit_bounds(&initial_output, egui::accesskit::Role::TextInput, "置換 ⭐️");
                let _ = run_search_root_frame(
                    &context,
                    &mut root,
                    vec![pointer_button(bounds.center(), true)],
                );
                let _ = run_search_root_frame(
                    &context,
                    &mut root,
                    vec![pointer_button(bounds.center(), false)],
                );
                vec![egui::Event::Ime(egui::ImeEvent::Commit(
                    "置換後 ⭐️👩‍💻".to_string(),
                ))]
            }
            "option" => {
                let (node, _) = accesskit_button(&initial_output, "大文字小文字 ⭐️");
                vec![egui::Event::AccessKitActionRequest(
                    egui::accesskit::ActionRequest {
                        action: egui::accesskit::Action::Click,
                        target_tree: egui::accesskit::TreeId::ROOT,
                        target_node: node,
                        data: None,
                    },
                )]
            }
            _ => unreachable!(),
        };
        let (_, frame) = run_search_root_frame(&context, &mut root, event);
        assert_eq!(frame.output.events().event_cardinality(), 0, "{operation}");
        assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 1);

        let mut forwarder = RetainingForwarder {
            calls: 0,
            transport_debug: None,
            transport: None,
        };
        let receipt = frame.forward_events_once(&mut forwarder);
        assert!(receipt.is_ok(), "{operation} outer forward");
        assert_eq!(*text_calls.borrow(), 0, "{operation} text pre-dispatch");
        assert_eq!(*unit_calls.borrow(), 0, "{operation} unit pre-dispatch");
        assert_eq!(forwarder.calls, 1, "{operation} outer forward call");
        assert!(frame.search_events.borrow().is_none());
        assert!(!format!("{frame:?}").contains("日本語 ⭐️👩‍💻"));
        assert!(!format!("{frame:?}").contains("置換後 ⭐️👩‍💻"));
        assert!(!format!("{frame:?}").contains("payload"));
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect),
            "{operation} host dispatch rejection"
        );
        assert_eq!(
            *text_calls.borrow(),
            usize::from(operation != "option"),
            "{operation} text host dispatch"
        );
        assert_eq!(
            *unit_calls.borrow(),
            usize::from(operation == "option"),
            "{operation} unit host dispatch"
        );
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed),
            "{operation} host dispatch replay"
        );
        assert_eq!(
            frame.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
        );
        assert_eq!(forwarder.calls, 1);
    }
}

#[test]
fn sanitized_physical_search_frame_is_stale_after_newer_same_identity_sync() {
    let text_calls = Rc::new(RefCell::new(0));
    let unit_calls = Rc::new(RefCell::new(0));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_rejecting_recorders(
            1,
            text_calls.clone(),
            unit_calls.clone(),
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (initial_output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let bounds = accesskit_bounds(
        &initial_output,
        egui::accesskit::Role::TextInput,
        "検索語 ⭐️",
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(bounds.center(), true)],
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(bounds.center(), false)],
    );
    let (_, frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語 ⭐️👩‍💻".to_string(),
        ))],
    );
    assert_eq!(frame.output.events().event_cardinality(), 0);
    assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 1);

    assert!(
        root.synchronize(input_with_rejecting_recorders(
            2,
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
        ))
        .expect("newer same-identity synchronization succeeds")
    );
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
    assert_eq!(*text_calls.borrow(), 0);
    assert_eq!(*unit_calls.borrow(), 0);
    assert_eq!(forwarder.calls, 0);
    assert!(frame.search_events.borrow().is_some());
    assert!(!format!("{frame:?}").contains("日本語 ⭐️👩‍💻"));
    assert!(!format!("{frame:?}").contains("payload"));
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
    assert_eq!(forwarder.calls, 0);
}

#[test]
fn forwarder_error_consumes_root_tab_and_search_batches() -> Result<(), String> {
    let factory = SanitizedDocumentRootFactory::new();
    let search_input = input_with_search(1)?;
    let mut root = factory.retain(search_input).expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (initial_output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let query_bounds = initial_output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::TextInput).then(|| node.bounds())
            })
        })
        .flatten()
        .expect("query bounds exist");
    let query = egui::pos2(
        ((query_bounds.x0 + query_bounds.x1) / 2.0) as f32,
        ((query_bounds.y0 + query_bounds.y1) / 2.0) as f32,
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: query,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: query,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let (_, frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    );
    assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 1);
    assert!(frame.tab_closed_events.borrow().is_some());

    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: true,
    };
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::Forwarder(()))
    );
    assert_eq!(forwarder.calls, 1);
    assert!(frame.tab_closed_events.borrow().is_none());
    assert!(frame.search_events.borrow().is_none());
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    Ok(())
}

#[test]
fn public_show_retains_tab_event_without_exposing_it_in_public_frame_data() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory.retain(input_with_tabs(1)).expect("retain succeeds");
    let context = egui::Context::default();

    let mut first = None;
    let mut first_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                first = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    first_output.textures_delta.clear();
    let first = first.expect("first frame exists");
    let mut root_only_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let root_only_receipt = first
        .forward_events_once(&mut root_only_forwarder)
        .expect("root-only forwarding succeeds");
    assert_eq!(root_only_forwarder.calls, 1);
    assert_eq!(root_only_receipt.event_cardinality(), 0);
    let target = first
        .tab_rects()
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("second tab rect exists");

    let mut pressed = None;
    let mut pressed_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events: vec![egui::Event::PointerButton {
                pos: target,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                pressed = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    pressed_output.textures_delta.clear();
    let pressed = pressed.expect("pressed frame exists");
    assert_eq!(pressed.tab_closed_event_count(), 0);
    let mut no_event_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let no_event_receipt = pressed
        .forward_events_once(&mut no_event_forwarder)
        .expect("no-event forwarding succeeds");
    assert_eq!(no_event_forwarder.calls, 1);

    let mut released = None;
    let mut released_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events: vec![egui::Event::PointerButton {
                pos: target,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                released = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    released_output.textures_delta.clear();
    let released = released.expect("released frame exists");
    assert_eq!(released.tab_closed_event_count(), 1);
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = released
        .forward_events_once(&mut forwarder)
        .expect("released tab event forwarding succeeds");
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_ne!(
        root_only_receipt.event_batch_fingerprint(),
        receipt.event_batch_fingerprint()
    );
    assert_ne!(
        no_event_receipt.event_batch_fingerprint(),
        receipt.event_batch_fingerprint()
    );
    assert_ne!(
        root_only_receipt.correlation_fingerprint(),
        receipt.correlation_fingerprint()
    );
    assert_ne!(
        no_event_receipt.correlation_fingerprint(),
        receipt.correlation_fingerprint()
    );

    let frame_debug = format!("{released:?}");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "payload"] {
        assert!(
            !frame_debug.contains(forbidden),
            "public frame Debug leaked `{forbidden}`: {frame_debug}"
        );
    }
    let transport_debug = forwarder
        .transport_debug
        .as_deref()
        .expect("forwarder recorded transport Debug");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(
            !transport_debug.contains(forbidden),
            "transport Debug leaked `{forbidden}`: {transport_debug}"
        );
    }
    assert!(transport_debug.contains("<opaque>"));
    let receipt_debug = format!("{receipt:?}");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(
            !receipt_debug.contains(forbidden),
            "receipt Debug leaked `{forbidden}`: {receipt_debug}"
        );
    }
    assert_eq!(
        released.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    assert_eq!(released.record().revision(), 1);
}

#[test]
fn missing_tab_event_batch_fails_closed_without_calling_forwarder() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input(1, b"document", "本文"))
        .expect("retain succeeds");
    let context = egui::Context::default();
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    output.textures_delta.clear();
    let frame = frame.expect("frame exists");
    let _ = frame.tab_closed_events.borrow_mut().take();
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };

    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::InconsistentTabEventBatch)
    );
    assert_eq!(forwarder.calls, 0);
}

#[test]
fn consumed_child_event_channels_fail_closed_through_real_root_forwarding() {
    for channel in ["search", "command", "context menu"] {
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(input(1, b"event-channel", "本文 ⭐️"))
            .expect("retain succeeds");
        let context = egui::Context::default();
        let mut frame = None;
        crate::run_ui_discard(
            &context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
                )),
                ..egui::RawInput::default()
            },
            |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    frame = Some(root.show(ui).expect("real root frame renders"));
                });
            },
        );
        let frame = frame.expect("real root frame exists");
        match channel {
            "search" => {
                let _ = frame.search_events.borrow_mut().take();
            }
            "command" => {
                let _ = frame.command_events.borrow_mut().take();
            }
            "context menu" => {
                let _ = frame.context_menu_events.borrow_mut().take();
            }
            _ => unreachable!(),
        }
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };

        let result = frame.forward_events_once(&mut forwarder);
        assert!(
            matches!(
                (channel, result),
                (
                    "search",
                    Err(SanitizedDocumentRootEventForwardError::InconsistentSearchEventBatch)
                ) | (
                    "command",
                    Err(SanitizedDocumentRootEventForwardError::InconsistentCommandEventBatch)
                ) | (
                    "context menu",
                    Err(SanitizedDocumentRootEventForwardError::InconsistentContextMenuEventBatch)
                )
            ),
            "{channel} channel must fail through its typed consistency error"
        );
        assert_eq!(forwarder.calls, 0);
    }
}

#[test]
fn raw_input_close_emits_one_opaque_intent_and_waits_for_next_projection() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory.retain(input_with_tabs(1)).expect("retain succeeds");
    let context = egui::Context::default();

    let mut first = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                first = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    output.textures_delta.clear();
    let first = first.expect("first frame exists");
    let select_target = first
        .tab_rects()
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("tab response exists");
    let close_target = first
        .tab_close_rects()
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("close response exists");

    let mut root_only_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let root_only_receipt = first
        .forward_events_once(&mut root_only_forwarder)
        .expect("root-only forwarding succeeds");

    let _ = run_root_frame(&context, &mut root, pointer_button(select_target, true));
    let selected = run_root_frame(&context, &mut root, pointer_button(select_target, false));
    let mut select_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let select_receipt = selected
        .forward_events_once(&mut select_forwarder)
        .expect("select forwarding succeeds");

    let _ = run_root_frame(&context, &mut root, pointer_button(close_target, true));
    let close_frame = run_root_frame(&context, &mut root, pointer_button(close_target, false));
    assert_eq!(close_frame.tab_closed_event_count(), 1);
    let mut close_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let close_receipt = close_frame
        .forward_events_once(&mut close_forwarder)
        .expect("close forwarding succeeds");

    assert_eq!(root_only_forwarder.calls, 1);
    assert_eq!(select_forwarder.calls, 1);
    assert_eq!(close_forwarder.calls, 1);
    assert_ne!(
        root_only_receipt.event_batch_fingerprint(),
        select_receipt.event_batch_fingerprint()
    );
    assert_ne!(
        select_receipt.event_batch_fingerprint(),
        close_receipt.event_batch_fingerprint()
    );
    assert_ne!(
        root_only_receipt.correlation_fingerprint(),
        select_receipt.correlation_fingerprint()
    );
    assert_ne!(
        select_receipt.correlation_fingerprint(),
        close_receipt.correlation_fingerprint()
    );
    assert_eq!(close_receipt.event_cardinality(), 1);

    let close_debug = format!("{close_receipt:?}");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(!close_debug.contains(forbidden));
    }
    let transport_debug = close_forwarder
        .transport_debug
        .as_deref()
        .expect("close transport debug exists");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(!transport_debug.contains(forbidden));
    }

    let retained = run_root_frame(
        &context,
        &mut root,
        egui::Event::PointerMoved(egui::Pos2::new(0.0, 0.0)),
    );
    assert!(
        retained
            .tab_close_rects()
            .iter()
            .any(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
    );
    assert_eq!(
        close_frame.forward_events_once(&mut close_forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(close_forwarder.calls, 1);

    root.synchronize(input_with_one_tab(2))
        .expect("new projection synchronizes");
    let synchronized = run_root_frame(
        &context,
        &mut root,
        egui::Event::PointerMoved(egui::Pos2::new(0.0, 0.0)),
    );
    assert!(synchronized.tab_close_rects().is_empty());
}

#[test]
fn accesskit_click_from_previous_frame_button_update_emits_one_close_intent() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory.retain(input_with_tabs(1)).expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let mut first_frame = None;
    let mut first_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                first_frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    first_output.textures_delta.clear();
    let first_frame = first_frame.expect("first frame exists");
    let update = first_output
        .platform_output
        .accesskit_update
        .expect("first frame emits AccessKit update");
    let (close_node, close_node_count) = update
        .nodes
        .iter()
        .filter(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("次の文書を閉じる")
        })
        .map(|(node_id, _)| (*node_id, 1usize))
        .fold((None, 0usize), |(_, count), (node_id, _)| {
            (Some(node_id), count + 1)
        });
    assert_eq!(
        close_node_count, 1,
        "the supplied close label must identify one button"
    );
    let close_node = close_node.expect("close button node exists");

    let mut accesskit_frame = None;
    let mut accesskit_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events: vec![egui::Event::AccessKitActionRequest(
                egui::accesskit::ActionRequest {
                    action: egui::accesskit::Action::Click,
                    target_tree: egui::accesskit::TreeId::ROOT,
                    target_node: close_node,
                    data: None,
                },
            )],
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                accesskit_frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    accesskit_output.textures_delta.clear();
    let accesskit_frame = accesskit_frame.expect("AccessKit frame exists");
    assert_eq!(accesskit_frame.tab_activation_event_count(), 0);
    assert_eq!(accesskit_frame.tab_close_request_event_count(), 1);
    assert_eq!(accesskit_frame.tab_closed_event_count(), 1);
    assert!(
        accesskit_frame
            .tab_close_rects()
            .iter()
            .any(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
    );

    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = accesskit_frame
        .forward_events_once(&mut forwarder)
        .expect("AccessKit close forwarding succeeds");
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(
        accesskit_frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);

    let frame_debug = format!("{accesskit_frame:?}");
    for forbidden in ["次の文書", "sanitized-tab-0-1"] {
        assert!(!frame_debug.contains(forbidden));
    }
    let transport_debug = forwarder
        .transport_debug
        .as_deref()
        .expect("transport debug exists");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(!transport_debug.contains(forbidden));
    }
    assert!(transport_debug.contains("<opaque>"));
    assert!(accesskit_output.platform_output.accesskit_update.is_some());
    drop(first_frame);
}

#[test]
fn physical_focused_close_button_enter_and_space_each_emit_one_opaque_intent() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let factory = SanitizedDocumentRootFactory::new();
        let mut root = factory.retain(input_with_tabs(1)).expect("retain succeeds");
        let context = egui::Context::default();

        let first = run_root_frame(
            &context,
            &mut root,
            egui::Event::PointerMoved(egui::Pos2::ZERO),
        );
        assert!(
            first
                .tab_close_rects()
                .iter()
                .any(|(_, rect)| rect.width() > 0.0)
        );

        for _ in 0..4 {
            let focused = run_root_frame(&context, &mut root, key_press(egui::Key::Tab));
            assert_eq!(focused.tab_closed_event_count(), 0);
            assert_eq!(focused.tab_activation_event_count(), 0);
        }

        let activated = run_root_frame(&context, &mut root, key_press(key));
        assert_eq!(activated.tab_activation_event_count(), 0);
        assert_eq!(activated.tab_close_request_event_count(), 1);
        assert_eq!(activated.tab_closed_event_count(), 1);
        assert!(
            activated
                .tab_close_rects()
                .iter()
                .any(|(_, rect)| rect.width() > 0.0),
            "close affordance must remain until the host projects the next state"
        );

        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        let receipt = activated
            .forward_events_once(&mut forwarder)
            .expect("physical key close forwarding succeeds");
        assert_eq!(receipt.event_cardinality(), 1);
        assert_eq!(forwarder.calls, 1);
        assert_eq!(
            activated.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
        );
        assert_eq!(forwarder.calls, 1);

        let transport_debug = forwarder
            .transport_debug
            .as_deref()
            .expect("transport debug exists");
        for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
            assert!(!transport_debug.contains(forbidden));
        }
        assert!(transport_debug.contains("<opaque>"));

        let retained = run_root_frame(
            &context,
            &mut root,
            egui::Event::PointerMoved(egui::Pos2::ZERO),
        );
        assert!(
            retained
                .tab_close_rects()
                .iter()
                .any(|(_, rect)| rect.width() > 0.0)
        );

        root.synchronize(input_with_one_tab(2))
            .expect("host next projection synchronizes");
        let projected = run_root_frame(
            &context,
            &mut root,
            egui::Event::PointerMoved(egui::Pos2::ZERO),
        );
        assert!(projected.tab_close_rects().is_empty());
    }
}

fn input_with_one_tab(revision: u64) -> SanitizedDocumentRootInput {
    input(revision, b"document", "本文 ⭐️").with_tab_projection(SanitizedTabProjection::new([
        SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "ドキュメント",
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([1]),
            0,
            "最初",
        )),
    ]))
}

fn command_input(
    revision: u64,
    calls: Rc<RefCell<usize>>,
    enabled: bool,
    visible: bool,
    capability: bool,
    dropdown: bool,
    reject: bool,
) -> SanitizedDocumentRootInput {
    command_input_with_callbacks(
        revision,
        calls.clone(),
        calls,
        enabled,
        visible,
        capability,
        dropdown,
        reject,
    )
}

fn command_input_with_callbacks(
    revision: u64,
    direct_calls: Rc<RefCell<usize>>,
    dropdown_calls: Rc<RefCell<usize>>,
    enabled: bool,
    visible: bool,
    capability: bool,
    dropdown: bool,
    reject: bool,
) -> SanitizedDocumentRootInput {
    use crate::text_command_surface::{
        SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
        SanitizedCommandProjection, SanitizedCommandTarget,
    };

    let target = |bytes: &[u8], calls: Rc<RefCell<usize>>| {
        let target = SanitizedCommandTarget::from_opaque_bytes(bytes.to_vec());
        if capability {
            target.with_unit_capability(move || {
                *calls.borrow_mut() += 1;
                if reject { Err(()) } else { Ok(()) }
            })
        } else {
            target
        }
    };
    let item = SanitizedCommandItem::new(
        target(b"direct-target-secret", direct_calls),
        0,
        "直接 日本語 ⭐️👩‍💻",
    )
    .enabled_state(enabled)
    .visible_state(visible);
    let item = if dropdown {
        item.dropdown_item(
            SanitizedCommandDropdownItem::new(
                target(b"dropdown-target-secret", dropdown_calls),
                0,
                "選択 日本語 ⭐️👩‍💻",
            )
            .enabled_state(enabled)
            .visible_state(visible),
        )
    } else {
        item
    };
    input(revision, b"command-document", "本文 日本語 ⭐️👩‍💻").with_command_projection(
        SanitizedCommandProjection::new([
            SanitizedCommandGroup::new(0, "操作 日本語 ⭐️👩‍💻").item(item)
        ]),
    )
}

fn floating_command_input(
    revision: u64,
    calls: Rc<RefCell<usize>>,
    enabled: bool,
    visible: bool,
    capability: bool,
    reject: bool,
) -> SanitizedDocumentRootInput {
    use crate::text_command_surface::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget,
    };

    let target = SanitizedCommandTarget::from_opaque_bytes(b"floating-target-secret".to_vec());
    let target = if capability {
        target.with_unit_capability(move || {
            *calls.borrow_mut() += 1;
            if reject { Err(()) } else { Ok(()) }
        })
    } else {
        target
    };
    let projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(0, "浮遊操作 日本語 ⭐️")
            .item(
                SanitizedCommandItem::new(target, 0, "太字 日本語 ⭐️")
                    .enabled_state(enabled)
                    .visible_state(visible),
            )]);
    input(revision, b"floating-document", "本文 日本語 ⭐️👩‍💻")
        .with_floating_command_projection(projection)
}

fn command_node(output: &egui::FullOutput, label: &str) -> (egui::accesskit::NodeId, egui::Rect) {
    accesskit_node(output, label, egui::accesskit::Role::Button)
}

fn context_menu_node(
    output: &egui::FullOutput,
    label: &str,
) -> (egui::accesskit::NodeId, egui::Rect) {
    accesskit_node(output, label, egui::accesskit::Role::MenuItem)
}

fn accesskit_node(
    output: &egui::FullOutput,
    label: &str,
    role: egui::accesskit::Role,
) -> (egui::accesskit::NodeId, egui::Rect) {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(node_id, node)| {
                (node.role() == role && node.label() == Some(label))
                    .then(|| {
                        node.bounds().map(|bounds| {
                            (
                                *node_id,
                                egui::Rect::from_min_max(
                                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                                ),
                            )
                        })
                    })
                    .flatten()
            })
        })
        .unwrap_or_else(|| {
            let labels = output
                .platform_output
                .accesskit_update
                .as_ref()
                .map(|update| {
                    update
                        .nodes
                        .iter()
                        .filter_map(|(_, node)| node.label().map(str::to_owned))
                        .collect::<Vec<_>>()
                });
            panic!("AccessKit node `{label}` is absent; labels={labels:?}")
        })
}

fn run_command_root_frame(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, super::SanitizedDocumentRootFrame) {
    let (output, frame) = run_command_root_frame_result(context, root, events);
    (output, frame.expect("command frame exists"))
}

fn run_command_root_frame_result(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> (
    egui::FullOutput,
    Result<super::SanitizedDocumentRootFrame, SanitizedDocumentRootFactoryError>,
) {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui));
            });
        },
    );
    output.textures_delta.clear();
    (output, frame.expect("command frame result exists"))
}

fn assert_command_forwarded_once(
    frame: &super::SanitizedDocumentRootFrame,
    calls: &Rc<RefCell<usize>>,
    forwarder: &mut RecordingForwarder,
) {
    assert_eq!(frame.output.events().event_cardinality(), 0);
    assert_eq!(
        frame.command_events.borrow().as_ref().map_or(0, Vec::len),
        1
    );
    let receipt = frame
        .forward_events_once(forwarder)
        .expect("command forwards");
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);
    for (name, debug) in [
        ("frame", format!("{frame:?}")),
        (
            "transport",
            forwarder.transport_debug.clone().expect("transport debug"),
        ),
        ("receipt", format!("{receipt:?}")),
    ] {
        for forbidden in [
            "直接 日本語",
            "選択 日本語",
            "⭐️",
            "👩‍💻",
            "direct-target-secret",
            "dropdown-target-secret",
        ] {
            assert!(
                !debug.contains(forbidden),
                "{name} leaked `{forbidden}`: {debug}"
            );
        }
    }
    assert_eq!(
        frame.forward_events_once(forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(forwarder.calls, 1);
}

#[test]
fn physical_accesskit_direct_command_forwards_one_opaque_activation() {
    let calls = Rc::new(RefCell::new(0));
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(command_input(
            1,
            calls.clone(),
            true,
            true,
            true,
            false,
            false,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
    let (node, _) = command_node(&output, "直接 日本語 ⭐️👩‍💻");
    let (_, frame) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    assert_command_forwarded_once(&frame, &calls, &mut forwarder);
}

#[test]
fn physical_split_command_primary_and_secondary_are_distinct_one_shot_targets() {
    let direct_calls = Rc::new(RefCell::new(0));
    let dropdown_calls = Rc::new(RefCell::new(0));
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(command_input_with_callbacks(
            1,
            direct_calls.clone(),
            dropdown_calls.clone(),
            true,
            true,
            true,
            true,
            false,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (_, initial_frame) = run_command_root_frame(&context, &mut root, Vec::new());
    let (primary_bounds, secondary_bounds) = initial_frame
        .command_action_rects()
        .first()
        .copied()
        .expect("command chrome action bounds");

    let primary_point = egui::pos2(
        primary_bounds.x as f32 + primary_bounds.width as f32 / 2.0,
        primary_bounds.y as f32 + primary_bounds.height as f32 / 2.0,
    );
    let (_, _) = run_command_root_frame(
        &context,
        &mut root,
        vec![pointer_button(primary_point, true)],
    );
    let (_, direct_frame) = run_command_root_frame(
        &context,
        &mut root,
        vec![pointer_button(primary_point, false)],
    );
    let mut direct_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    assert_command_forwarded_once(&direct_frame, &direct_calls, &mut direct_forwarder);
    assert_eq!(*dropdown_calls.borrow(), 0);

    let secondary_bounds = secondary_bounds.expect("split command secondary bounds");
    let secondary_point = egui::pos2(
        secondary_bounds.x as f32 + secondary_bounds.width as f32 / 2.0,
        secondary_bounds.y as f32 + secondary_bounds.height as f32 / 2.0,
    );
    let (_, opened) = run_command_root_frame(
        &context,
        &mut root,
        vec![pointer_button(secondary_point, true)],
    );
    let (_, opened_release) = run_command_root_frame(
        &context,
        &mut root,
        vec![pointer_button(secondary_point, false)],
    );
    assert_eq!(
        opened.command_events.borrow().as_ref().map_or(0, Vec::len),
        0
    );
    assert_eq!(
        opened_release
            .command_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        0
    );
    let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
    let (node, _) = command_node(&output, "選択 日本語 ⭐️👩‍💻");
    let (_, frame) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
    let mut dropdown_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    assert_command_forwarded_once(&frame, &dropdown_calls, &mut dropdown_forwarder);
    assert_eq!(*direct_calls.borrow(), 1);
}

#[test]
fn command_keyboard_activation_uses_root_raw_input_and_one_shot_transport() {
    let calls = Rc::new(RefCell::new(0));
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(command_input(
            1,
            calls.clone(),
            true,
            true,
            true,
            true,
            false,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (_, initial_frame) = run_command_root_frame(&context, &mut root, Vec::new());
    let secondary_bounds = initial_frame
        .command_action_rects()
        .first()
        .and_then(|(_, bounds)| *bounds)
        .expect("split command secondary bounds");
    let secondary_point = egui::pos2(
        secondary_bounds.x as f32 + secondary_bounds.width as f32 / 2.0,
        secondary_bounds.y as f32 + secondary_bounds.height as f32 / 2.0,
    );
    let _ = run_command_root_frame(
        &context,
        &mut root,
        vec![pointer_button(secondary_point, true)],
    );
    let _ = run_command_root_frame(
        &context,
        &mut root,
        vec![pointer_button(secondary_point, false)],
    );
    let _ = run_command_root_frame(&context, &mut root, vec![key_press(egui::Key::ArrowDown)]);
    let (_, frame) = run_command_root_frame(&context, &mut root, vec![key_press(egui::Key::Enter)]);
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    assert_command_forwarded_once(&frame, &calls, &mut forwarder);
}

#[test]
fn command_disabled_hidden_and_missing_capability_never_forward_callbacks() {
    for (enabled, visible, capability) in [
        (false, true, true),
        (true, false, true),
        (true, true, false),
    ] {
        let calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(command_input(
                1,
                calls.clone(),
                enabled,
                visible,
                capability,
                false,
                false,
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        context.enable_accesskit();
        let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
        if !visible {
            assert!(
                output
                    .platform_output
                    .accesskit_update
                    .as_ref()
                    .is_none_or(|update| {
                        !update
                            .nodes
                            .iter()
                            .any(|(_, node)| node.label() == Some("直接 日本語 ⭐️👩‍💻"))
                    })
            );
            continue;
        }
        let (node, _) = command_node(&output, "直接 日本語 ⭐️👩‍💻");
        if !capability {
            let (_, result) =
                run_command_root_frame_result(&context, &mut root, vec![accesskit_click(node)]);
            assert!(matches!(
                result,
                Err(SanitizedDocumentRootFactoryError::CommandCapability(
                    SanitizedCommandCapabilityRejection::Missing,
                ))
            ));
            assert_eq!(*calls.borrow(), 0);
            continue;
        }
        let (_, frame) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        if !enabled {
            assert_eq!(
                frame.command_events.borrow().as_ref().map_or(0, Vec::len),
                0
            );
            continue;
        }
        let result = frame.forward_events_once(&mut forwarder);
        assert!(result.is_ok());
        assert_eq!(*calls.borrow(), 0);
        assert_eq!(forwarder.calls, 0);
    }
}

#[test]
fn command_callback_rejection_is_typed_opaque_and_consumed() {
    let calls = Rc::new(RefCell::new(0));
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(command_input(
            1,
            calls.clone(),
            true,
            true,
            true,
            false,
            true,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
    let (node, _) = command_node(&output, "直接 日本語 ⭐️👩‍💻");
    let (_, frame) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
    let mut forwarder = RetainingForwarder {
        calls: 0,
        transport_debug: None,
        transport: None,
    };
    let result = frame.forward_events_once(&mut forwarder);
    assert!(result.is_ok());
    assert_eq!(*calls.borrow(), 0);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(
        forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect)
    );
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(
        forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
    );
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
}

#[test]
fn newer_revision_rejects_physical_command_frame_as_stale_before_callback() {
    let calls = Rc::new(RefCell::new(0));
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(command_input(
            1,
            calls.clone(),
            true,
            true,
            true,
            false,
            false,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
    let (node, _) = command_node(&output, "直接 日本語 ⭐️👩‍💻");
    let (_, frame) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
    root.synchronize(command_input(
        2,
        calls.clone(),
        true,
        true,
        true,
        false,
        false,
    ))
    .expect("new revision synchronizes");
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
    assert_eq!(*calls.borrow(), 0);
    assert_eq!(forwarder.calls, 0);
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
}

fn run_root_frame(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    event: egui::Event,
) -> super::SanitizedDocumentRootFrame {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events: vec![event],
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    output.textures_delta.clear();
    frame.expect("frame exists")
}

fn run_root_frame_events(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, super::SanitizedDocumentRootFrame) {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    output.textures_delta.clear();
    (output, frame.expect("frame exists"))
}

fn select_floating_surface(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
) -> (egui::FullOutput, super::SanitizedDocumentRootFrame) {
    let (_, initial) = run_root_frame_events(
        context,
        root,
        vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
    );
    let content_bounds = initial.output.evidence_text.record.frame.content_bounds;
    let start = egui::pos2(
        content_bounds.x as f32 + 8.0,
        content_bounds.y as f32 + content_bounds.height as f32 / 2.0,
    );
    let midpoint = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 / 2.0,
        start.y,
    );
    let end = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 - 8.0,
        start.y,
    );
    let _ = run_root_frame_events(context, root, vec![pointer_button(start, true)]);
    let _ = run_root_frame_events(context, root, vec![egui::Event::PointerMoved(midpoint)]);
    let _ = run_root_frame_events(context, root, vec![egui::Event::PointerMoved(end)]);
    run_root_frame_events(context, root, vec![pointer_button(end, false)])
}

fn run_root_frame_result(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> Result<super::SanitizedDocumentRootFrame, SanitizedDocumentRootFactoryError> {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui));
            });
        },
    );
    output.textures_delta.clear();
    frame.expect("frame exists")
}

fn secondary_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Secondary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

fn context_input(
    revision: u64,
    calls: Rc<RefCell<usize>>,
    nested: bool,
    enabled: bool,
    visible: bool,
    capability: bool,
    reject: bool,
) -> SanitizedDocumentRootInput {
    let target = |bytes: &[u8], calls: Rc<RefCell<usize>>| {
        let target = SanitizedContextMenuTarget::from_opaque_bytes(bytes.to_vec());
        if capability {
            target.with_unit_capability(move || {
                *calls.borrow_mut() += 1;
                if reject { Err(()) } else { Ok(()) }
            })
        } else {
            target
        }
    };
    let leaf = SanitizedContextMenuItem::new(
        target(b"context-leaf-secret", calls.clone()),
        0,
        "葉 日本語 ⭐️👩‍💻",
    )
    .enabled_state(enabled);
    let item = if nested {
        SanitizedContextMenuItem::new(
            SanitizedContextMenuTarget::from_opaque_bytes(b"submenu-secret".to_vec()),
            0,
            "親 日本語 ⭐️👩‍💻",
        )
        .submenu_item(leaf)
    } else {
        leaf
    };
    let projection = SanitizedContextMenuProjectionBuilder::new()
        .item(item)
        .build();
    let projection = if visible {
        projection
    } else {
        SanitizedContextMenuProjection::default()
    };
    input(revision, b"context-document", "本文 日本語 ⭐️👩‍💻").with_context_projection(projection)
}

#[test]
fn physical_context_pointer_nested_keyboard_and_accesskit_are_one_shot() {
    for route in ["pointer", "nested", "keyboard", "accesskit"] {
        let calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(context_input(
                1,
                calls.clone(),
                route == "nested",
                true,
                true,
                true,
                false,
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        context.enable_accesskit();
        let (initial_output, initial) = run_root_frame_events(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
        );
        let viewport = initial.output.evidence_text.record.frame.viewport_bounds;
        let point = egui::pos2(viewport.x as f32 + 20.0, viewport.y as f32 + 12.0);
        let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(point)]);
        let _ = run_root_frame_events(&context, &mut root, vec![secondary_button(point, true)]);
        let (opened_output, opened) =
            run_root_frame_events(&context, &mut root, vec![secondary_button(point, false)]);
        let frame = if route == "nested" {
            let item = opened
                .output
                .context_menu_record
                .as_ref()
                .expect("opened context menu record")
                .items
                .first()
                .expect("submenu item bounds");
            let p = egui::pos2(
                item.bounds.x as f32 + item.bounds.width as f32 / 2.0,
                item.bounds.y as f32 + item.bounds.height as f32 / 2.0,
            );
            let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(p)]);
            let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(p, true)]);
            let (_, submenu) =
                run_root_frame_events(&context, &mut root, vec![pointer_button(p, false)]);
            let child = submenu
                .output
                .context_menu_record
                .as_ref()
                .expect("submenu context menu record")
                .items
                .first()
                .expect("submenu leaf bounds");
            let cp = egui::pos2(
                child.bounds.x as f32 + child.bounds.width as f32 / 2.0,
                child.bounds.y as f32 + child.bounds.height as f32 / 2.0,
            );
            let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(cp)]);
            let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(cp, true)]);
            run_root_frame_events(&context, &mut root, vec![pointer_button(cp, false)]).1
        } else if route == "keyboard" {
            let _ =
                run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::ArrowDown)]);
            run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::Enter)]).1
        } else if route == "accesskit" {
            let node = context_menu_node(&opened_output, "葉 日本語 ⭐️👩‍💻").0;
            run_root_frame_events(
                &context,
                &mut root,
                vec![egui::Event::AccessKitActionRequest(
                    egui::accesskit::ActionRequest {
                        action: egui::accesskit::Action::Click,
                        target_tree: egui::accesskit::TreeId::ROOT,
                        target_node: node,
                        data: None,
                    },
                )],
            )
            .1
        } else {
            let item = opened
                .output
                .context_menu_record
                .as_ref()
                .expect("opened context menu record")
                .items
                .first()
                .expect("leaf item bounds");
            let p = egui::pos2(
                item.bounds.x as f32 + item.bounds.width as f32 / 2.0,
                item.bounds.y as f32 + item.bounds.height as f32 / 2.0,
            );
            let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(p)]);
            let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(p, true)]);
            run_root_frame_events(&context, &mut root, vec![pointer_button(p, false)]).1
        };
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        let context_event_count = frame
            .context_menu_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len);
        let receipt = frame
            .forward_events_once(&mut forwarder)
            .expect("context forwarding succeeds");
        assert_eq!(
            *calls.borrow(),
            1,
            "route={route} context_events={context_event_count}"
        );
        assert_eq!(receipt.event_cardinality(), 1);
        assert_eq!(forwarder.calls, 1);
        assert_eq!(frame.output.events().event_cardinality(), 0);
        assert_eq!(
            frame.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
        );
        assert_eq!(forwarder.calls, 1);
        assert!(!format!("{frame:?}").contains("context-leaf-secret"));
        assert!(!forwarder.transport_debug.unwrap().contains("葉 日本語"));
        let _ = initial_output;
    }
}

#[test]
fn physical_text_selection_controls_floating_surface_lifecycle() {
    use crate::text_command_surface::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget,
    };

    let top_calls = Rc::new(RefCell::new(0));
    let floating_calls = Rc::new(RefCell::new(0));
    let top_projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "トップ操作 日本語 ⭐️")
            .item(SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes(b"top-target-secret")
                    .with_unit_capability({
                        let calls = top_calls.clone();
                        move || {
                            *calls.borrow_mut() += 1;
                            Ok::<(), ()>(())
                        }
                    }),
                0,
                "トップ 日本語 ⭐️",
            ))]);
    let floating_projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(0, "選択操作 日本語 ⭐️")
            .item(SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes(b"floating-target-secret")
                    .with_unit_capability({
                        let calls = floating_calls.clone();
                        move || {
                            *calls.borrow_mut() += 1;
                            Ok::<(), ()>(())
                        }
                    }),
                0,
                "太字 日本語 ⭐️",
            ))]);
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(
            input(1, b"floating-document", "本文 日本語 ⭐️👩‍💻")
                .with_command_projection(top_projection)
                .with_floating_command_projection(floating_projection),
        )
        .expect("retain succeeds");
    let context = egui::Context::default();

    let (_, initial) = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
    );
    assert!(
        initial
            .output
            .evidence_text
            .record
            .frame
            .selection
            .range
            .is_collapsed()
    );
    assert!(
        initial
            .output
            .floating
            .as_ref()
            .is_none_or(|value| value.record.is_none())
    );

    let text_frame = &initial.output.evidence_text.record.frame;
    let content_bounds = text_frame.content_bounds;
    assert!(content_bounds.width > 24);
    let start = egui::pos2(
        content_bounds.x as f32 + 8.0,
        content_bounds.y as f32 + content_bounds.height as f32 / 2.0,
    );
    let midpoint = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 / 2.0,
        start.y,
    );
    let end = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 - 8.0,
        start.y,
    );

    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, true)]);
    let (_, dragging) = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(midpoint)],
    );
    let (_, _selected) =
        run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(end)]);
    let (_, released) =
        run_root_frame_events(&context, &mut root, vec![pointer_button(end, false)]);
    let selection = &released.output.evidence_text.record.frame.selection.range;
    let ordered_selection = selection.ordered();
    assert!(ordered_selection.start < ordered_selection.end);
    assert!(
        dragging
            .output
            .evidence_text
            .events
            .iter()
            .any(|event| matches!(
                event,
                katana_ui_core::text_surface::TextSurfaceEvent::SelectionChanged { .. }
            ))
    );
    assert!(
        released
            .output
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_some()
    );

    let escaped = run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::Escape)]).1;
    assert!(
        escaped
            .output
            .floating
            .as_ref()
            .is_none_or(|value| value.record.is_none())
    );
    let escaped_next = run_root_frame_events(&context, &mut root, Vec::new()).1;
    assert!(
        escaped_next
            .output
            .evidence_text
            .record
            .frame
            .accessibility
            .root
            .focused
    );

    let collapsed =
        run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::ArrowRight)]).1;
    let collapsed_selection = &collapsed.output.evidence_text.record.frame.selection.range;
    assert!(collapsed_selection.is_collapsed());
    assert!(
        collapsed
            .output
            .floating
            .as_ref()
            .is_none_or(|value| value.record.is_none())
    );

    let text_frame = &collapsed.output.evidence_text.record.frame;
    let content_bounds = text_frame.content_bounds;
    let start = egui::pos2(
        content_bounds.x as f32 + 8.0,
        content_bounds.y as f32 + content_bounds.height as f32 / 2.0,
    );
    let midpoint = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 / 2.0,
        start.y,
    );
    let end = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 - 8.0,
        start.y,
    );
    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, true)]);
    let _ = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(midpoint)],
    );
    let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(end)]);
    let (_, reselected) =
        run_root_frame_events(&context, &mut root, vec![pointer_button(end, false)]);
    let floating = reselected
        .output
        .floating
        .as_ref()
        .and_then(|value| value.record.as_ref())
        .expect("reselection opens floating surface");
    let floating_action = reselected
        .floating_action_rects()
        .first()
        .copied()
        .expect("floating action bounds are available only in the test frame");
    let floating_point = egui::pos2(
        floating_action.x as f32 + floating_action.width as f32 / 2.0,
        floating_action.y as f32 + floating_action.height as f32 / 2.0,
    );
    let _ = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(floating_point)],
    );
    let _ = run_root_frame_events(
        &context,
        &mut root,
        vec![pointer_button(floating_point, true)],
    );
    let floating_clicked = run_root_frame_events(
        &context,
        &mut root,
        vec![pointer_button(floating_point, false)],
    )
    .1;
    let mut floating_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    assert_command_forwarded_once(&floating_clicked, &floating_calls, &mut floating_forwarder);
    assert_eq!(*top_calls.borrow(), 0);
    assert_eq!(*floating_calls.borrow(), 1);
    let public_record_debug = format!("{:?}", floating_clicked.record());
    for forbidden in ["panel_bounds", "actions", "floating-target-secret"] {
        assert!(!public_record_debug.contains(forbidden));
    }
    for forbidden in [
        "floating-target-secret",
        "top-target-secret",
        "太字 日本語",
        "トップ 日本語",
        "選択操作 日本語",
    ] {
        assert!(!format!("{floating_clicked:?}").contains(forbidden));
        assert!(
            !floating_forwarder
                .transport_debug
                .as_deref()
                .expect("transport debug")
                .contains(forbidden)
        );
    }
    let text_bounds = reselected.output.evidence_text.record.frame.content_bounds;
    let outside = egui::pos2(
        text_bounds.x.saturating_sub(1) as f32,
        text_bounds.y.saturating_sub(1) as f32,
    );
    assert!((outside.x.round() as i32) < text_bounds.x);
    assert!((outside.y.round() as i32) < text_bounds.y);
    assert!(
        (outside.x.round() as i32) < floating.panel_bounds.x
            || (outside.x.round() as i32)
                >= floating
                    .panel_bounds
                    .x
                    .saturating_add_unsigned(floating.panel_bounds.width)
    );
    assert!(
        (outside.y.round() as i32) < floating.panel_bounds.y
            || (outside.y.round() as i32)
                >= floating
                    .panel_bounds
                    .y
                    .saturating_add_unsigned(floating.panel_bounds.height)
    );

    let outside_dismissed =
        run_root_frame_events(&context, &mut root, vec![pointer_button(outside, true)]).1;
    assert!(
        outside_dismissed
            .output
            .floating
            .as_ref()
            .is_none_or(|value| value.record.is_none())
    );
    let outside_dismissed_next = run_root_frame_events(&context, &mut root, Vec::new()).1;
    assert!(
        outside_dismissed_next
            .output
            .evidence_text
            .record
            .frame
            .accessibility
            .root
            .focused
    );
}

#[test]
fn physical_floating_keyboard_activation_routes_only_to_floating_target_once() {
    use crate::text_command_surface::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget,
    };

    let top_calls = Rc::new(RefCell::new(0));
    let floating_calls = Rc::new(RefCell::new(0));
    let mut input = command_input_with_callbacks(
        1,
        top_calls.clone(),
        Rc::new(RefCell::new(0)),
        false,
        true,
        true,
        false,
        false,
    );
    let floating_target =
        SanitizedCommandTarget::from_opaque_bytes(b"floating-keyboard-target-secret".to_vec())
            .with_unit_capability({
                let calls = floating_calls.clone();
                move || {
                    *calls.borrow_mut() += 1;
                    Ok::<(), ()>(())
                }
            });
    input = input.with_floating_command_projection(SanitizedCommandProjection::new([
        SanitizedCommandGroup::new(0, "浮遊操作 日本語 ⭐️").item(SanitizedCommandItem::new(
            floating_target,
            0,
            "太字 日本語 ⭐️",
        )),
    ]));

    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input)
        .expect("retain succeeds");
    let context = egui::Context::default();
    let (_, selected) = select_floating_surface(&context, &mut root);
    assert!(
        selected
            .output
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_some()
    );
    assert_eq!(
        selected
            .command_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        0,
        "selection release must not activate floating command"
    );

    let mut focused = None;
    for _ in 0..8 {
        let _ = run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::Tab)]);
        let candidate =
            run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::ArrowRight)]).1;
        assert_eq!(
            candidate
                .command_events
                .borrow()
                .as_ref()
                .map_or(0, Vec::len),
            0,
            "focus movement must not activate a command"
        );
        if candidate
            .output
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .and_then(|value| value.toolbar.focused_action_id.as_ref())
            .is_some()
        {
            focused = Some(candidate);
            break;
        }
    }
    assert!(
        focused.is_some(),
        "raw keyboard input must focus floating action"
    );

    let activated = run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::Enter)]).1;
    assert_eq!(activated.output.events().event_cardinality(), 0);
    let command_event_count = activated
        .command_events
        .borrow()
        .as_ref()
        .map_or(0, Vec::len);
    let floating_event_debug = activated
        .output
        .floating
        .as_ref()
        .map(|value| format!("{:?}", value.events));
    assert_eq!(
        command_event_count, 1,
        "command_event_count={command_event_count} floating_events={floating_event_debug:?}"
    );
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = activated
        .forward_events_once(&mut forwarder)
        .expect("floating keyboard activation forwards");
    assert_eq!(*floating_calls.borrow(), 1);
    assert_eq!(*top_calls.borrow(), 0);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(
        activated.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    for forbidden in [
        "floating-keyboard-target-secret",
        "浮遊操作 日本語",
        "太字 日本語",
        "panel_bounds",
        "actions",
    ] {
        assert!(!format!("{activated:?}").contains(forbidden));
        assert!(!format!("{receipt:?}").contains(forbidden));
        assert!(
            !forwarder
                .transport_debug
                .as_deref()
                .expect("transport debug")
                .contains(forbidden)
        );
    }
}

#[test]
fn physical_floating_accesskit_snapshot_click_routes_one_opaque_target() {
    use crate::text_command_surface::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget,
    };

    let calls = Rc::new(RefCell::new(0));
    let projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(0, "浮遊操作 日本語 ⭐️")
            .item(SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes(
                    b"floating-accesskit-target-secret".to_vec(),
                )
                .with_unit_capability({
                    let calls = calls.clone();
                    move || {
                        *calls.borrow_mut() += 1;
                        Ok::<(), ()>(())
                    }
                }),
                0,
                "太字 日本語 ⭐️",
            ))]);
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(
            input(1, b"floating-accesskit-document", "本文 日本語 ⭐️👩‍💻")
                .with_floating_command_projection(projection),
        )
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, selected) = select_floating_surface(&context, &mut root);
    let floating = selected
        .output
        .floating
        .as_ref()
        .and_then(|value| value.record.as_ref())
        .expect("selection opens floating");
    let action_bounds = floating
        .toolbar
        .actions
        .first()
        .expect("floating action record")
        .bounds;
    let (node, _) = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("current AccessKit snapshot")
        .nodes
        .iter()
        .find_map(|(node_id, node)| {
            let bounds = node.bounds()?;
            let matches_action = node.role() == egui::accesskit::Role::Button
                && bounds.x0 as i32 == action_bounds.x
                && bounds.y0 as i32 == action_bounds.y
                && bounds.x1 as i32 == action_bounds.x.saturating_add_unsigned(action_bounds.width)
                && bounds.y1 as i32
                    == action_bounds
                        .y
                        .saturating_add_unsigned(action_bounds.height);
            matches_action.then_some((*node_id, bounds))
        })
        .expect("floating action node from current snapshot");

    let (_, activated) = run_root_frame_events(&context, &mut root, vec![accesskit_click(node)]);
    assert_eq!(activated.output.events().event_cardinality(), 0);
    assert_eq!(
        activated
            .command_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        1
    );
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    let receipt = activated
        .forward_events_once(&mut forwarder)
        .expect("AccessKit floating activation forwards");
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(
        activated.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    for forbidden in [
        "floating-accesskit-target-secret",
        "浮遊操作 日本語",
        "太字 日本語",
        "panel_bounds",
        "actions",
    ] {
        assert!(!format!("{activated:?}").contains(forbidden));
        assert!(!format!("{receipt:?}").contains(forbidden));
        assert!(
            !forwarder
                .transport_debug
                .as_deref()
                .expect("transport debug")
                .contains(forbidden)
        );
    }
}

#[test]
fn physical_floating_failure_matrix_is_strict_and_stale_safe() {
    let cases = [
        ("disabled", false, true, true, false),
        ("hidden", true, false, true, false),
    ];
    for (name, enabled, visible, capability, reject) in cases {
        let calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(floating_command_input(
                1,
                calls.clone(),
                enabled,
                visible,
                capability,
                reject,
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        let (_, selected) = select_floating_surface(&context, &mut root);
        assert_eq!(
            selected
                .command_events
                .borrow()
                .as_ref()
                .map_or(0, Vec::len),
            0,
            "{name} selection event"
        );
        if name == "hidden" {
            assert!(selected.floating_action_rects().is_empty());
        } else {
            let action = selected
                .floating_action_rects()
                .first()
                .copied()
                .expect("disabled action remains physically represented");
            let point = egui::pos2(
                action.x as f32 + action.width as f32 / 2.0,
                action.y as f32 + action.height as f32 / 2.0,
            );
            let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(point, true)]);
            let (_, disabled_frame) =
                run_root_frame_events(&context, &mut root, vec![pointer_button(point, false)]);
            assert_eq!(
                disabled_frame
                    .command_events
                    .borrow()
                    .as_ref()
                    .map_or(0, Vec::len),
                0,
                "{name} activation event"
            );
            let mut forwarder = RecordingForwarder {
                calls: 0,
                transport_debug: None,
                reject_forwarding: false,
            };
            let receipt = disabled_frame
                .forward_events_once(&mut forwarder)
                .expect("disabled event batch forwards empty");
            assert_eq!(
                disabled_frame.output.events().event_cardinality(),
                0,
                "{name}: the floating overlay retains text focus"
            );
            assert!(disabled_frame.output.evidence_text.events.is_empty());
            assert_eq!(
                disabled_frame
                    .output
                    .floating
                    .as_ref()
                    .map_or(0, |value| value.events.len()),
                0
            );
            assert_eq!(receipt.event_cardinality(), 0);
            assert_eq!(forwarder.calls, 1);
            assert_eq!(*calls.borrow(), 0);
            assert_eq!(
                disabled_frame.forward_events_once(&mut forwarder),
                Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
            );
        }
    }

    let missing_calls = Rc::new(RefCell::new(0));
    let mut missing_root = SanitizedDocumentRootFactory::new()
        .retain(floating_command_input(
            1,
            missing_calls.clone(),
            true,
            true,
            false,
            false,
        ))
        .expect("retain succeeds");
    let missing_context = egui::Context::default();
    let (_, missing_selected) = select_floating_surface(&missing_context, &mut missing_root);
    let missing_action = missing_selected
        .floating_action_rects()
        .first()
        .copied()
        .expect("missing capability action bounds");
    let missing_point = egui::pos2(
        missing_action.x as f32 + missing_action.width as f32 / 2.0,
        missing_action.y as f32 + missing_action.height as f32 / 2.0,
    );
    let _ = run_root_frame_events(
        &missing_context,
        &mut missing_root,
        vec![pointer_button(missing_point, true)],
    );
    let missing_result = run_root_frame_result(
        &missing_context,
        &mut missing_root,
        vec![pointer_button(missing_point, false)],
    );
    assert!(matches!(
        missing_result,
        Err(SanitizedDocumentRootFactoryError::CommandCapability(
            SanitizedCommandCapabilityRejection::Missing
        ))
    ));
    assert_eq!(*missing_calls.borrow(), 0);

    let rejection_calls = Rc::new(RefCell::new(0));
    let mut rejection_root = SanitizedDocumentRootFactory::new()
        .retain(floating_command_input(
            1,
            rejection_calls.clone(),
            true,
            true,
            true,
            true,
        ))
        .expect("retain succeeds");
    let rejection_context = egui::Context::default();
    let (_, rejection_selected) = select_floating_surface(&rejection_context, &mut rejection_root);
    let rejection_action = rejection_selected
        .floating_action_rects()
        .first()
        .copied()
        .expect("rejecting action bounds");
    let rejection_point = egui::pos2(
        rejection_action.x as f32 + rejection_action.width as f32 / 2.0,
        rejection_action.y as f32 + rejection_action.height as f32 / 2.0,
    );
    let _ = run_root_frame_events(
        &rejection_context,
        &mut rejection_root,
        vec![pointer_button(rejection_point, true)],
    );
    let rejection_frame = run_root_frame_events(
        &rejection_context,
        &mut rejection_root,
        vec![pointer_button(rejection_point, false)],
    )
    .1;
    let mut rejection_forwarder = RetainingForwarder {
        calls: 0,
        transport_debug: None,
        transport: None,
    };
    assert!(
        rejection_frame
            .forward_events_once(&mut rejection_forwarder)
            .is_ok()
    );
    assert_eq!(*rejection_calls.borrow(), 0);
    assert_eq!(rejection_forwarder.calls, 1);
    assert_eq!(
        rejection_forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect)
    );
    assert_eq!(*rejection_calls.borrow(), 1);
    assert_eq!(
        rejection_forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
    );
    assert_eq!(
        rejection_frame.forward_events_once(&mut rejection_forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );

    let stale_calls = Rc::new(RefCell::new(0));
    let mut stale_root = SanitizedDocumentRootFactory::new()
        .retain(floating_command_input(
            1,
            stale_calls.clone(),
            true,
            true,
            true,
            false,
        ))
        .expect("retain succeeds");
    let stale_context = egui::Context::default();
    let (_, stale_selected) = select_floating_surface(&stale_context, &mut stale_root);
    let stale_action = stale_selected
        .floating_action_rects()
        .first()
        .copied()
        .expect("stale action bounds");
    let stale_point = egui::pos2(
        stale_action.x as f32 + stale_action.width as f32 / 2.0,
        stale_action.y as f32 + stale_action.height as f32 / 2.0,
    );
    let _ = run_root_frame_events(
        &stale_context,
        &mut stale_root,
        vec![pointer_button(stale_point, true)],
    );
    let stale_frame = run_root_frame_events(
        &stale_context,
        &mut stale_root,
        vec![pointer_button(stale_point, false)],
    )
    .1;
    assert!(
        stale_root
            .synchronize(floating_command_input(
                2,
                Rc::new(RefCell::new(0)),
                true,
                true,
                true,
                false,
            ))
            .expect("newer same-identity projection synchronizes")
    );
    let mut stale_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
        reject_forwarding: false,
    };
    assert_eq!(
        stale_frame.forward_events_once(&mut stale_forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
    assert_eq!(*stale_calls.borrow(), 0);
    assert_eq!(stale_forwarder.calls, 0);
    assert_eq!(
        stale_frame.forward_events_once(&mut stale_forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
}

fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

fn key_press(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}

#[test]
fn context_menu_failure_matrix_is_strict_and_opaque() {
    let cases = [
        ("disabled", false, true, true, false),
        ("invisible", true, false, true, false),
    ];
    for (name, enabled, visible, capability, reject) in cases {
        let calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(context_input(
                1,
                calls.clone(),
                false,
                enabled,
                visible,
                capability,
                reject,
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        let (_, initial) = run_root_frame_events(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
        );
        let viewport = initial.output.evidence_text.record.frame.viewport_bounds;
        let point = egui::pos2(viewport.x as f32 + 20.0, viewport.y as f32 + 12.0);
        let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(point)]);
        let _ = run_root_frame_events(&context, &mut root, vec![secondary_button(point, true)]);
        let (_, frame) =
            run_root_frame_events(&context, &mut root, vec![secondary_button(point, false)]);
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        assert_eq!(
            frame
                .context_menu_events
                .borrow()
                .as_ref()
                .map_or(0, Vec::len),
            0,
            "{name} sanitized event"
        );
        let receipt = frame
            .forward_events_once(&mut forwarder)
            .expect("empty context menu batch forwards");
        assert_eq!(receipt.event_cardinality(), 1, "{name} root event batch");
        assert_eq!(forwarder.calls, 1, "{name} root batch forward");
        assert_eq!(*calls.borrow(), 0, "{name}");
    }
}

#[test]
fn context_menu_missing_and_callback_rejection_fail_at_root_frame() {
    for (capability, reject, expected) in [
        (
            false,
            false,
            SanitizedContextMenuCapabilityRejection::Missing,
        ),
        (
            true,
            true,
            SanitizedContextMenuCapabilityRejection::CallbackRejected,
        ),
    ] {
        let calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(context_input(
                1,
                calls.clone(),
                false,
                true,
                true,
                capability,
                reject,
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        let (_, initial) = run_root_frame_events(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
        );
        let viewport = initial.output.evidence_text.record.frame.viewport_bounds;
        let point = egui::pos2(viewport.x as f32 + 20.0, viewport.y as f32 + 12.0);
        let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(point)]);
        let _ = run_root_frame_events(&context, &mut root, vec![secondary_button(point, true)]);
        let (_, opened) =
            run_root_frame_events(&context, &mut root, vec![secondary_button(point, false)]);
        let item = opened
            .output
            .context_menu_record
            .as_ref()
            .unwrap()
            .items
            .first()
            .unwrap();
        let item_point = egui::pos2(
            item.bounds.x as f32 + item.bounds.width as f32 / 2.0,
            item.bounds.y as f32 + item.bounds.height as f32 / 2.0,
        );
        let _ = run_root_frame_events(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(item_point)],
        );
        let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(item_point, true)]);
        if !capability {
            let result =
                run_root_frame_result(&context, &mut root, vec![pointer_button(item_point, false)]);
            assert!(matches!(
                result,
                Err(SanitizedDocumentRootFactoryError::ContextMenuCapability(value))
                    if value == expected
            ));
            assert_eq!(*calls.borrow(), 0, "missing capability callback");
            continue;
        }
        let frame = match run_root_frame_result(
            &context,
            &mut root,
            vec![pointer_button(item_point, false)],
        ) {
            Ok(frame) => frame,
            Err(error) => panic!("callback rejection should defer to host dispatch: {error:?}"),
        };
        let mut forwarder = RetainingForwarder {
            calls: 0,
            transport_debug: None,
            transport: None,
        };
        assert!(frame.forward_events_once(&mut forwarder).is_ok());
        assert_eq!(*calls.borrow(), 0, "callback rejection pre-dispatch");
        assert_eq!(forwarder.calls, 1);
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect)
        );
        assert_eq!(*calls.borrow(), 1, "callback rejection host dispatch");
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
        );
        assert_eq!(
            frame.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
        );
    }
}

#[test]
fn context_menu_parent_submenu_debug_is_opaque() {
    let projection = SanitizedContextMenuProjectionBuilder::new()
        .item(
            SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes(b"parent-secret"),
                0,
                "親 日本語 ⭐️",
            )
            .submenu_item(SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes(b"child-secret"),
                0,
                "子 日本語 ⭐️",
            )),
        )
        .build();
    let debug = format!("{projection:?}");
    assert!(!debug.contains("親 日本語"));
    assert!(!debug.contains("parent-secret"));
}

#[test]
fn real_root_output_reports_each_already_detached_event_channel() {
    for channel in ["search", "command", "context menu"] {
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(input(1, b"detach-document", "本文 ⭐️"))
            .expect("retaining the real root succeeds");
        let context = egui::Context::default();
        let mut output = None;
        crate::run_ui_discard(
            &context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
                )),
                ..egui::RawInput::default()
            },
            |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    output = Some(root.process.show(ui).expect("real root output renders"));
                });
            },
        );
        let output = output.expect("real root output exists");

        match channel {
            "search" => {
                output
                    .events()
                    .detach_search_events_exclusively()
                    .expect("first search detach succeeds");
            }
            "command" => {
                output
                    .events()
                    .detach_command_events()
                    .expect("first command detach succeeds");
            }
            "context menu" => {
                output
                    .events()
                    .detach_context_menu_events()
                    .expect("first context-menu detach succeeds");
            }
            _ => unreachable!(),
        }

        let error = root
            .finish_output(output)
            .expect_err("the second detach must fail closed");
        let message = error.to_string();
        assert!(message.contains(channel), "unexpected error: {message}");
        assert!(
            message.contains("AlreadyDetached"),
            "unexpected error: {message}"
        );
    }
}

#[test]
fn real_invalid_theme_error_maps_to_the_public_render_error() {
    let mut theme = katana_ui_core::theme::ThemeSnapshot::dark();
    theme.colors.retain(|token| token.name != "accent");
    let style_error = crate::text_command_surface::TextCommandSurfaceStyle::from_theme(&theme)
        .expect_err("the actual style route rejects a missing accent token");
    let process_error =
        super::super::sanitized_document_root_process::SanitizedDocumentRootProcessError::Style(
            style_error.to_string(),
        );

    assert!(matches!(
        SanitizedDocumentRootFactoryError::from(process_error),
        SanitizedDocumentRootFactoryError::Render(message)
            if message.contains("accent")
    ));
}
