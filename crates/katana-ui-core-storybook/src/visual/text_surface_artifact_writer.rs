use super::text_surface_artifact::{TextSurfacePlanPixels, write_gif, write_png};
use super::text_surface_script::{assert_sequence_contract, run_scripted_sequence};
use super::text_surface_script_types::{
    StorybookTextSurfaceManifest, TextSurfaceArtifactError, frame_png_name,
};
use std::fs;
use std::path::Path;

pub(super) const ARTIFACT_GIF_FILE: &str = "text-surface-motion.gif";
pub(super) const ARTIFACT_MANIFEST_FILE: &str = "text-surface-manifest.json";
const FRAME_NAME_PREFIX_LENGTH: usize = 3;
const FRAME_TENS_INDEX: usize = 0;
const FRAME_ONES_INDEX: usize = 1;
const FRAME_SEPARATOR_INDEX: usize = 2;

pub(super) fn write_scripted_artifact(output_dir: &Path) -> Result<(), TextSurfaceArtifactError> {
    let sequence = run_scripted_sequence()?;
    assert_sequence_contract(&sequence)?;
    fs::create_dir_all(output_dir).map_err(TextSurfaceArtifactError::Io)?;
    clear_previous_artifact_files(output_dir)?;
    for step in &sequence.steps {
        write_png(
            &step.pixels,
            &output_dir.join(frame_png_name(step.index, step.name)),
        )
        .map_err(TextSurfaceArtifactError::Image)?;
    }
    write_gif(
        &sequence
            .steps
            .iter()
            .map(|step| step.pixels.clone())
            .collect::<Vec<TextSurfacePlanPixels>>(),
        &output_dir.join(ARTIFACT_GIF_FILE),
    )
    .map_err(TextSurfaceArtifactError::Image)?;
    let manifest = StorybookTextSurfaceManifest::from_sequence(&sequence);
    let json = serde_json::to_vec_pretty(&manifest).map_err(TextSurfaceArtifactError::Json)?;
    fs::write(output_dir.join(ARTIFACT_MANIFEST_FILE), json).map_err(TextSurfaceArtifactError::Io)
}

fn clear_previous_artifact_files(output_dir: &Path) -> Result<(), TextSurfaceArtifactError> {
    for entry in fs::read_dir(output_dir).map_err(TextSurfaceArtifactError::Io)? {
        let entry = entry.map_err(TextSurfaceArtifactError::Io)?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if is_managed_artifact_file(&path, name) {
            fs::remove_file(path).map_err(TextSurfaceArtifactError::Io)?;
        }
    }
    Ok(())
}

fn is_managed_artifact_file(path: &Path, name: &str) -> bool {
    name == ARTIFACT_GIF_FILE
        || name == ARTIFACT_MANIFEST_FILE
        || (has_frame_prefix(name) && path.extension().is_some_and(|extension| extension == "png"))
}

fn has_frame_prefix(name: &str) -> bool {
    let Some(prefix) = name.as_bytes().get(..FRAME_NAME_PREFIX_LENGTH) else {
        return false;
    };
    prefix[FRAME_TENS_INDEX].is_ascii_digit()
        && prefix[FRAME_ONES_INDEX].is_ascii_digit()
        && prefix[FRAME_SEPARATOR_INDEX] == b'-'
}
