struct LifecycleDispatcher {
    calls: Vec<&'static str>,
    effect_calls: usize,
    effect_failed: bool,
}

impl KucRootEventBatchDispatcher for LifecycleDispatcher {
    type Error = &'static str;

    fn dispatch_text_events(&mut self, _events: Vec<TextSurfaceEvent>) -> Result<(), Self::Error> {
        self.calls.push("text");
        Ok(())
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        self.calls.push("toolbar");
        Ok(())
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        self.calls.push("floating");
        Ok(())
    }

    fn dispatch_search_events(
        &mut self,
        events: Vec<CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        self.calls.push("search");
        assert!(events.iter().any(|event| matches!(
            event,
            CommandChromeSearchEvent::Strip {
                event: katana_ui_core::molecule::structured::SearchControlStripEvent::SearchQueryChanged(value)
            } if value == "needle⭐️"
        )));
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        self.calls.push("context-menu");
        Ok(())
    }

    fn consume_opaque_host_effect_batch(
        &mut self,
        effect_batch: KucOpaqueHostEffectBatch,
    ) -> Result<(), KucOpaqueHostEffectError> {
        self.calls.push("effect");
        self.effect_calls += 1;
        let result = effect_batch.consume_once();
        if self.effect_failed {
            return Err(KucOpaqueHostEffectError);
        }
        result
    }
}

impl KucRootEventBatchForwarder for LifecycleDispatcher {
    type Error = EguiTextCommandSurfaceRootEventBatchDispatchError<&'static str>;

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        transport.dispatch_once(self).map(|_| ())
    }
}

#[test]
fn retained_root_routes_actual_search_payload_after_forwarding_and_consumes_effect_once()
-> Result<(), Box<dyn std::error::Error>> {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use crate::text_command_surface::{
        EguiTextCommandSurfaceCommandFamilyProjection, EguiTextCommandSurfaceHostProjectionEncoder,
        EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfaceHostRootFrame,
        EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootFactory,
        EguiTextCommandSurfaceSearchPresentation, TextCommandSurfaceStyle,
    };
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeCapability, CommandChromeSearchPresentation, CommandChromeText,
        SearchControlCapabilities, SearchControlIcons, SearchControlStrings,
        SearchResultSummaryTemplate,
    };
    use katana_ui_core::molecule::structured::{ReplaceMode, SearchOptions};
    use katana_ui_core::render_model::UiStateId;
    use katana_ui_core::text_surface::{TextSurfaceAccessibilityLabels, TextSurfacePresentation};

    fn label(value: &str) -> CommandChromeText {
        CommandChromeText::new(value, value, value)
    }

    fn search_presentation() -> EguiTextCommandSurfaceSearchPresentation {
        EguiTextCommandSurfaceSearchPresentation {
            state_id: UiStateId::new("root-lifecycle-search"),
            label: String::from("検索と置換"),
            value: CommandChromeSearchPresentation {
                query: String::from("needle"),
                options: SearchOptions::default(),
                result_count: Some(1),
                active_index: Some(0),
                replace_mode: ReplaceMode::Visible,
                replace_value: String::from("replacement"),
                strings: SearchControlStrings {
                    strip: label("検索と置換"),
                    query: label("検索語"),
                    replace: label("置換"),
                    match_case: label("大文字小文字"),
                    whole_word: label("単語"),
                    use_regex: label("正規表現"),
                    previous: label("前へ"),
                    next: label("次へ"),
                    replace_one: label("置換"),
                    replace_all: label("すべて置換"),
                    close: label("閉じる"),
                    result_summary: SearchResultSummaryTemplate {
                        empty: String::from("検索待機"),
                        zero_results: String::from("一致なし"),
                        single_result: String::from("1件"),
                        indexed_result: String::from("{active} / {count}"),
                        count_results: String::from("{count}件"),
                    },
                },
                capabilities: SearchControlCapabilities {
                    regex: CommandChromeCapability::available(),
                    replace: CommandChromeCapability::available(),
                    navigation: CommandChromeCapability::available(),
                    close: CommandChromeCapability::available(),
                },
                icons: SearchControlIcons::default(),
            },
        }
    }

    let presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(UiStateId::new("root-lifecycle-text")),
        text: TextSurfacePresentation {
            value: String::from("body"),
            selection_start: 0,
            selection_end: 0,
            spans: Vec::new(),
            annotations: Vec::new(),
            automatic_gutter: None,
            accessibility_label: String::from("本文"),
            accessibility_actions: TextSurfaceAccessibilityLabels::new(),
            context_target_label: None,
            disabled_reason: None,
            readonly: false,
            disabled: false,
            ime_enabled: true,
            scroll_request: None,
            focus_request: None,
        },
        toolbar: None,
        floating: None,
        search: Some(search_presentation()),
        context_menu: None,
    };
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
        1,
        b"root-lifecycle-target".to_vec(),
        presentation,
        TextCommandSurfaceStyle::standard()?,
        EguiTextCommandSurfaceCommandFamilyProjection::new(None, None),
    )?;

    let router_seen_query = Rc::new(RefCell::new(None::<String>));
    let host_mutations = Rc::new(Cell::new(0));
    let router_seen_query_for_router = Rc::clone(&router_seen_query);
    let host_mutations_for_effect = Rc::clone(&host_mutations);
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        move |context: KucRootEventBatchContext| {
            let query = context.search_events().iter().find_map(|event| match event {
            CommandChromeSearchEvent::Strip {
                event: katana_ui_core::molecule::structured::SearchControlStripEvent::SearchQueryChanged(value),
            } => Some(value.clone()),
            _ => None,
        });
            if let Some(query) = query {
                *router_seen_query_for_router.borrow_mut() = Some(query);
                let host_mutations = Rc::clone(&host_mutations_for_effect);
                return Ok(Some(KucOpaqueHostEffectBatch::from_handler(move || {
                    host_mutations.set(host_mutations.get() + 1);
                    Ok(())
                })));
            }
            Ok(None)
        },
    );
    let mut root = EguiTextCommandSurfaceRootFactory::new().retain_with_lease(lease)?;
    let context = egui::Context::default();
    let mut first_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(ROOT_EVENT_FRAME_WIDTH, ROOT_EVENT_FRAME_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = root.show(ui);
            });
        },
    );
    first_output.textures_delta.clear();
    let mut frame: Option<Result<EguiTextCommandSurfaceHostRootFrame, _>> = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(ROOT_EVENT_FRAME_WIDTH, ROOT_EVENT_FRAME_HEIGHT),
            )),
            events: vec![
                egui::Event::PointerButton {
                    pos: egui::pos2(ROOT_EVENT_INPUT_X, ROOT_EVENT_INPUT_Y),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::PointerButton {
                    pos: egui::pos2(ROOT_EVENT_INPUT_X, ROOT_EVENT_INPUT_Y),
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::Text(String::from("⭐️")),
            ],
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                frame = Some(root.show(ui));
            });
        },
    );
    output.textures_delta.clear();
    let frame = frame.ok_or_else(|| "root frame missing".to_owned())??;
    assert_eq!(router_seen_query.borrow().as_deref(), Some("needle⭐️"));
    assert_eq!(host_mutations.get(), 0);

    let mut dispatcher = LifecycleDispatcher {
        calls: Vec::new(),
        effect_calls: 0,
        effect_failed: false,
    };
    assert!(frame.forward_events_once(&mut dispatcher).is_ok());
    assert_eq!(
        dispatcher.calls,
        vec![
            "text",
            "toolbar",
            "floating",
            "search",
            "context-menu",
            "effect"
        ]
    );
    assert_eq!(dispatcher.effect_calls, 1);
    assert_eq!(host_mutations.get(), 1);
    assert!(frame.forward_events_once(&mut dispatcher).is_err());
    assert_eq!(dispatcher.effect_calls, 1);
    assert_eq!(host_mutations.get(), 1);

    let failing_effect = KucOpaqueHostEffectBatch::from_handler(|| Err(KucOpaqueHostEffectError));
    let mut failing_dispatcher = LifecycleDispatcher {
        calls: Vec::new(),
        effect_calls: 0,
        effect_failed: true,
    };
    assert!(
        failing_dispatcher
            .consume_opaque_host_effect_batch(failing_effect)
            .is_err()
    );
    assert_eq!(failing_dispatcher.calls, vec!["effect"]);

    Ok(())
}
