use super::FULL_ROOT_MANIFEST_FILE_NAME;
use super::model::FullRootArtifactError;
use super::process::{VIDEO_ENCODER, VIDEO_MUXER, VIDEO_PIXEL_FORMAT};
use super::sequence::{frame_sequence_sha256, run_scripted_sequence, validate_sequence};
use super::write_artifact;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn full_root_trace_repeats_closed_evidence() -> Result<(), FullRootArtifactError> {
    let first = run_scripted_sequence()?;
    let second = run_scripted_sequence()?;
    validate_sequence(&first)?;
    validate_sequence(&second)?;
    assert_eq!(
        first
            .steps
            .iter()
            .map(|step| &step.evidence)
            .collect::<Vec<_>>(),
        second
            .steps
            .iter()
            .map(|step| &step.evidence)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        first
            .steps
            .iter()
            .map(|step| step.frame.record().rgba_hash())
            .collect::<Vec<_>>(),
        second
            .steps
            .iter()
            .map(|step| step.frame.record().rgba_hash())
            .collect::<Vec<_>>()
    );
    Ok(())
}

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn full_root_artifact_generates_decodes_and_records_mp4() -> Result<(), FullRootArtifactError> {
    let output = PathBuf::from("target/text-command-root-storybook-test-artifact");
    write_artifact(&output)?;
    let output = fs::canonicalize(output)?;
    let manifest_path = output.join(FULL_ROOT_MANIFEST_FILE_NAME);
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(manifest_path)?)?;
    let sequence = run_scripted_sequence()?;
    let mp4_path = PathBuf::from(
        manifest["mp4"]["path"]
            .as_str()
            .ok_or_else(|| FullRootArtifactError::Contract("manifest mp4 path missing".into()))?,
    );
    assert!(mp4_path.is_absolute());
    assert!(mp4_path.is_file());
    assert_eq!(
        sha(&fs::read(&mp4_path)?),
        manifest["mp4"]["sha256"]
            .as_str()
            .ok_or_else(|| FullRootArtifactError::Contract("manifest mp4 SHA missing".into()))?
    );
    let frame_paths = sequence
        .steps
        .iter()
        .enumerate()
        .map(|(index, _)| output.join(format!("frame-{index:03}.png")))
        .collect::<Vec<_>>();
    assert_eq!(
        frame_sequence_sha256(&frame_paths)?,
        manifest["mp4"]["frame_sequence_sha256"]
            .as_str()
            .ok_or_else(|| {
                FullRootArtifactError::Contract("manifest frame sequence SHA missing".into())
            })?
    );
    assert_eq!(
        sequence.steps.len(),
        manifest["mp4"]["frame_count"]
            .as_u64()
            .ok_or_else(|| FullRootArtifactError::Contract("manifest frame count missing".into()))?
            as usize
    );
    assert_eq!(manifest["mp4"]["fps"]["numerator"], 1_000);
    assert_eq!(manifest["mp4"]["fps"]["denominator"], 180);
    assert_eq!(manifest["mp4"]["container"], VIDEO_MUXER);
    assert_eq!(manifest["mp4"]["codec"], VIDEO_ENCODER);
    assert_eq!(manifest["mp4"]["pixel_format"], VIDEO_PIXEL_FORMAT);
    assert_eq!(manifest["mp4"]["encoder_capability_verified"], true);
    assert_eq!(manifest["mp4"]["muxer_capability_verified"], true);
    assert_eq!(manifest["mp4"]["decoder"]["verified"], true);
    assert_eq!(
        manifest["mp4"]["decoder"]["decoded_frame_count"],
        sequence.steps.len()
    );
    assert!(
        manifest["mp4"]["ffmpeg_path"]
            .as_str()
            .is_some_and(|path| Path::new(path).is_absolute())
    );
    assert!(
        manifest["mp4"]["ffmpeg_version"]
            .as_str()
            .is_some_and(|version| version.starts_with("ffmpeg version "))
    );
    assert!(
        manifest["gif_path"]
            .as_str()
            .is_some_and(|path| Path::new(path).is_absolute())
    );
    assert!(
        manifest["gif_sha256"]
            .as_str()
            .is_some_and(|hash| !hash.is_empty())
    );
    assert!(output.join("motion.gif").is_file());
    for index in 0..sequence.steps.len() {
        assert!(
            output
                .join(format!("frame-{index:03}.manifest.json"))
                .is_file()
        );
    }
    Ok(())
}
