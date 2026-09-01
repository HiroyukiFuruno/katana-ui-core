use super::*;

#[test]
fn overlay_tree_propagates_real_missing_port_after_pointer_press_and_release() {
    let entry = || {
        TabStripMenuEntry::action(
            TabStripText::new("close"),
            TabStripText::new("close"),
            TabStripMenuOperation::RequestClose,
        )
    };
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab-with-missing-port-menu"),
                TabStripText::new("tab"),
            )
            .context_menu(TabStripContextMenuPresentation::new().entry(entry())),
        );
    let mut state = build_state_from_lease(TabStripProjectionLease::new(projection));
    let entries = vec![entry()];
    let context = egui::Context::default();
    let mut first = None;
    let mut output = context.run_ui(overlay_input(Vec::new()), |ui| {
        first = Some(
            state
                .render_overlay_tree(
                    ui,
                    &entries,
                    "root-tab-0-menu",
                    egui::pos2(10.0, 10.0),
                    Vec::new(),
                    &[],
                )
                .expect("initial menu frame should render"),
        );
    });
    output.textures_delta.clear();
    let pointer = first_row_center(
        &first
            .expect("initial frame should produce a plan")
            .paint_plan
            .operations,
    );

    let press = egui::Event::PointerButton {
        pos: pointer,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    };
    let mut output = context.run_ui(
        overlay_input(vec![egui::Event::PointerMoved(pointer), press]),
        |ui| {
            state
                .render_overlay_tree(
                    ui,
                    &entries,
                    "root-tab-0-menu",
                    egui::pos2(10.0, 10.0),
                    Vec::new(),
                    &[],
                )
                .expect("pointer press should be retained until release");
        },
    );
    output.textures_delta.clear();

    let release = egui::Event::PointerButton {
        pos: pointer,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    };
    let mut observed = None;
    let mut output = context.run_ui(
        overlay_input(vec![egui::Event::PointerMoved(pointer), release]),
        |ui| {
            observed = Some(state.render_overlay_tree(
                ui,
                &entries,
                "root-tab-0-menu",
                egui::pos2(10.0, 10.0),
                Vec::new(),
                &[],
            ));
        },
    );
    output.textures_delta.clear();

    assert!(matches!(
        observed.expect("release frame should execute the overlay tree"),
        Err(TabStripRetainedError::MissingPort)
    ));
}

#[test]
fn overlay_panel_propagates_real_empty_label_raster_failure_after_warm_frame() {
    let mut state = build_state();
    let context = egui::Context::default();
    let valid = vec![TabStripMenuEntry::action(
        TabStripText::new("valid"),
        TabStripText::new("valid"),
        TabStripMenuOperation::RequestClose,
    )];
    let mut output = context.run_ui(overlay_input(Vec::new()), |ui| {
        state
            .render_overlay_panel(
                ui,
                &valid,
                "raster-transition",
                egui::pos2(10.0, 10.0),
                0,
                &mut Vec::new(),
            )
            .expect("valid label should warm the real overlay raster route");
    });
    output.textures_delta.clear();

    let invalid = vec![TabStripMenuEntry::action(
        TabStripText::new(""),
        TabStripText::new("empty label"),
        TabStripMenuOperation::RequestClose,
    )];
    let mut observed = None;
    let mut output = context.run_ui(overlay_input(Vec::new()), |ui| {
        observed = Some(state.render_overlay_panel(
            ui,
            &invalid,
            "raster-transition",
            egui::pos2(10.0, 10.0),
            0,
            &mut Vec::new(),
        ));
    });
    output.textures_delta.clear();

    assert!(matches!(
        observed.expect("invalid frame should execute the overlay panel"),
        Err(TabStripRetainedError::Raster(_))
    ));
}

#[test]
fn overlay_tree_resets_submenu_path_when_childless_submenu_is_targeted() {
    let mut state = build_state();
    let entries = vec![TabStripMenuEntry::submenu(
        TabStripText::new("submenu"),
        TabStripText::new("submenu"),
    )];

    let context = egui::Context::default();
    let mut outcome = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        outcome = Some(
            state
                .render_overlay_tree(ui, &entries, "root", egui::pos2(10.0, 10.0), vec![0], &[])
                .expect("childless submenu should stay inside tree bounds"),
        );
    });
    platform_output.textures_delta.clear();

    let outcome = outcome.expect("overlay outcome should be produced");
    assert_eq!(outcome.submenu_path, Vec::<usize>::new());
    assert!(!outcome.closed);
    assert!(outcome.paint_plan.surface_bounds.width > 0);
    assert!(outcome.paint_plan.surface_bounds.height > 0);
}

#[test]
fn overlay_panel_closes_when_action_row_is_activated() {
    let close_entry = || {
        TabStripMenuEntry::action(
            TabStripText::new("close"),
            TabStripText::new("close"),
            TabStripMenuOperation::RequestClose,
        )
    };
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab-with-menu"),
                TabStripText::new("tab"),
            )
            .context_menu(TabStripContextMenuPresentation::new().entry(close_entry())),
        );
    let mut state = build_state_from_lease(
        TabStripProjectionLease::new(projection).with_proposal_port(NullPort),
    );
    let entries = vec![close_entry()];

    let row_x = 10.0 + 8.0 + 20.0;
    let row_y = 10.0 + 8.0 + 14.0;
    let pointer = egui::pos2(row_x, row_y);

    let context = egui::Context::default();
    let mut platform_output = context.run_ui(
        egui::RawInput {
            events: vec![
                egui::Event::PointerButton {
                    pos: pointer,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos: pointer,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(
                    SUBMENU_TEST_VIEWPORT_WIDTH_PX,
                    SUBMENU_TEST_VIEWPORT_HEIGHT_PX,
                ),
            )),
            ..Default::default()
        },
        |ui| {
            let mut operations = Vec::new();
            let panel = state
                .render_overlay_panel(
                    ui,
                    &entries,
                    "root-tab-0-menu",
                    egui::pos2(10.0, 10.0),
                    0,
                    &mut operations,
                )
                .expect("activated overlay entry should render");
            assert!(panel.closed);
            assert!(!operations.is_empty());
        },
    );
    platform_output.textures_delta.clear();
}
