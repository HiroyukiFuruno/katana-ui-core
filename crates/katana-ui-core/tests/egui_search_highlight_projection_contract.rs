#![cfg(feature = "egui")]
use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootEventTransport, EguiTextCommandSurfaceRootFactory, KucDialogControls,
    KucDialogTarget, KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
    SearchHighlightProjection, SearchHighlightProjectionError, SearchResultMode,
    SearchResultProjection, SearchResultRow, SelectionDialogEvent, SelectionDialogOperation,
    SelectionDialogProjection, SelectionDialogProjectionError, SelectionDialogProjectionLease,
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

#[test]
fn unicode_ranges_preserve_logical_matches_and_expand_only_the_display() {
    let text = "⭐️が👩‍💻";
    let logical = vec![
        (3, 6),
        (9, 12),
        (16, 19),
        (0, 0),
        (text.len(), text.len()),
        (0, 6),
        (3, 6),
    ];
    let projected = SearchHighlightProjection::new(text, &logical).unwrap();
    assert_eq!(projected.logical_ranges(), logical);
    assert_eq!(
        projected.display_ranges(),
        &[(0, 6), (6, 12), (12, text.len()), (0, 6), (0, 6)]
    );
    assert!(
        SearchHighlightProjection::new("", &[(0, 0)])
            .unwrap()
            .display_ranges()
            .is_empty()
    );
    let split =
        SearchHighlightProjection::new("a⭐️b", &[(0, 1), (4, 7), (1, 4), (7, 8), (4, 4)]).unwrap();
    assert_eq!(
        split.logical_ranges(),
        &[(0, 1), (4, 7), (1, 4), (7, 8), (4, 4)]
    );
    assert_eq!(split.display_ranges(), &[(0, 1), (1, 7), (1, 7), (7, 8)]);
}

#[test]
fn invalid_ranges_are_typed_and_validation_precedence_is_stable() {
    let error = SearchHighlightProjection::new("⭐️", &[(0, 1)]).unwrap_err();
    assert!(error.to_string().contains("InvalidUtf8Boundary"));
    let as_error: &dyn std::error::Error = &error;
    assert!(as_error.source().is_none());
    assert!(matches!(
        SearchHighlightProjection::new("⭐️", &[(7, 2)]),
        Err(SearchHighlightProjectionError::Reversed {
            index: 0,
            start: 7,
            end: 2
        })
    ));
    assert!(matches!(
        SearchHighlightProjection::new("⭐️", &[(1, 7)]),
        Err(SearchHighlightProjectionError::OutOfBounds { index: 0, .. })
    ));
    assert!(matches!(
        SearchHighlightProjection::new("⭐️", &[(0, 1)]),
        Err(SearchHighlightProjectionError::InvalidUtf8Boundary { index: 0, .. })
    ));
    assert!(matches!(
        SearchHighlightProjection::new("⭐️", &[(0, 0), (1, 1)]),
        Err(SearchHighlightProjectionError::InvalidUtf8Boundary { index: 1, .. })
    ));
}

#[test]
fn root_input_drives_projection_without_relaxing_strict_validator() {
    let context = egui::Context::default();
    let mut root = root();
    let mut host = Host::default();
    let mode = |row: SearchResultRow| SearchResultMode {
        target: target(1),
        label: "query".into(),
        query: String::new(),
        include: None,
        exclude: None,
        options: vec![],
        history: vec![],
        results: vec![row],
        searched: true,
    };
    let row = |highlights| SearchResultRow {
        target: target(2),
        label: "結果".into(),
        auxiliary_label: None,
        snippet: Some("⭐️".into()),
        highlights,
        disabled: false,
    };
    let projection = |row| {
        SelectionDialogProjection::Search(SearchResultProjection {
            modes: vec![mode(row)],
            active_mode: target(1),
        })
    };
    assert!(matches!(
        SelectionDialogProjectionLease::new(1, "検索", controls(), projection(row(vec![(3, 6)]))),
        Err(SelectionDialogProjectionError::InvalidHighlight)
    ));
    let mut initial = mode(row(vec![]));
    initial.results.clear();
    initial.searched = false;
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(
            1,
            "検索",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![initial],
                active_mode: target(1),
            }),
        )
        .unwrap(),
    )
    .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    frame(
        &context,
        &mut root,
        vec![egui::Event::Text("検索".into())],
        &mut host,
    );
    assert!(host.operations.iter().any(
        |op| matches!(op, SelectionDialogOperation::QueryChanged { value, .. } if value == "検索")
    ));
    let logical = [(3, 6)];
    let projected = SearchHighlightProjection::new("⭐️", &logical).unwrap();
    let mut result_mode = mode(row(projected.display_ranges().to_vec()));
    result_mode.query = "検索".into();
    result_mode.results.push(SearchResultRow {
        target: target(3),
        label: "隣接結果".into(),
        auxiliary_label: None,
        snippet: Some("隣⭐️".into()),
        highlights: vec![],
        disabled: false,
    });
    root.synchronize_selection_dialog(
        SelectionDialogProjectionLease::new(
            2,
            "検索",
            controls(),
            SelectionDialogProjection::Search(SearchResultProjection {
                modes: vec![result_mode],
                active_mode: target(1),
            }),
        )
        .unwrap(),
    )
    .unwrap();
    for _ in 0..3 {
        frame(&context, &mut root, vec![], &mut host);
    }
    let (_, output) = frame(&context, &mut root, vec![], &mut host);
    let ax = output.platform_output.accesskit_update.unwrap();
    let snippet = |label| {
        let (id, node) = ax
            .nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label))
            .unwrap();
        (*id, node.bounds().unwrap())
    };
    let (first_id, first) = snippet("⭐️");
    let (second_id, second) = snippet("隣⭐️");
    for (id, other, bounds) in [(first_id, second_id, first), (second_id, first_id, second)] {
        frame(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(egui::pos2(
                bounds.x0 as f32 + 12.0,
                bounds.y0 as f32 + 10.0,
            ))],
            &mut host,
        );
        let hits = context.interaction_snapshot(|snapshot| {
            snapshot
                .contains_pointer
                .iter()
                .copied()
                .collect::<Vec<_>>()
        });
        assert!(
            hits.iter().any(|target| target.accesskit_id() == id),
            "expected {id:?} at {bounds:?}, pointer {:?}, hits {:?}",
            context.pointer_hover_pos(),
            hits.iter()
                .map(|target| (
                    target.accesskit_id(),
                    context.read_response(*target).map(|r| r.rect)
                ))
                .collect::<Vec<_>>()
        );
        assert!(!hits.iter().any(|target| target.accesskit_id() == other));
    }
    assert_eq!(projected.logical_ranges(), &logical);
}
