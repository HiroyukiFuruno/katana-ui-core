#![cfg(feature = "native-ime-evidence")]

use katana_ui_core::native_ime_evidence::{
    AccessKitObservation, Bounds, HitTestObservation, NativeFrameObservations,
    NativeImeEvidenceArtifact, NativeImeTranscript, NativeImeVerificationExpectations,
    NativeObservationHashes, NativeRunBinding, RgbaCropObservation, SCHEMA_VERSION,
};
use std::{fs, path::PathBuf, process::Command};

fn hash(seed: u8) -> String {
    format!("{seed:02x}{:0>62}", "")
}

fn valid_artifact() -> NativeImeEvidenceArtifact {
    let crop = RgbaCropObservation {
        width: 1,
        height: 1,
        pixels: vec![[255, 0, 0, 255]],
        control_width: 1,
        control_height: 1,
        control_pixels: vec![[1, 1, 1, 255]],
        has_colored_pixels: false,
        rgba_sha256: String::new(),
        control_rgba_sha256: String::new(),
    }
    .seal()
    .unwrap();
    let frame = NativeFrameObservations {
        final_text: "日本語⭐️ ☆".to_owned(),
        committed_text: "日本語".to_owned(),
        committed_range_start: 0,
        committed_range_end: 9,
        measurement_width: 100,
        measurement_height: 24,
        caret: Bounds {
            x: 1,
            y: 1,
            width: 1,
            height: 20,
        },
        hit_tests: vec![
            HitTestObservation {
                target: "⭐️".to_owned(),
                target_bounds: Bounds {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1,
                },
                range_start: 9,
                range_end: 15,
                query_x: 0,
                query_y: 0,
            },
            HitTestObservation {
                target: "☆".to_owned(),
                target_bounds: Bounds {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1,
                },
                range_start: 16,
                range_end: 19,
                query_x: 0,
                query_y: 0,
            },
        ],
        accesskit: AccessKitObservation {
            role: "MultilineTextInput".to_owned(),
            value: "日本語⭐️ ☆".to_owned(),
            bounds: Bounds {
                x: 0,
                y: 0,
                width: 100,
                height: 24,
            },
            node_id: 1,
        },
        root_id: "root-1".to_owned(),
        frame_id: "frame-1".to_owned(),
    };
    let (measurement_sha256, caret_sha256, hit_test_sha256, accesskit_sha256) =
        frame.observation_hashes().unwrap();
    NativeImeEvidenceArtifact {
        schema_version: SCHEMA_VERSION.to_owned(),
        revision: "rev-cli".to_owned(),
        platform: "linux".to_owned(),
        run: NativeRunBinding {
            run_id: "run-cli".to_owned(),
            challenge: "challenge-cli".to_owned(),
            producer_id: "producer-cli".to_owned(),
            runner_id: "runner-cli".to_owned(),
            input_method: "Japanese IME".to_owned(),
            origin: "os-native".to_owned(),
        },
        ime: NativeImeTranscript {
            preedit: "にほんご".to_owned(),
            commit: "日本語".to_owned(),
            preedit_sequence: 1,
            commit_sequence: 2,
        },
        scalar_sequence: vec![0x2b50, 0xfe0f],
        rgba_crop: crop,
        observations: NativeObservationHashes {
            measurement_sha256,
            caret_sha256,
            hit_test_sha256,
            accesskit_sha256,
            frame_sha256: hash(7),
            root_sha256: hash(8),
            receipt_sha256: hash(9),
            font_file_sha256: hash(10),
            font_catalog_sha256: hash(11),
            root_id: "root-1".to_owned(),
            frame_id: "frame-1".to_owned(),
        },
        frame_observations: frame,
        artifact_sha256: String::new(),
    }
    .seal()
    .unwrap()
}

fn expectations(a: &NativeImeEvidenceArtifact) -> NativeImeVerificationExpectations {
    NativeImeVerificationExpectations {
        revision: a.revision.clone(),
        platform: a.platform.clone(),
        run_id: a.run.run_id.clone(),
        challenge: a.run.challenge.clone(),
        producer_id: a.run.producer_id.clone(),
        runner_id: a.run.runner_id.clone(),
        attested_artifact_sha256: a.artifact_sha256.clone(),
    }
}

fn paths(label: &str) -> (PathBuf, PathBuf) {
    let root =
        std::env::temp_dir().join(format!("kuc-native-ime-cli-{label}-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    (root.join("artifact.json"), root.join("expectations.json"))
}

#[test]
fn cli_verifies_artifact_against_independent_expectations() {
    let (artifact_path, expectations_path) = paths("success");
    let artifact = valid_artifact();
    fs::write(
        &artifact_path,
        serde_json::to_vec_pretty(&artifact).unwrap(),
    )
    .unwrap();
    fs::write(
        &expectations_path,
        serde_json::to_vec_pretty(&expectations(&artifact)).unwrap(),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_kuc-native-ime-verify"))
        .args([artifact_path.as_os_str(), expectations_path.as_os_str()])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("verified"));
    let _ = fs::remove_dir_all(artifact_path.parent().unwrap());
}

#[test]
fn cli_rejects_missing_arguments_and_shared_trusted_input() {
    let missing = Command::new(env!("CARGO_BIN_EXE_kuc-native-ime-verify"))
        .output()
        .unwrap();
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("usage:"));

    let (artifact_path, _) = paths("same");
    fs::write(&artifact_path, b"{}").unwrap();
    let same = Command::new(env!("CARGO_BIN_EXE_kuc-native-ime-verify"))
        .args([artifact_path.as_os_str(), artifact_path.as_os_str()])
        .output()
        .unwrap();
    assert!(!same.status.success());
    assert!(String::from_utf8_lossy(&same.stderr).contains("independently"));
    let _ = fs::remove_dir_all(artifact_path.parent().unwrap());
}

#[test]
fn cli_reports_verification_and_json_decode_failures() {
    let (artifact_path, expectations_path) = paths("verification-failure");
    let mut artifact = valid_artifact();
    artifact.run.origin = "synthetic".to_owned();
    artifact = artifact.seal().unwrap();
    fs::write(&artifact_path, serde_json::to_vec(&artifact).unwrap()).unwrap();
    fs::write(
        &expectations_path,
        serde_json::to_vec(&expectations(&artifact)).unwrap(),
    )
    .unwrap();

    let verification = Command::new(env!("CARGO_BIN_EXE_kuc-native-ime-verify"))
        .args([artifact_path.as_os_str(), expectations_path.as_os_str()])
        .output()
        .unwrap();
    assert!(!verification.status.success());
    let stderr = String::from_utf8_lossy(&verification.stderr);
    assert!(stderr.contains("InvalidOrigin"), "{stderr}");
    let _ = fs::remove_dir_all(artifact_path.parent().unwrap());

    let (artifact_path, expectations_path) = paths("decode-failure");
    fs::write(&artifact_path, b"{").unwrap();
    fs::write(&expectations_path, b"{}").unwrap();
    let decode = Command::new(env!("CARGO_BIN_EXE_kuc-native-ime-verify"))
        .args([artifact_path.as_os_str(), expectations_path.as_os_str()])
        .output()
        .unwrap();
    assert!(!decode.status.success());
    let stderr = String::from_utf8_lossy(&decode.stderr);
    assert!(
        stderr.contains("EOF") || stderr.contains("expected"),
        "{stderr}"
    );
    let _ = fs::remove_dir_all(artifact_path.parent().unwrap());
}
