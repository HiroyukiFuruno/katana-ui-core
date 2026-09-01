use super::*;

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
