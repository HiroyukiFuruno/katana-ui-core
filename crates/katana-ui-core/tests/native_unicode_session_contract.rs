#![cfg(all(feature = "egui", feature = "native-ime-evidence"))]

use katana_ui_core::egui::text_command_surface::{
    KucNativeUnicodeEvidenceSession, KucUnicodeColorGlyphEvidenceOptions,
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
