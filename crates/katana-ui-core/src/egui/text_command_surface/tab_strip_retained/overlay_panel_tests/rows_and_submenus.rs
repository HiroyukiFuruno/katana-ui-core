use super::*;

#[test]
fn overlay_panel_records_separator_and_checked_rows() {
    let mut state = build_state();
    let entries = vec![
        TabStripMenuEntry::separator(),
        TabStripMenuEntry::action(
            TabStripText::new("checked"),
            TabStripText::new("checked"),
            TabStripMenuOperation::RequestClose,
        )
        .checked(true),
    ];

    let context = egui::Context::default();
    let mut panel = None;
    let mut operations = Vec::new();
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        panel = Some(
            state
                .render_overlay_panel(
                    ui,
                    &entries,
                    "root",
                    egui::pos2(10.0, 10.0),
                    0,
                    &mut operations,
                )
                .expect("overlay panel should render"),
        );
    });
    platform_output.textures_delta.clear();

    let panel = panel.expect("panel rendered");
    assert_eq!(panel.row_positions.len(), 2);
    assert_eq!(panel.closed, false);
    assert!(matches!(panel.open_submenu, None));
    assert!(operations.len() >= 4);
}

#[test]
fn overlay_panel_opens_submenu_when_child_entry_is_hovered() {
    let mut state = build_state();
    let entries = vec![
        TabStripMenuEntry::submenu(TabStripText::new("submenu"), TabStripText::new("submenu"))
            .child(TabStripMenuEntry::action(
                TabStripText::new("leaf"),
                TabStripText::new("leaf"),
                TabStripMenuOperation::RequestClose,
            )),
    ];

    let context = egui::Context::default();
    let mut panel = None;
    let mut operations = Vec::new();
    let mut platform_output = context.run_ui(
        egui::RawInput {
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
            panel = Some(
                state
                    .render_overlay_panel(
                        ui,
                        &entries,
                        "root",
                        egui::pos2(10.0, 10.0),
                        0,
                        &mut operations,
                    )
                    .expect("overlay panel should render"),
            );
        },
    );
    platform_output.textures_delta.clear();

    let _panel = panel.expect("panel rendered");
    let row = operations
        .iter()
        .filter_map(|entry| match entry.kind {
            TabStripPaintOperationKind::Fill { bounds, .. } => Some(bounds),
            _ => None,
        })
        .nth(1)
        .unwrap_or_else(|| {
            panic!(
                "expected row paint operation; operations: {}",
                operations.len()
            );
        });
    let cursor = egui::pos2(
        (row.x as f32) + (row.width as f32) * 0.5,
        (row.y as f32) + (row.height as f32) * 0.5,
    );
    operations.clear();

    let mut last_frame_output = None;
    for events in [
        vec![egui::Event::PointerMoved(cursor)],
        vec![
            egui::Event::PointerMoved(cursor),
            egui::Event::PointerButton {
                pos: cursor,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: cursor,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        vec![
            egui::Event::PointerButton {
                pos: cursor,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: cursor,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    ]
    .into_iter()
    {
        let mut panel = None;
        operations.clear();
        let mut _platform_output = context.run_ui(
            egui::RawInput {
                events,
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
                panel = Some(
                    state
                        .render_overlay_panel(
                            ui,
                            &entries,
                            "root",
                            egui::pos2(10.0, 10.0),
                            0,
                            &mut operations,
                        )
                        .expect("overlay panel should render"),
                );
            },
        );
        _platform_output.textures_delta.clear();
        last_frame_output = panel;
        if last_frame_output
            .as_ref()
            .is_some_and(|frame| frame.open_submenu.is_some())
        {
            break;
        }
    }
    let Some(panel) = last_frame_output else {
        panic!("overlay panel should render on hover attempts");
    };
    assert!(
        panel.bounds.contains(cursor),
        "panel={:?}, cursor={:?}",
        panel.bounds,
        cursor
    );
    assert_eq!(panel.open_submenu, Some(0));
    assert_eq!(panel.closed, false);
    assert!(!operations.is_empty());
}
