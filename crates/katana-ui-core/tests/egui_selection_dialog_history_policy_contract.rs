#![cfg(feature = "egui")]
use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootEventTransport, EguiTextCommandSurfaceRootFactory,
    FileSelectionProjection, KucDialogControls, KucDialogTarget, KucRootEventBatchDispatcher,
    KucRootEventBatchForwarder, SearchQueryArrowKeyPolicy, SearchQueryHistoryDirection,
    SearchResultMode, SearchResultProjection, SearchResultRow, SelectionDialogEvent,
    SelectionDialogOperation, SelectionDialogProjection, SelectionDialogProjectionLease,
    TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::selection::ContextMenuEvent;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
#[derive(Default)]
struct Host {
    operations: Vec<SelectionDialogOperation>,
    text_changes: Vec<String>,
}

impl KucRootEventBatchDispatcher for Host {
    type Error = String;
    fn dispatch_selection_dialog_events(
        &mut self,
        events: Vec<SelectionDialogEvent>,
    ) -> Result<(), String> {
        self.operations
            .extend(events.into_iter().map(|event| event.operation));
        Ok(())
    }
    fn dispatch_text_events(&mut self, events: Vec<TextSurfaceEvent>) -> Result<(), String> {
        self.text_changes
            .extend(events.into_iter().filter_map(|event| match event {
                TextSurfaceEvent::TextArea(katana_ui_core::atom::TextAreaEvent::Change(value)) => {
                    Some(value)
                }
                _ => None,
            }));
        Ok(())
    }
    fn dispatch_toolbar_events(&mut self, _: Vec<CommandChromeToolbarEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_floating_events(
        &mut self,
        _: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_search_events(&mut self, _: Vec<CommandChromeSearchEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_context_menu_events(&mut self, _: Vec<ContextMenuEvent>) -> Result<(), String> {
        Ok(())
    }
}

impl KucRootEventBatchForwarder for Host {
    type Error = String;
    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), String> {
        transport
            .dispatch_once(self)
            .map(|_| ())
            .map_err(|error| format!("{error:?}"))
    }
}

fn controls() -> KucDialogControls {
    KucDialogControls {
        cancel: "取消".into(),
        select: "選択".into(),
        search: "検索".into(),
        include: "含める".into(),
        exclude: "除外".into(),
        clear_history: "履歴消去".into(),
        delete_history: "履歴削除".into(),
        empty: "結果なし".into(),
    }
}

fn target(value: u8) -> KucDialogTarget {
    KucDialogTarget::from_opaque_bytes([value])
}

fn root() -> EguiTextCommandSurfaceHostRoot {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("dialog-body").value("本文⭐️"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 900, 640),
    ));
    let presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(UiStateId::new("dialog-body")),
        text: TextSurfacePresentation::from_props(surface.props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    };
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"dialog-root",
        presentation,
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("root")
}

fn frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
    host: &mut Host,
) -> (EguiTextCommandSurfaceHostRootFrame, egui::FullOutput) {
    context.enable_accesskit();
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 640.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            result = Some(root.show(ui).expect("show"));
        },
    );
    output.textures_delta.clear();
    let result = result.expect("frame");
    result.forward_events_once(host).expect("forward");
    (result, output)
}

fn key(key: egui::Key) -> Vec<egui::Event> {
    [true, false]
        .map(|pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        })
        .to_vec()
}

fn mode(value: u8) -> SearchResultMode {
    SearchResultMode {
        target: target(value),
        label: format!("mode{value}"),
        query: String::new(),
        include: Some(String::new()),
        exclude: Some(String::new()),
        options: Vec::new(),
        history: Vec::new(),
        results: Vec::new(),
        searched: false,
    }
}

fn ax_click(output: &egui::FullOutput, label: &str) -> Vec<egui::Event> {
    let update = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AX update");
    let (id, node) = update
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
        })
        .expect(label);
    assert!(!node.is_disabled(), "{label}");
    vec![egui::Event::AccessKitActionRequest(
        egui::accesskit::ActionRequest {
            target_tree: egui::accesskit::TreeId::ROOT,
            action: egui::accesskit::Action::Click,
            target_node: *id,
            data: None,
        },
    )]
}

fn ax_focus_text_input(output: &egui::FullOutput, label: &str) -> Vec<egui::Event> {
    let update = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AX update");
    let (id, node) = update
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::TextInput && node.label() == Some(label)
        })
        .expect(label);
    assert!(!node.is_disabled(), "{label}");
    vec![egui::Event::AccessKitActionRequest(
        egui::accesskit::ActionRequest {
            target_tree: egui::accesskit::TreeId::ROOT,
            action: egui::accesskit::Action::Focus,
            target_node: *id,
            data: None,
        },
    )]
}

fn row(id: u8, label: &str) -> SearchResultRow {
    SearchResultRow {
        target: target(id),
        label: label.into(),
        auxiliary_label: None,
        snippet: None,
        highlights: vec![],
        disabled: false,
    }
}

fn projection(query: &str) -> SelectionDialogProjection {
    let mut first = mode(1);
    first.query = query.into();
    first.history = vec![row(3, "最新"), row(4, "最古")];
    first.results = vec![row(5, "結果")];
    SelectionDialogProjection::Search(SearchResultProjection {
        modes: vec![first, mode(2)],
        active_mode: target(1),
    })
}

fn lease(revision: u64, projection: SelectionDialogProjection) -> SelectionDialogProjectionLease {
    SelectionDialogProjectionLease::new(revision, "検索", controls(), projection)
        .expect("lease")
        .query_arrow_key_policy(target(1), SearchQueryArrowKeyPolicy::History)
        .expect("policy")
}

#[test]
fn history_arrows_keep_query_focus_and_restore_exact_draft_once() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("下書き⭐️")))
        .expect("attach");
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    let (_, focus) = frame(&context, &mut root, vec![], &mut host);
    let ax = focus.platform_output.accesskit_update.as_ref().expect("AX");
    assert!(ax.nodes.iter().any(|(id, node)| *id == ax.focus
        && node.role() == egui::accesskit::Role::TextInput
        && node.label() == Some("mode1")));
    for key_code in [
        egui::Key::ArrowUp,
        egui::Key::ArrowUp,
        egui::Key::ArrowUp,
        egui::Key::ArrowDown,
        egui::Key::ArrowDown,
    ] {
        let (result, _) = frame(&context, &mut root, key(key_code), &mut host);
        assert!(result.forward_events_once(&mut host).is_err());
    }
    assert_eq!(
        host.operations,
        vec![
            SelectionDialogOperation::QueryHistoryNavigated {
                mode: target(1),
                target: Some(target(3)),
                value: "最新".into(),
                direction: SearchQueryHistoryDirection::Older
            },
            SelectionDialogOperation::QueryHistoryNavigated {
                mode: target(1),
                target: Some(target(4)),
                value: "最古".into(),
                direction: SearchQueryHistoryDirection::Older
            },
            SelectionDialogOperation::QueryHistoryNavigated {
                mode: target(1),
                target: Some(target(3)),
                value: "最新".into(),
                direction: SearchQueryHistoryDirection::Newer
            },
            SelectionDialogOperation::QueryHistoryNavigated {
                mode: target(1),
                target: None,
                value: "下書き⭐️".into(),
                direction: SearchQueryHistoryDirection::Newer
            },
        ]
    );
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    let ax = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AX");
    assert!(ax.nodes.iter().any(|(id, node)| *id == ax.focus
        && node.label() == Some("mode1")
        && node.value() == Some("下書き⭐️")));
}

#[test]
fn same_projection_preserves_cursor_and_stale_lease_cannot_reset_it() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("下書き⭐️")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    root.synchronize_selection_dialog(lease(2, projection("最新")))
        .unwrap();
    assert!(
        root.synchronize_selection_dialog(lease(2, projection("wrong")))
            .is_err()
    );
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert!(
        matches!(host.operations.last(), Some(SelectionDialogOperation::QueryHistoryNavigated { target: None, value, .. }) if value == "下書き⭐️")
    );
}

#[test]
fn ime_arrows_are_not_history_navigation() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    let mut events = vec![egui::Event::Ime(egui::ImeEvent::Preedit {
        text: "にほん".into(),
        active_range_chars: None,
    })];
    events.extend(key(egui::Key::ArrowUp));
    frame(&context, &mut root, events, &mut host);
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    let mut events = vec![egui::Event::Ime(egui::ImeEvent::Commit("日本".into()))];
    events.extend(key(egui::Key::ArrowUp));
    frame(&context, &mut root, events, &mut host);
    assert!(host.operations.iter().all(|op| !matches!(
        op,
        SelectionDialogOperation::QueryHistoryNavigated { .. }
            | SelectionDialogOperation::ResultSelected { .. }
            | SelectionDialogOperation::SearchRequested { .. }
    )));
}

#[test]
fn non_query_focus_and_clear_history_reset_traversal() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("下書き⭐️")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    frame(
        &context,
        &mut root,
        ax_focus_text_input(&output, "含める"),
        &mut host,
    );
    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    assert!(
        host.operations[before..]
            .iter()
            .all(|op| !matches!(op, SelectionDialogOperation::QueryHistoryNavigated { .. }))
    );
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    frame(
        &context,
        &mut root,
        ax_click(&output, "履歴消去"),
        &mut host,
    );
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    frame(
        &context,
        &mut root,
        ax_focus_text_input(&output, "mode1"),
        &mut host,
    );
    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    assert_eq!(host.operations.len(), before);
}

#[test]
fn host_override_and_close_reset_history_draft() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("元")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    root.synchronize_selection_dialog(lease(2, projection("変更")))
        .unwrap();
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert!(matches!(
        host.operations.last(),
        Some(SelectionDialogOperation::QueryHistoryNavigated {
            target: Some(_),
            ..
        })
    ));
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert!(
        matches!(host.operations.last(), Some(SelectionDialogOperation::QueryHistoryNavigated { target: None, value, .. }) if value == "変更")
    );
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    frame(&context, &mut root, key(egui::Key::Escape), &mut host);
    root.synchronize_selection_dialog(lease(3, projection("再開")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert_eq!(host.operations.len(), before);
}

#[test]
fn invalid_mode_policy_is_rejected_and_default_navigates_results() {
    assert!(
        SelectionDialogProjectionLease::new(1, "検索", controls(), projection(""))
            .unwrap()
            .query_arrow_key_policy(target(99), SearchQueryArrowKeyPolicy::History)
            .is_err()
    );
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(1, "検索", controls(), projection("")).unwrap(),
    )
    .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    assert_eq!(
        host.operations.last(),
        Some(&SelectionDialogOperation::ResultSelected {
            mode: target(1),
            target: target(5)
        })
    );
}

#[test]
fn deleting_history_and_switching_modes_discard_saved_draft() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("元")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    frame(
        &context,
        &mut root,
        ax_click(&output, "履歴削除"),
        &mut host,
    );
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    frame(
        &context,
        &mut root,
        ax_focus_text_input(&output, "mode1"),
        &mut host,
    );
    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert_eq!(host.operations.len(), before);
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    frame(&context, &mut root, ax_click(&output, "mode2"), &mut host);
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    frame(&context, &mut root, ax_click(&output, "mode1"), &mut host);
    for _ in 0..2 {
        frame(&context, &mut root, vec![], &mut host);
    }
    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert_eq!(host.operations.len(), before);
}

#[test]
fn equal_history_labels_and_draft_advance_cursor_without_duplicate_events() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let mut p = projection("同じ⭐️");
    if let SelectionDialogProjection::Search(search) = &mut p {
        search.modes[0].history = vec![row(3, "同じ⭐️"), row(4, "同じ⭐️"), row(6, "別")];
    }
    root.synchronize_selection_dialog(lease(1, p)).unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    for code in [
        egui::Key::ArrowUp,
        egui::Key::ArrowUp,
        egui::Key::ArrowUp,
        egui::Key::ArrowDown,
        egui::Key::ArrowDown,
        egui::Key::ArrowDown,
    ] {
        frame(&context, &mut root, key(code), &mut host);
    }
    assert_eq!(
        host.operations,
        vec![
            SelectionDialogOperation::QueryHistoryNavigated {
                mode: target(1),
                target: Some(target(6)),
                value: "別".into(),
                direction: SearchQueryHistoryDirection::Older
            },
            SelectionDialogOperation::QueryHistoryNavigated {
                mode: target(1),
                target: Some(target(4)),
                value: "同じ⭐️".into(),
                direction: SearchQueryHistoryDirection::Newer
            },
        ]
    );
}

#[test]
fn normal_query_edit_resets_traversal_and_becomes_next_draft() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("元")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("追記".into())],
        &mut host,
    );
    let edited = match host.operations.last() {
        Some(SelectionDialogOperation::QueryChanged { value, .. }) => value.clone(),
        other => panic!("expected ordinary query edit: {other:?}"),
    };
    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert_eq!(host.operations.len(), before);
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert!(
        matches!(host.operations.last(), Some(SelectionDialogOperation::QueryHistoryNavigated { target: None, value, .. }) if value == &edited)
    );
}

#[test]
fn disabled_history_is_skipped_and_changed_membership_resets_cursor() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let mut p = projection("元");
    if let SelectionDialogProjection::Search(search) = &mut p {
        search.modes[0].history[0].disabled = true;
    }
    root.synchronize_selection_dialog(lease(1, p.clone()))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    assert!(
        matches!(host.operations.last(), Some(SelectionDialogOperation::QueryHistoryNavigated { target: Some(id), .. }) if id == &target(4))
    );
    if let SelectionDialogProjection::Search(search) = &mut p {
        search.modes[0].query = "最古".into();
        search.modes[0].history[0].disabled = false;
    }
    root.synchronize_selection_dialog(lease(2, p)).unwrap();
    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert_eq!(host.operations.len(), before);
}

#[test]
fn file_lease_rejects_search_policy() {
    let file = SelectionDialogProjection::File(FileSelectionProjection {
        items: vec![],
        failure: None,
    });
    assert!(
        SelectionDialogProjectionLease::new(1, "選択", controls(), file)
            .unwrap()
            .query_arrow_key_policy(target(1), SearchQueryArrowKeyPolicy::History)
            .is_err()
    );
}

#[test]
fn readonly_and_disabled_queries_do_not_navigate_history() {
    for disabled in [false, true] {
        let context = egui::Context::default();
        let mut root = root();
        let mut host = Host::default();
        let policy = lease(1, projection("元"))
            .disabled(disabled)
            .readonly(!disabled);
        root.synchronize_selection_dialog(policy).unwrap();
        for _ in 0..3 {
            frame(&context, &mut root, vec![], &mut host);
        }
        frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
        assert!(host.operations.iter().all(|event| !matches!(
            event,
            SelectionDialogOperation::QueryHistoryNavigated { .. }
        )));
    }
}

#[test]
fn changing_policy_discards_saved_draft() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("元")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(2, "検索", controls(), projection("最新")).unwrap(),
    )
    .unwrap();
    root.synchronize_selection_dialog(lease(3, projection("最新")))
        .unwrap();
    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert_eq!(host.operations.len(), before);
}

#[test]
fn same_frame_arrow_presses_and_repeats_follow_input_order() {
    for (keys, expected) in [
        (
            vec![(egui::Key::ArrowUp, false), (egui::Key::ArrowUp, false)],
            vec!["最新", "最古"],
        ),
        (
            vec![(egui::Key::ArrowUp, false), (egui::Key::ArrowDown, false)],
            vec!["最新", "下書き⭐️"],
        ),
        (
            vec![(egui::Key::ArrowDown, false), (egui::Key::ArrowUp, false)],
            vec!["最新"],
        ),
        (
            vec![(egui::Key::ArrowUp, false), (egui::Key::ArrowUp, true)],
            vec!["最新", "最古"],
        ),
    ] {
        let context = egui::Context::default();
        let mut root = root();
        let mut host = Host::default();
        root.synchronize_selection_dialog(lease(1, projection("下書き⭐️")))
            .unwrap();
        for _ in 0..3 {
            frame(&context, &mut root, vec![], &mut host);
        }
        let input = keys
            .into_iter()
            .map(|(key, repeat)| egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat,
                modifiers: egui::Modifiers::NONE,
            })
            .collect();
        let (result, _) = frame(&context, &mut root, input, &mut host);
        assert!(result.forward_events_once(&mut host).is_err());
        context.input(|state| {
            assert!(!state.events.iter().any(|event| matches!(event,
                egui::Event::Key { key: egui::Key::ArrowUp | egui::Key::ArrowDown, pressed: true, modifiers, .. }
                    if *modifiers == egui::Modifiers::NONE)));
        });
        let actual = host
            .operations
            .iter()
            .map(|operation| match operation {
                SelectionDialogOperation::QueryHistoryNavigated { value, .. } => value.as_str(),
                other => panic!("unexpected operation: {other:?}"),
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}

#[test]
fn modified_arrow_and_other_keys_are_not_consumed_by_history() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("元")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    let modified = egui::Event::Key {
        key: egui::Key::ArrowUp,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::SHIFT,
    };
    let other = egui::Event::Key {
        key: egui::Key::F2,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    frame(
        &context,
        &mut root,
        vec![modified.clone(), other.clone()],
        &mut host,
    );
    assert!(
        host.operations
            .iter()
            .all(|op| !matches!(op, SelectionDialogOperation::QueryHistoryNavigated { .. }))
    );
    context.input(|input| {
        assert!(input.events.contains(&modified));
        assert!(input.events.contains(&other));
    });
}

#[test]
fn history_and_text_input_in_one_frame_follow_original_order() {
    let press = |key, repeat| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat,
        modifiers: egui::Modifiers::NONE,
    };
    for (input, expected) in [
        (
            vec![
                press(egui::Key::ArrowUp, false),
                egui::Event::Text("x".into()),
            ],
            vec!["h:最新", "q:最新x"],
        ),
        (
            vec![
                egui::Event::Text("x".into()),
                press(egui::Key::ArrowUp, false),
            ],
            vec!["q:元x", "h:最新"],
        ),
        (
            vec![
                press(egui::Key::ArrowUp, false),
                egui::Event::Text("x".into()),
                press(egui::Key::ArrowDown, false),
            ],
            vec!["h:最新", "q:最新x"],
        ),
        (
            vec![
                press(egui::Key::ArrowUp, false),
                press(egui::Key::ArrowUp, true),
                egui::Event::Text("x".into()),
            ],
            vec!["h:最新", "h:最古", "q:最古x"],
        ),
        (
            vec![
                press(egui::Key::ArrowUp, false),
                press(egui::Key::ArrowLeft, false),
                egui::Event::Text("x".into()),
            ],
            vec!["h:最新", "q:最x新"],
        ),
    ] {
        let context = egui::Context::default();
        let mut root = root();
        let mut host = Host::default();
        root.synchronize_selection_dialog(lease(1, projection("元")))
            .unwrap();
        for _ in 0..3 {
            frame(&context, &mut root, vec![], &mut host);
        }
        frame(&context, &mut root, input, &mut host);
        let actual = host
            .operations
            .iter()
            .map(|operation| match operation {
                SelectionDialogOperation::QueryHistoryNavigated { value, .. } => {
                    format!("h:{value}")
                }
                SelectionDialogOperation::QueryChanged { value, .. } => format!("q:{value}"),
                other => panic!("unexpected operation: {other:?}"),
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}

#[test]
fn explicit_history_selection_resets_traversal_and_becomes_next_draft() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(lease(1, projection("下書き⭐️")))
        .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    let (clicked, _) = frame(&context, &mut root, ax_click(&output, "最古"), &mut host);
    assert!(clicked.forward_events_once(&mut host).is_err());
    assert_eq!(
        host.operations,
        vec![
            SelectionDialogOperation::QueryHistoryNavigated {
                mode: target(1),
                target: Some(target(3)),
                value: "最新".into(),
                direction: SearchQueryHistoryDirection::Older
            },
            SelectionDialogOperation::HistorySelected {
                mode: target(1),
                target: target(4)
            },
        ]
    );
    root.synchronize_selection_dialog(lease(2, projection("最古")))
        .unwrap();
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    frame(
        &context,
        &mut root,
        ax_focus_text_input(&output, "mode1"),
        &mut host,
    );
    let (_, focused) = frame(&context, &mut root, vec![], &mut host);
    let ax = focused
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AX");
    assert!(ax.nodes.iter().any(|(id, node)| *id == ax.focus
        && node.label() == Some("mode1")
        && node.value() == Some("最古")));
    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert_eq!(host.operations.len(), before);
    frame(&context, &mut root, key(egui::Key::ArrowUp), &mut host);
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    assert_eq!(
        &host.operations[before..],
        &[
            SelectionDialogOperation::QueryHistoryNavigated {
                mode: target(1),
                target: Some(target(3)),
                value: "最新".into(),
                direction: SearchQueryHistoryDirection::Older
            },
            SelectionDialogOperation::QueryHistoryNavigated {
                mode: target(1),
                target: None,
                value: "最古".into(),
                direction: SearchQueryHistoryDirection::Newer
            },
        ]
    );
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    let ax = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AX");
    assert!(ax.nodes.iter().any(|(id, node)| *id == ax.focus
        && node.label() == Some("mode1")
        && node.value() == Some("最古")));
}
