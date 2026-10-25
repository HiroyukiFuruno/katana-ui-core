#![cfg(feature = "egui")]

#[path = "support/modal_tab_host.rs"]
mod modal_tab_host;

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootEventTransport, EguiTextCommandSurfaceRootFactory, FileSelectionItem,
    FileSelectionProjection, KucDialogControls, KucDialogTarget, KucRootEventBatchDispatcher,
    KucRootEventBatchForwarder, SearchResultMode, SearchResultProjection, SearchResultRow,
    SelectionDialogEvent, SelectionDialogOperation, SelectionDialogProjection,
    SelectionDialogProjectionError, SelectionDialogProjectionLease, TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::selection::ContextMenuEvent;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfaceFocusRequest, TextSurfaceFocusRequestToken,
    TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use std::sync::{Arc, Mutex};

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

fn focused_root() -> EguiTextCommandSurfaceHostRoot {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("dialog-body")
            .value("本文⭐️")
            .ime_enabled(true),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 900, 640),
    ));
    let mut text = TextSurfacePresentation::from_props(surface.props());
    text.focus_request = Some(TextSurfaceFocusRequest::new(
        TextSurfaceFocusRequestToken::new("dialog-body-focus"),
        true,
    ));
    let presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(UiStateId::new("dialog-body")),
        text,
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    };
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"dialog-root-focused",
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

fn frame_with_max_texture_side(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
    host: &mut Host,
    max_texture_side: usize,
) -> egui::FullOutput {
    context.enable_accesskit();
    let mut result = None;
    let output = context.run_ui(
        egui::RawInput {
            max_texture_side: Some(max_texture_side),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 640.0),
            )),
            events,
            ..Default::default()
        },
        |ui| result = Some(root.show(ui).expect("show")),
    );
    result
        .expect("frame")
        .forward_events_once(host)
        .expect("forward");
    output
}

fn frame_width(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
    host: &mut Host,
    width: f32,
) -> (EguiTextCommandSurfaceHostRootFrame, egui::FullOutput) {
    context.enable_accesskit();
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, 640.0),
            )),
            events,
            ..Default::default()
        },
        |ui| result = Some(root.show(ui).expect("show")),
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

#[test]
fn full_root_file_dialog_navigates_selects_and_forwards_once() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let files = |item| {
        SelectionDialogProjection::File(FileSelectionProjection {
            items: vec![item],
            failure: None,
        })
    };
    let folder = FileSelectionItem {
        target: target(1),
        label: "資料⭐️".into(),
        depth: 0,
        navigable: true,
        selectable: false,
        disabled: false,
    };
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(1, "ファイル選択", controls(), files(folder))
            .expect("lease"),
    )
    .expect("attach");
    frame(&context, &mut root, Vec::new(), &mut host);
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    let (navigated, _) = frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    assert_eq!(
        host.operations,
        vec![SelectionDialogOperation::Navigated { target: target(1) }]
    );
    assert!(navigated.forward_events_once(&mut host).is_err());
    let item = FileSelectionItem {
        target: target(2),
        label: "日本語.txt".into(),
        depth: 0,
        navigable: false,
        selectable: true,
        disabled: false,
    };
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(2, "ファイル選択", controls(), files(item))
            .expect("lease"),
    )
    .expect("attach");
    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    assert_eq!(
        host.operations.last(),
        Some(&SelectionDialogOperation::FileSelected { target: target(2) })
    );
    let count = host.operations.len();
    frame(&context, &mut root, Vec::new(), &mut host);
    assert_eq!(host.operations.len(), count);
    assert!(host.text_changes.is_empty());
}

#[test]
fn full_root_modal_open_frame_blocks_background_release_and_keeps_dialog_input() {
    for search in [false, true] {
        for drag in [false, true] {
            let tab_a =
                katana_ui_core::egui::text_command_surface::TabStripTabTarget::from_opaque_bytes(
                    b"drag-first",
                );
            let tab_b =
                katana_ui_core::egui::text_command_surface::TabStripTabTarget::from_opaque_bytes(
                    b"drag-second",
                );
            let tab_host = Arc::new(Mutex::new(modal_tab_host::HostState::default()));
            if drag {
                tab_host.lock().unwrap().tab_order = vec![tab_a.clone(), tab_b.clone()];
            }
            let mut root = modal_tab_host::root_with_host(
                if drag {
                    modal_tab_host::projection_with_draggable_tabs(1, tab_a.clone(), tab_b.clone())
                } else {
                    modal_tab_host::projection(1, tab_a.clone(), tab_b.clone())
                },
                Arc::clone(&tab_host),
            );
            let context = egui::Context::default();
            let mut host = Host::default();
            let initial = frame(&context, &mut root, Vec::new(), &mut host).1;
            let label = if drag {
                "ドラッグタブ"
            } else {
                "同じ名前"
            };
            let mut tabs = modal_tab_host::button_bounds_all(&initial, label);
            tabs.sort_by(|left, right| left.min.x.total_cmp(&right.min.x));
            let background_point = tabs[0].center();
            frame(
                &context,
                &mut root,
                vec![
                    egui::Event::PointerMoved(background_point),
                    egui::Event::PointerButton {
                        pos: background_point,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                &mut host,
            );
            if drag {
                frame(
                    &context,
                    &mut root,
                    vec![egui::Event::PointerMoved(egui::pos2(
                        background_point.x + 10.0,
                        background_point.y,
                    ))],
                    &mut host,
                );
                assert!(tab_host.lock().unwrap().active_drag.is_some());
            }
            let projection = if search {
                SelectionDialogProjection::Search(SearchResultProjection {
                    modes: vec![mode(1)],
                    active_mode: target(1),
                })
            } else {
                SelectionDialogProjection::File(FileSelectionProjection {
                    items: vec![FileSelectionItem {
                        target: target(7),
                        label: "選択対象".into(),
                        depth: 0,
                        navigable: false,
                        selectable: true,
                        disabled: false,
                    }],
                    failure: None,
                })
            };
            root.synchronize_selection_dialog(
                SelectionDialogProjectionLease::new(
                    1,
                    if search {
                        "検索結果"
                    } else {
                        "ファイル選択"
                    },
                    controls(),
                    projection,
                )
                .expect("lease"),
            )
            .expect("attach");
            frame(
                &context,
                &mut root,
                vec![egui::Event::PointerButton {
                    pos: background_point,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                &mut host,
            );
            assert!(tab_host.lock().unwrap().selected.is_none());
            let tab_state = tab_host.lock().unwrap();
            assert!(tab_state.active_drag.is_none());
            assert_eq!(tab_state.operations, if drag { 2 } else { 0 });
            drop(tab_state);
            if search {
                frame(
                    &context,
                    &mut root,
                    vec![egui::Event::Text("modal query".to_owned())],
                    &mut host,
                );
                assert!(host.operations.iter().any(|operation| matches!(
                    operation,
                    SelectionDialogOperation::QueryChanged { mode, value }
                        if mode == &target(1) && value.contains("modal query")
                )));
                assert!(host.text_changes.is_empty());
                frame(&context, &mut root, key(egui::Key::Enter), &mut host);
                assert!(host.operations.iter().any(|operation| matches!(
                    operation,
                    SelectionDialogOperation::SearchRequested { .. }
                )));
                frame(&context, &mut root, key(egui::Key::Escape), &mut host);
            } else {
                frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
                frame(&context, &mut root, key(egui::Key::Enter), &mut host);
                assert_eq!(
                    host.operations.last(),
                    Some(&SelectionDialogOperation::FileSelected { target: target(7) })
                );
            }
            for _ in 0..2 {
                frame(&context, &mut root, Vec::new(), &mut host);
            }
            let tab_click = vec![
                egui::Event::PointerMoved(background_point),
                egui::Event::PointerButton {
                    pos: background_point,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos: background_point,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ];
            frame(&context, &mut root, tab_click, &mut host);
            assert_eq!(tab_host.lock().unwrap().selected, Some(tab_a.clone()));
            if drag {
                let after_click = frame(&context, &mut root, Vec::new(), &mut host).1;
                let source = modal_tab_host::button_bounds(&after_click, "ドラッグタブ");
                let destination = modal_tab_host::button_bounds(&after_click, "移動先");
                frame(
                    &context,
                    &mut root,
                    vec![
                        egui::Event::PointerMoved(source.center()),
                        egui::Event::PointerButton {
                            pos: source.center(),
                            button: egui::PointerButton::Primary,
                            pressed: true,
                            modifiers: egui::Modifiers::NONE,
                        },
                    ],
                    &mut host,
                );
                frame(
                    &context,
                    &mut root,
                    vec![egui::Event::PointerMoved(egui::pos2(
                        source.center().x + 10.0,
                        source.center().y,
                    ))],
                    &mut host,
                );
                let drop = egui::pos2(destination.max.x - 1.0, destination.center().y);
                frame(
                    &context,
                    &mut root,
                    vec![egui::Event::PointerMoved(drop)],
                    &mut host,
                );
                frame(
                    &context,
                    &mut root,
                    vec![egui::Event::PointerButton {
                        pos: drop,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: egui::Modifiers::NONE,
                    }],
                    &mut host,
                );
                let tab_state = tab_host.lock().unwrap();
                assert!(tab_state.active_drag.is_none());
                assert_eq!(tab_state.tab_order, vec![tab_b, tab_a]);
            }
        }
    }
}

#[test]
fn file_dialog_tab_enter_on_cancel_does_not_replay_selected_row() {
    for (value, navigable, selectable) in [(1, true, false), (2, false, true)] {
        let context = egui::Context::default();
        let mut root = root();
        let mut host = Host::default();
        root.synchronize_selection_dialog(
            SelectionDialogProjectionLease::new(
                1,
                "ファイル選択",
                controls(),
                SelectionDialogProjection::File(FileSelectionProjection {
                    items: vec![FileSelectionItem {
                        target: target(value),
                        label: format!("項目{value}"),
                        depth: 0,
                        navigable,
                        selectable,
                        disabled: false,
                    }],
                    failure: None,
                }),
            )
            .expect("lease"),
        )
        .expect("attach");
        for _ in 0..3 {
            frame(&context, &mut root, Vec::new(), &mut host);
        }
        frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
        let mut cancel_focused = false;
        for _ in 0..4 {
            let (_, output) = frame(&context, &mut root, key(egui::Key::Tab), &mut host);
            let cancel_id = output
                .platform_output
                .accesskit_update
                .as_ref()
                .and_then(|update| {
                    update
                        .nodes
                        .iter()
                        .find_map(|(id, node)| (node.label() == Some("取消")).then_some(*id))
                });
            cancel_focused = cancel_id.is_some_and(|id| {
                context.memory(|memory| memory.focused().map(|focused| focused.value()))
                    == Some(u64::from(id))
            });
            if cancel_focused {
                break;
            }
        }
        assert!(
            cancel_focused,
            "Tab must focus Cancel after the selected row"
        );
        let before = host.operations.len();
        frame(&context, &mut root, key(egui::Key::Enter), &mut host);
        assert_eq!(
            &host.operations[before..],
            &[SelectionDialogOperation::Cancelled]
        );
    }
}

#[test]
fn file_dialog_cancel_disabled_failure_and_stale_are_distinct() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let files = || {
        SelectionDialogProjection::File(FileSelectionProjection {
            items: vec![FileSelectionItem {
                target: target(1),
                label: "disabled".into(),
                depth: 0,
                navigable: false,
                selectable: true,
                disabled: true,
            }],
            failure: Some("Unavailable".into()),
        })
    };
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(3, "選択", controls(), files()).expect("lease"),
    )
    .expect("attach");
    frame(&context, &mut root, Vec::new(), &mut host);
    frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    frame(&context, &mut root, key(egui::Key::Escape), &mut host);
    assert_eq!(
        host.operations,
        vec![
            SelectionDialogOperation::Failed {
                message: "Unavailable".into()
            },
            SelectionDialogOperation::Cancelled
        ]
    );
    assert!(matches!(
        root.synchronize_selection_dialog(
            SelectionDialogProjectionLease::new(2, "選択", controls(), files()).expect("lease")
        ),
        Err(SelectionDialogProjectionError::StaleRevision {
            current: 3,
            received: 2
        })
    ));
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

#[test]
fn search_dialog_query_and_modes_are_owned_by_the_same_root() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let projection = SelectionDialogProjection::Search(SearchResultProjection {
        modes: vec![mode(1), mode(2)],
        active_mode: target(1),
    });
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(1, "検索結果", controls(), projection).expect("lease"),
    )
    .expect("attach");
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }
    let (_, focused) = frame(&context, &mut root, Vec::new(), &mut host);
    let update = focused
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("query AX");
    assert!(
        update.nodes.iter().any(|(id, node)| {
            *id == update.focus
                && node.role() == egui::accesskit::Role::TextInput
                && node.label() == Some("mode1")
        }),
        "the modal must focus its query without host intervention: focus={:?}, nodes={:?}",
        update.focus,
        update
            .nodes
            .iter()
            .map(|(id, node)| (id, node.role(), node.label()))
            .collect::<Vec<_>>()
    );
    for label in ["mode1", "含める", "除外"] {
        assert!(
            update.nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::TextInput && node.label() == Some(label)
            }),
            "missing localized input label: {label}"
        );
    }
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("日本語⭐️".into())],
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::QueryChanged {
                mode: target(1),
                value: "日本語⭐️".into()
            })
    );
    frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::SearchRequested { mode: target(1) })
    );
    assert!(host.text_changes.is_empty());
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

fn ax_activate(output: &egui::FullOutput, label: &str) -> Vec<egui::Event> {
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

#[test]
fn search_dialog_full_root_routes_history_filters_and_rejects_invalid_leases() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let mut first = mode(1);
    first.include = Some(String::new());
    first.exclude = Some(String::new());
    first.history = vec![SearchResultRow {
        target: target(7),
        label: "履歴⭐️".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: vec![(0, "履歴".len())],
        disabled: false,
    }];
    let projection = SelectionDialogProjection::Search(SearchResultProjection {
        modes: vec![first, mode(2)],
        active_mode: target(1),
    });
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(1, "検索", controls(), projection).expect("lease"),
    )
    .expect("attach");
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }
    frame(&context, &mut root, Vec::new(), &mut host);
    frame(&context, &mut root, key(egui::Key::Tab), &mut host);
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("日本語".into())],
        &mut host,
    );
    let (_, output) = frame(&context, &mut root, key(egui::Key::Tab), &mut host);
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("⭐️".into())],
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::IncludeChanged {
                mode: target(1),
                value: "日本語".into()
            })
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::ExcludeChanged {
                mode: target(1),
                value: "⭐️".into()
            })
    );
    frame(
        &context,
        &mut root,
        ax_click(&output, "履歴削除"),
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::HistoryDeleted {
                mode: target(1),
                target: target(7)
            })
    );

    let duplicate = SearchResultProjection {
        modes: vec![mode(1), mode(1)],
        active_mode: target(1),
    };
    assert!(matches!(
        SelectionDialogProjectionLease::new(
            2,
            "重複",
            controls(),
            SelectionDialogProjection::Search(duplicate),
        ),
        Err(SelectionDialogProjectionError::DuplicateTarget)
    ));
    let invalid = SearchResultRow {
        target: target(8),
        label: "⭐️".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: vec![(1, 2)],
        disabled: false,
    };
    let invalid_mode = SearchResultMode {
        results: vec![invalid],
        ..mode(1)
    };
    assert!(matches!(
        SelectionDialogProjectionLease::new(
            3,
            "不正",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![invalid_mode],
                active_mode: target(1),
            }),
        ),
        Err(SelectionDialogProjectionError::InvalidHighlight)
    ));
    assert!(matches!(
        root.synchronize_selection_dialog(
            SelectionDialogProjectionLease::new(
                0,
                "stale",
                controls(),
                SelectionDialogProjection::Search(SearchResultProjection {
                    modes: vec![mode(1)],
                    active_mode: target(1),
                }),
            )
            .expect("stale lease")
        ),
        Err(SelectionDialogProjectionError::StaleRevision { .. })
    ));
}

#[test]
fn search_dialog_wrapped_labels_and_snippets_respect_texture_limit() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let mut active = mode(1);
    active.searched = true;
    active.results = vec![SearchResultRow {
        target: target(8),
        label: "wrapped label ".repeat(400),
        auxiliary_label: Some("auxiliary label".into()),
        snippet: Some(
            (1..=96)
                .map(|line| format!("snippet line {line:02}"))
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        highlights: vec![(0, 7)],
        disabled: false,
    }];
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(
            1,
            "検索",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![active, mode(2)],
                active_mode: target(1),
            }),
        )
        .expect("lease"),
    )
    .expect("attach");
    let mut sizes = Vec::new();
    for _ in 0..4 {
        let mut output =
            frame_with_max_texture_side(&context, &mut root, Vec::new(), &mut host, 1024);
        sizes.extend(
            output
                .textures_delta
                .set
                .values()
                .flat_map(|deltas| deltas.iter().map(|delta| delta.image.size())),
        );
        output.textures_delta.clear();
    }
    assert!(
        !sizes.is_empty(),
        "dialog must upload wrapped text textures"
    );
    assert!(
        sizes
            .iter()
            .all(|[width, height]| *width <= 1024 && *height <= 1024),
        "dialog texture sizes exceeded backend limit: {sizes:?}"
    );
}

#[test]
fn search_dialog_tab_enter_activates_history_instead_of_searching() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let mut search = mode(1);
    search.history = vec![SearchResultRow {
        target: target(7),
        label: "履歴項目".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: Vec::new(),
        disabled: false,
    }];
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(
            1,
            "検索",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![search],
                active_mode: target(1),
            }),
        )
        .expect("lease"),
    )
    .expect("attach");
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }

    let mut focused_history = false;
    for _ in 0..12 {
        let (_, output) = frame(&context, &mut root, key(egui::Key::Tab), &mut host);
        let update = output.platform_output.accesskit_update.as_ref();
        focused_history = update.is_some_and(|update| {
            update
                .nodes
                .iter()
                .any(|(id, node)| *id == update.focus && node.label() == Some("履歴項目"))
        });
        if focused_history {
            break;
        }
    }
    assert!(focused_history, "Tab must focus the history item");

    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    assert_eq!(
        &host.operations[before..],
        &[SelectionDialogOperation::HistorySelected {
            mode: target(1),
            target: target(7),
        }]
    );
}

#[test]
fn search_dialog_mode_switch_does_not_keep_stale_input_enter_focus() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let mut second = mode(2);
    second.history = vec![SearchResultRow {
        target: target(8),
        label: "切替後履歴".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: Vec::new(),
        disabled: false,
    }];
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(
            1,
            "検索",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![mode(1), second],
                active_mode: target(1),
            }),
        )
        .expect("lease"),
    )
    .expect("attach");
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }
    let (_, initial) = frame(&context, &mut root, Vec::new(), &mut host);
    frame(
        &context,
        &mut root,
        ax_activate(&initial, "mode2"),
        &mut host,
    );

    let mut focused_history = false;
    for _ in 0..12 {
        let (_, output) = frame(&context, &mut root, key(egui::Key::Tab), &mut host);
        focused_history = output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(id, node)| *id == update.focus && node.label() == Some("切替後履歴"))
            });
        if focused_history {
            break;
        }
    }
    assert!(
        focused_history,
        "Tab must focus the switched mode history item"
    );

    let before = host.operations.len();
    frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    assert_eq!(
        &host.operations[before..],
        &[SelectionDialogOperation::HistorySelected {
            mode: target(2),
            target: target(8),
        }]
    );
}

#[test]
fn search_dialog_readonly_and_disabled_leases_fail_closed_but_allow_cancel() {
    let context = egui::Context::default();
    let projection = SelectionDialogProjection::Search(SearchResultProjection {
        modes: vec![mode(1)],
        active_mode: target(1),
    });
    let mut root = root();
    let mut host = Host::default();
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(1, "readonly", controls(), projection.clone())
            .expect("lease")
            .readonly(true),
    )
    .expect("readonly attach");
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("変更".into())],
        &mut host,
    );
    frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    assert!(host.operations.is_empty());
    let (_, output) = frame(&context, &mut root, Vec::new(), &mut host);
    frame(&context, &mut root, ax_click(&output, "取消"), &mut host);
    assert_eq!(host.operations, vec![SelectionDialogOperation::Closed]);

    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(2, "disabled", controls(), projection)
            .expect("lease")
            .disabled(true),
    )
    .expect("disabled attach");
    let before = host.operations.len();
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("変更".into())],
        &mut host,
    );
    frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    assert_eq!(before, host.operations.len());
}

#[test]
fn search_dialog_mode_switch_round_trip_retains_each_query() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let projection = SelectionDialogProjection::Search(SearchResultProjection {
        modes: vec![mode(1), mode(2)],
        active_mode: target(1),
    });
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(1, "modes", controls(), projection).expect("lease"),
    )
    .expect("attach");
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("日本語".into())],
        &mut host,
    );
    let (_, output) = frame(&context, &mut root, Vec::new(), &mut host);
    let (_, output) = frame(&context, &mut root, ax_click(&output, "mode2"), &mut host);
    frame(&context, &mut root, ax_click(&output, "mode1"), &mut host);
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::ModeChanged { mode: target(2) })
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::ModeChanged { mode: target(1) })
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::QueryChanged {
                mode: target(1),
                value: "日本語".into()
            })
    );
}

#[test]
fn selection_dialog_narrow_root_keeps_controls_in_bounds_and_reaches_last_file() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let items = (0..40)
        .map(|value| FileSelectionItem {
            target: target(value),
            label: format!("末尾項目 {value} 日本語"),
            depth: 0,
            navigable: false,
            selectable: true,
            disabled: false,
        })
        .collect();
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(
            1,
            "狭幅ファイル選択",
            controls(),
            SelectionDialogProjection::File(FileSelectionProjection {
                items,
                failure: None,
            }),
        )
        .expect("lease"),
    )
    .expect("attach");
    let (_, output) = frame_width(&context, &mut root, Vec::new(), &mut host, 320.0);
    let update = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("narrow file dialog AccessKit update");
    for (_, node) in &update.nodes {
        if let Some(bounds) = node.bounds() {
            assert!(
                bounds.x0 >= 0.0 && bounds.x1 <= 320.0,
                "node bounds overflow"
            );
        }
    }
    for _ in 0..40 {
        frame_width(
            &context,
            &mut root,
            key(egui::Key::ArrowDown),
            &mut host,
            320.0,
        );
    }
    let (_, idle_output) = frame_width(&context, &mut root, Vec::new(), &mut host, 320.0);
    let update = idle_output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("idle AccessKit update after keyboard scroll");
    let last = update
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label() == Some("末尾項目 39 日本語")).then_some(node))
        .expect("last file remains in the viewport after scroll");
    let bounds = last.bounds().expect("last file bounds");
    assert!(bounds.y0 >= 0.0 && bounds.y1 <= 640.0);
    frame_width(&context, &mut root, key(egui::Key::Enter), &mut host, 320.0);
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::FileSelected { target: target(39) })
    );
}

#[test]
fn selection_dialog_narrow_search_scrolls_results_and_selects_last_result() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let mut search_mode = mode(1);
    search_mode.searched = true;
    search_mode.results = (0..40)
        .map(|value| SearchResultRow {
            target: target(value),
            label: format!("検索結果 {value}"),
            auxiliary_label: None,
            snippet: None,
            highlights: vec![],
            disabled: false,
        })
        .collect();
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(
            1,
            "狭幅検索",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![search_mode],
                active_mode: target(1),
            }),
        )
        .expect("lease"),
    )
    .expect("attach");
    let (_, initial) = frame_width(&context, &mut root, Vec::new(), &mut host, 320.0);
    let initial_update = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("initial search AccessKit update");
    let initial_last = initial_update
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label() == Some("検索結果 39")).then_some(node))
        .expect("initial last search result");
    let initial_bounds = initial_last.bounds().expect("initial last bounds");
    assert!(initial_bounds.y0 > 640.0 || initial_bounds.y1 < 0.0);
    for _ in 0..40 {
        frame_width(
            &context,
            &mut root,
            key(egui::Key::ArrowDown),
            &mut host,
            320.0,
        );
    }
    let (_, idle) = frame_width(&context, &mut root, Vec::new(), &mut host, 320.0);
    let update = idle
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("idle search AccessKit update");
    let last = update
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label() == Some("検索結果 39")).then_some(node))
        .expect("last search result after scroll");
    let bounds = last.bounds().expect("last search result bounds");
    assert!(bounds.x0 >= 0.0 && bounds.x1 <= 320.0);
    assert!(bounds.y0 >= 0.0 && bounds.y1 <= 640.0);
    frame_width(
        &context,
        &mut root,
        ax_click(&idle, "検索結果 39"),
        &mut host,
        320.0,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::ResultSelected {
                mode: target(1),
                target: target(39),
            })
    );
}

#[test]
fn selection_dialog_escape_returns_text_input_to_the_host() {
    let context = egui::Context::default();
    let mut root = focused_root();
    let mut host = Host::default();
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }
    let (before, _) = frame(
        &context,
        &mut root,
        vec![egui::Event::Text("事前".into())],
        &mut host,
    );
    assert!(
        before.ui_selection_after_input().snapshot().is_some(),
        "body must be focused before opening modal"
    );
    assert!(host.text_changes.iter().any(|value| value.contains("事前")));
    let original_focus = context
        .memory(|memory| memory.focused())
        .expect("original body focus");
    host.text_changes.clear();
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(
            1,
            "検索",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![mode(1)],
                active_mode: target(1),
            }),
        )
        .expect("lease"),
    )
    .expect("attach");
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("検索入力".into())],
        &mut host,
    );
    assert!(
        host.text_changes.is_empty(),
        "modal must own input while open"
    );
    frame(&context, &mut root, key(egui::Key::Escape), &mut host);
    assert_eq!(
        context.memory(|memory| memory.focused()),
        Some(original_focus),
        "close restores original focus ID"
    );
    assert_eq!(
        host.operations.last(),
        Some(&SelectionDialogOperation::Closed)
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("本文入力".into())],
        &mut host,
    );
    let (after, _) = frame(&context, &mut root, Vec::new(), &mut host);
    assert!(
        host.text_changes
            .iter()
            .any(|value| value.contains("本文入力")),
        "text changes: {:?}, body availability {:?}",
        host.text_changes,
        after.ui_selection_after_input().availability()
    );
    let delivered = host.text_changes.len();
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }
    assert_eq!(
        host.text_changes.len(),
        delivered,
        "deferred input is consumed once"
    );
    assert_eq!(
        host.text_changes
            .last()
            .unwrap()
            .matches("本文入力")
            .count(),
        1
    );
}

#[test]
fn search_dialog_routes_opaque_options_history_results_and_reopens() {
    use katana_ui_core::egui::text_command_surface::{SearchResultOption, SearchResultRow};
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let row = |value, label: &str| SearchResultRow {
        target: target(value),
        label: label.into(),
        auxiliary_label: Some("資料.txt:2".into()),
        snippet: Some("日本語⭐️".into()),
        highlights: vec![(0, 9)],
        disabled: false,
    };
    let mut first = mode(1);
    first.options = vec![SearchResultOption {
        target: target(3),
        label: "大文字区別".into(),
        enabled: true,
        selected: false,
    }];
    first.history = vec![row(4, "以前の検索"), row(5, "残す履歴")];
    first.results = vec![row(6, "検索結果")];
    first.searched = true;
    let projection = SelectionDialogProjection::Search(SearchResultProjection {
        modes: vec![first, mode(2)],
        active_mode: target(1),
    });
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(1, "検索結果画面", controls(), projection.clone())
            .expect("lease"),
    )
    .expect("attach");
    for _ in 0..2 {
        frame(&context, &mut root, vec![], &mut host);
    }
    let (_, initial) = frame(&context, &mut root, vec![], &mut host);
    let (_, output) = frame(
        &context,
        &mut root,
        ax_click(&initial, "大文字区別"),
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::OptionChanged {
                mode: target(1),
                option: target(3),
                selected: true
            })
    );
    let (_, output) = frame(
        &context,
        &mut root,
        ax_click(&output, "以前の検索"),
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::HistorySelected {
                mode: target(1),
                target: target(4)
            })
    );
    let (_, output) = frame(
        &context,
        &mut root,
        ax_click(&output, "履歴消去"),
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::HistoryCleared { mode: target(1) })
    );
    let (_, output) = frame(
        &context,
        &mut root,
        ax_click(&output, "検索結果"),
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::ResultSelected {
                mode: target(1),
                target: target(6)
            })
    );
    let (_, output) = frame(&context, &mut root, ax_click(&output, "mode2"), &mut host);
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::ModeChanged { mode: target(2) })
    );
    frame(&context, &mut root, ax_click(&output, "取消"), &mut host);
    assert_eq!(
        host.operations.last(),
        Some(&SelectionDialogOperation::Closed)
    );
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(2, "再表示", controls(), projection).expect("lease"),
    )
    .expect("reopen");
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    assert!(
        output
            .platform_output
            .accesskit_update
            .expect("AX")
            .nodes
            .iter()
            .any(|(_, node)| node.label() == Some("再表示"))
    );
    assert!(host.text_changes.is_empty());
}

#[test]
fn search_dialog_history_query_invalidates_keyboard_result_before_enter() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let mut search = mode(1);
    search.searched = true;
    search.history = vec![SearchResultRow {
        target: target(7),
        label: "履歴query".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: Vec::new(),
        disabled: false,
    }];
    search.results = vec![SearchResultRow {
        target: target(6),
        label: "旧結果".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: Vec::new(),
        disabled: false,
    }];
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(
            1,
            "履歴選択",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![search],
                active_mode: target(1),
            }),
        )
        .expect("lease"),
    )
    .expect("attach");
    for _ in 0..3 {
        frame(&context, &mut root, Vec::new(), &mut host);
    }

    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
    let (_, before_history) = frame(&context, &mut root, Vec::new(), &mut host);
    frame(
        &context,
        &mut root,
        ax_click(&before_history, "履歴query"),
        &mut host,
    );
    assert_eq!(
        host.operations.last(),
        Some(&SelectionDialogOperation::HistorySelected {
            mode: target(1),
            target: target(7),
        })
    );

    let (_, history_query) = frame(&context, &mut root, Vec::new(), &mut host);
    let update = history_query
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("history query AX");
    let query = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| {
            node.role() == egui::accesskit::Role::TextInput && node.label() == Some("mode1")
        })
        .expect("query input");
    assert_eq!(query.value(), Some("履歴query"));
    frame(
        &context,
        &mut root,
        ax_focus_text_input(&history_query, "mode1"),
        &mut host,
    );

    let before_enter = host.operations.len();
    frame(&context, &mut root, key(egui::Key::Enter), &mut host);
    assert_eq!(
        &host.operations[before_enter..],
        &[SelectionDialogOperation::SearchRequested { mode: target(1) }]
    );
    assert!(!host.operations.iter().any(|operation| {
        matches!(
            operation,
            SelectionDialogOperation::ResultSelected {
                target: selected_target,
                ..
            } if *selected_target == target(6)
        )
    }));
}

#[test]
fn search_dialog_filter_and_option_changes_clear_keyboard_result_before_enter() {
    let run = |changed: &str, change: fn(&SelectionDialogOperation) -> bool| {
        let context = egui::Context::default();
        let mut root = root();
        let mut host = Host::default();
        let mut search = mode(1);
        search.searched = true;
        search.options = vec![
            katana_ui_core::egui::text_command_surface::SearchResultOption {
                target: target(3),
                label: "オプション".into(),
                enabled: true,
                selected: false,
            },
        ];
        search.results = vec![SearchResultRow {
            target: target(6),
            label: "旧結果".into(),
            auxiliary_label: None,
            snippet: None,
            highlights: Vec::new(),
            disabled: false,
        }];
        root.synchronize_selection_dialog(
            SelectionDialogProjectionLease::new(
                1,
                "条件変更",
                controls(),
                SelectionDialogProjection::Search(SearchResultProjection {
                    modes: vec![search],
                    active_mode: target(1),
                }),
            )
            .expect("lease"),
        )
        .expect("attach");
        for _ in 0..3 {
            frame(&context, &mut root, Vec::new(), &mut host);
        }
        frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
        let (_, output) = frame(&context, &mut root, Vec::new(), &mut host);
        let events = if changed == "オプション" {
            ax_click(&output, changed)
        } else {
            ax_focus_text_input(&output, changed)
        };
        frame(&context, &mut root, events, &mut host);
        if changed != "オプション" {
            frame(
                &context,
                &mut root,
                vec![egui::Event::Text("条件".into())],
                &mut host,
            );
        }
        let (_, after_change) = frame(&context, &mut root, Vec::new(), &mut host);
        frame(
            &context,
            &mut root,
            ax_focus_text_input(&after_change, "mode1"),
            &mut host,
        );
        let before_enter = host.operations.len();
        frame(&context, &mut root, key(egui::Key::Enter), &mut host);
        assert_eq!(
            &host.operations[before_enter..],
            &[SelectionDialogOperation::SearchRequested { mode: target(1) }],
            "{changed} must search rather than activate the stale result"
        );
        assert!(host.operations.iter().any(change), "{changed}");
        assert!(!host.operations.iter().any(|operation| {
            matches!(
                operation,
                SelectionDialogOperation::ResultSelected {
                    target: selected_target,
                    ..
                } if *selected_target == target(6)
            )
        }));
    };

    run("含める", |operation| {
        matches!(operation, SelectionDialogOperation::IncludeChanged { .. })
    });
    run("除外", |operation| {
        matches!(operation, SelectionDialogOperation::ExcludeChanged { .. })
    });
    run("オプション", |operation| {
        matches!(operation, SelectionDialogOperation::OptionChanged { .. })
    });
}

#[test]
fn search_dialog_ime_confirmation_does_not_submit_query_or_activate_results() {
    for label in ["mode1", "含める", "除外"] {
        for selected in [false, true] {
            for commit_first in [false, true] {
                let context = egui::Context::default();
                let mut root = root();
                let mut host = Host::default();
                let mut search = mode(1);
                search.results = vec![SearchResultRow {
                    target: target(6),
                    label: "旧結果".into(),
                    auxiliary_label: None,
                    snippet: None,
                    highlights: Vec::new(),
                    disabled: false,
                }];
                root.synchronize_selection_dialog(
                    SelectionDialogProjectionLease::new(
                        1,
                        "日本語検索",
                        controls(),
                        SelectionDialogProjection::Search(SearchResultProjection {
                            modes: vec![search, mode(2)],
                            active_mode: target(1),
                        }),
                    )
                    .expect("lease"),
                )
                .expect("attach");
                for _ in 0..3 {
                    frame(&context, &mut root, Vec::new(), &mut host);
                }
                if selected {
                    frame(&context, &mut root, key(egui::Key::ArrowDown), &mut host);
                }
                let (_, output) = frame(&context, &mut root, Vec::new(), &mut host);
                frame(
                    &context,
                    &mut root,
                    ax_focus_text_input(&output, label),
                    &mut host,
                );
                let before = host.operations.len();
                let mut preedit = vec![egui::Event::Ime(egui::ImeEvent::Preedit {
                    text: "にほん".into(),
                    active_range_chars: None,
                })];
                preedit.extend(key(egui::Key::Enter));
                frame(&context, &mut root, preedit, &mut host);
                frame(&context, &mut root, key(egui::Key::Enter), &mut host);
                let commit = egui::Event::Ime(egui::ImeEvent::Commit("日本".into()));
                let mut confirmation = key(egui::Key::Enter);
                if commit_first {
                    confirmation.insert(0, commit);
                } else {
                    confirmation.push(commit);
                }
                let (_, committed) = frame(&context, &mut root, confirmation, &mut host);
                assert!(
                    host.operations[before..].iter().all(|operation| !matches!(
                        operation,
                        SelectionDialogOperation::SearchRequested { .. }
                            | SelectionDialogOperation::ResultSelected { .. }
                    )),
                    "{label}, selected={selected}, commit_first={commit_first}"
                );
                assert!(
                    committed
                        .platform_output
                        .accesskit_update
                        .as_ref()
                        .expect("AX")
                        .nodes
                        .iter()
                        .any(|(_, node)| node.label() == Some(label)
                            && node.value().is_some_and(|value| value.contains("日本"))),
                    "the focused input must accept the committed text: {label}"
                );
                let after_commit = host.operations.len();
                frame(&context, &mut root, key(egui::Key::Enter), &mut host);
                assert_eq!(
                    &host.operations[after_commit..],
                    &[SelectionDialogOperation::SearchRequested { mode: target(1) }],
                    "ordinary Enter must resume after composition: {label}"
                );
                let (_, composing) = frame(
                    &context,
                    &mut root,
                    vec![egui::Event::Ime(egui::ImeEvent::Preedit {
                        text: "途中".into(),
                        active_range_chars: None,
                    })],
                    &mut host,
                );
                frame(
                    &context,
                    &mut root,
                    ax_activate(&composing, "mode2"),
                    &mut host,
                );
                let (_, switched) = frame(&context, &mut root, Vec::new(), &mut host);
                frame(
                    &context,
                    &mut root,
                    ax_focus_text_input(&switched, "mode2"),
                    &mut host,
                );
                let after_switch = host.operations.len();
                frame(&context, &mut root, key(egui::Key::Enter), &mut host);
                assert_eq!(
                    &host.operations[after_switch..],
                    &[SelectionDialogOperation::SearchRequested { mode: target(2) }],
                    "an inactive mode's composition must not block the focused input"
                );
            }
        }
    }
}

#[test]
fn selection_dialog_lease_flags_and_validation_errors_are_explicit() {
    let file_projection = SelectionDialogProjection::File(FileSelectionProjection {
        items: Vec::new(),
        failure: None,
    });
    let lease = SelectionDialogProjectionLease::new(1, "closed", controls(), file_projection)
        .expect("empty file projection")
        .open(false)
        .disabled(true)
        .readonly(true);
    assert!(matches!(
        SelectionDialogProjectionLease::new(
            2,
            "missing",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![mode(1)],
                active_mode: target(9),
            }),
        ),
        Err(SelectionDialogProjectionError::MissingMode)
    ));
    let error = SelectionDialogProjectionError::MissingMode;
    assert_eq!(
        error.to_string(),
        "selection dialog projection rejected: MissingMode"
    );

    let context = egui::Context::default();
    let mut dialog_root = root();
    let mut host = Host::default();
    dialog_root
        .synchronize_selection_dialog(lease)
        .expect("closed lease");
    let (_frame, output) = frame(&context, &mut dialog_root, Vec::new(), &mut host);
    assert!(
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_none_or(|update| {
                update
                    .nodes
                    .iter()
                    .all(|(_, node)| node.label() != Some("closed"))
            })
    );
    assert!(host.operations.is_empty());
}

#[test]
fn file_dialog_pointer_activation_select_button_and_reverse_navigation_are_real() {
    let context = egui::Context::default();
    let mut dialog_root = root();
    let mut host = Host::default();
    let items = vec![
        FileSelectionItem {
            target: target(1),
            label: "資料フォルダ".into(),
            depth: 0,
            navigable: true,
            selectable: false,
            disabled: false,
        },
        FileSelectionItem {
            target: target(2),
            label: "文書.txt".into(),
            depth: 0,
            navigable: false,
            selectable: true,
            disabled: false,
        },
    ];
    let projection = || {
        SelectionDialogProjection::File(FileSelectionProjection {
            items: items.clone(),
            failure: None,
        })
    };
    dialog_root
        .synchronize_selection_dialog(
            SelectionDialogProjectionLease::new(1, "ファイル", controls(), projection())
                .expect("lease"),
        )
        .expect("attach");
    for _ in 0..3 {
        frame(&context, &mut dialog_root, Vec::new(), &mut host);
    }
    let (_, initial) = frame(&context, &mut dialog_root, Vec::new(), &mut host);
    let (_, after_folder) = frame(
        &context,
        &mut dialog_root,
        ax_activate(&initial, "資料フォルダ"),
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::Navigated { target: target(1) })
    );
    frame(
        &context,
        &mut dialog_root,
        key(egui::Key::ArrowUp),
        &mut host,
    );
    let (_, _after_file) = frame(
        &context,
        &mut dialog_root,
        ax_activate(&after_folder, "文書.txt"),
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::FileSelected { target: target(2) })
    );

    let mut select_root = root();
    let mut select_host = Host::default();
    select_root
        .synchronize_selection_dialog(
            SelectionDialogProjectionLease::new(1, "選択", controls(), projection())
                .expect("select lease"),
        )
        .expect("select attach");
    let (_, initial) = frame(&context, &mut select_root, Vec::new(), &mut select_host);
    let _ = frame(
        &context,
        &mut select_root,
        key(egui::Key::ArrowDown),
        &mut select_host,
    );
    let (_, selected) = frame(
        &context,
        &mut select_root,
        key(egui::Key::ArrowDown),
        &mut select_host,
    );
    let (_, _closed) = frame(
        &context,
        &mut select_root,
        ax_activate(&selected, "選択"),
        &mut select_host,
    );
    assert!(
        select_host
            .operations
            .contains(&SelectionDialogOperation::FileSelected { target: target(2) })
    );
    assert!(
        initial
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| update
                .nodes
                .iter()
                .any(|(_, node)| node.label() == Some("選択")))
    );
}

#[test]
fn search_dialog_search_click_enter_result_and_empty_state_are_distinct() {
    let context = egui::Context::default();
    let mut dialog_root = root();
    let mut host = Host::default();
    let mut search_mode = mode(1);
    search_mode.searched = true;
    search_mode.results = vec![SearchResultRow {
        target: target(4),
        label: "結果".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: Vec::new(),
        disabled: false,
    }];
    dialog_root
        .synchronize_selection_dialog(
            SelectionDialogProjectionLease::new(
                1,
                "検索",
                controls(),
                SelectionDialogProjection::Search(SearchResultProjection {
                    modes: vec![search_mode],
                    active_mode: target(1),
                }),
            )
            .expect("search lease"),
        )
        .expect("search attach");
    for _ in 0..3 {
        frame(&context, &mut dialog_root, Vec::new(), &mut host);
    }
    let (_, initial) = frame(&context, &mut dialog_root, Vec::new(), &mut host);
    let (_, searched) = frame(
        &context,
        &mut dialog_root,
        ax_activate(&initial, "検索"),
        &mut host,
    );
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::SearchRequested { mode: target(1) })
    );
    frame(
        &context,
        &mut dialog_root,
        key(egui::Key::ArrowDown),
        &mut host,
    );
    frame(&context, &mut dialog_root, key(egui::Key::Enter), &mut host);
    assert!(
        host.operations
            .contains(&SelectionDialogOperation::ResultSelected {
                mode: target(1),
                target: target(4)
            })
    );
    assert!(
        searched
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| update
                .nodes
                .iter()
                .any(|(_, node)| node.label() == Some("結果")))
    );

    let mut empty_root = root();
    let mut empty_host = Host::default();
    let mut empty_mode = mode(1);
    empty_mode.searched = true;
    empty_root
        .synchronize_selection_dialog(
            SelectionDialogProjectionLease::new(
                1,
                "空結果",
                controls(),
                SelectionDialogProjection::Search(SearchResultProjection {
                    modes: vec![empty_mode],
                    active_mode: target(1),
                }),
            )
            .expect("empty lease"),
        )
        .expect("empty attach");
    let (_, empty) = frame(&context, &mut empty_root, Vec::new(), &mut empty_host);
    assert!(
        empty
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| update
                .nodes
                .iter()
                .any(|(_, node)| node.label() == Some("結果なし")))
    );
}
