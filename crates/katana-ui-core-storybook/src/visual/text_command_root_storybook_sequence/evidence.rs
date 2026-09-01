use super::super::model::{
    EventReceiptEvidence, FullRootArtifactError, FullRootStep, RootEvidence,
};
use super::super::{FRAME_HEIGHT, FRAME_WIDTH};
use super::scenario::Scenario;
use eframe::egui;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceRootEventTransport,
    KucRootEventBatchForwarder,
};
use std::convert::Infallible;

struct RootEventReceiptForwarder {
    calls: usize,
}

impl KucRootEventBatchForwarder for RootEventReceiptForwarder {
    type Error = Infallible;

    fn forward_root_event_batch(
        &mut self,
        _transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.calls = self.calls.saturating_add(1);
        Ok(())
    }
}

pub(super) fn capture_step(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    step: Scenario,
) -> Result<FullRootStep, FullRootArtifactError> {
    let Scenario {
        name,
        input,
        events,
    } = step;
    let mut output = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(FRAME_WIDTH, FRAME_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| output = Some(root.show(ui)),
    );
    full_output.textures_delta.clear();
    let output = output.ok_or(FullRootArtifactError::Contract(
        "egui did not execute the root-frame UI closure".into(),
    ))??;
    let mut forwarder = RootEventReceiptForwarder { calls: 0 };
    let receipt = output.forward_events_once(&mut forwarder)?;
    let dimensions = output.record().dimensions();
    let evidence = RootEvidence {
        identity: output.record().identity().to_string(),
        state_revision: output.record().state_revision(),
        width: dimensions.width(),
        height: dimensions.height(),
        rgba_sha256: output.record().rgba_hash().to_string(),
        plan_sha256: output.record().paint_plan_hash().to_string(),
        record_sha256: output.record().record_hash().to_string(),
        accesskit_snapshot_sha256: output.record().accessibility_snapshot_hash().to_string(),
        event_receipt: EventReceiptEvidence {
            root_identity: receipt.root_identity().to_string(),
            state_revision: receipt.state_revision(),
            correlation_fingerprint: receipt.correlation_fingerprint().to_string(),
            event_batch_fingerprint: receipt.event_batch_fingerprint().to_string(),
            consumed_once: receipt.consumed_once(),
            event_cardinality: receipt.event_cardinality(),
            forwarder_calls: forwarder.calls,
        },
    };
    assert!(
        evidence.width > 0 && evidence.height > 0,
        "{name} must produce a non-empty root frame"
    );
    Ok(FullRootStep {
        name,
        input,
        evidence,
        frame: output,
    })
}
