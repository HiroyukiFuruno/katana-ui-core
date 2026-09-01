use crate::egui::FullRootArtifact;
use crate::egui::motion_artifact_writer::{
    MotionArtifactError, MotionArtifactSettings, MotionArtifactWriter,
};
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const RED_AND_TRANSPARENT_PIXELS: [u8; 8] = [u8::MAX, 0, 0, u8::MAX, 0, 0, 0, u8::MAX];

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn receipt(
    root: &std::path::Path,
    stage: &str,
    width: u32,
    height: u32,
    rgba: &[u8],
) -> FullRootArtifact {
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
        .write_image(rgba, width, height, ColorType::Rgba8.into())
        .expect("test PNG should encode");
    let png_path = root.join(format!("{stage}.png"));
    let manifest_path = root.join(format!("{stage}.manifest.json"));
    std::fs::write(&png_path, &png).expect("test PNG should write");
    let root_record_hash = format!("record-{stage}");
    let provenance = serde_json::json!({
        "width": width,
        "height": height,
        "root_record_hash": root_record_hash,
        "pixel_hash": sha256(rgba),
        "png_sha256": sha256(&png),
    });
    std::fs::write(
        &manifest_path,
        serde_json::to_vec(&provenance).expect("test provenance should encode"),
    )
    .expect("test provenance should write");
    FullRootArtifact::from_test_parts(
        stage.to_owned(),
        png_path,
        manifest_path,
        width,
        height,
        format!("record-{stage}"),
        sha256(rgba),
        sha256(&png),
    )
}

fn one_receipt(root: &std::path::Path) -> FullRootArtifact {
    receipt(root, "frame-000", 2, 1, &RED_AND_TRANSPARENT_PIXELS)
}

fn writer_error(receipts: &[FullRootArtifact], root: &std::path::Path) -> MotionArtifactError {
    MotionArtifactWriter::new()
        .write(
            receipts,
            root,
            MotionArtifactSettings::new(receipts.len(), 2, 1),
        )
        .expect_err("invalid receipt must be rejected")
}

#[test]
fn rejects_empty_sequence() {
    let root = tempfile_dir("empty");
    let error = writer_error(&[], &root);
    assert_eq!(error, MotionArtifactError::EmptySequence);
}

#[test]
fn rejects_duplicate_stage() {
    let root = tempfile_dir("duplicate");
    let first = one_receipt(&root);
    let second_root = root.join("second");
    std::fs::create_dir_all(&second_root).expect("second test directory should create");
    let second = receipt(
        &second_root,
        "frame-000",
        2,
        1,
        &[0, 255, 0, 255, 0, 0, 0, 255],
    );
    let error = writer_error(&[first, second], &root);
    assert!(
        matches!(error, MotionArtifactError::DuplicateStage(_)),
        "{error:?}"
    );
}

#[test]
fn rejects_stale_stage() {
    let root = tempfile_dir("stale");
    let stale = receipt(&root, "frame-001", 2, 1, &[255, 0, 0, 255, 0, 0, 0, 255]);
    assert!(matches!(
        writer_error(&[stale], &root),
        MotionArtifactError::StaleStage { .. }
    ));
}

#[test]
fn rejects_missing_png_and_provenance() {
    let root = tempfile_dir("missing");
    let missing_png = one_receipt(&root);
    std::fs::remove_file(missing_png.png_path()).expect("test PNG should remove");
    assert!(matches!(
        writer_error(&[missing_png], &root),
        MotionArtifactError::MissingPng(_)
    ));
    let missing_provenance = one_receipt(&root);
    std::fs::remove_file(missing_provenance.manifest_path())
        .expect("test provenance should remove");
    assert!(matches!(
        writer_error(&[missing_provenance], &root),
        MotionArtifactError::MissingProvenance(_)
    ));
}

#[test]
fn rejects_bad_sha_wrong_dimension_and_empty_pixels() {
    let root = tempfile_dir("content");
    let mut bad_sha = one_receipt(&root);
    bad_sha = FullRootArtifact::from_test_parts(
        bad_sha.stage_id().to_owned(),
        bad_sha.png_path().to_owned(),
        bad_sha.manifest_path().to_owned(),
        2,
        1,
        bad_sha.root_record_hash().to_owned(),
        bad_sha.pixel_hash().to_owned(),
        "bad".into(),
    );
    assert!(matches!(
        writer_error(&[bad_sha], &root),
        MotionArtifactError::BadPngSha { .. }
    ));
    let wrong = receipt(&root, "frame-000", 1, 2, &[255, 0, 0, 255, 0, 0, 0, 255]);
    assert!(matches!(
        writer_error(&[wrong], &root),
        MotionArtifactError::WrongDimensions { .. }
    ));
    let empty = receipt(&root, "frame-000", 2, 1, &[0; 8]);
    assert!(matches!(
        writer_error(&[empty], &root),
        MotionArtifactError::EmptyPixels(_)
    ));
}

#[cfg(unix)]
#[test]
fn rejects_missing_encoder_and_decode_failure() {
    let missing =
        super::ffmpeg::Ffmpeg::discover_at(std::path::Path::new("/definitely-missing/ffmpeg"))
            .expect_err("missing ffmpeg must fail");
    assert!(matches!(missing, MotionArtifactError::Encoder(_)));

    let empty_version_root = tempfile_dir("empty-version");
    let empty_version_script = empty_version_root.join("ffmpeg");
    std::fs::write(&empty_version_script, "#!/bin/sh\nexit 0\n")
        .expect("empty-version ffmpeg should write");
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(
        &empty_version_script,
        std::fs::Permissions::from_mode(0o755),
    )
    .expect("empty-version ffmpeg should be executable");
    let empty_version = super::ffmpeg::Ffmpeg::discover_at(&empty_version_script)
        .expect_err("empty ffmpeg version must fail");
    assert!(matches!(empty_version, MotionArtifactError::Encoder(_)));

    let root = tempfile_dir("decode");
    let script = root.join("ffmpeg");
    std::fs::write(&script, "#!/bin/sh\nexit 17\n").expect("fake ffmpeg should write");
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755))
        .expect("fake ffmpeg should be executable");
    let decode = (super::ffmpeg::Ffmpeg {
        path: script,
        version: "test".into(),
    })
    .decode(&root.join("missing.mp4"));
    assert!(matches!(decode, Err(MotionArtifactError::Encoder(_))));
}

fn tempfile_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("kuc-motion-{label}-{}", std::process::id()));
    std::fs::create_dir_all(&path).expect("test directory should create");
    path
}
