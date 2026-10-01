use super::*;

const SURFACE_WIDTH: f32 = 640.0;
const SURFACE_HEIGHT: f32 = 240.0;

fn input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SURFACE_WIDTH, SURFACE_HEIGHT),
        )),
        ..Default::default()
    }
}

#[test]
fn new_rejects_a_readable_file_that_is_not_a_color_emoji_font() {
    let path = std::env::temp_dir().join(format!(
        "kuc-native-ime-invalid-font-{}",
        std::process::id()
    ));
    std::fs::write(&path, b"not a font").expect("the invalid-font fixture must be writable");
    let mut options = KucUnicodeColorGlyphEvidenceOptions::default();
    options.config.emoji_candidates = vec![path.clone()];

    let result = KucNativeUnicodeEvidenceSession::new(options);
    let _ = std::fs::remove_file(path);

    assert!(matches!(
        result,
        Err(KucUnicodeColorGlyphEvidenceError::ColorEmojiUnavailable { .. })
    ));
}

#[test]
fn context_returns_the_session_egui_context() {
    let session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    let id = egui::Id::new("native-session-context-contract");
    session
        .context()
        .data_mut(|data| data.insert_temp(id, 17_u32));
    assert_eq!(
        session.context().data(|data| data.get_temp::<u32>(id)),
        Some(17)
    );
}

#[test]
fn run_frame_rejects_serial_overflow_before_rendering() {
    let mut session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    session.frame_serial = u64::MAX;

    assert!(matches!(
        session.run_frame(input()),
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "native frame serial overflow"
    ));
}

#[test]
fn observation_rejects_a_tree_without_an_accesskit_text_input_node() {
    let mut session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    let mut rendered = None;
    let mut output = session.context.run_ui(input(), |ui| {
        rendered = Some(session.root.show(ui, &surface::trace_style()));
    });
    output.textures_delta.clear();
    let root_output = rendered
        .expect("root closure must run")
        .expect("root frame must render");
    let update = egui::accesskit::TreeUpdate {
        nodes: vec![],
        tree: Some(egui::accesskit::Tree {
            root: 0.into(),
            toolkit_name: None,
            toolkit_version: None,
        }),
        tree_id: egui::accesskit::TreeId::ROOT,
        focus: 0.into(),
    };

    assert!(matches!(
        observations::extract(&session.root, &root_output, &update, "日本語", 1),
        Err(KucUnicodeColorGlyphEvidenceError::MissingAccessKitNode)
    ));
}

#[test]
fn observation_rejects_an_accesskit_text_input_without_bounds() {
    let mut session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    let mut rendered = None;
    let mut output = session.context.run_ui(input(), |ui| {
        rendered = Some(session.root.show(ui, &surface::trace_style()));
    });
    output.textures_delta.clear();
    let root_output = rendered
        .expect("root closure must run")
        .expect("root frame must render");
    let node = egui::accesskit::Node::new(egui::accesskit::Role::MultilineTextInput);
    let update = egui::accesskit::TreeUpdate {
        nodes: vec![(1.into(), node)],
        tree: None,
        tree_id: egui::accesskit::TreeId::ROOT,
        focus: 1.into(),
    };

    assert!(matches!(
        observations::extract(&session.root, &root_output, &update, "日本語", 1),
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "native AccessKit bounds missing"
    ));
}

#[test]
fn observation_rejects_an_accesskit_text_input_without_value() {
    let mut session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    let mut rendered = None;
    let mut output = session.context.run_ui(input(), |ui| {
        rendered = Some(session.root.show(ui, &surface::trace_style()));
    });
    output.textures_delta.clear();
    let root_output = rendered
        .expect("root closure must run")
        .expect("root frame must render");
    let mut node = egui::accesskit::Node::new(egui::accesskit::Role::MultilineTextInput);
    node.set_bounds(egui::accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 100.0,
        y1: 24.0,
    });
    let update = egui::accesskit::TreeUpdate {
        nodes: vec![(1.into(), node)],
        tree: None,
        tree_id: egui::accesskit::TreeId::ROOT,
        focus: 1.into(),
    };

    assert!(matches!(
        observations::extract(&session.root, &root_output, &update, "日本語", 1),
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "native AccessKit text missing"
    ));
}

#[test]
fn observation_rejects_forwarding_the_same_root_events_twice() {
    let mut session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    let mut rendered = None;
    let mut output = session.context.run_ui(input(), |ui| {
        rendered = Some(session.root.show(ui, &surface::trace_style()));
    });
    output.textures_delta.clear();
    let root_output = rendered
        .expect("root closure must run")
        .expect("root frame must render");
    let mut node = egui::accesskit::Node::new(egui::accesskit::Role::MultilineTextInput);
    node.set_bounds(egui::accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 100.0,
        y1: 24.0,
    });
    node.set_value(root_output.evidence_text.raster.text.clone());
    let update = egui::accesskit::TreeUpdate {
        nodes: vec![(1.into(), node)],
        tree: None,
        tree_id: egui::accesskit::TreeId::ROOT,
        focus: 1.into(),
    };

    observations::extract(&session.root, &root_output, &update, "日本語", 1)
        .expect("the first forwarding receipt must be accepted");
    let second = observations::extract(&session.root, &root_output, &update, "日本語", 1);
    assert!(matches!(
        second,
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message.contains("AlreadyConsumed")
    ));
}

#[test]
fn observation_rejects_a_texture_offset_outside_the_root_image() {
    let mut session =
        KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
            .expect("platform emoji font must be installed for the root contract");
    let mut rendered = None;
    let mut output = session.context.run_ui(input(), |ui| {
        rendered = Some(session.root.show(ui, &surface::trace_style()));
    });
    output.textures_delta.clear();
    let mut root_output = rendered
        .expect("root closure must run")
        .expect("root frame must render");
    let texture = root_output.evidence_text.record.texture_bounds;
    root_output.evidence_text.record.texture_bounds =
        crate::render_model::UiRect::new(i32::MAX, i32::MAX, texture.width, texture.height);

    let update = egui::accesskit::TreeUpdate {
        nodes: vec![],
        tree: None,
        tree_id: egui::accesskit::TreeId::ROOT,
        focus: 0.into(),
    };
    assert!(matches!(
        observations::extract(&session.root, &root_output, &update, "日本語", 1),
        Err(KucUnicodeColorGlyphEvidenceError::RootTrace(message))
            if message == "composite crop pixel missing"
    ));
}
