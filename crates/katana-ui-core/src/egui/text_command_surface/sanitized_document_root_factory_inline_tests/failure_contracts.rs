use super::command_support::floating_command_input;
use super::root_frame_support::*;
use super::support::*;
use super::*;

#[test]
fn real_invalid_theme_error_maps_to_the_public_render_error() {
    let mut theme = crate::theme::ThemeSnapshot::dark();
    theme.colors.retain(|token| token.name != "accent");
    let style_error =
        crate::egui::text_command_surface::TextCommandSurfaceStyle::from_theme(&theme)
            .expect_err("the actual style route rejects a missing accent token");
    let render_error =
        super::super::super::sanitized_document_root_process::render_style_error(style_error);
    let mut process_theme = crate::theme::ThemeSnapshot::dark();
    process_theme.colors.retain(|token| token.name != "accent");
    let process_error = crate::egui::text_command_surface::TextCommandSurfaceStyle::from_theme(
        &process_theme,
    )
    .map_err(
        super::super::super::sanitized_document_root_process::SanitizedDocumentRootProcessError::from,
    )
    .expect_err("the actual style route rejects a missing accent token");

    assert!(matches!(
        SanitizedDocumentRootFactoryError::from(process_error),
        SanitizedDocumentRootFactoryError::Render(message)
            if message.contains("accent")
    ));
    assert!(render_error.contains("accent"));
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
