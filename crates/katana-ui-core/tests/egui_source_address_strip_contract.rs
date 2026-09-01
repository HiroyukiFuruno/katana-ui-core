#![cfg(feature = "egui")]
use katana_ui_core::egui::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core::egui::source_address_strip::{
    EguiSourceAddressStripAdapter, EguiSourceAddressStripError, EguiSourceAddressStripOutput,
    SourceAddressFrameEventClass, SourceAddressPaintOperationKind, SourceAddressRenderStyle,
    SourceAddressSubmissionForwarder,
};
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressAction, SourceAddressEntry, SourceAddressEvent, SourceAddressPresentation,
    SourceAddressStrip,
};
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::convert::Infallible;
use std::rc::Rc;
use std::sync::Arc;

fn strip() -> SourceAddressStrip {
    let mut strip = SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソースアドレス",
        "ソースアドレスを入力",
        "ソースアドレス入力",
    ));
    strip.set_history(vec![SourceAddressEntry::new(
        SourceAddressPresentation::new("履歴項目", "履歴の説明", "履歴項目"),
        b"opaque-history-target",
    )]);
    strip.set_candidates(vec![SourceAddressEntry::new(
        SourceAddressPresentation::new("候補項目", "候補の説明", "候補項目"),
        b"opaque-candidate-target",
    )]);
    strip
}

fn frame(
    context: &egui::Context,
    adapter: &mut EguiSourceAddressStripAdapter,
    strip: &mut SourceAddressStrip,
) -> Result<(egui::FullOutput, EguiSourceAddressStripOutput), String> {
    frame_with_events(context, adapter, strip, Vec::new())
}

fn frame_with_events(
    context: &egui::Context,
    adapter: &mut EguiSourceAddressStripAdapter,
    strip: &mut SourceAddressStrip,
    events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, EguiSourceAddressStripOutput), String> {
    let mut receipt = None;
    let mut output = context.run_ui(
        egui::RawInput {
            events,
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 120.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                receipt = Some(adapter.show(ui, strip));
            });
        },
    );
    output.textures_delta.clear();
    let receipt = receipt
        .ok_or_else(|| "source-address receipt was not produced".to_owned())?
        .map_err(|error| error.to_string())?;
    Ok((output, receipt))
}

fn direct_frame_with_events(
    context: &egui::Context,
    adapter: &mut EguiSourceAddressStripAdapter,
    strip: &mut SourceAddressStrip,
    events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, EguiSourceAddressStripOutput), String> {
    let mut receipt = None;
    let mut output = context.run_ui(
        egui::RawInput {
            events,
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 120.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| receipt = Some(adapter.show(ui, strip)),
    );
    output.textures_delta.clear();
    let receipt = receipt
        .ok_or_else(|| "source-address direct receipt was not produced".to_owned())?
        .map_err(|error| error.to_string())?;
    Ok((output, receipt))
}

fn accesskit_click(node: egui::accesskit::NodeId) -> egui::Event {
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        action: egui::accesskit::Action::Click,
        target_tree: egui::accesskit::TreeId::ROOT,
        target_node: node,
        data: None,
    })
}

fn accesskit_button(
    output: &egui::FullOutput,
    label: &str,
) -> Result<egui::accesskit::NodeId, String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(node_id, node)| {
                (node.role() == egui::accesskit::Role::Button && node.label() == Some(label))
                    .then_some(*node_id)
            })
        })
        .ok_or_else(|| format!("AccessKit button `{label}` was not published"))
}

fn assert_label_raster_failure(mut strip: SourceAddressStrip) -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-label-error")
        .map_err(|error| error.to_string())?;
    let mut style = SourceAddressRenderStyle::default();
    style.label_font.size = f32::NAN;
    let mut observed = None;

    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 120.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| observed = Some(adapter.show_with_style(ui, &mut strip, &style)),
    );
    output.textures_delta.clear();
    let error = observed
        .ok_or_else(|| "source-address label error route did not run".to_owned())?
        .err()
        .ok_or_else(|| "non-finite label font must fail rasterization".to_owned())?;
    assert!(matches!(error, EguiSourceAddressStripError::Raster(_)));
    Ok(())
}

fn assert_entry_raster_failure(mut strip: SourceAddressStrip, history: bool) -> Result<(), String> {
    if history {
        strip
            .apply_action(SourceAddressAction::OpenHistory)
            .ok_or_else(|| "history must open before entry rendering".to_owned())?;
    } else {
        strip
            .apply_action(SourceAddressAction::OpenCandidates)
            .ok_or_else(|| "candidates must open before entry rendering".to_owned())?;
    }
    let context = egui::Context::default();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-entry-error")
        .map_err(|error| error.to_string())?;
    let mut style = SourceAddressRenderStyle::default();
    style.label_font.size = 14.0;
    let mut observed = None;

    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 120.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| observed = Some(adapter.show_with_style(ui, &mut strip, &style)),
    );
    output.textures_delta.clear();
    let error = observed
        .ok_or_else(|| "source-address entry error route did not run".to_owned())?
        .err()
        .ok_or_else(|| "empty entry label must fail rasterization".to_owned())?;
    assert!(matches!(error, EguiSourceAddressStripError::Raster(_)));
    Ok(())
}

#[derive(Default)]
struct Forwarder(Vec<String>);

impl SourceAddressSubmissionForwarder for Forwarder {
    type Error = Infallible;

    fn forward_submission(
        &mut self,
        submission: katana_ui_core::molecule::structured::source_address_strip::SourceAddressSubmission,
    ) -> Result<(), Self::Error> {
        self.0.push(submission.into_draft());
        Ok(())
    }
}

#[test]
fn actual_egui_surface_publishes_generic_japanese_accesskit_labels() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-contract")
        .map_err(|error| error.to_string())?;
    let mut strip = strip();

    let (output, _) = frame(&context, &mut adapter, &mut strip)?;
    let update = output
        .platform_output
        .accesskit_update
        .ok_or_else(|| "enabled egui context must publish AccessKit".to_owned())?;

    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::TextInput
            && node.label() == Some("ソースアドレス入力")
    }));
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button && node.label() == Some("履歴を開く")
    }));
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button && node.label() == Some("候補を開く")
    }));
    let serialized = format!("{update:?}");
    assert!(!serialized.contains("opaque-history-target"));
    assert!(!serialized.contains("opaque-candidate-target"));
    Ok(())
}

#[test]
fn actual_egui_button_raster_failures_propagate_through_each_control_route() -> Result<(), String> {
    let mut history = SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソースアドレス",
        "入力",
        "入力",
    ));
    history.set_history(vec![SourceAddressEntry::new(
        SourceAddressPresentation::new("履歴", "履歴", "履歴"),
        b"history",
    )]);
    assert_label_raster_failure(history)?;

    let mut candidates = SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソースアドレス",
        "入力",
        "入力",
    ));
    candidates.set_candidates(vec![SourceAddressEntry::new(
        SourceAddressPresentation::new("候補", "候補", "候補"),
        b"candidate",
    )]);
    assert_label_raster_failure(candidates)?;

    assert_label_raster_failure(SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソースアドレス",
        "入力",
        "入力",
    )))?;
    Ok(())
}

#[test]
fn actual_egui_entry_raster_failures_propagate_for_history_and_candidates() -> Result<(), String> {
    let mut history = SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソースアドレス",
        "入力",
        "入力",
    ));
    history.set_history(vec![SourceAddressEntry::new(
        SourceAddressPresentation::new("", "空の履歴", "空の履歴"),
        b"history",
    )]);
    assert_entry_raster_failure(history, true)?;

    let mut candidates = SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソースアドレス",
        "入力",
        "入力",
    ));
    candidates.set_candidates(vec![SourceAddressEntry::new(
        SourceAddressPresentation::new("", "空の候補", "空の候補"),
        b"candidate",
    )]);
    assert_entry_raster_failure(candidates, false)?;
    Ok(())
}

#[test]
fn core_open_state_renders_presentation_only_history_and_targetless_selection() -> Result<(), String>
{
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-history-contract")
        .map_err(|error| error.to_string())?;
    let mut strip = strip();
    assert!(matches!(
        strip.apply_action(SourceAddressAction::OpenHistory),
        Some(SourceAddressEvent::HistoryOpened)
    ));

    let (output, _) = frame(&context, &mut adapter, &mut strip)?;
    let update = output
        .platform_output
        .accesskit_update
        .ok_or_else(|| "open history must remain accessible".to_owned())?;
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button && node.label() == Some("履歴項目")
    }));
    assert!(
        update
            .nodes
            .iter()
            .all(|(_, node)| node.label() != Some("opaque-history-target"))
    );

    assert!(matches!(
        strip.apply_action(SourceAddressAction::SelectHistory(0)),
        Some(SourceAddressEvent::HistorySelected)
    ));
    assert_eq!(strip.draft(), "履歴項目");
    Ok(())
}

#[test]
fn disabled_surface_has_no_mutating_submission_event() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-disabled-contract")
        .map_err(|error| error.to_string())?;
    let mut strip = strip();
    assert!(
        strip
            .apply_action(SourceAddressAction::SetEnabled(false))
            .is_some()
    );

    let (output, _) = frame(&context, &mut adapter, &mut strip)?;
    assert!(output.platform_output.accesskit_update.is_some());
    assert!(strip.apply_action(SourceAddressAction::Submit).is_none());
    assert!(!strip.focused());
    assert!(!strip.history_open());
    assert!(!strip.candidates_open());
    Ok(())
}

#[test]
fn accesskit_activation_reaches_only_generic_event_classes() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-accesskit-contract")
        .map_err(|error| error.to_string())?;
    let mut strip = strip();

    let (initial, _) = frame(&context, &mut adapter, &mut strip)?;
    let history = accesskit_button(&initial, "履歴を開く")?;
    let (_, receipt) = frame_with_events(
        &context,
        &mut adapter,
        &mut strip,
        vec![accesskit_click(history)],
    )?;

    assert_eq!(
        receipt.event_classes(),
        &[SourceAddressFrameEventClass::HistoryOpened]
    );
    assert!(strip.history_open());
    Ok(())
}

#[test]
fn actual_egui_history_can_open_and_close_via_accesskit() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-history-toggle")
        .map_err(|error| error.to_string())?;
    let mut strip = strip();

    let (initial, _) = frame(&context, &mut adapter, &mut strip)?;
    let open = accesskit_button(&initial, "履歴を開く")?;
    let (_, open_receipt) = frame_with_events(
        &context,
        &mut adapter,
        &mut strip,
        vec![accesskit_click(open)],
    )?;
    assert!(
        open_receipt
            .event_classes()
            .contains(&SourceAddressFrameEventClass::HistoryOpened)
    );
    assert!(strip.history_open());

    let (opened, _) = frame(&context, &mut adapter, &mut strip)?;
    let close = accesskit_button(&opened, "履歴を閉じる")?;
    let (_, close_receipt) = frame_with_events(
        &context,
        &mut adapter,
        &mut strip,
        vec![accesskit_click(close)],
    )?;
    assert!(
        close_receipt
            .event_classes()
            .contains(&SourceAddressFrameEventClass::HistoryClosed)
    );
    assert!(!strip.history_open());
    Ok(())
}

#[test]
fn actual_egui_candidates_can_open_and_close_via_accesskit() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-candidates-toggle")
        .map_err(|error| error.to_string())?;
    let mut strip = strip();

    let (initial, _) = frame(&context, &mut adapter, &mut strip)?;
    let open = accesskit_button(&initial, "候補を開く")?;
    let (_, open_receipt) = frame_with_events(
        &context,
        &mut adapter,
        &mut strip,
        vec![accesskit_click(open)],
    )?;
    assert!(
        open_receipt
            .event_classes()
            .contains(&SourceAddressFrameEventClass::CandidatesOpened)
    );
    assert!(strip.candidates_open());

    let (opened, _) = frame(&context, &mut adapter, &mut strip)?;
    let close = accesskit_button(&opened, "候補を閉じる")?;
    let (_, close_receipt) = frame_with_events(
        &context,
        &mut adapter,
        &mut strip,
        vec![accesskit_click(close)],
    )?;
    assert!(
        close_receipt
            .event_classes()
            .contains(&SourceAddressFrameEventClass::CandidatesClosed)
    );
    assert!(!strip.candidates_open());
    Ok(())
}

#[test]
fn accesskit_submit_forwards_a_one_shot_value_without_public_event_payload() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-submit-contract")
        .map_err(|error| error.to_string())?;
    let mut strip = strip();
    strip
        .apply_action(SourceAddressAction::SetDraft("日本語⭐️".to_owned()))
        .ok_or_else(|| "draft update was rejected".to_owned())?;

    let (initial, _) = frame(&context, &mut adapter, &mut strip)?;
    let submit = accesskit_button(&initial, "開く")?;
    let (_, receipt) = frame_with_events(
        &context,
        &mut adapter,
        &mut strip,
        vec![accesskit_click(submit)],
    )?;
    assert_eq!(
        receipt.event_classes(),
        &[SourceAddressFrameEventClass::Submitted]
    );

    let mut forwarder = Forwarder::default();
    receipt
        .forward_submissions_once(&mut forwarder)
        .map_err(|error| error.to_string())?;
    assert_eq!(forwarder.0, ["日本語⭐️"]);
    Ok(())
}

#[test]
fn actual_egui_disabled_strip_disallows_accesskit_activation() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-disabled-toggle")
        .map_err(|error| error.to_string())?;
    let mut strip = strip();
    assert!(
        strip
            .apply_action(SourceAddressAction::SetEnabled(false))
            .is_some()
    );

    let (initial, _) = frame(&context, &mut adapter, &mut strip)?;
    let history = accesskit_button(&initial, "履歴を開く")?;
    let submit = accesskit_button(&initial, "開く")?;
    let candidate = accesskit_button(&initial, "候補を開く")?;

    let (_, history_receipt) = frame_with_events(
        &context,
        &mut adapter,
        &mut strip,
        vec![accesskit_click(history)],
    )?;
    let (_, submit_receipt) = frame_with_events(
        &context,
        &mut adapter,
        &mut strip,
        vec![accesskit_click(submit)],
    )?;
    let (_, candidate_receipt) = frame_with_events(
        &context,
        &mut adapter,
        &mut strip,
        vec![accesskit_click(candidate)],
    )?;

    assert!(!strip.history_open());
    assert!(!strip.candidates_open());
    assert!(history_receipt.event_classes().is_empty());
    assert!(submit_receipt.event_classes().is_empty());
    assert!(candidate_receipt.event_classes().is_empty());
    let mut forwarder = Forwarder::default();
    history_receipt
        .forward_submissions_once(&mut forwarder)
        .map_err(|error| error.to_string())?;
    submit_receipt
        .forward_submissions_once(&mut forwarder)
        .map_err(|error| error.to_string())?;
    candidate_receipt
        .forward_submissions_once(&mut forwarder)
        .map_err(|error| error.to_string())?;
    assert!(forwarder.0.is_empty());
    Ok(())
}

#[test]
fn source_address_error_display_reaches_raster_and_test_only_variants() -> Result<(), String> {
    let config = katana_ui_core::text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(katana_ui_core::text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    let mismatched_config = config.clone().with_emoji_candidate_sha256([
        katana_ui_core::text_raster::PlatformFontSha256::from_bytes([0; 32]),
    ]);
    if mismatched_config.catalog_policy() == *catalog.policy() {
        return Err("test setup did not create a mismatched catalog policy".to_owned());
    }
    let metrics = Rc::new(RefCell::new(
        katana_ui_core::text_raster::PlatformTextMetricsFrame::new(),
    ));
    let catalog_error = EguiSourceAddressStripAdapter::with_catalog_and_metrics(
        "source-address-mismatch-message-contract",
        catalog,
        mismatched_config,
        metrics,
    )
    .err()
    .ok_or_else(|| "mismatched catalog policy should fail".to_owned())?;
    assert_eq!(
        EguiSourceAddressStripError::PaintPlanNotProduced.to_string(),
        "source-address did not produce a paint plan"
    );
    assert_eq!(
        EguiSourceAddressStripError::FrameNotProduced.to_string(),
        "source-address did not produce an input frame"
    );
    match &catalog_error {
        EguiSourceAddressStripError::Raster(error) => {
            let display = format!("{catalog_error}");
            assert!(display.contains("source-address raster failed"));
            let detail = format!("{error}");
            assert!(detail.to_lowercase().contains("catalog"));
        }
        _ => return Err("mismatched catalog policy should fail with raster variant".to_owned()),
    }

    Ok(())
}

#[test]
fn source_address_show_propagates_a_real_text_surface_raster_failure() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-text-error")
        .map_err(|error| error.to_string())?;
    let mut source = strip();
    let mut style = SourceAddressRenderStyle::default();
    style.input_raster.font.size = f32::NAN;
    let mut observed = None;

    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        observed = Some(adapter.show_with_style(ui, &mut source, &style));
    });
    platform_output.textures_delta.clear();
    let error = observed
        .ok_or_else(|| "source-address render did not run".to_owned())?
        .err()
        .ok_or_else(|| "invalid text style should fail closed".to_owned())?;
    assert!(matches!(error, EguiSourceAddressStripError::TextSurface(_)));
    assert!(
        error
            .to_string()
            .contains("source-address text surface failed")
    );
    assert!(adapter.artifact_paint_plan().is_none());
    Ok(())
}

#[test]
fn rendered_input_and_labels_use_kuc_raster_artifacts_with_exact_emoji_variants()
-> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let config = katana_ui_core::text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(katana_ui_core::text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    let metrics = Rc::new(RefCell::new(
        katana_ui_core::text_raster::PlatformTextMetricsFrame::new(),
    ));
    let mut adapter = EguiSourceAddressStripAdapter::with_catalog_and_metrics(
        "source-address-raster-contract",
        catalog,
        config,
        metrics,
    )
    .map_err(|error| error.to_string())?;
    let mut strip = SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソースアドレス",
        "入力",
        "ソースアドレス入力",
    ));
    strip.set_history(vec![
        SourceAddressEntry::new(
            SourceAddressPresentation::new("⭐️", "color emoji", "⭐️"),
            b"opaque-star-emoji",
        ),
        SourceAddressEntry::new(
            SourceAddressPresentation::new("☆", "text star", "☆"),
            b"opaque-star-control",
        ),
    ]);
    strip
        .apply_action(SourceAddressAction::SetDraft("日本語⭐️".to_owned()))
        .ok_or_else(|| "draft update was rejected".to_owned())?;
    strip
        .apply_action(SourceAddressAction::OpenHistory)
        .ok_or_else(|| "history open was rejected".to_owned())?;

    let (_, _) = frame(&context, &mut adapter, &mut strip)?;
    let evidence = adapter
        .raster_evidence()
        .ok_or_else(|| "input must be produced by EguiTextSurfaceAdapter".to_owned())?;
    assert!(evidence.input_has_text_texture());
    assert_eq!(evidence.input_paint_plan_hash().len(), 64);

    let fingerprint = |value: &str| hex::encode(Sha256::digest(value.as_bytes()));
    let emoji = evidence
        .label_rasters()
        .iter()
        .find(|raster| raster.label_fingerprint == fingerprint("⭐️"))
        .ok_or_else(|| "VS16 emoji label raster was not produced".to_owned())?;
    let control = evidence
        .label_rasters()
        .iter()
        .find(|raster| raster.label_fingerprint == fingerprint("☆"))
        .ok_or_else(|| "text-star label raster was not produced".to_owned())?;
    assert!(emoji.chromatic_pixel_count > 0);
    assert_eq!(control.chromatic_pixel_count, 0);
    assert_ne!(emoji.sha256, control.sha256);
    assert!(
        evidence
            .label_rasters()
            .iter()
            .any(|raster| raster.label_fingerprint == fingerprint("履歴を閉じる"))
    );
    Ok(())
}

#[test]
fn mismatched_catalog_policy_is_returned_from_with_catalog_and_metrics() -> Result<(), String> {
    let config = katana_ui_core::text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(katana_ui_core::text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    let mismatched_config = config.clone().with_emoji_candidate_sha256([
        katana_ui_core::text_raster::PlatformFontSha256::from_bytes([0; 32]),
    ]);
    if mismatched_config.catalog_policy() == *catalog.policy() {
        return Err("test setup did not create a mismatched catalog policy".to_owned());
    }
    let metrics = Rc::new(RefCell::new(
        katana_ui_core::text_raster::PlatformTextMetricsFrame::new(),
    ));
    let result = EguiSourceAddressStripAdapter::with_catalog_and_metrics(
        "source-address-mismatched-policy",
        catalog,
        mismatched_config,
        metrics,
    );
    assert!(matches!(
        result,
        Err(
            katana_ui_core::egui::source_address_strip::EguiSourceAddressStripError::Raster(
                katana_ui_core::text_raster::PlatformTextRasterError::CatalogConfigurationMismatch
            )
        )
    ));
    Ok(())
}

#[test]
fn root_source_address_artifact_contains_every_visible_control_and_changes_with_buttons()
-> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-artifact")
        .map_err(|error| error.to_string())?;
    let mut closed = strip();
    let (_, _) = frame(&context, &mut adapter, &mut closed)?;
    let closed_plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "closed root plan was not produced".to_owned())?;
    let closed_hash = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(closed_plan.surface_bounds),
        plans: &[ArtifactPaintPlanRef::SourceAddress(closed_plan)],
    })
    .map_err(|error| error.to_string())?
    .pixel_hash;

    let mut opened = strip();
    opened
        .apply_action(SourceAddressAction::OpenHistory)
        .ok_or_else(|| "open history was rejected".to_owned())?;
    let (_, _) = frame(&context, &mut adapter, &mut opened)?;
    let opened_plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "open root plan was not produced".to_owned())?;
    let opened_hash = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(opened_plan.surface_bounds),
        plans: &[ArtifactPaintPlanRef::SourceAddress(opened_plan)],
    })
    .map_err(|error| error.to_string())?
    .pixel_hash;

    assert_ne!(
        closed_hash, opened_hash,
        "visible history must affect the root artifact"
    );
    assert!(
        opened_plan.operations.len() >= 10,
        "input, three controls, and history item"
    );
    assert!(matches!(
        opened_plan
            .operations
            .first()
            .map(|operation| &operation.kind),
        Some(SourceAddressPaintOperationKind::Input(_))
    ));
    let button_fills: Vec<_> = opened_plan
        .operations
        .iter()
        .filter_map(|operation| match operation.kind {
            SourceAddressPaintOperationKind::Fill { bounds, .. } => Some(bounds),
            _ => None,
        })
        .collect();
    assert!(button_fills.len() >= 3);
    for (index, left) in button_fills.iter().enumerate() {
        for right in button_fills.iter().skip(index + 1) {
            assert!(!rectangles_overlap(*left, *right), "button bounds overlap");
        }
    }
    Ok(())
}

#[test]
fn source_address_artifact_plan_is_closed_and_preserves_vs16_raster_difference()
-> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-plan-leak")
        .map_err(|error| error.to_string())?;
    let mut source =
        SourceAddressStrip::new(SourceAddressPresentation::new("入力⭐️", "入力", "入力⭐️"));
    source.set_history(vec![
        SourceAddressEntry::new(
            SourceAddressPresentation::new("⭐️", "star", "⭐️"),
            b"opaque-target",
        ),
        SourceAddressEntry::new(
            SourceAddressPresentation::new("☆", "star-control", "☆"),
            b"opaque-target-2",
        ),
    ]);
    source
        .apply_action(SourceAddressAction::SetDraft("日本語⭐️".to_owned()))
        .ok_or_else(|| "draft was rejected".to_owned())?;
    source
        .apply_action(SourceAddressAction::OpenHistory)
        .ok_or_else(|| "history was rejected".to_owned())?;
    let (_, _) = frame(&context, &mut adapter, &mut source)?;
    let plan = adapter
        .artifact_paint_plan()
        .ok_or_else(|| "artifact plan was not produced".to_owned())?;
    let encoded = serde_json::to_string(plan).map_err(|error| error.to_string())?;
    for forbidden in [
        "日本語⭐️",
        "入力⭐️",
        "opaque-target",
        "opaque-target-2",
        "⭐️",
        "☆",
    ] {
        assert!(
            !encoded.contains(forbidden),
            "raw value leaked: {forbidden}"
        );
    }
    let chromatic_counts: Vec<_> = plan
        .operations
        .iter()
        .filter_map(|operation| match &operation.kind {
            SourceAddressPaintOperationKind::Texture { texture, .. } => {
                Some(chromatic_pixels(&texture.rgba_pixels))
            }
            SourceAddressPaintOperationKind::Input(
                katana_ui_core::egui::text_surface::TextSurfacePaintOperationKind::Texture {
                    texture,
                    ..
                },
            ) => Some(chromatic_pixels(&texture.rgba_pixels)),
            _ => None,
        })
        .collect();
    assert!(chromatic_counts.iter().any(|count| *count > 0));
    assert!(chromatic_counts.contains(&0));
    Ok(())
}

#[test]
fn physical_text_and_enter_submit_through_the_source_address_adapter() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-physical-submit")
        .map_err(|error| error.to_string())?;
    let mut source = SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソースアドレス",
        "ソースアドレスを入力",
        "ソースアドレス入力",
    ));

    let _ = direct_frame_with_events(&context, &mut adapter, &mut source, Vec::new())?;
    let input_bounds = adapter
        .artifact_paint_plan()
        .and_then(|plan| {
            plan.operations.iter().find_map(|operation| {
                matches!(operation.kind, SourceAddressPaintOperationKind::Input(_))
                    .then_some(operation.clip_bounds)
            })
        })
        .ok_or_else(|| "source-address input must have a KUC-owned paint bound".to_owned())?;
    let input_point = egui::pos2(
        input_bounds.x as f32 + input_bounds.width as f32 / 2.0,
        input_bounds.y as f32 + input_bounds.height as f32 / 2.0,
    );
    let (_, focused) = direct_frame_with_events(
        &context,
        &mut adapter,
        &mut source,
        vec![
            egui::Event::PointerMoved(input_point),
            egui::Event::PointerButton {
                pos: input_point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: input_point,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
    )?;
    assert!(
        focused
            .event_classes()
            .contains(&SourceAddressFrameEventClass::Focused),
        "input bounds: {input_bounds:?}"
    );
    assert!(
        !focused
            .event_classes()
            .contains(&SourceAddressFrameEventClass::Blurred)
    );
    let (_, typed) = direct_frame_with_events(
        &context,
        &mut adapter,
        &mut source,
        vec![egui::Event::Text("draft-one".to_owned())],
    )?;
    assert!(
        typed
            .event_classes()
            .contains(&SourceAddressFrameEventClass::DraftChanged)
    );
    let (_, submitted) = direct_frame_with_events(
        &context,
        &mut adapter,
        &mut source,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    assert!(
        submitted
            .event_classes()
            .contains(&SourceAddressFrameEventClass::Submitted)
    );
    let mut forwarder = Forwarder::default();
    submitted
        .forward_submissions_once(&mut forwarder)
        .map_err(|error| error.to_string())?;
    assert_eq!(forwarder.0, ["draft-one"]);
    Ok(())
}

#[test]
fn direct_frame_path_with_no_accesskit_keeps_state_transitions_predictable() -> Result<(), String> {
    let context = egui::Context::default();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-direct-contract")
        .map_err(|error| error.to_string())?;
    let mut source = strip();

    let (initial, initial_receipt) =
        direct_frame_with_events(&context, &mut adapter, &mut source, Vec::new())?;
    assert!(initial.platform_output.accesskit_update.is_none());
    assert!(initial_receipt.event_classes().is_empty());

    let _ = source
        .apply_action(SourceAddressAction::OpenHistory)
        .ok_or_else(|| "history action must be accepted".to_owned())?;
    let (history, history_receipt) =
        direct_frame_with_events(&context, &mut adapter, &mut source, Vec::new())?;
    assert!(history.platform_output.accesskit_update.is_none());
    assert!(source.history_open());
    assert!(history_receipt.event_classes().is_empty());

    let input_bounds = adapter
        .artifact_paint_plan()
        .and_then(|plan| {
            plan.operations.iter().find_map(|operation| {
                matches!(operation.kind, SourceAddressPaintOperationKind::Input(_))
                    .then_some(operation.clip_bounds)
            })
        })
        .ok_or_else(|| "source-address input must have a paint bound".to_owned())?;
    let input_point = egui::pos2(
        input_bounds.x as f32 + input_bounds.width as f32 / 2.0,
        input_bounds.y as f32 + input_bounds.height as f32 / 2.0,
    );
    let (_, focused) = direct_frame_with_events(
        &context,
        &mut adapter,
        &mut source,
        vec![
            egui::Event::PointerMoved(input_point),
            egui::Event::PointerButton {
                button: egui::PointerButton::Primary,
                pressed: true,
                pos: input_point,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                button: egui::PointerButton::Primary,
                pressed: false,
                pos: input_point,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    )?;
    assert!(
        focused
            .event_classes()
            .contains(&SourceAddressFrameEventClass::Focused),
        "direct path should be able to focus without accesskit"
    );

    let (_, typed) = direct_frame_with_events(
        &context,
        &mut adapter,
        &mut source,
        vec![egui::Event::Text("from direct path".to_owned())],
    )?;
    assert!(
        typed
            .event_classes()
            .iter()
            .any(|class| *class == SourceAddressFrameEventClass::DraftChanged),
        "direct text input must still emit a draft change"
    );
    Ok(())
}

#[test]
fn actual_egui_selects_history_and_candidate_entries_via_accesskit() -> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-entry-selection")
        .map_err(|error| error.to_string())?;
    let mut source = strip();

    let (initial, _) = frame(&context, &mut adapter, &mut source)?;
    let history_toggle = accesskit_button(&initial, "履歴を開く")?;
    let (_, opened_history) = frame_with_events(
        &context,
        &mut adapter,
        &mut source,
        vec![accesskit_click(history_toggle)],
    )?;
    assert!(
        opened_history
            .event_classes()
            .contains(&SourceAddressFrameEventClass::HistoryOpened)
    );

    let (history_frame, _) = frame(&context, &mut adapter, &mut source)?;
    let history_entry = accesskit_button(&history_frame, "履歴項目")?;
    let (_, selected_history) = frame_with_events(
        &context,
        &mut adapter,
        &mut source,
        vec![accesskit_click(history_entry)],
    )?;
    assert_eq!(source.draft(), "履歴項目");
    assert!(
        selected_history
            .event_classes()
            .contains(&SourceAddressFrameEventClass::HistorySelected)
    );

    let (closed_history, _) = frame(&context, &mut adapter, &mut source)?;
    let candidates_toggle = accesskit_button(&closed_history, "候補を開く")?;
    let (_, opened_candidates) = frame_with_events(
        &context,
        &mut adapter,
        &mut source,
        vec![accesskit_click(candidates_toggle)],
    )?;
    assert!(
        opened_candidates
            .event_classes()
            .contains(&SourceAddressFrameEventClass::CandidatesOpened)
    );

    let (candidate_frame, _) = frame(&context, &mut adapter, &mut source)?;
    let candidate_entry = accesskit_button(&candidate_frame, "候補項目")?;
    let (_, selected_candidate) = frame_with_events(
        &context,
        &mut adapter,
        &mut source,
        vec![accesskit_click(candidate_entry)],
    )?;
    assert_eq!(source.draft(), "候補項目");
    assert!(
        selected_candidate
            .event_classes()
            .contains(&SourceAddressFrameEventClass::CandidateSelected)
    );
    Ok(())
}

fn rectangles_overlap(
    left: katana_ui_core::render_model::UiRect,
    right: katana_ui_core::render_model::UiRect,
) -> bool {
    let left_right = left.x + left.width as i32;
    let right_right = right.x + right.width as i32;
    let left_bottom = left.y + left.height as i32;
    let right_bottom = right.y + right.height as i32;
    left.x < right_right && right.x < left_right && left.y < right_bottom && right.y < left_bottom
}

fn chromatic_pixels(rgba: &[u8]) -> usize {
    rgba.chunks_exact(4)
        .filter(|pixel| pixel[3] > 0 && (pixel[0] != pixel[1] || pixel[1] != pixel[2]))
        .count()
}
