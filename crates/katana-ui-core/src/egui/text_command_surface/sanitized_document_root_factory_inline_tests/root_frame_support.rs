use super::support::*;
use super::*;

const SELECTION_HORIZONTAL_INSET: f32 = 8.0;

pub(super) fn run_root_frame(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    event: egui::Event,
) -> super::SanitizedDocumentRootFrame {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events: vec![event],
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    output.textures_delta.clear();
    frame.expect("frame exists")
}

pub(super) fn run_root_frame_events(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, super::SanitizedDocumentRootFrame) {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    output.textures_delta.clear();
    (output, frame.expect("frame exists"))
}

pub(super) fn select_floating_surface(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
) -> (egui::FullOutput, super::SanitizedDocumentRootFrame) {
    let (_, initial) = run_root_frame_events(
        context,
        root,
        vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
    );
    let content_bounds = initial.output.evidence_text.record.frame.content_bounds;
    let start = egui::pos2(
        content_bounds.x as f32 + SELECTION_HORIZONTAL_INSET,
        content_bounds.y as f32 + content_bounds.height as f32 / 2.0,
    );
    let midpoint = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 / 2.0,
        start.y,
    );
    let end = egui::pos2(
        content_bounds.x as f32 + content_bounds.width as f32 - SELECTION_HORIZONTAL_INSET,
        start.y,
    );
    let _ = run_root_frame_events(context, root, vec![pointer_button(start, true)]);
    let _ = run_root_frame_events(context, root, vec![egui::Event::PointerMoved(midpoint)]);
    let _ = run_root_frame_events(context, root, vec![egui::Event::PointerMoved(end)]);
    run_root_frame_events(context, root, vec![pointer_button(end, false)])
}

pub(super) fn run_root_frame_result(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> Result<super::SanitizedDocumentRootFrame, SanitizedDocumentRootFactoryError> {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui));
            });
        },
    );
    output.textures_delta.clear();
    frame.expect("frame exists")
}

pub(super) fn secondary_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Secondary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

pub(super) fn context_input(
    revision: u64,
    calls: Rc<RefCell<usize>>,
    nested: bool,
    enabled: bool,
    visible: bool,
    capability: bool,
    reject: bool,
) -> SanitizedDocumentRootInput {
    let target = |bytes: &[u8], calls: Rc<RefCell<usize>>| {
        let target = SanitizedContextMenuTarget::from_opaque_bytes(bytes.to_vec());
        if capability {
            target.with_unit_capability(move || {
                *calls.borrow_mut() += 1;
                if reject { Err(()) } else { Ok(()) }
            })
        } else {
            target
        }
    };
    let leaf = SanitizedContextMenuItem::new(
        target(b"context-leaf-secret", calls.clone()),
        0,
        "葉 日本語 ⭐️👩‍💻",
    )
    .enabled_state(enabled);
    let item = if nested {
        SanitizedContextMenuItem::new(
            SanitizedContextMenuTarget::from_opaque_bytes(b"submenu-secret".to_vec()),
            0,
            "親 日本語 ⭐️👩‍💻",
        )
        .submenu_item(leaf)
    } else {
        leaf
    };
    let projection = SanitizedContextMenuProjectionBuilder::new()
        .item(item)
        .build();
    let projection = if visible {
        projection
    } else {
        SanitizedContextMenuProjection::default()
    };
    input(revision, b"context-document", "本文 日本語 ⭐️👩‍💻").with_context_projection(projection)
}

pub(super) fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

pub(super) fn key_press(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}
