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
use image::GenericImageView;
#[cfg(test)]
use image::ImageEncoder;
#[cfg(test)]
use sha2::{Digest, Sha256};
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::path::Path;

pub(super) fn run_scripted_sequence() -> Result<FullRootSequence, FullRootArtifactError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = build_root().map_err(FullRootArtifactError::Adapter)?;
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
        .map(|step| step.evidence.identity.as_str())
        .ok_or_else(|| FullRootArtifactError::Contract("full-root trace is empty".into()))?;
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
    let first = fs::read(frame_paths[0].as_ref())?;
    let first_image = image::load_from_memory_with_format(&first, image::ImageFormat::Png)?;
    let canonical_size = first_image.dimensions();
    for path in frame_paths {
        let bytes = fs::read(path.as_ref())?;
        let image =
            image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)?.to_rgba8();
        let mut canvas = image::RgbaImage::from_pixel(
            canonical_size.0,
            canonical_size.1,
            image::Rgba([0, 0, 0, 0]),
        );
        image::imageops::overlay(&mut canvas, &image, 0, 0);
        let mut canonical = Vec::new();
        image::codecs::png::PngEncoder::new(&mut canonical)
            .write_image(
                canvas.as_raw(),
                canvas.width(),
                canvas.height(),
                image::ColorType::Rgba8.into(),
            )
            .map_err(|error| FullRootArtifactError::Contract(error.to_string()))?;
        digest.update((canonical.len() as u64).to_le_bytes());
        digest.update(canonical);
    }
    Ok(format!("{:x}", digest.finalize()))
}
