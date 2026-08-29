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
        let artifact = match writer.write(&step.frame, &output_dir, &format!("frame-{index:03}")) {
            Ok(artifact) => artifact,
            Err(error) => return Err(FullRootArtifactError::Adapter(error.to_string())),
        };
        receipts.push(artifact);
    }
    let video = process::write_mp4(&receipts, &output_dir)?;
    let manifest_path = output_dir.join(FULL_ROOT_MANIFEST_FILE_NAME);
    let manifest = FullRootManifest::from_sequence(&sequence, &receipts, &video);
    fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn write_artifact_rejects_file_path_output_root() -> Result<(), FullRootArtifactError> {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "kuc-text-command-root-artifact-file-{:.0}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as f64
        ));
        let mut marker = PathBuf::from(&path);
        std::fs::create_dir_all(&path).map_err(FullRootArtifactError::Io)?;
        marker.push("output-marker");
        std::fs::File::create(&marker)
            .map_err(FullRootArtifactError::Io)?
            .write_all(b"marker")
            .map_err(FullRootArtifactError::Io)?;

        let result = write_artifact(&marker);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn write_artifact_reports_adapter_output_failure() -> Result<(), FullRootArtifactError> {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "kuc-text-command-root-artifact-blocked-{:.0}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as f64
        ));
        std::fs::create_dir_all(path.join("frame-000.png")).map_err(FullRootArtifactError::Io)?;
        assert!(matches!(
            write_artifact(&path),
            Err(FullRootArtifactError::Adapter(_))
        ));
        Ok(())
    }
}
