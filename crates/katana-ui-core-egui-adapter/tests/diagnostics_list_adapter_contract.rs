use katana_ui_core::molecule::{
    CodeDiff, CodeDiffLine, CodeDiffLineKind, DiagnosticAction, DiagnosticFixPreview, DiagnosticId,
    DiagnosticItem, DiagnosticLocation, DiagnosticSeverity, DiagnosticsList, DiagnosticsListAction,
    DiagnosticsListEvent,
};
use katana_ui_core_egui_adapter::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core_egui_adapter::diagnostics_list::{
    DiagnosticsListPaintOperationKind, DiagnosticsTargetIdentity, EguiDiagnosticsListAdapter,
    EguiDiagnosticsListError,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    KucInteractionActionClass, KucInteractionSelector,
};
use katana_ui_core_text_raster::PlatformTextRasterError;

#[test]
fn target_identities_are_opaque_stable_and_distinct_from_display_text() {
    let severity = DiagnosticsTargetIdentity::severity_filter(DiagnosticSeverity::Error);
    let item = DiagnosticsTargetIdentity::item("diagnostic-with-a-localized-label");
    let fix = DiagnosticsTargetIdentity::fix("diagnostic-with-a-localized-label");

    assert_eq!(
        severity,
        DiagnosticsTargetIdentity::severity_filter(DiagnosticSeverity::Error)
    );
    assert_eq!(
        item,
        DiagnosticsTargetIdentity::item("diagnostic-with-a-localized-label")
    );
    assert_ne!(item, fix);
    assert!(!item.contains("diagnostic-with-a-localized-label"));
    assert!(!item.contains("日本語"));
    assert!(
        format!(
            "{:?}",
            KucInteractionSelector::new(item, KucInteractionActionClass::DiagnosticsItem)
        )
        .contains("kuc.diagnostics.target.v1.item.")
    );
}

#[test]
fn diagnostics_list_error_display_and_conversion_coverage() {
    let raster: EguiDiagnosticsListError = PlatformTextRasterError::EmptyText.into();
    assert_eq!(
        raster.to_string(),
        "diagnostics raster failed: platform text raster request must not be empty"
    );

    let missing_plan = EguiDiagnosticsListError::PaintPlanNotProduced;
    assert_eq!(
        missing_plan.to_string(),
        "diagnostics did not produce a paint plan"
    );
    assert!(format!("{missing_plan:?}").contains("PaintPlanNotProduced"));
}

fn diagnostics() -> DiagnosticsList {
    DiagnosticsList::new("診断 ⭐️")
        .item(
            katana_ui_core::molecule::DiagnosticItem::new(
                "error",
                DiagnosticSeverity::Error,
                "日本語の構文エラー ⭐️",
                DiagnosticLocation::new("src/lib.rs", 3, 12),
            )
            .quickfix(DiagnosticAction::new("fix", "修正を適用"))
            .fix_preview(DiagnosticFixPreview::new(
                CodeDiff::new("修正プレビュー")
                    .line(CodeDiffLine {
                        old_number: Some(3),
                        new_number: Some(3),
                        kind: CodeDiffLineKind::Removed,
                        text: "古い ⭐️".to_string(),
                    })
                    .line(CodeDiffLine {
                        old_number: None,
                        new_number: Some(3),
                        kind: CodeDiffLineKind::Added,
                        text: "新しい ⭐️".to_string(),
                    }),
            )),
        )
        .item(katana_ui_core::molecule::DiagnosticItem::new(
            "warning",
            DiagnosticSeverity::Warning,
            "未使用の値",
            DiagnosticLocation::new("src/main.rs", 7, 4),
        ))
}

fn scoped_diagnostics(scope_count: usize) -> DiagnosticsList {
    let mut list = DiagnosticsList::new("診断 ⭐️");
    let scopes = [
        ("all", "すべて ⭐️", "すべての診断 ⭐️"),
        ("active", "現在", "現在の範囲"),
    ];
    for (key, label, accessible_label) in scopes.into_iter().take(scope_count) {
        list = list.scope(key, label, accessible_label);
    }
    list.item(
        katana_ui_core::molecule::DiagnosticItem::new(
            "all",
            DiagnosticSeverity::Error,
            "全体 ⭐️",
            DiagnosticLocation::new("a.rs", 1, 1),
        )
        .scope("all"),
    )
    .item(
        katana_ui_core::molecule::DiagnosticItem::new(
            "active",
            DiagnosticSeverity::Warning,
            "現在",
            DiagnosticLocation::new("b.rs", 2, 1),
        )
        .scope("active"),
    )
}

#[test]
fn scope_locator_hashes_key_and_scope_labels_raster_with_japanese_emoji() -> Result<(), String> {
    let identity = DiagnosticsTargetIdentity::scope("opaque-scope-key");
    assert!(!identity.contains("opaque-scope-key"));
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter =
        EguiDiagnosticsListAdapter::new("scope-raster").map_err(|error| error.to_string())?;
    let mut diagnostics = scoped_diagnostics(2);
    let (output, _) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|e| e.text == "すべて ⭐️")
    );
    assert!(
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some("すべての診断 ⭐️"))
            })
    );
    Ok(())
}

#[test]
fn scope_pointer_keyboard_and_stale_accesskit_use_current_resolver() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter =
        EguiDiagnosticsListAdapter::new("scope-events").map_err(|error| error.to_string())?;
    let mut diagnostics = scoped_diagnostics(2);
    let (initial, _) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    let second = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find_map(|(id, node)| (node.label() == Some("現在の範囲")).then_some(*id))
        })
        .ok_or_else(|| "second scope AccessKit node".to_string())?;
    let (_, selected) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: second,
                data: None,
            },
        )],
    )?;
    assert!(
        matches!(selected.as_slice(), [DiagnosticsListEvent::ScopeSelected { scope_key }] if scope_key.as_str() == "active")
    );
    assert_eq!(1, diagnostics.render_snapshot().visible.total_count);

    let (_, keyboard) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::ArrowLeft,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(
        matches!(keyboard.as_slice(), [DiagnosticsListEvent::ScopeSelected { scope_key }] if scope_key.as_str() == "all")
    );

    let (_, keyboard) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::ArrowRight,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(
        matches!(keyboard.as_slice(), [DiagnosticsListEvent::ScopeSelected { scope_key }] if scope_key.as_str() == "active")
    );

    let (stale_frame, _) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    let old_second = stale_frame
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find_map(|(id, node)| (node.label() == Some("現在の範囲")).then_some(*id))
        })
        .ok_or_else(|| "scope node before removal".to_string())?;
    diagnostics.set_scopes(vec![(
        "all".into(),
        "すべて ⭐️".into(),
        "すべての診断 ⭐️".into(),
    )]);
    let (_, stale) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: old_second,
                data: None,
            },
        )],
    )?;
    assert!(stale.is_empty());
    Ok(())
}

#[test]
fn removed_focused_scope_rejects_keyboard_navigation_without_a_replacement() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiDiagnosticsListAdapter::new("removed-focused-scope")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = scoped_diagnostics(2);
    let (initial, _) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    let active_scope = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find_map(|(id, node)| (node.label() == Some("現在の範囲")).then_some(*id))
        })
        .ok_or_else(|| "active scope AccessKit node".to_owned())?;
    let (_, selected) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: active_scope,
                data: None,
            },
        )],
    )?;
    assert!(matches!(
        selected.as_slice(),
        [DiagnosticsListEvent::ScopeSelected { scope_key }] if scope_key.as_str() == "active"
    ));

    diagnostics.set_scopes(Vec::<(String, String, String)>::new());
    let (_, rejected) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::ArrowRight,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;

    assert!(rejected.is_empty());
    assert!(
        context.memory(|memory| memory.focused()).is_none(),
        "removing the focused scope must surrender its egui focus"
    );
    assert!(
        diagnostics
            .render_snapshot()
            .state
            .selected_scope_key
            .is_none()
    );

    diagnostics.set_scopes([
        (
            "all".to_owned(),
            "すべて".to_owned(),
            "すべての診断".to_owned(),
        ),
        (
            "active".to_owned(),
            "現在".to_owned(),
            "現在の範囲".to_owned(),
        ),
    ]);
    let (_, stale_focus_events) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::ArrowRight,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(
        stale_focus_events.is_empty(),
        "reintroducing the scope must not restore stale keyboard focus"
    );
    Ok(())
}

#[test]
fn one_scope_is_visible_but_disabled_for_accesskit_activation() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter =
        EguiDiagnosticsListAdapter::new("scope-disabled").map_err(|error| error.to_string())?;
    let mut diagnostics = scoped_diagnostics(1);
    let (initial, _) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    let target = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find_map(|(id, node)| (node.label() == Some("すべての診断 ⭐️")).then_some(*id))
        })
        .ok_or_else(|| "disabled scope AccessKit node".to_string())?;
    let node = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .ok_or_else(|| "AccessKit update missing".to_string())?
        .nodes
        .iter()
        .find_map(|(id, node)| (*id == target).then_some(node))
        .ok_or_else(|| "disabled scope node missing".to_string())?;
    assert!(!node.supports_action(egui::accesskit::Action::Click));
    let (_, events) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: target,
                data: None,
            },
        )],
    )?;
    assert!(events.is_empty());
    Ok(())
}

#[test]
fn diagnostics_list_fails_closed_when_a_scope_label_cannot_be_rasterized() {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("invalid-scope-label")
        .expect("diagnostics adapter should initialize");
    let mut diagnostics = DiagnosticsList::new("Diagnostics")
        .scope("all", "", "All diagnostics")
        .item(DiagnosticItem::new(
            "error",
            DiagnosticSeverity::Error,
            "Error",
            DiagnosticLocation::new("src/lib.rs", 1, 1),
        ));

    let error = match frame(&context, &mut adapter, &mut diagnostics, Vec::new()) {
        Ok(_) => panic!("an empty scope label must fail closed"),
        Err(error) => error,
    };
    assert!(error.contains("platform text raster request must not be empty"));
}

fn many_diagnostics() -> DiagnosticsList {
    (0..12).fold(DiagnosticsList::new("診断 ⭐️"), |list, index| {
        list.item(katana_ui_core::molecule::DiagnosticItem::new(
            format!("item-{index}"),
            if index % 2 == 0 {
                DiagnosticSeverity::Error
            } else {
                DiagnosticSeverity::Warning
            },
            format!("項目 {index} ⭐️"),
            DiagnosticLocation::new(format!("src/{index}.rs"), index + 1, 2),
        ))
    })
}

fn frame(
    context: &egui::Context,
    adapter: &mut EguiDiagnosticsListAdapter,
    diagnostics: &mut DiagnosticsList,
    events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, Vec<DiagnosticsListEvent>), String> {
    let mut output_events = Vec::new();
    let mut show_error = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 240.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| match adapter.show(ui, diagnostics) {
                Ok(output) => output_events.extend(output.events().iter().cloned()),
                Err(error) => show_error = Some(error.to_string()),
            });
        },
    );
    output.textures_delta.clear();
    show_error.map_or_else(|| Ok((output, output_events)), Err)
}

fn frame_at_pointer(
    context: &egui::Context,
    adapter: &mut EguiDiagnosticsListAdapter,
    diagnostics: &mut DiagnosticsList,
    mut events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, Vec<DiagnosticsListEvent>), String> {
    events.insert(0, egui::Event::PointerMoved(egui::pos2(400.0, 100.0)));
    frame(context, adapter, diagnostics, events)
}

fn frame_with_raster_surface(
    context: &egui::Context,
    adapter: &mut EguiDiagnosticsListAdapter,
    diagnostics: &mut DiagnosticsList,
    width: f32,
    scale: f32,
) -> Result<bool, String> {
    let mut show_error = None;
    let mut input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width, 240.0),
        )),
        ..egui::RawInput::default()
    };
    input.viewports.insert(
        egui::ViewportId::ROOT,
        egui::ViewportInfo {
            native_pixels_per_point: Some(scale),
            ..egui::ViewportInfo::default()
        },
    );
    let mut output = context.run_ui(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Err(error) = adapter.show(ui, diagnostics) {
                show_error = Some(error.to_string());
            }
        });
    });
    let emitted_textures = !output.textures_delta.set.is_empty();
    output.textures_delta.clear();
    show_error.map_or_else(|| Ok(emitted_textures), Err)
}

fn texture_identity_for_raster(
    adapter: &EguiDiagnosticsListAdapter,
    text: &str,
) -> Result<String, String> {
    let pixel_hash = adapter
        .raster_evidence()
        .iter()
        .find(|evidence| evidence.text == text)
        .map(|evidence| evidence.sha256.as_str())
        .ok_or_else(|| format!("missing raster evidence for `{text}`"))?;
    let identity = format!("diagnostics-text:{pixel_hash}");
    adapter
        .artifact_paint_plan()
        .into_iter()
        .flat_map(|plan| plan.operations.iter())
        .find_map(|operation| match &operation.kind {
            DiagnosticsListPaintOperationKind::Texture { texture, .. }
                if texture.identity == identity =>
            {
                Some(texture.identity.clone())
            }
            DiagnosticsListPaintOperationKind::Fill { .. }
            | DiagnosticsListPaintOperationKind::Texture { .. } => None,
        })
        .ok_or_else(|| format!("missing texture for raster identity `{identity}`"))
}

#[test]
fn raster_plan_contains_japanese_emoji_location_and_quickfix_without_labels() -> Result<(), String>
{
    let context = egui::Context::default();
    let mut adapter =
        EguiDiagnosticsListAdapter::new("diagnostics-raster").map_err(|error| error.to_string())?;
    let mut diagnostics = diagnostics();
    let (_, events) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    assert!(events.is_empty());
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.text.contains("⭐️"))
    );
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.text.contains("src/lib.rs:3:12"))
    );
    let plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "adapter-local diagnostics plan".to_string())?;
    assert!(plan.operations.iter().any(|operation| matches!(
        operation.kind,
        DiagnosticsListPaintOperationKind::Texture { .. }
    )));
    assert!(!plan.operations.is_empty());
    let composed = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(plan.surface_bounds),
        plans: &[ArtifactPaintPlanRef::DiagnosticsList(plan)],
    })
    .map_err(|error| {
        format!("diagnostics plan composes through the KUC artifact compositor: {error}")
    })?;
    assert!(composed.non_transparent_pixel_count > 0);
    Ok(())
}

#[test]
fn retained_diagnostics_textures_reuse_equal_pixels_and_refresh_for_scale_and_width()
-> Result<(), String> {
    let text = "同じ診断テキストを長い幅制約で rasterize し、保持 texture の更新を検証します ⭐️";
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-retained-raster")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = DiagnosticsList::new(text);

    let initial = frame_with_raster_surface(&context, &mut adapter, &mut diagnostics, 420.0, 1.0)?;
    let initial_identity = texture_identity_for_raster(&adapter, text)?;
    assert!(initial);

    let reused = frame_with_raster_surface(&context, &mut adapter, &mut diagnostics, 420.0, 1.0)?;
    assert_eq!(
        initial_identity,
        texture_identity_for_raster(&adapter, text)?
    );
    assert!(!reused);

    let scaled = frame_with_raster_surface(&context, &mut adapter, &mut diagnostics, 420.0, 2.0)?;
    let scaled_identity = texture_identity_for_raster(&adapter, text)?;
    assert_ne!(initial_identity, scaled_identity);
    assert!(scaled);

    let narrowed = frame_with_raster_surface(&context, &mut adapter, &mut diagnostics, 96.0, 1.0)?;
    let narrowed_identity = texture_identity_for_raster(&adapter, text)?;
    assert_ne!(initial_identity, narrowed_identity);
    assert!(narrowed);
    Ok(())
}

#[test]
fn raw_input_keyboard_actions_return_existing_core_events() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-keyboard")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = diagnostics();
    let (initial, _) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    let warning = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(id, node)| {
                (node.role() == egui::accesskit::Role::ListItem
                    && node
                        .label()
                        .is_some_and(|label| label.contains("未使用の値")))
                .then_some(*id)
            })
        })
        .ok_or_else(|| "warning diagnostics AccessKit node".to_string())?;
    let (_, focused) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: warning,
                data: None,
            },
        )],
    )?;
    assert!(
        matches!(focused.as_slice(), [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "warning")
    );
    let (_, selected) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::ArrowUp,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(
        matches!(selected.as_slice(), [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "error")
    );
    let (_, navigated) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(
        matches!(navigated.as_slice(), [DiagnosticsListEvent::NavigateRequested { id }] if id.as_str() == "error")
    );
    let (_, fixed) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::Space,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(
        matches!(fixed.as_slice(), [DiagnosticsListEvent::DiagnosticFixApplied { id }] if id.as_str() == "error")
    );
    let (_, previous_error) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::F8,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers {
                shift: true,
                ..egui::Modifiers::NONE
            },
        }],
    )?;
    assert!(matches!(
        previous_error.as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "error"
    ));
    Ok(())
}

#[test]
fn diagnostics_list_preview_renders_no_lines_and_special_diff_kinds() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-preview-special")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = DiagnosticsList::new("診断 ⭐️")
        .item(
            DiagnosticItem::new(
                "empty",
                DiagnosticSeverity::Error,
                "空の差分",
                DiagnosticLocation::new("src/empty.rs", 1, 1),
            )
            .fix_preview(DiagnosticFixPreview::new(CodeDiff::new("empty diff"))),
        )
        .item(
            DiagnosticItem::new(
                "multi",
                DiagnosticSeverity::Warning,
                "差分の種類",
                DiagnosticLocation::new("src/special.rs", 2, 1),
            )
            .fix_preview(DiagnosticFixPreview::new(
                CodeDiff::new("special")
                    .line(CodeDiffLine {
                        old_number: Some(1),
                        new_number: Some(1),
                        kind: CodeDiffLineKind::Context,
                        text: "context".to_string(),
                    })
                    .line(CodeDiffLine {
                        old_number: Some(2),
                        new_number: None,
                        kind: CodeDiffLineKind::Placeholder,
                        text: "placeholder".to_string(),
                    })
                    .line(CodeDiffLine {
                        old_number: Some(3),
                        new_number: Some(4),
                        kind: CodeDiffLineKind::Removed,
                        text: "removed".to_string(),
                    })
                    .line(CodeDiffLine {
                        old_number: Some(4),
                        new_number: Some(5),
                        kind: CodeDiffLineKind::Added,
                        text: "added".to_string(),
                    }),
            )),
        );
    diagnostics.apply_action(DiagnosticsListAction::ToggleFixPreview(DiagnosticId::new(
        "empty",
    )));
    diagnostics.apply_action(DiagnosticsListAction::ToggleFixPreview(DiagnosticId::new(
        "multi",
    )));

    let _ = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;

    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.text == "差分なし")
    );
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.text.contains(" context"))
    );
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.text.contains("… placeholder"))
    );
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.text.contains(" removed")),
        "{:?}",
        adapter
            .raster_evidence()
            .iter()
            .map(|evidence| evidence.text.as_str())
            .collect::<Vec<_>>()
    );
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.text.contains(" added"))
    );
    Ok(())
}

#[test]
fn accesskit_click_returns_generic_selection_event() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-accesskit")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = diagnostics();
    let (output, _) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    let target = output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(id, node)| {
                (node.role() == egui::accesskit::Role::ListItem).then_some(*id)
            })
        })
        .ok_or_else(|| "diagnostic item AccessKit node".to_string())?;
    let (_, events) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: target,
                data: None,
            },
        )],
    )?;
    assert!(matches!(
        events.as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { .. }]
    ));
    let (_, navigated) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(matches!(
        navigated.as_slice(),
        [DiagnosticsListEvent::NavigateRequested { .. }]
    ));
    Ok(())
}

#[test]
fn accesskit_click_quickfix_runs_apply_fix_event() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-quickfix")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = diagnostics();
    let initial = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?.0;
    let quickfix_target = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find_map(|(id, node)| (node.label() == Some("修正を適用")).then_some(*id))
        })
        .ok_or_else(|| "missing quickfix accesskit node".to_string())?;

    let (_, events) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: quickfix_target,
                data: None,
            },
        )],
    )?;

    assert!(events.iter().any(|event| {
        matches!(
            event,
            DiagnosticsListEvent::DiagnosticFixApplied { id } if id.as_str() == "error"
        )
    }));
    Ok(())
}

#[test]
fn accesskit_click_severity_filter_toggles_filter_state() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-severity-filter")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = diagnostics();
    let initial = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?.0;
    let warning_filter = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(id, node)| {
                (node.role() == egui::accesskit::Role::CheckBox
                    && node.label() == Some("Warning")
                    && node.supports_action(egui::accesskit::Action::Click))
                .then_some(*id)
            })
        })
        .ok_or_else(|| "missing warning severity filter node".to_string())?;
    let before_warning = diagnostics
        .render_snapshot()
        .options
        .severity_filter
        .contains(&DiagnosticSeverity::Warning);

    let (_, events) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: warning_filter,
                data: None,
            },
        )],
    )?;

    assert!(events.contains(&DiagnosticsListEvent::FilterChanged));
    let after = diagnostics
        .render_snapshot()
        .options
        .severity_filter
        .contains(&DiagnosticSeverity::Warning);
    assert_ne!(before_warning, after);

    let (_, restored_events) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: warning_filter,
                data: None,
            },
        )],
    )?;
    assert!(restored_events.contains(&DiagnosticsListEvent::FilterChanged));
    assert_eq!(
        before_warning,
        diagnostics
            .render_snapshot()
            .options
            .severity_filter
            .contains(&DiagnosticSeverity::Warning)
    );
    Ok(())
}

#[test]
fn accesskit_scroll_up_updates_scroll_position() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-scroll-up")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = many_diagnostics();

    let (initial, _) = frame_at_pointer(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    let list_nodes = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .map(|update| {
            update
                .nodes
                .iter()
                .filter_map(|(id, node)| {
                    (node.role() == egui::accesskit::Role::List
                        && node.supports_action(egui::accesskit::Action::ScrollDown)
                        && node.supports_action(egui::accesskit::Action::ScrollUp))
                    .then_some(*id)
                })
                .collect::<Vec<_>>()
        })
        .ok_or_else(|| "missing diagnostics list accesskit node".to_string())?;
    let list_node = list_nodes
        .first()
        .copied()
        .ok_or_else(|| "missing diagnostics list accesskit node".to_string())?;

    let (wheel_output, events_down) = frame_at_pointer(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -1.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(events_down.is_empty());
    assert!(adapter.scroll_y() > 0.0);
    let list_node = wheel_output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(id, node)| {
                (node.role() == egui::accesskit::Role::List
                    && node.supports_action(egui::accesskit::Action::ScrollDown)
                    && node.supports_action(egui::accesskit::Action::ScrollUp))
                .then_some(*id)
            })
        })
        .unwrap_or(list_node);
    let before_up = adapter.scroll_y();
    let (_after_output, events_up) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::ScrollUp,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: list_node,
                data: None,
            },
        )],
    )?;
    assert!(events_up.is_empty());
    assert!(adapter.scroll_y() < before_up);
    Ok(())
}

#[test]
fn pointer_disclosure_opens_and_closes_with_one_generic_event() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-pointer-disclosure")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = diagnostics();
    let _ = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    let surface = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "initial disclosure plan".to_string())?
        .surface_bounds;
    let point = egui::pos2(surface.x as f32 + 16.0, surface.y as f32 + 47.0);
    let (_, pressed) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![
            egui::Event::PointerMoved(point),
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    )?;
    assert!(pressed.is_empty());
    let (_, opened) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![
            egui::Event::PointerMoved(point),
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    )?;
    assert_eq!(
        opened,
        vec![DiagnosticsListEvent::DiagnosticFixPreviewToggled {
            id: katana_ui_core::molecule::DiagnosticId::new("error"),
            expanded: true,
        }]
    );
    assert!(
        diagnostics
            .render_snapshot()
            .state
            .expanded_ids
            .contains(&katana_ui_core::molecule::DiagnosticId::new("error"))
    );
    Ok(())
}

#[test]
fn disclosure_supports_pointer_keyboard_accesskit_and_closes_outside() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-disclosure")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = diagnostics();

    let (initial, _) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    let disclosure = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(id, node)| {
                (node.role() == egui::accesskit::Role::Button
                    && node.label().is_some_and(|label| label == "展開"))
                .then_some(*id)
            })
        })
        .ok_or_else(|| "visible disclosure AccessKit node".to_string())?;
    let (_, opened) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: disclosure,
                data: None,
            },
        )],
    )?;
    assert_eq!(
        opened,
        vec![DiagnosticsListEvent::DiagnosticFixPreviewToggled {
            id: katana_ui_core::molecule::DiagnosticId::new("error"),
            expanded: true,
        }]
    );
    let before_open = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "closed plan".to_string())?
        .operations
        .len();
    let (opened_frame, _) = frame(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.text.contains("古い ⭐️"))
    );
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|evidence| evidence.text.contains("新しい ⭐️"))
    );
    assert!(
        adapter
            .artifact_paint_plan()
            .ok_or_else(|| "opened plan".to_string())?
            .operations
            .len()
            > before_open
    );
    assert!(
        opened_frame
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some("折りたたむ"))
            })
    );

    let (_, closed_by_keyboard) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::ArrowLeft,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert_eq!(
        closed_by_keyboard,
        vec![DiagnosticsListEvent::DiagnosticFixPreviewToggled {
            id: katana_ui_core::molecule::DiagnosticId::new("error"),
            expanded: false,
        }]
    );
    let (_, reopened) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::ArrowRight,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert_eq!(
        reopened,
        vec![DiagnosticsListEvent::DiagnosticFixPreviewToggled {
            id: katana_ui_core::molecule::DiagnosticId::new("error"),
            expanded: true,
        }]
    );

    context.memory_mut(|memory| memory.request_focus(egui::Id::new("outside-control")));
    let (_, ignored_escape) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(ignored_escape.is_empty());
    assert!(
        diagnostics
            .render_snapshot()
            .state
            .expanded_ids
            .contains(&katana_ui_core::molecule::DiagnosticId::new("error"))
    );

    let (_, closed_outside) = frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(880.0, 220.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert_eq!(
        closed_outside,
        vec![DiagnosticsListEvent::DiagnosticFixPreviewToggled {
            id: katana_ui_core::molecule::DiagnosticId::new("error"),
            expanded: false,
        }]
    );
    assert!(diagnostics.render_snapshot().state.expanded_ids.is_empty());
    assert!(!format!("{closed_outside:?}").contains("Key"));
    Ok(())
}

#[test]
fn retained_viewport_scrolls_many_items_and_republishes_only_visible_targets() -> Result<(), String>
{
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-virtualized")
        .map_err(|error| error.to_string())?;
    let mut diagnostics = many_diagnostics();

    let (initial, _) = frame_at_pointer(&context, &mut adapter, &mut diagnostics, Vec::new())?;
    assert_eq!(adapter.scroll_y(), 0.0);
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|e| e.text.contains("項目 0"))
    );
    assert!(
        !adapter
            .raster_evidence()
            .iter()
            .any(|e| e.text.contains("項目 8"))
    );
    let stale_item = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(id, node)| {
                (node.role() == egui::accesskit::Role::ListItem
                    && node.label().is_some_and(|label| label.contains("項目 0")))
                .then_some(*id)
            })
        })
        .ok_or_else(|| "first visible item AccessKit node".to_string())?;
    let scroll_target = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find_map(|(id, node)| (node.role() == egui::accesskit::Role::List).then_some(*id))
        })
        .ok_or_else(|| "scroll viewport AccessKit node".to_string())?;
    let initial_frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(
            adapter
                .artifact_paint_plan()
                .ok_or_else(|| "initial diagnostics plan".to_string())?
                .surface_bounds,
        ),
        plans: &[ArtifactPaintPlanRef::DiagnosticsList(
            adapter
                .artifact_paint_plan()
                .ok_or_else(|| "initial diagnostics plan".to_string())?,
        )],
    })
    .map_err(|error| format!("initial diagnostics artifact: {error}"))?;

    let (_, wheel_events) = frame_at_pointer(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -120.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(wheel_events.is_empty());
    assert!(adapter.scroll_y() > 0.0);
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|e| e.text.contains("項目 4"))
    );
    assert!(
        !adapter
            .raster_evidence()
            .iter()
            .any(|e| e.text.contains("項目 0"))
    );
    let scrolled_plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "scrolled diagnostics plan".to_string())?;
    let scrolled_frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(scrolled_plan.surface_bounds),
        plans: &[ArtifactPaintPlanRef::DiagnosticsList(scrolled_plan)],
    })
    .map_err(|error| format!("scrolled diagnostics artifact: {error}"))?;
    assert_ne!(initial_frame.pixel_hash, scrolled_frame.pixel_hash);
    assert!(scrolled_plan.operations.iter().all(|operation| {
        operation.clip_bounds.y >= scrolled_plan.surface_bounds.y
            && operation.clip_bounds.y + operation.clip_bounds.height as i32
                <= scrolled_plan.surface_bounds.y + scrolled_plan.surface_bounds.height as i32
    }));

    let (_, stale_events) = frame_at_pointer(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: stale_item,
                data: None,
            },
        )],
    )?;
    assert!(
        stale_events.is_empty(),
        "offscreen target must be rejected: {stale_events:?}"
    );

    let (_, accesskit_events) = frame_at_pointer(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::ScrollDown,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: scroll_target,
                data: None,
            },
        )],
    )?;
    assert!(accesskit_events.is_empty());
    assert!(adapter.scroll_y() > 100.0);
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .any(|e| e.text.contains("⭐️"))
    );
    let (_, revealed_events) = frame_at_pointer(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::Key {
            key: egui::Key::F8,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(revealed_events.iter().any(|event| matches!(
        event,
        DiagnosticsListEvent::DiagnosticSelected { id } if id.as_str() == "item-0"
    )));
    assert_eq!(adapter.scroll_y(), 0.0);
    let (_, second_scroll_events) = frame_at_pointer(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::ScrollDown,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: scroll_target,
                data: None,
            },
        )],
    )?;
    assert!(second_scroll_events.is_empty());
    assert!(adapter.scroll_y() > 0.0);
    let (_, reset_events) = frame_at_pointer(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 10_000.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::NONE,
        }],
    )?;
    assert!(reset_events.is_empty());
    assert_eq!(adapter.scroll_y(), 0.0);
    let (_, focus_events) = frame_at_pointer(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: stale_item,
                data: None,
            },
        )],
    )?;
    assert!(matches!(
        focus_events.as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "item-0"
    ));
    let before_keyboard = adapter.scroll_y();
    for step in 0..8 {
        let focus_before = context.memory(|memory| memory.focused());
        let (_, events) = frame_at_pointer(
            &context,
            &mut adapter,
            &mut diagnostics,
            vec![egui::Event::Key {
                key: egui::Key::ArrowDown,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
        )?;
        assert!(
            events
                .iter()
                .any(|event| { matches!(event, DiagnosticsListEvent::DiagnosticSelected { .. }) }),
            "keyboard step {step} must select a diagnostic: events={events:?}, focus_before={focus_before:?}, focus_after={:?}",
            context.memory(|memory| memory.focused())
        );
    }
    assert!(adapter.scroll_y() > before_keyboard);
    Ok(())
}
