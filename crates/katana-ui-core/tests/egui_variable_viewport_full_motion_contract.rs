#![cfg(feature = "egui")]
#![cfg(feature = "storybook-artifacts")]

use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceRootFactory,
    FullTextCommandSurfaceMotionPlan, FullTextCommandSurfaceScenarioFactory,
    FullTextCommandSurfaceScenarioId,
};
use katana_ui_core::egui::{
    MotionArtifactWriter, OpaqueMotionReceiptSequence, OpaqueRootArtifactReceiptWriter,
};
use sha2::{Digest, Sha256};

fn initial_motion_input() -> egui::RawInput {
    let mut input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1280.0, 720.0),
        )),
        ..egui::RawInput::default()
    };
    input.viewports.insert(
        egui::ViewportId::ROOT,
        egui::ViewportInfo {
            native_pixels_per_point: Some(1.0),
            ..egui::ViewportInfo::default()
        },
    );
    input
}

fn retain_root(scenario_id: FullTextCommandSurfaceScenarioId) -> EguiTextCommandSurfaceHostRoot {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(scenario_id)
        .expect("every KUC-issued motion scenario must retain");
    EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            scenario
                .into_lease()
                .expect("a newly issued scenario owns one lease"),
        )
        .expect("KUC-issued scenario lease must retain a root")
}

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn accesskit_input_hash(node: &egui::accesskit::Node) -> Option<String> {
    #[derive(serde::Serialize)]
    struct Bounds {
        x0_bits: u64,
        y0_bits: u64,
        x1_bits: u64,
        y1_bits: u64,
    }
    #[derive(serde::Serialize)]
    struct Snapshot<'a> {
        role: &'static str,
        value: &'a str,
        scalar_sequence: Vec<u32>,
        bounds: Bounds,
    }
    let role = match node.role() {
        egui::accesskit::Role::TextInput => "text-input",
        egui::accesskit::Role::MultilineTextInput => "multiline-text-input",
        _ => return None,
    };
    let value = node.value()?;
    let bounds = node.bounds()?;
    if !value.contains("入力")
        || ![bounds.x0, bounds.y0, bounds.x1, bounds.y1]
            .into_iter()
            .all(f64::is_finite)
        || bounds.x1 <= bounds.x0
        || bounds.y1 <= bounds.y0
    {
        return None;
    }
    let material = Snapshot {
        role,
        value,
        scalar_sequence: value.chars().map(u32::from).collect(),
        bounds: Bounds {
            x0_bits: bounds.x0.to_bits(),
            y0_bits: bounds.y0.to_bits(),
            x1_bits: bounds.x1.to_bits(),
            y1_bits: bounds.y1.to_bits(),
        },
    };
    let canonical =
        serde_json::to_value(&material).expect("actual AccessKit projection must serialize");
    Some(sha256(canonical.to_string().as_bytes()))
}

#[test]
fn variable_viewport_consumer_rejects_an_empty_receipt_sequence() {
    let output_directory = tempfile::tempdir().expect("artifact directory must create");
    assert!(matches!(
        MotionArtifactWriter::new().write_opaque_variable_viewport(
            &OpaqueMotionReceiptSequence::new(),
            output_directory.path(),
        ),
        Err(
            katana_ui_core::egui::VariableViewportMotionArtifactError::Motion(
                katana_ui_core::egui::MotionArtifactError::EmptySequence
            )
        )
    ));
}

#[test]
fn full_motion_plan_exports_a_variable_viewport_artifact_with_bound_semantics() {
    let plan = FullTextCommandSurfaceMotionPlan::issue(
        FullTextCommandSurfaceMotionPlan::minimum_frame_count(),
    )
    .expect("the complete KUC motion catalogue must issue");
    let receipt_directory = tempfile::tempdir().expect("receipt directory must create");
    let output_directory = tempfile::tempdir().expect("artifact directory must create");
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut active_scenario = None;
    let mut root = None;
    let mut continuation = None;
    let mut sequence = OpaqueMotionReceiptSequence::new();
    let mut accesskit_frames = std::collections::BTreeMap::new();

    for (index, motion_frame) in plan.frames().iter().enumerate() {
        if active_scenario != Some(motion_frame.scenario_id()) {
            assert!(
                continuation.is_none(),
                "an opaque continuation must finish before a scenario root changes"
            );
            root = Some(retain_root(motion_frame.scenario_id()));
            active_scenario = Some(motion_frame.scenario_id());
        }

        let mut input = initial_motion_input();
        motion_frame
            .apply_to(&mut input, &mut continuation)
            .expect("each KUC-issued motion frame must apply");
        let mut captured = None;
        let mut output = context.run_ui(input, |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                captured = Some(
                    root.as_mut()
                        .expect("the current scenario retains a root")
                        .show(ui),
                );
            });
        });
        output.textures_delta.clear();
        let frame = captured
            .expect("egui must invoke the retained root")
            .expect("KUC root must render the issued motion frame");
        let accesskit_update = output
            .platform_output
            .accesskit_update
            .take()
            .expect("each KUC motion frame must publish its AccessKit projection");
        accesskit_frames.insert(
            frame.record().record_hash().to_owned(),
            accesskit_update.nodes,
        );
        motion_frame
            .capture_continuation(frame.interaction_locator(), &mut continuation)
            .expect("the KUC root must resolve its own continuation");

        let stage_id = format!("frame-{index:03}");
        let receipt = OpaqueRootArtifactReceiptWriter::new()
            .write(&frame, receipt_directory.path(), &stage_id)
            .expect("the same root frame must produce an opaque receipt");
        sequence
            .push(&stage_id, receipt)
            .expect("the KUC receipt sequence remains ordered");
    }
    assert!(
        continuation.is_none(),
        "the complete motion plan must not leave an interaction pending"
    );

    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;

        let invalid_output = output_directory
            .path()
            .join(std::ffi::OsString::from_vec(vec![b'o', 0xff]));
        assert!(matches!(
            MotionArtifactWriter::new().write_opaque_variable_viewport(&sequence, &invalid_output),
            Err(
                katana_ui_core::egui::VariableViewportMotionArtifactError::Motion(
                    katana_ui_core::egui::MotionArtifactError::InvalidSettings
                )
            )
        ));
        assert!(
            !invalid_output.exists(),
            "a path that the manifest cannot represent must not create any output"
        );
    }

    let non_directory_output = output_directory.path().join("not-a-directory");
    std::fs::write(&non_directory_output, b"not a directory")
        .expect("non-directory output fixture should write");
    assert!(matches!(
        MotionArtifactWriter::new()
            .write_opaque_variable_viewport(&sequence, &non_directory_output),
        Err(
            katana_ui_core::egui::VariableViewportMotionArtifactError::Motion(
                katana_ui_core::egui::MotionArtifactError::Io(_)
            )
        )
    ));

    let artifact = MotionArtifactWriter::new()
        .write_opaque_variable_viewport(&sequence, output_directory.path())
        .expect("the full KUC motion plan must export through the public API");
    let manifest = artifact.manifest();

    assert_eq!(manifest.source_frame_count, plan.frames().len());
    assert_eq!(manifest.decoded_frame_count, plan.frames().len());
    assert_eq!(manifest.source_viewports.len(), plan.frames().len());
    assert_eq!(manifest.source_png_sha256.len(), plan.frames().len());
    assert_eq!(manifest.root_record_hashes.len(), plan.frames().len());
    assert_eq!(manifest.source_frame_hashes, manifest.decoded_frame_hashes);
    assert!(
        manifest
            .source_viewports
            .iter()
            .map(|viewport| (viewport.width, viewport.height))
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            > 1,
        "the full plan must preserve its resize viewport as source provenance"
    );
    assert!(manifest.source_viewports.iter().all(|viewport| {
        viewport.width <= manifest.width && viewport.height <= manifest.height
    }));
    assert!(
        manifest
            .source_png_sha256
            .iter()
            .all(|hash| { hash.len() == 64 && hash.bytes().all(|byte| byte.is_ascii_hexdigit()) })
    );

    let semantic = &manifest.semantic_evidence;
    assert_eq!(semantic.star_scalar_sequence, [0x2b50, 0xfe0f]);
    assert!(semantic.ime_preedit_event_seen);
    assert!(semantic.ime_commit_event_seen);
    assert_eq!(semantic.hit_test_count, 1);
    assert!(!semantic.accesskit_snapshot_hash.is_empty());
    let commit_nodes = accesskit_frames
        .get(&semantic.root_record_hash)
        .expect("semantic evidence must reference an actually published frame");
    assert!(
        commit_nodes.iter().any(|(_, node)| {
            accesskit_input_hash(node).as_deref() == Some(semantic.accesskit_snapshot_hash.as_str())
        }),
        "manifest must bind the real commit-frame text-input value, scalars and bounds"
    );
    assert!(semantic.root_record_hashes.iter().all(|root_hash| {
        manifest
            .root_record_hashes
            .iter()
            .any(|frame_hash| frame_hash == root_hash)
    }));

    let mut canonical_manifest = manifest.clone();
    canonical_manifest.canonical_sha256.clear();
    assert_eq!(
        manifest.canonical_sha256,
        sha256(&serde_json::to_vec(&canonical_manifest).expect("manifest must serialize"))
    );
    let mut canonical_semantic = semantic.clone();
    canonical_semantic.artifact_sha256.clear();
    assert_eq!(
        semantic.artifact_sha256,
        sha256(&serde_json::to_vec(&canonical_semantic).expect("semantic evidence must serialize"))
    );
    assert!(artifact.manifest_path().is_file());
    for (path, expected_hash) in [
        (&manifest.gif_path, &manifest.gif_sha256),
        (&manifest.mp4_path, &manifest.mp4_sha256),
    ] {
        let bytes = std::fs::read(path).expect("published artifact must remain readable");
        assert!(!bytes.is_empty());
        assert_eq!(sha256(&bytes), *expected_hash);
    }

    assert!(matches!(
        MotionArtifactWriter::new()
            .write_opaque_variable_viewport(&sequence, output_directory.path()),
        Err(katana_ui_core::egui::VariableViewportMotionArtifactError::OccupiedOutputTarget { .. })
    ));
}
