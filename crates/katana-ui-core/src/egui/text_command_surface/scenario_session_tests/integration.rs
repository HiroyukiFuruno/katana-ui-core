#[test]
fn text_change_forwarding_updates_the_next_session_projection() {
    let session = FullTextCommandSurfaceScenarioSession::new(
        FullTextCommandSurfaceScenarioId::ResizeScrollIme,
    );
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::ResizeScrollIme)
        .expect("scenario stages issue");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(session.retain_lease().expect("initial session lease"))
        .expect("session root retains");
    let context = egui::Context::default();

    for stage in scenario.stages().iter().take(2) {
        let mut input = egui::RawInput::default();
        stage.apply_to(&mut input);
        let _ = render_and_forward(
            &context,
            &mut root,
            session.synchronize_lease().expect("current session lease"),
            input,
        );
    }

    let mut input = egui::RawInput::default();
    input
        .events
        .push(egui::Event::Text(String::from("session text ⭐️")));
    let _ = render_and_forward(
        &context,
        &mut root,
        session
            .synchronize_lease()
            .expect("session lease synchronizes"),
        input,
    );

    let updated = render_and_forward(
        &context,
        &mut root,
        session
            .synchronize_lease()
            .expect("updated session lease synchronizes"),
        egui::RawInput::default(),
    );
    assert!(
        updated
            .evidence_text
            .record
            .frame
            .layout_identity
            .contains("session text ⭐️"),
        "a forwarded text change must be retained by the next opaque projection"
    );
}

#[test]
fn physical_selection_is_retained_by_the_next_scenario_projection() {
    let session =
        FullTextCommandSurfaceScenarioSession::new(FullTextCommandSurfaceScenarioId::Selection);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(session.retain_lease().expect("initial session lease"))
        .expect("session root retains");
    let context = egui::Context::default();
    let motion_input = || {
        let mut input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..egui::RawInput::default()
        };
        input.viewports.insert(
            egui::ViewportId::ROOT,
            egui::ViewportInfo {
                native_pixels_per_point: Some(1.0),
                ..egui::ViewportInfo::default()
            },
        );
        input
    };
    let initial = render_and_forward(
        &context,
        &mut root,
        session.synchronize_lease().expect("initial frame lease"),
        motion_input(),
    );
    let mut continuation = initial
        .interaction_locator()
        .begin_text_selection()
        .expect("selection continuation");

    for _ in 0..5 {
        let mut input = motion_input();
        continuation
            .apply_to_raw_input_once(&mut input)
            .expect("selection step applies");
        let output = render_current_and_forward(&context, &mut root, input);
        match continuation
            .advance(output.interaction_locator())
            .expect("selection step advances")
        {
            Some(next) => continuation = next,
            None => break,
        }
    }

    let selection = session
        .state
        .borrow()
        .selection
        .expect("physical selection is retained by the session router");
    assert!(selection.0 < selection.1);
}
