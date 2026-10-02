#![cfg(feature = "native-ime-evidence")]

type EvidenceMutation = Box<dyn Fn(&mut NativeImeEvidenceArtifact)>;
type ExpectationsMutation = Box<dyn Fn(&mut NativeImeVerificationExpectations)>;

use katana_ui_core::native_ime_evidence::{
    AccessKitObservation, Bounds, HitTestObservation, NativeFrameObservations,
    NativeImeEvidenceArtifact, NativeImeTranscript, NativeImeVerificationError,
    NativeImeVerificationExpectations, NativeObservationHashes, NativeRunBinding,
    RgbaCropObservation, SCHEMA_VERSION,
};

fn hash(seed: u8) -> String {
    format!("{seed:02x}{:0>62}", "")
}

fn artifact() -> NativeImeEvidenceArtifact {
    let rgba_crop = RgbaCropObservation {
        width: 2,
        height: 2,
        pixels: vec![
            [255, 0, 0, 255],
            [0, 255, 0, 255],
            [0, 0, 255, 255],
            [1, 1, 1, 255],
        ],
        control_width: 2,
        control_height: 2,
        control_pixels: vec![[1, 1, 1, 255]; 4],
        has_colored_pixels: false,
        rgba_sha256: String::new(),
        control_rgba_sha256: String::new(),
    }
    .seal()
    .unwrap();
    let frame_observations = NativeFrameObservations {
        final_text: "日本語⭐️ ☆".to_owned(),
        committed_text: "日本語".to_owned(),
        committed_range_start: 0,
        committed_range_end: 9,
        measurement_width: 100,
        measurement_height: 24,
        caret: Bounds {
            x: 10,
            y: 2,
            width: 1,
            height: 20,
        },
        hit_tests: vec![
            HitTestObservation {
                target: "⭐️".to_owned(),
                target_bounds: Bounds {
                    x: 0,
                    y: 0,
                    width: 2,
                    height: 2,
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
                    width: 2,
                    height: 2,
                },
                range_start: 16,
                range_end: 19,
                query_x: 1,
                query_y: 1,
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
        frame_observations.observation_hashes().unwrap();
    NativeImeEvidenceArtifact {
        schema_version: SCHEMA_VERSION.to_owned(),
        revision: "rev-81".to_owned(),
        platform: "linux".to_owned(),
        run: NativeRunBinding {
            run_id: "run-1".to_owned(),
            challenge: "challenge-1".to_owned(),
            producer_id: "kuc-producer".to_owned(),
            runner_id: "runner-1".to_owned(),
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
        rgba_crop,
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
        frame_observations,
        artifact_sha256: String::new(),
    }
    .seal()
    .unwrap()
}

fn expectations(artifact: &NativeImeEvidenceArtifact) -> NativeImeVerificationExpectations {
    NativeImeVerificationExpectations {
        revision: artifact.revision.clone(),
        platform: artifact.platform.clone(),
        run_id: artifact.run.run_id.clone(),
        challenge: artifact.run.challenge.clone(),
        producer_id: artifact.run.producer_id.clone(),
        runner_id: artifact.run.runner_id.clone(),
        attested_artifact_sha256: artifact.artifact_sha256.clone(),
    }
}

fn reseal(mut evidence: NativeImeEvidenceArtifact) -> NativeImeEvidenceArtifact {
    evidence.rgba_crop = evidence.rgba_crop.seal().unwrap();
    let (measurement_sha256, caret_sha256, hit_test_sha256, accesskit_sha256) =
        evidence.frame_observations.observation_hashes().unwrap();
    evidence.observations.measurement_sha256 = measurement_sha256;
    evidence.observations.caret_sha256 = caret_sha256;
    evidence.observations.hit_test_sha256 = hit_test_sha256;
    evidence.observations.accesskit_sha256 = accesskit_sha256;
    evidence.seal().unwrap()
}

#[test]
fn verifies_one_fresh_native_run() {
    let evidence = artifact();
    assert_eq!(evidence.verify(&expectations(&evidence)), Ok(()));
}

#[test]
fn requires_commit_range_to_cover_the_committed_text() {
    let cases: Vec<EvidenceMutation> = vec![
        Box::new(|e| {
            e.frame_observations.committed_range_start = 9;
            e.frame_observations.committed_range_end = 15;
        }),
        Box::new(|e| {
            e.frame_observations.committed_range_start = 1;
            e.frame_observations.committed_range_end = 9;
        }),
        Box::new(|e| e.frame_observations.committed_range_end = 8),
    ];
    for mutate in cases {
        let mut evidence = artifact();
        mutate(&mut evidence);
        let evidence = reseal(evidence);
        assert_eq!(
            evidence.verify(&expectations(&evidence)),
            Err(NativeImeVerificationError::InvalidTranscript)
        );
    }
}

#[test]
fn requires_hit_queries_and_target_bounds_inside_measurement() {
    let cases: Vec<EvidenceMutation> = vec![
        Box::new(|e| e.frame_observations.hit_tests[0].query_x = 100),
        Box::new(|e| e.frame_observations.hit_tests[0].target_bounds.x = 95),
        Box::new(|e| {
            e.frame_observations.hit_tests[0].target_bounds.x = u32::MAX;
            e.frame_observations.hit_tests[0].target_bounds.width = 1;
        }),
        Box::new(|e| e.frame_observations.hit_tests[0].target = "unknown".to_owned()),
    ];
    for mutate in cases {
        let mut evidence = artifact();
        mutate(&mut evidence);
        let evidence = reseal(evidence);
        assert_eq!(
            evidence.verify(&expectations(&evidence)),
            Err(NativeImeVerificationError::InvalidObservation("hit tests"))
        );
    }
}

#[test]
fn requires_each_target_bounds_size_to_match_its_crop() {
    let mut main_crop = artifact();
    main_crop.rgba_crop.width = 1;
    main_crop.rgba_crop.height = 4;
    let main_crop = reseal(main_crop);
    assert_eq!(
        main_crop.verify(&expectations(&main_crop)),
        Err(NativeImeVerificationError::InvalidObservation("hit tests"))
    );

    let mut control_crop = artifact();
    control_crop.rgba_crop.control_width = 4;
    control_crop.rgba_crop.control_height = 1;
    let control_crop = reseal(control_crop);
    assert_eq!(
        control_crop.verify(&expectations(&control_crop)),
        Err(NativeImeVerificationError::InvalidObservation("hit tests"))
    );
}

#[test]
fn repeated_target_with_different_bounds_cannot_bypass_crop_contract() {
    let mut evidence = artifact();
    let mut repeated = evidence.frame_observations.hit_tests[0].clone();
    repeated.target_bounds.width = 3;
    evidence.frame_observations.hit_tests.push(repeated);
    let evidence = reseal(evidence);
    assert_eq!(
        evidence.verify(&expectations(&evidence)),
        Err(NativeImeVerificationError::InvalidObservation("hit tests"))
    );
}

#[test]
fn rejects_self_attested_or_synthetic_origin() {
    let mut evidence = artifact();
    evidence.run.origin = "synthetic".to_owned();
    evidence = evidence.seal().unwrap();
    assert_eq!(
        evidence.verify(&expectations(&evidence)),
        Err(NativeImeVerificationError::InvalidOrigin)
    );
}

#[test]
fn rejects_external_digest_mismatch_even_when_payload_is_valid() {
    let evidence = artifact();
    let mut expected = expectations(&evidence);
    expected.attested_artifact_sha256 = hash(99);
    assert_eq!(
        evidence.verify(&expected),
        Err(NativeImeVerificationError::DigestMismatch)
    );
}

#[test]
fn rejects_replay_from_another_run() {
    let evidence = artifact();
    let mut expected = expectations(&evidence);
    expected.run_id = "run-2".to_owned();
    assert_eq!(
        evidence.verify(&expected),
        Err(NativeImeVerificationError::BindingMismatch("run_id"))
    );
}

#[test]
fn rejects_invalid_scalar_and_same_crop_control() {
    let mut evidence = artifact();
    evidence.scalar_sequence = vec![0x2b50];
    evidence = evidence.seal().unwrap();
    assert_eq!(
        evidence.verify(&expectations(&evidence)),
        Err(NativeImeVerificationError::InvalidScalars)
    );

    let mut evidence = artifact();
    evidence.rgba_crop.control_rgba_sha256 = evidence.rgba_crop.rgba_sha256.clone();
    evidence = evidence.seal().unwrap();
    assert_eq!(
        evidence.verify(&expectations(&evidence)),
        Err(NativeImeVerificationError::InvalidCrop)
    );
}

#[test]
fn rejects_schema_platform_and_transcript_variants() {
    let cases: Vec<(&str, EvidenceMutation)> = vec![
        ("schema", Box::new(|e| e.schema_version = "old".to_owned())),
        ("platform", Box::new(|e| e.platform = "plan9".to_owned())),
        ("order", Box::new(|e| e.ime.preedit_sequence = 3)),
        ("latin", Box::new(|e| e.ime.commit = "english".to_owned())),
        (
            "commit mismatch",
            Box::new(|e| e.frame_observations.committed_text = "別".to_owned()),
        ),
    ];
    for (name, mutate) in cases {
        let mut evidence = artifact();
        mutate(&mut evidence);
        evidence = evidence.seal().unwrap();
        assert!(evidence.verify(&expectations(&evidence)).is_err(), "{name}");
    }
}

#[test]
fn rejects_observation_and_pixel_variants() {
    let cases: Vec<(&str, EvidenceMutation)> = vec![
        (
            "transparent chromatic",
            Box::new(|e| e.rgba_crop.pixels[0] = [255, 0, 0, 0]),
        ),
        (
            "overflow dimensions",
            Box::new(|e| e.rgba_crop.width = u32::MAX),
        ),
        (
            "crop hash",
            Box::new(|e| e.rgba_crop.rgba_sha256 = "x".repeat(64)),
        ),
        (
            "measurement hash",
            Box::new(|e| e.observations.measurement_sha256 = "f".repeat(64)),
        ),
        (
            "bad hit byte",
            Box::new(|e| e.frame_observations.hit_tests[0].range_start = 10),
        ),
        (
            "accesskit role",
            Box::new(|e| e.frame_observations.accesskit.role = "Button".to_owned()),
        ),
    ];
    for (name, mutate) in cases {
        let mut evidence = artifact();
        mutate(&mut evidence);
        evidence = evidence.seal().unwrap();
        assert!(evidence.verify(&expectations(&evidence)).is_err(), "{name}");
    }
}

#[test]
fn rejects_crop_dimensions_and_pixel_count_mismatches_after_sealing() {
    let cases: Vec<(&str, EvidenceMutation)> = vec![
        ("width pixel count", Box::new(|e| e.rgba_crop.width = 3)),
        ("zero height", Box::new(|e| e.rgba_crop.height = 0)),
        (
            "control width pixel count",
            Box::new(|e| e.rgba_crop.control_width = 3),
        ),
        (
            "zero control height",
            Box::new(|e| e.rgba_crop.control_height = 0),
        ),
        (
            "pixel count",
            Box::new(|e| {
                e.rgba_crop.pixels.pop();
            }),
        ),
        (
            "control pixel count",
            Box::new(|e| {
                e.rgba_crop.control_pixels.pop();
            }),
        ),
    ];
    for (name, mutate) in cases {
        let mut evidence = artifact();
        mutate(&mut evidence);
        evidence = evidence.seal().unwrap();
        assert_eq!(
            evidence.verify(&expectations(&evidence)),
            Err(NativeImeVerificationError::InvalidCrop),
            "{name}"
        );
    }
}

#[test]
fn rejects_empty_external_expectations_and_ids() {
    let evidence = artifact();
    let mut expected = expectations(&evidence);
    expected.revision.clear();
    assert_eq!(
        evidence.verify(&expected),
        Err(NativeImeVerificationError::BindingMismatch("revision"))
    );
    let mut evidence = artifact();
    evidence.frame_observations.frame_id.clear();
    evidence = evidence.seal().unwrap();
    assert_eq!(
        evidence.verify(&expectations(&evidence)),
        Err(NativeImeVerificationError::InvalidObservation(
            "frame observations"
        ))
    );
}

#[test]
fn rejects_missing_binding_fields_and_invalid_artifact_digest() {
    let fields: Vec<(&str, EvidenceMutation)> = vec![
        ("revision", Box::new(|e| e.revision.clear())),
        ("run_id", Box::new(|e| e.run.run_id.clear())),
        ("challenge", Box::new(|e| e.run.challenge.clear())),
        ("producer_id", Box::new(|e| e.run.producer_id.clear())),
        ("runner_id", Box::new(|e| e.run.runner_id.clear())),
        ("input_method", Box::new(|e| e.run.input_method.clear())),
        ("artifact_sha256", Box::new(|e| e.artifact_sha256.clear())),
    ];
    for (name, mutate) in fields {
        let mut evidence = artifact();
        mutate(&mut evidence);
        let expected = expectations(&evidence);
        assert_eq!(
            evidence.verify(&expected),
            Err(NativeImeVerificationError::MissingField(name)),
            "{name}"
        );
    }

    let mut evidence = artifact();
    evidence.platform.clear();
    assert_eq!(
        evidence.verify(&expectations(&evidence)),
        Err(NativeImeVerificationError::UnsupportedPlatform)
    );

    let mut evidence = artifact();
    evidence.artifact_sha256 = "z".repeat(64);
    let mut expected = expectations(&evidence);
    expected.attested_artifact_sha256 = evidence.artifact_sha256.clone();
    assert_eq!(
        evidence.verify(&expected),
        Err(NativeImeVerificationError::InvalidHash("artifact_sha256"))
    );

    let mut evidence = artifact();
    evidence.artifact_sha256 = "0".repeat(64);
    let expected = expectations(&evidence);
    assert_eq!(
        evidence.verify(&expected),
        Err(NativeImeVerificationError::InvalidHash("artifact_sha256"))
    );
}

#[test]
fn rejects_each_external_binding_and_observation_identity_mismatch() {
    let bindings: Vec<(&str, ExpectationsMutation)> = vec![
        ("platform", Box::new(|e| e.platform = "macos".to_owned())),
        ("challenge", Box::new(|e| e.challenge = "other".to_owned())),
        (
            "producer_id",
            Box::new(|e| e.producer_id = "other".to_owned()),
        ),
        ("runner_id", Box::new(|e| e.runner_id = "other".to_owned())),
    ];
    for (name, mutate) in bindings {
        let evidence = artifact();
        let mut expected = expectations(&evidence);
        mutate(&mut expected);
        assert_eq!(
            evidence.verify(&expected),
            Err(NativeImeVerificationError::BindingMismatch(name)),
            "{name}"
        );
    }

    let mut evidence = artifact();
    evidence.observations.root_id = "other-root".to_owned();
    let expected = expectations(&evidence);
    assert_eq!(
        evidence.verify(&expected),
        Err(NativeImeVerificationError::InvalidObservation(
            "frame observations"
        ))
    );
}

#[test]
fn rejects_hit_target_ranges_that_do_not_cover_the_declared_scalar() {
    let cases: Vec<EvidenceMutation> = vec![
        Box::new(|e| e.frame_observations.hit_tests[0].range_end = 14),
        Box::new(|e| e.frame_observations.hit_tests[0].target = "☆".to_owned()),
        Box::new(|e| e.frame_observations.hit_tests[1].target = "⭐️".to_owned()),
        Box::new(|e| e.frame_observations.hit_tests[0].range_start = 1),
        Box::new(|e| e.frame_observations.hit_tests[1].range_end = 200),
    ];
    for mutate in cases {
        let mut evidence = artifact();
        mutate(&mut evidence);
        evidence = evidence.seal().unwrap();
        assert_eq!(
            evidence.verify(&expectations(&evidence)),
            Err(NativeImeVerificationError::InvalidObservation("hit tests"))
        );
    }
}

#[test]
fn rejects_invalid_observation_hashes_and_accesskit_dimensions() {
    let fields: Vec<EvidenceMutation> = vec![
        Box::new(|e| e.observations.caret_sha256 = "bad".to_owned()),
        Box::new(|e| e.observations.hit_test_sha256 = "bad".to_owned()),
        Box::new(|e| e.observations.accesskit_sha256 = "bad".to_owned()),
        Box::new(|e| e.observations.frame_sha256 = "bad".to_owned()),
        Box::new(|e| e.observations.root_sha256 = "bad".to_owned()),
        Box::new(|e| e.observations.receipt_sha256 = "bad".to_owned()),
        Box::new(|e| e.observations.font_file_sha256 = "bad".to_owned()),
        Box::new(|e| e.observations.font_catalog_sha256 = "bad".to_owned()),
        Box::new(|e| e.frame_observations.accesskit.bounds.width = 0),
        Box::new(|e| e.frame_observations.accesskit.bounds.height = 0),
        Box::new(|e| e.frame_observations.accesskit.value = "other".to_owned()),
        Box::new(|e| e.frame_observations.accesskit.role = "Button".to_owned()),
    ];
    for mutate in fields {
        let mut evidence = artifact();
        mutate(&mut evidence);
        evidence = evidence.seal().unwrap();
        assert!(evidence.verify(&expectations(&evidence)).is_err());
    }
}
