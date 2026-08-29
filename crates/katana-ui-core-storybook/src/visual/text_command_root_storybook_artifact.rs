use super::FULL_ROOT_MANIFEST_FILE_NAME;
use super::model::{FullRootArtifactError, FullRootManifest};
use super::process;
use super::sequence::{run_scripted_sequence, validate_sequence};
use katana_ui_core_egui_adapter::FullRootArtifactWriter;
use std::fs;
use std::path::Path;

pub(super) fn write_artifact(output_dir: &Path) -> Result<(), FullRootArtifactError> {
    let sequence = run_scripted_sequence()?;
    validate_sequence(&sequence)?;
    fs::create_dir_all(output_dir)?;
    let output_dir = fs::canonicalize(output_dir)?;
    let writer = FullRootArtifactWriter::new();
    let mut receipts = Vec::with_capacity(sequence.steps.len());
    for (index, step) in sequence.steps.iter().enumerate() {
        let artifact = writer
            .write(&step.frame, &output_dir, &format!("frame-{index:03}"))
            .map_err(|error| FullRootArtifactError::Adapter(error.to_string()))?;
        receipts.push(artifact);
    }
    let video = process::write_mp4(&receipts, &output_dir)?;
    let manifest_path = output_dir.join(FULL_ROOT_MANIFEST_FILE_NAME);
    let manifest = FullRootManifest::from_sequence(&sequence, &receipts, &video);
    fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;
    Ok(())
}
