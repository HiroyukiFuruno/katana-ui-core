use super::*;

fn run_menu() -> Result<(UiRect, usize, String), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter =
        EguiContextMenuAdapter::new(crate::text_raster::PlatformTextRasterConfig::default())?;
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("format", "整形 ⭐️"),
            ContextMenuPresentationItem::action("code", "コード種別").child(
                ContextMenuPresentationItem::action("opaque-code-kind", "code kind"),
            ),
            ContextMenuPresentationItem {
                id: "disabled".to_owned(),
                label: "利用不可".to_owned(),
                accessibility_label: "利用不可".to_owned(),
                icon: None,
                enabled: false,
                checked: false,
                kind: ContextMenuItemKind::Action,
                children: Vec::new(),
            },
        ],
    });
    adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
        CONTEXT_X,
        CONTEXT_Y,
        UiTextSelectionRange::caret(0),
        UiRect::new(0, 0, FRAME_WIDTH_PX as u32, FRAME_HEIGHT_PX as u32),
    ));
    let mut output = None;
    let mut frame_output = context.run_ui(frame_input(), |ui| {
        output = Some(adapter.show(ui, &raster_style(), &paint_style()));
    });
    frame_output.textures_delta.clear();
    let output = output.ok_or_else(|| std::io::Error::other("actual egui frame did not run"))??;
    let record = output
        .record
        .ok_or_else(|| std::io::Error::other("visible menu record was absent"))?;
    let artifact = output
        .artifact
        .ok_or_else(|| std::io::Error::other("visible menu artifact was absent"))?;
    let composite = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            0,
            0,
            FRAME_WIDTH_PX as u32,
            FRAME_HEIGHT_PX as u32,
        )),
        plans: &[ArtifactPaintPlanRef::ContextMenu(&artifact.paint_plan)],
    })?;
    Ok((
        record.bounds,
        composite.non_transparent_pixel_count,
        composite.pixel_hash,
    ))
}

#[test]
fn actual_context_menu_adapter_keeps_opaque_tree_and_composites_repeatably()
-> Result<(), Box<dyn std::error::Error>> {
    let first = run_menu()?;
    let second = run_menu()?;
    assert_eq!(first, second);
    assert!(first.0.width > 0);
    assert!(first.0.x < CONTEXT_X);
    assert!(first.1 > 0);
    Ok(())
}

#[test]
fn actual_context_menu_artifact_route_emits_both_content_hashes()
-> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    let mut adapter =
        EguiContextMenuAdapter::new(crate::text_raster::PlatformTextRasterConfig::default())?;
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![ContextMenuPresentationItem::action("artifact", "Artifact")],
    });
    adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
        CONTEXT_X,
        CONTEXT_Y,
        UiTextSelectionRange::caret(0),
        UiRect::new(0, 0, FRAME_WIDTH_PX as u32, FRAME_HEIGHT_PX as u32),
    ));
    let mut output = None;
    let mut frame_output = context.run_ui(frame_input(), |ui| {
        output = Some(adapter.show(ui, &raster_style(), &paint_style()));
    });
    frame_output.textures_delta.clear();
    let artifact = output
        .ok_or_else(|| std::io::Error::other("actual egui frame did not run"))??
        .artifact
        .ok_or_else(|| std::io::Error::other("visible menu artifact was absent"))?;
    assert_eq!(artifact.frame_record_hash.len(), 64);
    assert_eq!(artifact.paint_plan_hash.len(), 64);
    assert_ne!(artifact.frame_record_hash, artifact.paint_plan_hash);
    assert!(!artifact.paint_plan.operations.is_empty());
    Ok(())
}

#[test]
fn context_menu_wheel_scroll_only_affects_hovered_bounds() {
    let context = egui::Context::default();
    let Some(mut adapter) = require_ok(
        EguiContextMenuAdapter::new(crate::text_raster::PlatformTextRasterConfig::default()),
        "context menu adapter should be created",
    ) else {
        return;
    };
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![ContextMenuPresentationItem::action("a", "A")],
    });
    adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
        0,
        0,
        UiTextSelectionRange::caret(0),
        UiRect::new(0, 0, 40, 20),
    ));
    let run = |events: Vec<egui::Event>, adapter: &mut EguiContextMenuAdapter| {
        let mut frame_output = context.run_ui(
            egui::RawInput {
                events,
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(80.0, 40.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| {
                adapter.apply_wheel_scroll(ui, UiRect::new(0, 0, 40, 20), 80);
            },
        );
        frame_output.textures_delta.clear();
        adapter.vertical_scroll_offset
    };
    let wheel = || egui::Event::MouseWheel {
        unit: egui::MouseWheelUnit::Point,
        delta: egui::vec2(0.0, 20.0),
        phase: egui::TouchPhase::Move,
        modifiers: egui::Modifiers::NONE,
    };
    let without_hover = run(vec![wheel()], &mut adapter);
    let with_hover = run(
        vec![egui::Event::PointerMoved(egui::pos2(10.0, 10.0)), wheel()],
        &mut adapter,
    );
    assert_eq!(0.0, without_hover);
    assert!(with_hover > 0.0);
}

#[test]
fn context_menu_reveal_keyboard_highlight_keeps_scrolled_path_visible() {
    let Some(mut adapter) = require_ok(
        EguiContextMenuAdapter::new(crate::text_raster::PlatformTextRasterConfig::default()),
        "context menu adapter should be created",
    ) else {
        return;
    };
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: (0..3)
            .map(|i| ContextMenuPresentationItem::action(i.to_string(), i.to_string()))
            .collect(),
    });
    adapter.apply_actions([
        ContextMenuAction::OpenWithLayout {
            anchor: ContextMenuAnchor::Pointer { x: 0, y: 0 },
            menu_size: ContextMenuSize::new(60, 120),
            viewport: ContextMenuViewport::new(320, 80),
        },
        ContextMenuAction::Highlight { path: vec![2] },
    ]);
    adapter.reveal_keyboard_highlight(UiRect::new(0, 0, 80, 30), 300, 3);
    let Some(index) = adapter.menu.current_highlighted_path().last().copied() else {
        return;
    };
    let row_bottom = (super::super::types::MENU_PADDING_PX
        + u32::try_from(index)
            .unwrap_or(u32::MAX)
            .saturating_mul(super::super::types::ROW_HEIGHT_PX)) as f32
        + super::super::types::ROW_HEIGHT_PX as f32;
    assert_eq!(
        (row_bottom - 30.0).clamp(0.0, 270.0),
        adapter.vertical_scroll_offset
    );
}
