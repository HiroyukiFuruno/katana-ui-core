use super::*;
use crate::text_command_surface::{
    TabStripContextMenuPresentation, TabStripCorrelation, TabStripMenuEntry, TabStripMenuOperation,
    TabStripProjection, TabStripProjectionLease, TabStripProposalPort, TabStripProposalPortError,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use std::sync::Arc;

fn build_state_from_lease(lease: TabStripProjectionLease) -> TabStripRetainedState {
    let config = katana_ui_core_text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(katana_ui_core_text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    TabStripRetainedState::from_lease(lease, catalog, config)
        .expect("tab strip retained state should be constructible")
}

fn build_state() -> TabStripRetainedState {
    build_state_from_lease(TabStripProjectionLease::new(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    )))
}

struct NullPort;

impl TabStripProposalPort for NullPort {
    fn forward_proposal(
        &mut self,
        _proposal: crate::text_command_surface::tab_strip_proposal_port::TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        Ok(())
    }
}

fn overlay_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        events,
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(480.0, 240.0),
        )),
        ..Default::default()
    }
}

fn first_row_center(operations: &[TabStripPaintOperation]) -> egui::Pos2 {
    let Some(bounds) = operations
        .iter()
        .find_map(|operation| match operation.kind {
            TabStripPaintOperationKind::Fill { bounds, color_rgba }
                if color_rgba == OVERLAY_BACKGROUND_RGBA
                    && bounds.width < OVERLAY_WIDTH_PX.round() as u32 =>
            {
                Some(bounds)
            }
            _ => None,
        })
    else {
        panic!("overlay row fill operation should be present");
    };
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

#[test]
fn overlay_panel_uses_disabled_row_sense() {
    let mut state = build_state();
    let entries = vec![
        TabStripMenuEntry::action(
            TabStripText::new("disabled"),
            TabStripText::new("disabled"),
            TabStripMenuOperation::RequestClose,
        )
        .enabled(false),
    ];

    let context = egui::Context::default();
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let mut operations = Vec::new();
        let panel = state
            .render_overlay_panel(
                ui,
                &entries,
                "root",
                egui::pos2(10.0, 10.0),
                0,
                &mut operations,
            )
            .expect("disabled row should render");
        assert!(!panel.closed);
        assert!(panel.open_submenu.is_none());
        assert!(!operations.is_empty());
    });
    platform_output.textures_delta.clear();
}

#[test]
fn overlay_tree_truncates_submenu_path_when_deeper_node_is_missing() {
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
    let mut outcome = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let result = state
            .render_overlay_tree(
                ui,
                &entries,
                "root",
                egui::pos2(10.0, 10.0),
                vec![0, 7],
                &[],
            )
            .expect("overlay traversal should be recoverable");
        outcome = Some(result);
    });
    platform_output.textures_delta.clear();

    assert_eq!(
        outcome.expect("run_ui produced outcome").submenu_path,
        vec![0]
    );
}

#[test]
fn overlay_tree_is_fail_closed_by_external_click_when_no_protected_bounds() {
    let mut state = build_state();
    let entries = vec![TabStripMenuEntry::action(
        TabStripText::new("outside click target"),
        TabStripText::new("outside click target"),
        TabStripMenuOperation::RequestClose,
    )];
    let context = egui::Context::default();
    let mut outcome = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::PointerButton {
                pos: egui::pos2(1.0, 1.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(200.0, 120.0),
            )),
            ..Default::default()
        },
        |ui| {
            let result = state
                .render_overlay_tree(
                    ui,
                    &entries,
                    "root",
                    egui::pos2(100.0, 100.0),
                    Vec::new(),
                    &[],
                )
                .expect("overlay traversal should be recoverable");
            outcome = Some(result);
        },
    );
    platform_output.textures_delta.clear();

    assert!(outcome.expect("run_ui produced outcome").closed);
}

#[test]
fn overlay_tree_is_fail_closed_when_click_is_outside_panel_and_protected_bounds() {
    let mut state = build_state();
    let entries = vec![TabStripMenuEntry::action(
        TabStripText::new("outside protected bounds"),
        TabStripText::new("outside protected bounds"),
        TabStripMenuOperation::RequestClose,
    )];
    let protected = egui::Rect::from_min_size(egui::pos2(20.0, 20.0), egui::vec2(20.0, 20.0));
    let context = egui::Context::default();
    let mut outcome = None;
    let mut platform_output = context.run_ui(
        overlay_input(vec![egui::Event::PointerButton {
            pos: egui::pos2(1.0, 1.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }]),
        |ui| {
            outcome = Some(
                state
                    .render_overlay_tree(
                        ui,
                        &entries,
                        "root-with-protected-bounds",
                        egui::pos2(100.0, 100.0),
                        Vec::new(),
                        &[protected],
                    )
                    .expect("overlay traversal should be recoverable"),
            );
        },
    );
    platform_output.textures_delta.clear();

    assert!(outcome.expect("run_ui produced outcome").closed);
}

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
                egui::vec2(200.0, 120.0),
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
                    egui::vec2(200.0, 120.0),
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

#[test]
fn overlay_tree_with_no_entries_returns_empty_closed_surface() {
    let mut state = build_state();
    let context = egui::Context::default();
    let entries = vec![TabStripMenuEntry::action(
        TabStripText::new("present"),
        TabStripText::new("present"),
        TabStripMenuOperation::RequestClose,
    )];
    let mut populated = None;
    let mut output = context.run_ui(overlay_input(Vec::new()), |ui| {
        populated = Some(
            state
                .render_overlay_tree(
                    ui,
                    &entries,
                    "empty-transition",
                    egui::pos2(10.0, 10.0),
                    Vec::new(),
                    &[],
                )
                .expect("populated overlay tree should render before removal"),
        );
    });
    output.textures_delta.clear();
    assert!(
        populated
            .expect("populated frame should produce an outcome")
            .paint_plan
            .surface_bounds
            .width
            > 0
    );

    let mut outcome = None;
    let mut output = context.run_ui(overlay_input(Vec::new()), |ui| {
        outcome = Some(
            state
                .render_overlay_tree(
                    ui,
                    &[],
                    "empty-transition",
                    egui::pos2(10.0, 10.0),
                    Vec::new(),
                    &[],
                )
                .expect("empty overlay tree should remain deterministic"),
        );
    });
    output.textures_delta.clear();

    let outcome = outcome.expect("overlay outcome should be produced");
    assert!(outcome.closed);
    assert_eq!(outcome.submenu_path, Vec::<usize>::new());
    assert_eq!(outcome.paint_plan.surface_bounds, UiRect::new(0, 0, 0, 0));
    assert!(outcome.paint_plan.operations.is_empty());
}

#[test]
fn overlay_tree_opens_hovered_submenu_across_real_pointer_frames() {
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
    let mut first = None;
    let mut output = context.run_ui(overlay_input(Vec::new()), |ui| {
        first = Some(
            state
                .render_overlay_tree(
                    ui,
                    &entries,
                    "submenu-tree",
                    egui::pos2(10.0, 10.0),
                    Vec::new(),
                    &[],
                )
                .expect("initial submenu frame should render"),
        );
    });
    output.textures_delta.clear();
    let cursor = first_row_center(
        &first
            .expect("initial frame should produce a plan")
            .paint_plan
            .operations,
    );

    let mut pointer_moved = None;
    let mut output = context.run_ui(
        overlay_input(vec![egui::Event::PointerMoved(cursor)]),
        |ui| {
            pointer_moved = Some(
                state
                    .render_overlay_tree(
                        ui,
                        &entries,
                        "submenu-tree",
                        egui::pos2(10.0, 10.0),
                        Vec::new(),
                        &[],
                    )
                    .expect("hovered submenu frame should render its child panel"),
            );
        },
    );
    output.textures_delta.clear();
    assert!(pointer_moved.is_some());

    let mut hovered = None;
    let mut output = context.run_ui(
        overlay_input(vec![
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
        ]),
        |ui| {
            hovered = Some(
                state
                    .render_overlay_tree(
                        ui,
                        &entries,
                        "submenu-tree",
                        egui::pos2(10.0, 10.0),
                        Vec::new(),
                        &[],
                    )
                    .expect("settled hover frame should render its child panel"),
            );
        },
    );
    output.textures_delta.clear();

    let hovered = hovered.expect("hover frame should produce a plan");
    assert_eq!(hovered.submenu_path, vec![0]);
    assert!(hovered.paint_plan.surface_bounds.width > OVERLAY_WIDTH_PX.round() as u32);
}

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
                egui::vec2(200.0, 120.0),
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
