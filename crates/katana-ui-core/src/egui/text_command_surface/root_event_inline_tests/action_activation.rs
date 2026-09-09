#[test]
fn command_activation_predicate_binds_target_and_toolbar_slot() {
    let mut payload = RootEventPayload::empty();
    payload.toolbar = Some(vec![CommandChromeToolbarEvent::CommandActivated {
        action_id: "toolbar-action".into(),
    }]);
    payload.floating = Some(vec![FloatingCommandToolbarEvent::Toolbar {
        event: CommandChromeToolbarEvent::CommandActivated {
            action_id: "floating-action".into(),
        },
    }]);
    payload.context_menu = Some(vec![crate::molecule::selection::ContextMenuEvent::Opened {
        anchor: crate::render_model::UiContextMenuAnchor::Pointer { x: 1, y: 2 },
        placement_used: crate::render_model::UiContextMenuPlacement::BelowStart,
    }]);
    let batch = EguiTextCommandSurfaceRootEventBatch::new(payload, "batch".to_owned());

    assert!(batch.contains_command_activation("toolbar-action", false));
    assert!(batch.contains_command_activation("floating-action", true));
    assert!(!batch.contains_command_activation("other-action", false));
    assert!(!batch.contains_command_activation("toolbar-action", true));
    assert!(batch.contains_context_menu_opened());

    let _ = batch.transport.borrow_mut().take();
    assert!(!batch.contains_command_activation("toolbar-action", false));
    assert!(!batch.contains_context_menu_opened());
}
