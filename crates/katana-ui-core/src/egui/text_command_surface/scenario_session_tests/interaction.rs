#[test]
fn physical_search_trace_routes_query_non_value_and_close_events() {
    let session =
        FullTextCommandSurfaceScenarioSession::new(FullTextCommandSurfaceScenarioId::Find);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(session.retain_lease().expect("initial session lease"))
        .expect("session root retains");
    let context = egui::Context::default();
    let initial = render_and_forward(
        &context,
        &mut root,
        session.synchronize_lease().expect("initial frame lease"),
        egui::RawInput::default(),
    );
    let match_case = initial
        .search_record
        .as_ref()
        .and_then(|record| {
            record
                .controls
                .iter()
                .find(|control| control.control_id.ends_with(":match-case"))
        })
        .expect("search exposes match-case control")
        .bounds;
    let pointer = egui::pos2(
        match_case.x as f32 + match_case.width as f32 / 2.0,
        match_case.y as f32 + match_case.height as f32 / 2.0,
    );
    let pointer_event = |pressed| egui::Event::PointerButton {
        pos: pointer,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    let _ = render_and_forward(
        &context,
        &mut root,
        session.synchronize_lease().expect("match-case press lease"),
        egui::RawInput {
            events: vec![egui::Event::PointerMoved(pointer), pointer_event(true)],
            ..egui::RawInput::default()
        },
    );
    let toggled = render_and_forward(
        &context,
        &mut root,
        session
            .synchronize_lease()
            .expect("match-case release lease"),
        egui::RawInput {
            events: vec![pointer_event(false)],
            ..egui::RawInput::default()
        },
    );
    assert_eq!(
        session
            .state
            .borrow()
            .search_options
            .map(|options| options.match_case),
        Some(true)
    );

    let mut continuation = toggled
        .interaction_locator()
        .begin_search_trace()
        .expect("search continuation");

    for _ in 0..7 {
        let mut input = egui::RawInput::default();
        continuation
            .apply_to_raw_input_once(&mut input)
            .expect("search step applies");
        let output = render_and_forward(
            &context,
            &mut root,
            session.synchronize_lease().expect("search frame lease"),
            input,
        );
        match continuation
            .advance(output.interaction_locator())
            .expect("search step advances")
        {
            Some(next) => continuation = next,
            None => break,
        }
    }

    assert!(
        session
            .state
            .borrow()
            .search_query
            .as_deref()
            .is_some_and(|query| query.contains("入力 ⭐️"))
    );
    assert!(
        session.state.borrow().search_visible == Some(false),
        "the close receipt must update the next session projection"
    );
    assert!(
        session.state.borrow().result_position.is_none(),
        "navigation receipts must not invent a result position"
    );

    session.reopen_search();
    let reopened = render_and_forward(
        &context,
        &mut root,
        session.synchronize_lease().expect("reopened session lease"),
        egui::RawInput::default(),
    );
    assert!(
        reopened.search_record.is_some(),
        "explicit reopen restores search"
    );
    assert!(
        reopened
            .search_record
            .as_ref()
            .expect("reopened search record")
            .controls
            .iter()
            .any(|control| control.control_id.ends_with(":match-case") && control.active)
    );
    assert!(
        !reopened.accesskit_text_input_nodes.is_empty(),
        "the reopened frame retains current AccessKit evidence"
    );
    assert!(
        reopened.evidence_composite.non_transparent_pixel_count > 0,
        "the reopened frame retains KUC-owned frame evidence"
    );
}

#[test]
fn physical_replace_input_is_extracted_from_the_actual_root_context() {
    let mut presentation = super::scenario::presentation(FullTextCommandSurfaceScenarioId::Find);
    let search = presentation.search.as_mut().expect("find search");
    search.value.replace_mode = ReplaceMode::Visible;
    search.value.capabilities.replace = CommandChromeCapability::available();
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"scenario-replace-target",
        presentation,
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("replace token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("replace root retains");
    let context = egui::Context::default();
    let initial = render_current(&context, &mut root, egui::RawInput::default());
    let bounds = initial
        .search_record
        .as_ref()
        .and_then(|record| record.replace.as_ref())
        .expect("visible replace input")
        .frame
        .content_bounds;
    let pointer = egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    );
    let pointer_event = |pressed| egui::Event::PointerButton {
        pos: pointer,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    let _ = render_current(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![egui::Event::PointerMoved(pointer), pointer_event(true)],
            ..egui::RawInput::default()
        },
    );
    let _ = render_current(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_event(false)],
            ..egui::RawInput::default()
        },
    );
    let changed = render_current(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![egui::Event::Text(String::from("replacement ⭐️"))],
            ..egui::RawInput::default()
        },
    );
    let update = ScenarioSessionUpdate::from_context(&changed.events().current_context());

    assert!(
        update
            .replace_value
            .as_deref()
            .is_some_and(|value| value.contains("replacement ⭐️"))
    );
}
