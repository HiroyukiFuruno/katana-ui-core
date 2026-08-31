use super::*;

#[test]
fn render_overlay_group_popup_submits_rename_when_text_is_entered() {
    let rename_path = Rc::new(std::cell::RefCell::new(Vec::new()));
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group-popup"),
                TabStripText::new("group"),
            )
            .popup(
                TabStripGroupPopupPresentation::new()
                    .rename_placeholder(TabStripText::new("rename group")),
            ),
        );
    let mut state = build_state_from_lease(
        TabStripProjectionLease::new(projection)
            .with_proposal_port(RecordingPort::new(Rc::clone(&rename_path))),
    );
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_string(),
        anchor: egui::pos2(10.0, 10.0),
        submenu_path: Vec::new(),
        rename: Some(Box::new(TabStripRenameDraft::new("before", "rename group"))),
    };

    let context = egui::Context::default();
    let mut root_output = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            events: vec![
                egui::Event::Text("Renamed".to_owned()),
                egui::Event::Key {
                    key: egui::Key::Enter,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            ..egui::RawInput::default()
        },
        |ui| {
            root_output = Some(
                state
                    .show(ui)
                    .expect("rename overlay should render and submit"),
            );
        },
    );
    platform_output.textures_delta.clear();

    assert!(
        root_output
            .expect("root output should be produced")
            .overlay_paint_plan
            .is_none()
    );
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
    let operations = rename_path.borrow();
    assert!(
        operations
            .iter()
            .any(|operation| *operation == TabStripProposalOperationClass::RenameGroup)
    );
}

#[test]
fn render_overlay_is_fail_closed_when_group_popup_path_is_missing() {
    let mut state = build_state();
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "missing-path".to_string(),
        anchor: egui::pos2(10.0, 10.0),
        submenu_path: Vec::new(),
        rename: None,
    };

    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group-no-path"),
                TabStripText::new("group"),
            )
            .tab(TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"group-tab"),
                TabStripText::new("tab"),
            )),
        );

    let context = egui::Context::default();
    let mut output = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        output = state
            .render_overlay(ui, &projection)
            .expect("overlay render should fail-closed when path is missing");
    });
    platform_output.textures_delta.clear();

    assert!(output.is_none());
}

#[test]
fn render_overlay_is_fail_closed_when_group_popup_missing_popup_presentation() {
    let mut state = build_state();
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_string(),
        anchor: egui::pos2(10.0, 10.0),
        submenu_path: Vec::new(),
        rename: None,
    };

    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"group-missing-popup"),
            TabStripText::new("group"),
        ));

    let context = egui::Context::default();
    let mut output = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        output = state
            .render_overlay(ui, &projection)
            .expect("overlay render should fail-closed when popup is missing");
    });
    platform_output.textures_delta.clear();

    assert!(output.is_none());
}

#[test]
fn render_overlay_group_popup_with_swatch_entries_renders_prefix_and_menu() {
    let mut state = build_state();
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group-with-swatch"),
                TabStripText::new("group"),
            )
            .swatch(TabStripSwatchDescriptor::new(
                TabStripSwatchTarget::from_opaque_bytes(b"swatch"),
                RgbaColor::new(1, 2, 3, 255),
            ))
            .popup(
                TabStripGroupPopupPresentation::new().entry(TabStripMenuEntry::action(
                    TabStripText::new("action"),
                    TabStripText::new("action"),
                    TabStripMenuOperation::RequestClose,
                )),
            ),
        );

    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_string(),
        anchor: egui::pos2(12.0, 12.0),
        submenu_path: Vec::new(),
        rename: None,
    };

    let context = egui::Context::default();
    let mut paint_plan = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(220.0, 140.0),
            )),
            ..Default::default()
        },
        |ui| {
            paint_plan = state
                .render_overlay(ui, &projection)
                .expect("group popup with swatch should render");
        },
    );
    platform_output.textures_delta.clear();

    let Some(plan) = paint_plan else {
        panic!("expected overlay plan");
    };
    assert!(plan.surface_bounds.width > 0);
    assert!(plan.surface_bounds.height > 0);
}

#[test]
fn render_overlay_propagates_real_missing_port_after_pointer_press_and_release() {
    let entry = || {
        TabStripMenuEntry::action(
            TabStripText::new("close"),
            TabStripText::new("close"),
            TabStripMenuOperation::RequestClose,
        )
    };
    let projection = || {
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr")).tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab-with-missing-port-overlay"),
                TabStripText::new("tab"),
            )
            .context_menu(
                crate::text_command_surface::TabStripContextMenuPresentation::new().entry(entry()),
            ),
        )
    };
    let mut state = build_state_from_lease(TabStripProjectionLease::new(projection()));
    let projection = projection();
    state.overlay = TabStripOverlayState::TabMenu {
        path: "root-tab-0".to_owned(),
        anchor: egui::pos2(10.0, 10.0),
        submenu_path: Vec::new(),
    };

    let context = egui::Context::default();
    let mut first = None;
    let mut output = context.run_ui(overlay_input(Vec::new()), |ui| {
        first = state
            .render_overlay(ui, &projection)
            .expect("initial overlay frame should render");
    });
    output.textures_delta.clear();
    let pointer = first_overlay_row_center(&first.expect("initial overlay should remain open"));

    let mut output = context.run_ui(
        overlay_input(vec![
            egui::Event::PointerMoved(pointer),
            egui::Event::PointerButton {
                pos: pointer,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ]),
        |ui| {
            state
                .render_overlay(ui, &projection)
                .expect("pointer press should be retained until release")
                .expect("overlay should remain open after press");
        },
    );
    output.textures_delta.clear();

    let mut observed = None;
    let mut output = context.run_ui(
        overlay_input(vec![
            egui::Event::PointerMoved(pointer),
            egui::Event::PointerButton {
                pos: pointer,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ]),
        |ui| observed = Some(state.render_overlay(ui, &projection)),
    );
    output.textures_delta.clear();

    assert!(matches!(
        observed.expect("release frame should execute the overlay"),
        Err(TabStripRetainedError::MissingPort)
    ));
}
