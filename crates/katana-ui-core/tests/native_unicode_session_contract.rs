#![cfg(all(feature = "egui", feature = "native-ime-evidence"))]

use katana_ui_core::egui::text_command_surface::{
    KucNativeUnicodeEvidenceSession, KucUnicodeColorGlyphEvidenceOptions,
};
use katana_ui_core::native_ime_evidence::{
    NativeImeEvidenceArtifact, NativeImeTranscript, NativeImeVerificationExpectations,
    NativeRunBinding, SCHEMA_VERSION,
};

fn input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(640.0, 240.0),
        )),
        events,
        ..Default::default()
    }
}

fn focused_session() -> KucNativeUnicodeEvidenceSession {
    let mut session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    session.run_frame(input(Vec::new())).expect("initial frame");
    for pressed in [true, false] {
        session
            .run_frame(input(vec![egui::Event::PointerButton {
                pos: egui::pos2(620.0, 12.0),
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            }]))
            .expect("pointer frame");
    }
    session
}

#[test]
fn native_session_accepts_ime_without_a_pointer_click() {
    let mut session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    session.run_frame(input(Vec::new())).expect("initial frame");
    session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "にほんご".into(),
            active_range_chars: None,
        })]))
        .expect("preedit frame");
    session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: String::new(),
            active_range_chars: None,
        })]))
        .expect("composition clear frame");
    let committed = session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語".into(),
        ))]))
        .expect("commit frame");
    assert!(committed.observations.is_some());
}

#[test]
fn missing_font_rejects_session_instead_of_fabricating_evidence() {
    let mut options = KucUnicodeColorGlyphEvidenceOptions::default();
    options.config.emoji_candidates.clear();
    assert!(KucNativeUnicodeEvidenceSession::new(options).is_err());
}

#[test]
fn root_contract_collects_real_pixels_and_accesskit_without_claiming_native_origin() {
    let mut session = focused_session();
    let preedit = session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "にほんご".into(),
            active_range_chars: None,
        })]))
        .expect("preedit frame");
    assert_eq!(preedit.preedit.as_deref(), Some("にほんご"));
    assert!(preedit.observations.is_none());
    let clear = session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: String::new(),
            active_range_chars: None,
        })]))
        .expect("composition clear frame");
    assert_eq!(clear.preedit.as_deref(), Some("にほんご"));
    let committed = session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語".into(),
        ))]))
        .expect("commit frame");
    let (observations, crops, hashes) = committed
        .observations
        .expect("accepted commit must have observations");
    assert_eq!(observations.committed_text, "日本語");
    assert!(observations.final_text.contains("⭐️"));
    assert_eq!(observations.accesskit.value, observations.final_text);
    assert_eq!(observations.accesskit.role, "MultilineTextInput");
    assert_eq!(hashes.root_id, observations.root_id);
    assert!(crops.has_colored_pixels);
    assert_ne!(crops.rgba_sha256, crops.control_rgba_sha256);
    assert_eq!(
        committed.rgba_pixels.len(),
        committed.width as usize * committed.height as usize * 4
    );
    assert_eq!(
        observations.observation_hashes().expect("hashes").0,
        hashes.measurement_sha256
    );
}

#[test]
fn synthetic_root_observations_round_trip_through_the_registry_verifier() {
    let mut session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    session.run_frame(input(Vec::new())).expect("initial frame");
    session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "にほんご".into(),
            active_range_chars: None,
        })]))
        .expect("preedit frame");
    session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: String::new(),
            active_range_chars: None,
        })]))
        .expect("composition clear frame");
    let frame = session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語".into(),
        ))]))
        .expect("commit frame");
    let (frame_observations, rgba_crop, observations) = frame
        .observations
        .expect("accepted commit must have observations");
    let mut artifact = NativeImeEvidenceArtifact {
        schema_version: SCHEMA_VERSION.to_owned(),
        revision: "synthetic-contract-revision".into(),
        platform: "macos".into(),
        run: NativeRunBinding {
            run_id: "synthetic-run".into(),
            challenge: "synthetic-challenge".into(),
            producer_id: "synthetic-producer".into(),
            runner_id: "synthetic-runner".into(),
            input_method: "contract-fixture".into(),
            origin: "os-native".into(),
        },
        ime: NativeImeTranscript {
            preedit: "にほんご".into(),
            commit: "日本語".into(),
            preedit_sequence: 1,
            commit_sequence: 2,
        },
        scalar_sequence: vec![0x2b50, 0xfe0f],
        rgba_crop,
        observations,
        frame_observations,
        artifact_sha256: String::new(),
    };
    artifact = artifact.seal().expect("synthetic artifact seals");
    let expected = NativeImeVerificationExpectations {
        revision: artifact.revision.clone(),
        platform: artifact.platform.clone(),
        run_id: artifact.run.run_id.clone(),
        challenge: artifact.run.challenge.clone(),
        producer_id: artifact.run.producer_id.clone(),
        runner_id: artifact.run.runner_id.clone(),
        attested_artifact_sha256: artifact.artifact_sha256.clone(),
    };
    if let Err(error) = artifact.verify(&expected) {
        panic!(
            "synthetic root artifact verification failed: {error:?}; measurement={}x{}, caret={:?}, accesskit_bounds={:?}, accesskit_role={:?}, accesskit_value_len={}, root_id={:?}, frame_id={:?}, hit_tests={:?}",
            artifact.frame_observations.measurement_width,
            artifact.frame_observations.measurement_height,
            artifact.frame_observations.caret,
            artifact.frame_observations.accesskit.bounds,
            artifact.frame_observations.accesskit.role,
            artifact.frame_observations.accesskit.value.len(),
            artifact.frame_observations.root_id,
            artifact.frame_observations.frame_id,
            artifact.frame_observations.hit_tests,
        );
    }
    let encoded = serde_json::to_vec(&artifact).expect("artifact serializes");
    let decoded: NativeImeEvidenceArtifact =
        serde_json::from_slice(&encoded).expect("artifact deserializes");
    if let Err(error) = decoded.verify(&expected) {
        panic!(
            "decoded synthetic root artifact verification failed: {error:?}; measurement={}x{}, caret={:?}, accesskit_bounds={:?}, accesskit_role={:?}, accesskit_value_len={}, root_id={:?}, frame_id={:?}, hit_tests={:?}",
            decoded.frame_observations.measurement_width,
            decoded.frame_observations.measurement_height,
            decoded.frame_observations.caret,
            decoded.frame_observations.accesskit.bounds,
            decoded.frame_observations.accesskit.role,
            decoded.frame_observations.accesskit.value.len(),
            decoded.frame_observations.root_id,
            decoded.frame_observations.frame_id,
            decoded.frame_observations.hit_tests,
        );
    }
}

#[test]
fn root_rejects_commit_without_accepted_preedit() {
    let mut session = focused_session();
    let result = session.run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Commit(
        "日本語".into(),
    ))]));
    assert!(result.is_err());
}

#[test]
fn root_rejects_a_commit_that_removes_required_evidence_scalars() {
    let mut session = focused_session();
    session
        .run_frame(input(vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers {
                ctrl: true,
                mac_cmd: true,
                command: true,
                ..Default::default()
            },
        }]))
        .expect("select-all frame");
    session
        .run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "にほんご".into(),
            active_range_chars: None,
        })]))
        .expect("preedit frame");
    assert!(matches!(
        session.run_frame(input(vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語".into(),
        ))])),
        Err(katana_ui_core::egui::text_command_surface::KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "native star/control scalar range missing"
    ));
}
