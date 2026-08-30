use katana_ui_core::molecule::{
    ProgressMeterShape, ProgressMeterSpec, StatusBar, StatusBarDensity, StatusBarEvent,
    StatusBarMode, StatusBarPopoverSpec, StatusBarSegment, StatusBarSegmentAlignment,
};
use katana_ui_core::render_model::{UiRect, UiTone};
use katana_ui_core_egui_adapter::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core_egui_adapter::status_bar::{
    EguiStatusBarAdapter, StatusBarPaintOperationKind, StatusBarRenderStyle,
};
use serde_json::{Value, json};

fn status_bar() -> StatusBar {
    StatusBar::new("文書状態")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("progress", "⭐️ 進捗")
                .alignment(StatusBarSegmentAlignment::Leading)
                .interactive(true)
                .accessibility_label("進捗を開く"),
        )
        .segment(
            StatusBarSegment::new("position", "行 12:4")
                .alignment(StatusBarSegmentAlignment::Trailing),
        )
        .segment(
            StatusBarSegment::new("outline", "☆").alignment(StatusBarSegmentAlignment::Trailing),
        )
}

fn frame(
    context: &egui::Context,
    adapter: &mut EguiStatusBarAdapter,
    status: &mut StatusBar,
    events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, Vec<StatusBarEvent>), String> {
    let mut receipt = None;
    let mut output = context.run_ui(
        egui::RawInput {
            events,
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 80.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                receipt = Some(adapter.show(ui, status));
            });
        },
    );
    output.textures_delta.clear();
    let receipt = receipt
        .ok_or_else(|| "status-bar receipt was not produced".to_owned())?
        .map_err(|error| error.to_string())?;
    Ok((output, receipt.events().to_vec()))
}

fn with_open_popover(mut status: StatusBar, id: &str) -> Result<StatusBar, String> {
    let mut serialized: Value = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    let state = serialized
        .get_mut("state")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "status-bar state is missing".to_owned())?;
    state.insert("open_popover".to_owned(), json!(id));
    status = serde_json::from_value(serialized).map_err(|error| error.to_string())?;
    Ok(status)
}

fn rects_overlap(a: UiRect, b: UiRect) -> bool {
    let a_left = a.x;
    let a_right = a.x + a.width as i32;
    let a_top = a.y;
    let a_bottom = a.y + a.height as i32;
    let b_left = b.x;
    let b_right = b.x + b.width as i32;
    let b_top = b.y;
    let b_bottom = b.y + b.height as i32;
    a_left < b_right && b_left < a_right && a_top < b_bottom && b_top < a_bottom
}

#[test]
fn generic_status_bar_uses_platform_raster_for_japanese_and_emoji() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter =
        EguiStatusBarAdapter::new("status-raster-contract").map_err(|error| error.to_string())?;
    let mut status = status_bar();

    let (output, events) = frame(&context, &mut adapter, &mut status, Vec::new())?;

    assert!(events.is_empty());
    assert!(output.platform_output.accesskit_update.is_some());
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.chromatic_pixel_count > 0)
    );
    let plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "status-bar keeps its closed paint plan".to_owned())?;
    assert!(plan.operations.iter().any(|operation| {
        matches!(operation.kind, StatusBarPaintOperationKind::Texture { .. })
    }));
    let texture_hashes = plan
        .operations
        .iter()
        .filter_map(|operation| match &operation.kind {
            StatusBarPaintOperationKind::Texture { texture, .. } => Some(&texture.identity),
            StatusBarPaintOperationKind::Fill { .. } => None,
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        texture_hashes.len() >= 3,
        "exact U+2B50 U+FE0F and U+2606 must remain separate platform-raster textures"
    );
    let composed = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(plan.surface_bounds),
        plans: &[ArtifactPaintPlanRef::StatusBar(plan)],
    })
    .map_err(|error| error.to_string())?;
    assert!(composed.non_transparent_pixel_count > 0);
    Ok(())
}

#[test]
fn accesskit_activation_maps_to_existing_generic_status_action() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiStatusBarAdapter::new("status-accesskit-contract")
        .map_err(|error| error.to_string())?;
    let mut status = status_bar();
    let (output, _) = frame(&context, &mut adapter, &mut status, Vec::new())?;
    let target = output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(node_id, node)| {
                (node.role() == egui::accesskit::Role::Button && node.label() == Some("進捗を開く"))
                    .then_some(*node_id)
            })
        })
        .ok_or_else(|| {
            "interactive KUC status segment must publish an AccessKit button".to_owned()
        })?;
    let (_, events) = frame(
        &context,
        &mut adapter,
        &mut status,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: target,
                data: None,
            },
        )],
    )?;

    assert_eq!(
        vec![StatusBarEvent::SegmentPressed {
            id: "progress".to_string(),
        }],
        events
    );
    Ok(())
}

#[test]
fn status_bar_single_message_uses_single_message_path_and_rasterizes() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-single-message-contract")
        .map_err(|error| error.to_string())?;
    let mut status = StatusBar::new("単一メッセージ")
        .mode(StatusBarMode::SingleMessage)
        .message("保存が完了しました");

    let (_output, events) = frame(&context, &mut adapter, &mut status, Vec::new())?;

    assert!(events.is_empty());
    let plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "status-bar keeps its closed paint plan".to_owned())?;
    assert!(
        plan.operations
            .iter()
            .any(|operation| matches!(operation.kind, StatusBarPaintOperationKind::Texture { .. }))
    );

    Ok(())
}

#[test]
fn compact_status_bar_uses_the_reduced_public_frame_height() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter =
        EguiStatusBarAdapter::new("status-compact-contract").map_err(|error| error.to_string())?;
    let mut status = StatusBar::new("compact")
        .mode(StatusBarMode::SingleMessage)
        .density(StatusBarDensity::Compact)
        .message("ready");

    let _ = frame(&context, &mut adapter, &mut status, Vec::new())?;
    let plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "compact status-bar keeps its paint plan".to_owned())?;
    assert_eq!(
        plan.surface_bounds.height,
        StatusBarRenderStyle::standard().height_px.saturating_sub(4)
    );
    Ok(())
}

#[test]
fn status_bar_center_alignment_path_is_rendered() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter =
        EguiStatusBarAdapter::new("status-center-contract").map_err(|error| error.to_string())?;
    let mut status = StatusBar::new("センター")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("center", "中央セグメント")
                .alignment(StatusBarSegmentAlignment::Center),
        );

    let (_, events) = frame(&context, &mut adapter, &mut status, Vec::new())?;
    assert!(events.is_empty());
    let plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "center status-bar keeps its paint plan".to_owned())?;
    assert!(
        plan.operations
            .iter()
            .any(|operation| matches!(operation.kind, StatusBarPaintOperationKind::Texture { .. })),
        "center-aligned rendering must produce a textured segment"
    );

    Ok(())
}

#[test]
fn status_bar_open_popover_id_mismatch_does_not_render_overlay() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-popover-missing-contract")
        .map_err(|error| error.to_string())?;
    let status = StatusBar::new("状態")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("progress", "進捗").alignment(StatusBarSegmentAlignment::Leading),
        );
    let mut status = with_open_popover(status, "missing").map_err(|error| error.to_string())?;

    let (_, events) = frame(&context, &mut adapter, &mut status, Vec::new())?;
    assert!(events.is_empty());
    let plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "status-bar keeps paint plan on missing popover".to_owned())?;
    assert!(!plan
        .operations
        .iter()
        .any(|operation| matches!(&operation.kind, StatusBarPaintOperationKind::Texture { texture, .. } if texture.identity.starts_with("status-bar-overlay:"))));

    Ok(())
}

#[test]
fn status_bar_open_popover_without_spec_is_noop() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-popover-no-spec-contract")
        .map_err(|error| error.to_string())?;
    let status = StatusBar::new("状態")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("plain", "説明").alignment(StatusBarSegmentAlignment::Center),
        );
    let mut status = with_open_popover(status, "plain").map_err(|error| error.to_string())?;

    let (_, events) = frame(&context, &mut adapter, &mut status, Vec::new())?;
    assert!(events.is_empty());
    let plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "status-bar keeps paint plan on spec-less popover".to_owned())?;
    assert!(!plan
        .operations
        .iter()
        .any(|operation| matches!(&operation.kind, StatusBarPaintOperationKind::Texture { texture, .. } if texture.identity.starts_with("status-bar-overlay:"))));

    Ok(())
}

#[test]
fn progress_icon_tooltip_and_popover_are_rasterized_and_one_shot() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter =
        EguiStatusBarAdapter::new("status-full-contract").map_err(|error| error.to_string())?;
    let mut status = StatusBar::new("状態")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("build", "⭐️ 進捗")
                .icon("⚙️")
                .tooltip("ビルドの状態")
                .progress(
                    ProgressMeterSpec::new(ProgressMeterShape::Linear, 65)
                        .label("65%")
                        .tone(UiTone::Accent)
                        .tooltip("進捗率"),
                )
                .popover(StatusBarPopoverSpec::new("ビルド", "日本語の詳細 ⭐️")),
        )
        .segment(StatusBarSegment::new("disabled", "無効").interactive(false));

    let (first, first_events) = frame(
        &context,
        &mut adapter,
        &mut status,
        vec![egui::Event::PointerMoved(egui::pos2(30.0, 12.0))],
    )?;
    assert!(
        first_events.is_empty(),
        "hover without a hit must not dispatch an event"
    );
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.width > 0)
    );
    let plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "retained plan".to_owned())?;
    let fills = plan
        .operations
        .iter()
        .filter_map(|operation| match operation.kind {
            StatusBarPaintOperationKind::Fill { bounds, color_rgba } => Some((bounds, color_rgba)),
            StatusBarPaintOperationKind::Texture { .. } => None,
        })
        .collect::<Vec<_>>();
    assert!(
        fills
            .iter()
            .any(|(_, color)| *color == [100, 175, 255, 255])
    );
    assert!(first.platform_output.accesskit_update.is_some());

    let target = first
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find_map(|(node_id, node)| (node.label() == Some("⭐️ 進捗")).then_some(*node_id))
        })
        .ok_or_else(|| "popover segment AccessKit target".to_owned())?;
    let (_, open_events) = frame(
        &context,
        &mut adapter,
        &mut status,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: target,
                data: None,
            },
        )],
    )?;
    assert_eq!(
        open_events,
        vec![
            StatusBarEvent::SegmentPressed { id: "build".into() },
            StatusBarEvent::SegmentPopoverOpened { id: "build".into() },
            StatusBarEvent::SegmentTooltipShown { id: "build".into() },
        ]
    );
    let opened_plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "opened retained plan".to_owned())?;
    assert!(opened_plan.operations.iter().any(|operation| {
        matches!(operation.kind, StatusBarPaintOperationKind::Texture { ref texture, .. } if texture.identity.starts_with("status-bar-overlay:"))
    }));
    let mut overlay_bounds = opened_plan
        .operations
        .iter()
        .filter_map(|operation| match &operation.kind {
            StatusBarPaintOperationKind::Texture { bounds, texture }
                if texture.identity.starts_with("status-bar-overlay:") =>
            {
                Some(*bounds)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        overlay_bounds.len(),
        2,
        "opened popover must rasterize title and body"
    );
    overlay_bounds.sort_by_key(|rect| rect.y);
    assert!(
        !rects_overlap(overlay_bounds[0], overlay_bounds[1]),
        "popover title/body textures must not overlap"
    );
    assert!(
        overlay_bounds[0].y + overlay_bounds[0].height as i32 <= overlay_bounds[1].y,
        "popover title/body should be stacked in layout order"
    );

    let (_, close_events) = frame(
        &context,
        &mut adapter,
        &mut status,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert_eq!(
        close_events,
        vec![StatusBarEvent::SegmentPopoverClosed { id: "build".into() }]
    );
    let (_, no_events) = frame(&context, &mut adapter, &mut status, Vec::new())?;
    assert!(
        no_events.is_empty(),
        "transport must be one-shot after close: {no_events:?}"
    );
    Ok(())
}
