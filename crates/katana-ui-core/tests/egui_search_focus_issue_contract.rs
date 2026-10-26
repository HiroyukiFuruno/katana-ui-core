#![cfg(feature = "egui")]

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceHostRootFrame,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootEventTransport,
    EguiTextCommandSurfaceRootFactory, EguiTextCommandSurfaceSearchPresentation,
    KucInteractionActionClass, KucInteractionSelector, KucRootEventBatchContext,
    KucRootEventBatchForwarder, TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeSearchPresentation, CommandChromeText,
    SearchControlIcons, SearchControlStrings, SearchResultSummaryTemplate,
};
use katana_ui_core::molecule::structured::{
    ReplaceMode, SearchControlStripEvent, SearchNavigationDirection, SearchOptions,
};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
struct SearchOwner {
    query: String,
    active: usize,
    navigations: usize,
    document_changes: usize,
}

struct Forwarder;
impl KucRootEventBatchForwarder for Forwarder {
    type Error = ();
    fn forward_root_event_batch(
        &mut self,
        _: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), ()> {
        Ok(())
    }
}

fn strings() -> SearchControlStrings {
    let text = |value: &str| CommandChromeText::new(value, value, value);
    SearchControlStrings {
        strip: text("Search"),
        query: text("Find"),
        replace: text("Replace"),
        match_case: text("Match case"),
        whole_word: text("Whole word"),
        use_regex: text("Regex"),
        previous: text("Previous"),
        next: text("Next"),
        replace_one: text("Replace one"),
        replace_all: text("Replace all"),
        close: text("Close"),
        result_summary: SearchResultSummaryTemplate {
            empty: "0".into(),
            zero_results: "0".into(),
            single_result: "1".into(),
            indexed_result: "{active}/{count}".into(),
            count_results: "{count}".into(),
        },
    }
}

fn lease(
    revision: u64,
    owner: Rc<RefCell<SearchOwner>>,
) -> EguiTextCommandSurfaceHostProjectionLease {
    lease_with_results(revision, owner, Some(3), Some(0))
}

fn lease_with_results(
    revision: u64,
    owner: Rc<RefCell<SearchOwner>>,
    result_count: Option<usize>,
    active_index: Option<usize>,
) -> EguiTextCommandSurfaceHostProjectionLease {
    let text = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("Body").value("日x本x\n⭐️ / ☆\n"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    let model = owner.borrow();
    let presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(UiStateId::new("search-body")),
        text: TextSurfacePresentation::from_props(text.props()),
        toolbar: None,
        floating: None,
        context_menu: None,
        search: Some(EguiTextCommandSurfaceSearchPresentation {
            state_id: UiStateId::new("search-fixture"),
            label: "Search".into(),
            value: CommandChromeSearchPresentation {
                query: model.query.clone(),
                options: SearchOptions {
                    use_regex: true,
                    ..Default::default()
                },
                result_count,
                active_index,
                replace_mode: ReplaceMode::Hidden,
                replace_value: String::new(),
                strings: strings(),
                capabilities: Default::default(),
                icons: SearchControlIcons::default(),
            },
        }),
    };
    drop(model);
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"search-focus-root",
        presentation,
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        move |context: KucRootEventBatchContext| {
            let mut owner = owner.borrow_mut();
            for event in context.search_events() {
                if let CommandChromeSearchEvent::Strip { event } = event {
                    match event {
                        SearchControlStripEvent::SearchQueryChanged(query) => {
                            owner.query = query.clone()
                        }
                        SearchControlStripEvent::SearchNavigationRequested { direction } => {
                            if let Some(count) = result_count.filter(|count| *count > 0) {
                                owner.active = match direction {
                                    SearchNavigationDirection::Previous => {
                                        (owner.active + count - 1) % count
                                    }
                                    SearchNavigationDirection::Next => (owner.active + 1) % count,
                                };
                            }
                            owner.navigations += 1;
                        }
                        _ => {}
                    }
                }
            }
            owner.document_changes += context
                .text_events()
                .iter()
                .filter(|event| {
                    matches!(
                        event,
                        TextSurfaceEvent::TextArea(katana_ui_core::atom::TextAreaEvent::Change(_))
                    )
                })
                .count();
            Ok(None)
        },
    )
}

fn frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> (EguiTextCommandSurfaceHostRootFrame, egui::FullOutput) {
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            events,
            ..Default::default()
        },
        |ui| result = Some(root.show(ui).expect("show")),
    );
    output.textures_delta.clear();
    let result = result.expect("frame");
    result.forward_events_once(&mut Forwarder).expect("forward");
    assert!(result.forward_events_once(&mut Forwarder).is_err());
    (result, output)
}

fn assert_query_focus(output: &egui::FullOutput) {
    let update = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AccessKit");
    let id = update
        .nodes
        .iter()
        .find_map(|(id, node)| {
            (node.label() == Some("Find") && node.role() == egui::accesskit::Role::TextInput)
                .then_some(*id)
        })
        .expect("Find node");
    assert_eq!(update.focus, id);
}

fn key(key: egui::Key, shift: bool) -> Vec<egui::Event> {
    [true, false]
        .map(|pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers {
                shift,
                ..Default::default()
            },
        })
        .to_vec()
}

#[test]
fn full_root_arrows_keep_query_focus_and_wrap_three_results_without_focus_recovery() {
    for (direction, expected) in [
        (egui::Key::ArrowUp, [2, 1, 0]),
        (egui::Key::ArrowDown, [1, 2, 0]),
    ] {
        let context = egui::Context::default();
        context.enable_accesskit();
        let owner = Rc::new(RefCell::new(SearchOwner::default()));
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(lease(1, Rc::clone(&owner)))
            .expect("root");
        let (initial, _) = frame(&context, &mut root, Vec::new());
        let mut click = initial
            .interaction_locator()
            .begin_click(KucInteractionSelector::new(
                "search-fixture:query",
                KucInteractionActionClass::TextInput,
            ))
            .expect("query click");
        for _ in 0..3 {
            let mut input = egui::RawInput::default();
            click
                .apply_to_raw_input_once(&mut input)
                .expect("apply click");
            let (current, _) = frame(&context, &mut root, input.events);
            match click
                .advance(current.interaction_locator())
                .expect("advance")
            {
                Some(next) => click = next,
                None => break,
            }
        }
        let (_, focused) = frame(&context, &mut root, vec![egui::Event::Text("日|x".into())]);
        assert_query_focus(&focused);
        assert_eq!(owner.borrow().query, "日|x");
        for (index, expected) in expected.into_iter().enumerate() {
            root.synchronize_with_lease(lease(index as u64 + 2, Rc::clone(&owner)))
                .expect("sync");
            let (_, output) = frame(&context, &mut root, key(direction, false));
            assert_query_focus(&output);
            assert_eq!(owner.borrow().active, expected);
            assert_eq!(owner.borrow().navigations, index + 1);
        }
        assert_eq!(owner.borrow().document_changes, 0);
        frame(&context, &mut root, key(egui::Key::Enter, true));
        assert_eq!(owner.borrow().active, 2);
        assert_eq!(owner.borrow().navigations, 4);
    }
}

#[test]
fn full_root_empty_zero_and_invalid_queries_preserve_host_state() {
    for (query, result_count, active_index) in [
        ("", Some(0), None),
        ("no-match", Some(0), None),
        ("[invalid", None, None),
    ] {
        let context = egui::Context::default();
        context.enable_accesskit();
        let owner = Rc::new(RefCell::new(SearchOwner {
            query: query.to_owned(),
            ..Default::default()
        }));
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(lease(1, Rc::clone(&owner)))
            .expect("root");
        let (initial, _) = frame(&context, &mut root, Vec::new());
        let mut click = initial
            .interaction_locator()
            .begin_click(KucInteractionSelector::new(
                "search-fixture:query",
                KucInteractionActionClass::TextInput,
            ))
            .expect("query click");
        for _ in 0..3 {
            let mut input = egui::RawInput::default();
            click
                .apply_to_raw_input_once(&mut input)
                .expect("apply click");
            let (current, _) = frame(&context, &mut root, input.events);
            if let Some(next) = click
                .advance(current.interaction_locator())
                .expect("advance")
            {
                click = next;
            } else {
                break;
            }
        }
        let (_, output) = frame(&context, &mut root, Vec::new());
        assert_query_focus(&output);
        root.synchronize_with_lease(lease_with_results(
            2,
            Rc::clone(&owner),
            result_count,
            active_index,
        ))
        .expect("empty-result projection");
        frame(&context, &mut root, Vec::new());
        let before = owner.borrow().active;
        frame(&context, &mut root, key(egui::Key::ArrowUp, false));
        {
            let state = owner.borrow();
            assert_eq!(state.active, before);
            assert_eq!(state.query, query);
            assert_eq!(state.document_changes, 0);
        }
        frame(&context, &mut root, key(egui::Key::ArrowDown, false));
        let state = owner.borrow();
        assert_eq!(state.active, before);
        assert_eq!(state.query, query);
        assert_eq!(state.navigations, 2);
        assert_eq!(state.document_changes, 0);
    }
}

#[test]
fn full_root_body_focus_does_not_route_search_navigation() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let owner = Rc::new(RefCell::new(SearchOwner::default()));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, Rc::clone(&owner)))
        .expect("root");
    frame(&context, &mut root, Vec::new());
    let body_click = vec![
        egui::Event::PointerMoved(egui::pos2(120.0, 300.0)),
        egui::Event::PointerButton {
            pos: egui::pos2(120.0, 300.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::PointerButton {
            pos: egui::pos2(120.0, 300.0),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ];
    frame(&context, &mut root, body_click);
    frame(&context, &mut root, key(egui::Key::ArrowUp, false));
    assert_eq!(owner.borrow().navigations, 0);
}
