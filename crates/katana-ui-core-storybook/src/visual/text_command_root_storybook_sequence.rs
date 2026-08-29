#[path = "text_command_root_storybook_sequence/evidence.rs"]
mod evidence;
#[path = "text_command_root_storybook_sequence/input.rs"]
mod input;
#[path = "text_command_root_storybook_sequence/scenario.rs"]
mod scenario;

use super::model::{FullRootArtifactError, FullRootSequence};
use super::{FULL_ROOT_FRAME_COUNT, build_root, validate_full_root_frame_count};
use eframe::egui;
#[cfg(test)]
use sha2::{Digest, Sha256};
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::path::Path;

pub(super) fn run_scripted_sequence() -> Result<FullRootSequence, FullRootArtifactError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = build_root()?;
    let steps = scenario::scripted_steps()
        .into_iter()
        .map(|step| evidence::capture_step(&context, &mut root, step))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(FullRootSequence { steps })
}

pub(super) fn validate_sequence(sequence: &FullRootSequence) -> Result<(), FullRootArtifactError> {
    validate_full_root_frame_count(sequence)?;
    if sequence.steps.len() != FULL_ROOT_FRAME_COUNT {
        return Err(FullRootArtifactError::Contract(format!(
            "full-root trace must contain exactly {FULL_ROOT_FRAME_COUNT} steps"
        )));
    }
    let identity = sequence
        .steps
        .first()
        .ok_or(FullRootArtifactError::Contract(
            "full-root trace is empty".into(),
        ))?
        .evidence
        .identity
        .as_str();
    if identity.is_empty() {
        return Err(FullRootArtifactError::Contract(format!(
            "unexpected root identity: {identity}"
        )));
    }
    if sequence
        .steps
        .iter()
        .any(|step| step.evidence.event_receipt.root_identity != step.evidence.identity)
    {
        return Err(FullRootArtifactError::Contract(
            "root event receipt identity does not match the retained root record".into(),
        ));
    }
    if sequence
        .steps
        .iter()
        .any(|step| step.evidence.event_receipt.forwarder_calls != 1)
    {
        return Err(FullRootArtifactError::Contract(
            "a root event batch was not forwarded exactly once".into(),
        ));
    }
    for required in scenario::required_step_names() {
        let step = sequence
            .steps
            .iter()
            .find(|step| step.name == *required)
            .ok_or_else(|| FullRootArtifactError::Contract(format!("missing step {required}")))?;
        if step.evidence.event_receipt.event_cardinality == 0 {
            return Err(FullRootArtifactError::Contract(format!(
                "step {required} produced no typed root event"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn frame_sequence_sha256(
    frame_paths: &[impl AsRef<Path>],
) -> Result<String, FullRootArtifactError> {
    let mut digest = Sha256::new();
    digest.update((frame_paths.len() as u64).to_le_bytes());
    for path in frame_paths {
        let bytes = fs::read(path.as_ref())?;
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(bytes);
    }
    Ok(hex::encode(digest.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_sequence_rejects_short_sequence() {
        assert!(matches!(
            validate_sequence(&FullRootSequence { steps: Vec::new() }),
            Err(FullRootArtifactError::Contract(error))
                if error == format!(
                    "full-root trace must contain at least {FULL_ROOT_FRAME_COUNT} steps"
                )
        ));
    }

    #[test]
    fn validate_sequence_rejects_mismatched_root_identity() -> Result<(), FullRootArtifactError> {
        let mut sequence = run_scripted_sequence()?;
        sequence.steps[0].evidence.identity = String::new();
        assert!(matches!(
            validate_sequence(&sequence),
            Err(FullRootArtifactError::Contract(error))
                if error.contains("unexpected root identity")
        ));
        Ok(())
    }

    #[test]
    fn validate_sequence_rejects_wrong_root_receipt() -> Result<(), FullRootArtifactError> {
        let mut sequence = run_scripted_sequence()?;
        sequence
            .steps
            .iter_mut()
            .for_each(|step| step.evidence.event_receipt.root_identity = "mismatch".into());
        assert!(matches!(
            validate_sequence(&sequence),
            Err(FullRootArtifactError::Contract(error))
                if error
                    == "root event receipt identity does not match the retained root record"
        ));
        Ok(())
    }

    #[test]
    fn validate_sequence_rejects_missing_required_step() -> Result<(), FullRootArtifactError> {
        let mut sequence = run_scripted_sequence()?;
        if let Some(step) = sequence.steps.get_mut(1) {
            step.name = "not-a-required-step";
        }
        assert!(matches!(
            validate_sequence(&sequence),
            Err(FullRootArtifactError::Contract(error))
                if error.contains("missing step focus-and-multiline-input")
        ));
        Ok(())
    }

    #[test]
    fn validate_sequence_rejects_unforwarded_root_events() -> Result<(), FullRootArtifactError> {
        let mut sequence = run_scripted_sequence()?;
        sequence
            .steps
            .iter_mut()
            .for_each(|step| step.evidence.event_receipt.forwarder_calls = 0);
        assert!(matches!(
            validate_sequence(&sequence),
            Err(FullRootArtifactError::Contract(error))
                if error == "a root event batch was not forwarded exactly once"
        ));
        Ok(())
    }

    #[test]
    fn validate_sequence_rejects_empty_required_event_batch() -> Result<(), FullRootArtifactError> {
        let mut sequence = run_scripted_sequence()?;
        sequence.steps[1].evidence.event_receipt.event_cardinality = 0;
        assert!(matches!(
            validate_sequence(&sequence),
            Err(FullRootArtifactError::Contract(error))
                if error.contains("produced no typed root event")
        ));
        Ok(())
    }

    #[test]
    fn frame_sequence_sha256_reports_file_io_failure() {
        let result = frame_sequence_sha256(&[std::path::Path::new("/tmp/does-not-exist.bin")]);
        assert!(matches!(
            result,
            Err(FullRootArtifactError::Io(error))
                if error.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn validate_sequence_rejects_missing_required_steps_when_empty() {
        assert!(matches!(
            validate_sequence(&FullRootSequence { steps: Vec::new() }),
            Err(FullRootArtifactError::Contract(error))
                if error == format!(
                    "full-root trace must contain at least {FULL_ROOT_FRAME_COUNT} steps"
                )
        ));
    }

    #[test]
    fn validate_sequence_rejects_sequence_with_wrong_frame_count_upper_bound()
    -> Result<(), FullRootArtifactError> {
        let sequence = run_scripted_sequence()?;
        let mut too_many = sequence;
        let extra = run_scripted_sequence()?.steps.pop();
        let extra = extra.ok_or(FullRootArtifactError::Contract(
            "scripted sequence has no frame".into(),
        ));
        let extra = extra?;
        too_many.steps.push(extra);
        assert!(matches!(
            validate_sequence(&too_many),
            Err(FullRootArtifactError::Contract(error))
                if error == format!(
                    "full-root trace must contain exactly {FULL_ROOT_FRAME_COUNT} steps"
                )
        ));
        Ok(())
    }
}
