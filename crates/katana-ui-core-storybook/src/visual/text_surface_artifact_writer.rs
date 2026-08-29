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
    if name.len() < FRAME_NAME_PREFIX_LENGTH {
        return false;
    }
    let prefix = &name.as_bytes()[..FRAME_NAME_PREFIX_LENGTH];
    prefix[FRAME_TENS_INDEX].is_ascii_digit()
        && prefix[FRAME_ONES_INDEX].is_ascii_digit()
        && prefix[FRAME_SEPARATOR_INDEX] == b'-'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::text_surface_script_types::TextSurfaceArtifactError;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    #[cfg(target_os = "linux")]
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    #[test]
    fn frame_prefix_predicate_matches_only_expected_names() {
        assert!(has_frame_prefix("08-anything.png"));
        assert!(has_frame_prefix("00-any.png"));
        assert!(!has_frame_prefix("a1-any.png"));
        assert!(!has_frame_prefix("0x-any"));
        assert!(!has_frame_prefix("abc"));
        assert!(!has_frame_prefix("x"));
    }

    #[test]
    fn clear_previous_artifact_files_removes_managed_outputs_only()
    -> Result<(), TextSurfaceArtifactError> {
        let output_dir = temp_dir("text-surface-artifact-writer")?;
        let files = [
            ARTIFACT_GIF_FILE,
            ARTIFACT_MANIFEST_FILE,
            "00-frame.png",
            "01-frame.png",
            "note.txt",
            "00-keep.txt",
        ];
        for file in files {
            fs::write(output_dir.join(file), b"data").map_err(TextSurfaceArtifactError::Io)?;
        }

        clear_previous_artifact_files(&output_dir)?;

        assert!(!output_dir.join(ARTIFACT_GIF_FILE).exists());
        assert!(!output_dir.join(ARTIFACT_MANIFEST_FILE).exists());
        assert!(!output_dir.join("00-frame.png").exists());
        assert!(!output_dir.join("01-frame.png").exists());
        assert!(output_dir.join("note.txt").exists());
        assert!(output_dir.join("00-keep.txt").exists());
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn clear_previous_artifact_files_ignores_non_utf8_names() -> Result<(), TextSurfaceArtifactError>
    {
        let output_dir = temp_dir("text-surface-artifact-writer-non-utf8")?;
        let path = output_dir.join(OsString::from_vec(vec![0xff]));
        fs::write(&path, b"data").map_err(TextSurfaceArtifactError::Io)?;
        clear_previous_artifact_files(&output_dir)?;
        assert!(path.exists());
        Ok(())
    }

    fn temp_dir(prefix: &str) -> Result<PathBuf, TextSurfaceArtifactError> {
        let mut path = env::temp_dir();
        path.push("katana-storybook-text-surface-artifact-writer");
        path.push(format!(
            "{prefix}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        fs::create_dir_all(&path).map_err(TextSurfaceArtifactError::Io)?;
        Ok(path)
    }
}
