use super::stage_interactions::render_stage;
use super::support::{
    cleanup_stage_output, map_root_error, preflight_output, sha256, validate_decoded_png,
    write_manifest,
};
use super::unicode_evidence::{bind_unicode_evidence, capture_unicode_evidence};
use super::{
    ConsumerArtifactEvidence, ConsumerArtifactForwardingReceipt, ConsumerArtifactPlanError,
    ConsumerArtifactPlanV1, ConsumerArtifactStageBinding, EguiTextCommandSurfaceHostRoot,
    GenericEffectClass, GenericInteractionClass, SCHEMA_VERSION,
};
use crate::egui::OpaqueRootArtifactReceiptWriter;
use crate::egui::text_command_surface::{
    EguiTextCommandSurfaceRootFactory, KucUnicodeColorGlyphEvidenceOptions,
};
use std::collections::BTreeSet;
use std::path::Path;

/// KUC issuer for opaque consumer artifact plans.
#[derive(Debug, Clone)]
pub struct ConsumerArtifactPlanIssuer {
    pub(super) unicode_evidence_options: KucUnicodeColorGlyphEvidenceOptions,
}

impl ConsumerArtifactPlanIssuer {
    #[must_use]
    pub fn new() -> Self {
        Self::with_unicode_evidence_options(
            super::unicode_evidence::artifact_unicode_evidence_options(),
        )
    }

    /// Supplies a release-verified color-emoji pin for artifact Unicode evidence.
    #[must_use]
    pub fn with_unicode_evidence_options(
        unicode_evidence_options: KucUnicodeColorGlyphEvidenceOptions,
    ) -> Self {
        Self {
            unicode_evidence_options,
        }
    }
}

impl Default for ConsumerArtifactPlanIssuer {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsumerArtifactPlanIssuer {
    pub fn issue(
        &self,
        mut plan: ConsumerArtifactPlanV1,
    ) -> Result<IssuedConsumerArtifactPlan, ConsumerArtifactPlanError> {
        if plan.schema_version != SCHEMA_VERSION {
            return Err(ConsumerArtifactPlanError::UnsupportedSchemaVersion(
                plan.schema_version,
            ));
        }
        if plan.bindings.is_empty() {
            return Err(ConsumerArtifactPlanError::EmptyPlan);
        }
        let mut leaves = BTreeSet::new();
        for binding in &plan.bindings {
            if binding.effect != GenericEffectClass::NoHostEffect {
                return Err(ConsumerArtifactPlanError::UnsupportedEffectClass(
                    binding.effect,
                ));
            }
            if !leaves.insert(binding.leaf.clone()) {
                return Err(ConsumerArtifactPlanError::DuplicateLeaf(
                    binding.leaf.0.clone(),
                ));
            }
        }
        if plan.bindings.len() != GenericInteractionClass::FULL_EDITOR_SEQUENCE.len()
            || plan
                .bindings
                .iter()
                .zip(GenericInteractionClass::FULL_EDITOR_SEQUENCE)
                .any(|(binding, expected)| binding.interaction != expected)
        {
            return Err(ConsumerArtifactPlanError::IncompleteStageSequence);
        }
        for (index, binding) in plan.bindings.iter().enumerate() {
            let expected = plan
                .initial_revision
                .checked_add(index as u64)
                .ok_or(ConsumerArtifactPlanError::RevisionOverflow)?;
            let actual = binding
                .token
                .as_ref()
                .ok_or(ConsumerArtifactPlanError::StageAlreadyConsumed(index))?
                .revision();
            if actual != expected {
                return Err(ConsumerArtifactPlanError::StaleRevision {
                    stage: index,
                    expected,
                    actual,
                });
            }
        }
        let factory = EguiTextCommandSurfaceRootFactory::new();
        let first_token = plan.bindings[0]
            .token
            .as_ref()
            .ok_or(ConsumerArtifactPlanError::StageAlreadyConsumed(0))?;
        for (index, binding) in plan.bindings.iter().enumerate().skip(1) {
            let token = binding
                .token
                .as_ref()
                .ok_or(ConsumerArtifactPlanError::StageAlreadyConsumed(index))?;
            if !factory
                .has_same_root_identity(first_token, token)
                .map_err(map_root_error)?
            {
                return Err(ConsumerArtifactPlanError::TokenRootMismatch);
            }
        }
        let first = plan.bindings[0]
            .token
            .take()
            .ok_or(ConsumerArtifactPlanError::StageAlreadyConsumed(0))?;
        let root = factory.retain(first).map_err(map_root_error)?;
        Ok(IssuedConsumerArtifactPlan {
            root,
            bindings: plan.bindings,
            unicode_evidence_options: self.unicode_evidence_options.clone(),
            next_stage: 0,
            prepared_stage: None,
            failed_stage: None,
            root_revision: plan.initial_revision,
            receipt_root_identity_fingerprint: None,
        })
    }
}

/// Issued stages retain one KUC root and can only run in their KUC-defined order.
pub struct IssuedConsumerArtifactPlan {
    root: EguiTextCommandSurfaceHostRoot,
    bindings: Vec<ConsumerArtifactStageBinding>,
    unicode_evidence_options: KucUnicodeColorGlyphEvidenceOptions,
    next_stage: usize,
    prepared_stage: Option<usize>,
    failed_stage: Option<usize>,
    root_revision: u64,
    receipt_root_identity_fingerprint: Option<String>,
}

impl IssuedConsumerArtifactPlan {
    #[must_use]
    pub fn remaining_stage_count(&self) -> usize {
        self.bindings.len().saturating_sub(self.next_stage)
    }

    /// Consumes one receipt only when it originated from this retained root.
    pub fn consume_forwarding_receipt_once(
        &self,
        receipt: ConsumerArtifactForwardingReceipt,
    ) -> Result<(), ConsumerArtifactPlanError> {
        let root_identity_fingerprint = self
            .receipt_root_identity_fingerprint
            .as_deref()
            .ok_or(ConsumerArtifactPlanError::ReceiptCrossBind)?;
        let leaf = receipt.leaf.clone();
        let stage_id = receipt.stage_id.clone();
        let root_revision = receipt.root_revision;
        receipt.consume_once(root_identity_fingerprint, &leaf, &stage_id, root_revision)
    }

    pub fn execute_next(
        &mut self,
        context: &egui::Context,
        output_dir: &Path,
    ) -> Result<ConsumerArtifactEvidence, ConsumerArtifactPlanError> {
        if let Some(stage) = self.failed_stage {
            return Err(ConsumerArtifactPlanError::StageExecutionFailed(stage));
        }
        let index = self.next_stage;
        let stage_id = format!("consumer-stage-{index:04}");
        preflight_output(output_dir, &stage_id)?;
        let (interaction, leaf, action_target) = {
            let binding = self
                .bindings
                .get_mut(index)
                .ok_or(ConsumerArtifactPlanError::PlanComplete)?;
            if index > 0 && self.prepared_stage != Some(index) {
                let token = binding
                    .token
                    .take()
                    .ok_or(ConsumerArtifactPlanError::StageAlreadyConsumed(index))?;
                self.root.synchronize(token).map_err(map_root_error)?;
                self.prepared_stage = Some(index);
            }
            (
                binding.interaction,
                binding.leaf.clone(),
                binding.action_target().to_owned(),
            )
        };
        self.failed_stage = Some(index);
        let result = (|| {
            let frame = render_stage(&mut self.root, context, interaction, action_target.as_str())?;
            let receipt = write_stage_artifact(&frame, output_dir, &stage_id)?;
            let artifact = receipt.artifact().clone();
            validate_decoded_png(&artifact)?;
            let root_revision = self.root_revision + index as u64;
            let root_identity_fingerprint = sha256(frame.record().identity().as_bytes());
            self.receipt_root_identity_fingerprint = Some(root_identity_fingerprint.clone());
            let receipt_fingerprint = sha256(
                format!(
                    "{root_identity_fingerprint}:{}:{}:{}",
                    leaf.0,
                    stage_id,
                    frame.record().record_hash()
                )
                .as_bytes(),
            );
            let unicode_json = capture_unicode_evidence(self.unicode_evidence_options.clone())?;
            let unicode_hash = bind_unicode_evidence(
                &unicode_json,
                &stage_id,
                &leaf,
                root_revision,
                frame.record().record_hash(),
                frame.record().accessibility_snapshot_hash(),
                &receipt_fingerprint,
            );
            let evidence = ConsumerArtifactEvidence {
                stage_id: stage_id.clone(),
                leaf: leaf.clone(),
                root_revision,
                png_sha256: artifact.png_sha256().to_owned(),
                pixel_hash: artifact.pixel_hash().to_owned(),
                root_record_hash: artifact.root_record_hash().to_owned(),
                accesskit_snapshot_hash: frame.record().accessibility_snapshot_hash().to_owned(),
                unicode_evidence_hash: unicode_hash,
                unicode_evidence_json: unicode_json,
                receipt: ConsumerArtifactForwardingReceipt {
                    leaf: leaf.clone(),
                    stage_id: stage_id.clone(),
                    root_revision: self.root_revision + index as u64,
                    root_identity_fingerprint,
                    consumed: false,
                    fingerprint: receipt_fingerprint,
                },
            };
            write_manifest(output_dir, &evidence)?;
            Ok(evidence)
        })();
        match result {
            Ok(evidence) => {
                self.next_stage += 1;
                self.prepared_stage = None;
                self.failed_stage = None;
                Ok(evidence)
            }
            Err(error) => {
                cleanup_stage_output(output_dir, &stage_id)?;
                Err(error)
            }
        }
    }
}

pub(super) fn show_frame(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    input: egui::RawInput,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let mut frame = None;
    let mut output = context.run_ui(input, |ui| {
        frame = Some(root.show(ui));
    });
    output.textures_delta.clear();
    frame
        .ok_or(ConsumerArtifactPlanError::MissingFrame)?
        .map_err(map_root_error)
}

pub(super) fn interaction_error(error: impl std::fmt::Display) -> ConsumerArtifactPlanError {
    ConsumerArtifactPlanError::Artifact(format!("KUC interaction protocol failed: {error}"))
}

fn write_stage_artifact(
    frame: &crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    output_dir: &Path,
    stage_id: &str,
) -> Result<crate::egui::OpaqueRootArtifactReceipt, ConsumerArtifactPlanError> {
    OpaqueRootArtifactReceiptWriter::new()
        .write(frame, output_dir, stage_id)
        .map_err(|error| ConsumerArtifactPlanError::Artifact(error.to_string()))
}
