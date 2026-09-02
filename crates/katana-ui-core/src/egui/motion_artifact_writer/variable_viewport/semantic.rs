use crate::egui::opaque_motion_receipt::MotionFrameSemanticEvidence;
use crate::egui::text_command_surface::STAR_TEXT;
use crate::egui::{OpaqueRootArtifactReceipt, VariableViewportSemanticEvidence};
use sha2::{Digest, Sha256};

use super::error::VariableViewportMotionArtifactError;

pub(super) fn semantic_evidence(
    receipts: &[OpaqueRootArtifactReceipt],
) -> Result<VariableViewportSemanticEvidence, VariableViewportMotionArtifactError> {
    let observations = receipts
        .iter()
        .map(|receipt| {
            let evidence = receipt.motion_semantics().ok_or_else(|| {
                VariableViewportMotionArtifactError::InvalidSemanticEvidence(
                    "receipt is missing same-frame semantic evidence".into(),
                )
            })?;
            if evidence.root_record_hash != receipt.artifact().root_record_hash() {
                return Err(
                    VariableViewportMotionArtifactError::UnrelatedSemanticEvidence {
                        root_record_hash: evidence.root_record_hash.clone(),
                    },
                );
            }
            Ok(evidence)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let expected_star = STAR_TEXT.chars().map(u32::from).collect::<Vec<_>>();
    let star = observations
        .iter()
        .copied()
        .find(|evidence| valid_star_evidence(evidence, &expected_star))
        .ok_or_else(|| {
            VariableViewportMotionArtifactError::InvalidSemanticEvidence(
                "exact chromatic star and hit-test evidence are required".into(),
            )
        })?;
    let preedit = find_observation(
        &observations,
        |evidence| evidence.ime_preedit_event_seen,
        "IME preedit",
    )?;
    let commit = find_observation(
        &observations,
        |evidence| evidence.ime_commit_event_seen,
        "IME commit",
    )?;
    if commit.accesskit_snapshot_hash.is_empty() {
        return Err(
            VariableViewportMotionArtifactError::InvalidSemanticEvidence(
                "commit frame AccessKit snapshot is required".into(),
            ),
        );
    }

    let mut root_record_hashes = Vec::new();
    for root_record_hash in [
        &star.root_record_hash,
        &preedit.root_record_hash,
        &commit.root_record_hash,
    ] {
        if !root_record_hashes.contains(root_record_hash) {
            root_record_hashes.push(root_record_hash.clone());
        }
    }
    let mut summary = VariableViewportSemanticEvidence {
        artifact_sha256: String::new(),
        root_record_hash: commit.root_record_hash.clone(),
        root_record_hashes,
        star_scalar_sequence: expected_star,
        ime_preedit_event_seen: true,
        ime_commit_event_seen: true,
        hit_test_count: 1,
        accesskit_snapshot_hash: commit.accesskit_snapshot_hash.clone(),
    };
    let canonical_json = serde_json::to_vec(&summary).map_err(semantic_serialization_error)?;
    summary.artifact_sha256 = hex::encode(Sha256::digest(canonical_json));
    Ok(summary)
}

pub(super) fn semantic_serialization_error(
    error: serde_json::Error,
) -> VariableViewportMotionArtifactError {
    VariableViewportMotionArtifactError::InvalidSemanticEvidence(error.to_string())
}

fn valid_star_evidence(evidence: &MotionFrameSemanticEvidence, expected_star: &[u32]) -> bool {
    evidence.star_scalar_sequence == expected_star
        && evidence.star_chromatic_pixel_count > evidence.control_star_chromatic_pixel_count
        && evidence.star_hit_test_seen
}

fn find_observation<'a>(
    observations: &'a [&MotionFrameSemanticEvidence],
    predicate: impl Fn(&MotionFrameSemanticEvidence) -> bool,
    kind: &str,
) -> Result<&'a MotionFrameSemanticEvidence, VariableViewportMotionArtifactError> {
    observations
        .iter()
        .copied()
        .find(|evidence| predicate(evidence))
        .ok_or_else(|| {
            VariableViewportMotionArtifactError::InvalidSemanticEvidence(format!(
                "{kind} evidence is required from the motion sequence"
            ))
        })
}
