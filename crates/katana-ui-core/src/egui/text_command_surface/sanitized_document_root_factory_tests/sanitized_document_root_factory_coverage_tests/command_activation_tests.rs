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
    };
    assert_command_forwarded_once(&frame, &calls, &mut forwarder);
}
