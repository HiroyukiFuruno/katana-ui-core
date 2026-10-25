#![cfg(feature = "egui")]

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceHostRootFrame,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootFactory,
    KucRootEventBatchContext, KucUiSelectionAvailability, KucUiSelectionPhase,
    KucUiSelectionRestoreError, TextCommandSurfaceStyle,
};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfaceFocusRequest, TextSurfaceFocusRequestToken,
    TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use std::{cell::RefCell, rc::Rc};

type Events = Rc<RefCell<Vec<TextSurfaceEvent>>>;

#[test]
fn a_pass_marked_for_discard_before_show_never_publishes_selection_state() {
    let context = egui::Context::default();
    let (mut root, _) = fixture("abc", 1);
    for _ in 0..3 {
        frame(&context, &mut root, None);
    }
    let mut first = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 360.0),
            )),
            ..Default::default()
        },
        |ui| {
            if ui.ctx().current_pass_index() == 0 {
                ui.ctx().request_discard("discard before root input");
            }
            let shown = root.show(ui).expect("show");
            if first.is_none() {
                first = Some(shown.ui_selection_after_input().clone());
            }
        },
    );
    output.textures_delta.clear();
    let observation = first.expect("first pass observation");
    assert_eq!(
        observation.availability(),
        KucUiSelectionAvailability::Provisional
    );
    assert!(observation.snapshot().is_none());
}

#[test]
fn restore_rejects_a_snapshot_invalidated_by_a_late_discard() {
    use katana_ui_core::egui::text_command_surface::{
        KucUiSelectionObservation, KucUiSelectionSnapshot,
    };
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    };

    let context = egui::Context::default();
    let (mut root, _) = fixture("abc", 1);
    for _ in 0..3 {
        frame(&context, &mut root, None);
    }
    let observation = Arc::new(Mutex::new(None::<KucUiSelectionObservation>));
    let saved = Arc::new(Mutex::new(None::<KucUiSelectionSnapshot>));
    let once = Arc::new(AtomicBool::new(true));
    let end_observation = Arc::clone(&observation);
    let end_saved = Arc::clone(&saved);
    context.on_end_pass(
        "zz late selection discard",
        Arc::new(move |ui| {
            if once.swap(false, Ordering::SeqCst) {
                *end_saved.lock().expect("saved lock") = end_observation
                    .lock()
                    .expect("observation lock")
                    .as_ref()
                    .expect("rendered observation")
                    .snapshot()
                    .cloned();
                ui.ctx().request_discard("late selection discard");
            }
        }),
    );
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 360.0),
            )),
            ..Default::default()
        },
        |ui| {
            let shown = root.show(ui).expect("show");
            *observation.lock().expect("observation lock") =
                Some(shown.ui_selection_after_input().clone());
        },
    );
    output.textures_delta.clear();
    let snapshot = saved
        .lock()
        .expect("saved lock")
        .take()
        .expect("end-pass snapshot");
    assert_eq!(
        root.restore_ui_selection_once(&snapshot.restore_request()),
        Err(KucUiSelectionRestoreError::Unavailable(
            KucUiSelectionAvailability::Provisional
        ))
    );
}

fn fixture(text: &str, caret: usize) -> (EguiTextCommandSurfaceHostRoot, Events) {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("snapshot-contract").value(text),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 640, 360),
    ));
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.selection_start = caret;
    presentation.selection_end = caret;
    presentation.focus_request = Some(TextSurfaceFocusRequest::new(
        TextSurfaceFocusRequestToken::new("initial-focus"),
        true,
    ));
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"snapshot-root",
        EguiTextCommandSurfacePresentation {
            text_state_id: None,
            text: presentation,
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    let events = Rc::new(RefCell::new(Vec::new()));
    let sink = Rc::clone(&events);
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        move |context: KucRootEventBatchContext| {
            sink.borrow_mut()
                .extend(context.text_events().iter().cloned());
            Ok(None)
        },
    );
    (
        EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(lease)
            .expect("root"),
        events,
    )
}

fn frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    key: Option<egui::Key>,
) -> EguiTextCommandSurfaceHostRootFrame {
    let events = key
        .map(|key| {
            [true, false]
                .map(|pressed| egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                })
                .to_vec()
        })
        .unwrap_or_default();
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 360.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            result = Some(root.show(ui).expect("show"));
        },
    );
    output.textures_delta.clear();
    result.expect("frame")
}

fn last_selection(events: &Events) -> Option<(usize, usize)> {
    events.borrow().iter().rev().find_map(|event| match event {
        TextSurfaceEvent::SelectionChanged {
            selection_start,
            selection_end,
        } => Some((*selection_start, *selection_end)),
        _ => None,
    })
}

#[test]
fn saved_short_line_caret_restores_desired_column_and_reuses_snapshot() {
    let context = egui::Context::default();
    let (mut root, events) = fixture("abcdef\nx\nabcdef", 6);
    for _ in 0..3 {
        frame(&context, &mut root, None);
    }
    let before = root.ui_selection_before_input();
    let short = frame(&context, &mut root, Some(egui::Key::ArrowDown));
    assert_eq!(last_selection(&events), Some((8, 8)));
    let saved = short
        .ui_selection_after_input()
        .snapshot()
        .expect("focused snapshot")
        .clone();
    assert_eq!(format!("{saved:?}"), "KucUiSelectionSnapshot(..)");
    assert_eq!(before.phase(), KucUiSelectionPhase::BeforeInput);
    assert_eq!(
        short.ui_selection_after_input().phase(),
        KucUiSelectionPhase::AfterInput
    );
    assert_eq!(
        before.frame_number(),
        short.ui_selection_after_input().frame_number()
    );
    let idle = frame(&context, &mut root, None);
    assert_eq!(Some(&saved), idle.ui_selection_after_input().snapshot());
    for _ in 0..2 {
        frame(&context, &mut root, Some(egui::Key::Home));
        let request = saved.restore_request();
        root.restore_ui_selection_once(&request).expect("restore");
        assert_eq!(
            root.restore_ui_selection_once(&request),
            Err(KucUiSelectionRestoreError::DuplicateRequest)
        );
        frame(&context, &mut root, Some(egui::Key::ArrowDown));
        assert_eq!(last_selection(&events), Some((15, 15)));
    }
}

#[test]
fn observations_distinguish_focus_composition_and_root_binding() {
    let context = egui::Context::default();
    let (mut root, _) = fixture("日本語⭐️\n☆", 3);
    assert_eq!(
        root.ui_selection_before_input().availability(),
        KucUiSelectionAvailability::Unfocused
    );
    for _ in 0..3 {
        frame(&context, &mut root, None);
    }
    let saved = root
        .ui_selection_before_input()
        .snapshot()
        .expect("focused")
        .clone();
    let (mut other, _) = fixture("日本語⭐️\n☆", 3);
    let different_root = other
        .restore_ui_selection_once(&saved.restore_request())
        .expect_err("different root must reject the snapshot");
    assert_eq!(different_root, KucUiSelectionRestoreError::DifferentRoot);
    assert_eq!(
        different_root.to_string(),
        "UI selection restore rejected: DifferentRoot"
    );
    assert!(format!("{different_root:?}").contains("DifferentRoot"));
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 360.0),
            )),
            events: vec![egui::Event::Ime(egui::ImeEvent::Preedit {
                text: "にほん".into(),
                active_range_chars: None,
            })],
            ..Default::default()
        },
        |ui| {
            result = Some(root.show(ui).expect("show"));
        },
    );
    output.textures_delta.clear();
    let composed = result.expect("frame");
    assert_eq!(
        composed.ui_selection_after_input().availability(),
        KucUiSelectionAvailability::Composing
    );
    assert!(composed.ui_selection_after_input().snapshot().is_none());
    assert_eq!(
        root.restore_ui_selection_once(&saved.restore_request()),
        Err(KucUiSelectionRestoreError::Unavailable(
            KucUiSelectionAvailability::Composing
        ))
    );
}

fn synchronize(
    root: &mut EguiTextCommandSurfaceHostRoot,
    revision: u64,
    text: &str,
    readonly: bool,
    disabled: bool,
    selection: (usize, usize),
) {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("snapshot-contract").value(text),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 640, 360),
    ));
    let mut text = TextSurfacePresentation::from_props(surface.props());
    text.readonly = readonly;
    text.disabled = disabled;
    text.selection_start = selection.0;
    text.selection_end = selection.1;
    root.synchronize(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            revision,
            b"snapshot-root",
            EguiTextCommandSurfacePresentation {
                text_state_id: None,
                text,
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            TextCommandSurfaceStyle::standard().expect("style"),
        )
        .expect("token"),
    )
    .expect("sync");
}

#[test]
fn snapshot_rejects_document_changes_and_unavailable_state_without_consuming_request() {
    let context = egui::Context::default();
    let (mut root, _) = fixture("abcdef", 2);
    for _ in 0..3 {
        frame(&context, &mut root, None);
    }
    let saved = root
        .ui_selection_before_input()
        .snapshot()
        .expect("saved")
        .clone();
    let request = saved.restore_request();
    synchronize(&mut root, 2, "abcdef", true, false, (2, 2));
    assert_eq!(
        root.ui_selection_before_input().availability(),
        KucUiSelectionAvailability::Readonly
    );
    assert_eq!(
        root.restore_ui_selection_once(&request),
        Err(KucUiSelectionRestoreError::Unavailable(
            KucUiSelectionAvailability::Readonly
        ))
    );
    synchronize(&mut root, 3, "abcdef", false, true, (2, 2));
    assert_eq!(
        root.ui_selection_before_input().availability(),
        KucUiSelectionAvailability::Disabled
    );
    assert_eq!(
        root.restore_ui_selection_once(&request),
        Err(KucUiSelectionRestoreError::Unavailable(
            KucUiSelectionAvailability::Disabled
        ))
    );
    synchronize(&mut root, 4, "changed", false, false, (2, 2));
    assert_eq!(
        root.restore_ui_selection_once(&request),
        Err(KucUiSelectionRestoreError::DifferentDocument)
    );
    synchronize(&mut root, 5, "abcdef", false, false, (2, 2));
    root.restore_ui_selection_once(&request)
        .expect("rejection must not consume request");
    assert_eq!(root.ui_selection_before_input().snapshot(), Some(&saved));
    synchronize(&mut root, 6, "abcdef", false, false, (5, 1));
    let reversed = root
        .ui_selection_before_input()
        .snapshot()
        .expect("reverse")
        .clone();
    synchronize(&mut root, 7, "abcdef", false, false, (1, 5));
    assert_ne!(root.ui_selection_before_input().snapshot(), Some(&reversed));
    root.restore_ui_selection_once(&reversed.restore_request())
        .expect("restore reverse");
    assert_eq!(root.ui_selection_before_input().snapshot(), Some(&reversed));
}

#[test]
fn discarded_pass_cannot_publish_a_reusable_snapshot() {
    let context = egui::Context::default();
    let (mut root, _) = fixture("日本語⭐️ abc", 3);
    for _ in 0..3 {
        frame(&context, &mut root, None);
    }
    let before = root.ui_selection_before_input();
    let mut passes = Vec::new();
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 360.0),
            )),
            ..Default::default()
        },
        |ui| {
            let shown = root.show(ui).expect("show");
            assert!(
                shown.ui_selection_after_input().snapshot().is_none(),
                "pass is still provisional"
            );
            if ui.ctx().current_pass_index() == 0 {
                ui.ctx().request_discard("snapshot regression");
            }
            passes.push(shown);
        },
    );
    output.textures_delta.clear();
    assert_eq!(passes.len(), 2);
    assert_eq!(
        passes[0].ui_selection_after_input().availability(),
        KucUiSelectionAvailability::Provisional
    );
    assert!(passes[0].ui_selection_after_input().snapshot().is_none());
    assert_eq!(
        passes[1].ui_selection_after_input().availability(),
        KucUiSelectionAvailability::FocusedEditable
    );
    assert!(passes[1].ui_selection_after_input().snapshot().is_some());
    for pass in &passes {
        assert_eq!(
            pass.ui_selection_after_input().frame_number(),
            before.frame_number()
        );
    }
}

#[test]
fn hidden_root_reports_unavailable_without_erasing_saved_ui_state() {
    let context = egui::Context::default();
    let (mut root, _) = fixture("日本語⭐️", 3);
    for _ in 0..3 {
        frame(&context, &mut root, None);
    }
    let saved = root
        .ui_selection_before_input()
        .snapshot()
        .expect("saved")
        .clone();
    let mut hidden = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 360.0),
            )),
            ..Default::default()
        },
        |ui| {
            ui.set_invisible();
            hidden = Some(root.show(ui).expect("hidden show"));
        },
    );
    output.textures_delta.clear();
    let hidden = hidden.expect("frame");
    assert_eq!(
        hidden.ui_selection_after_input().availability(),
        KucUiSelectionAvailability::Hidden
    );
    assert!(hidden.ui_selection_after_input().snapshot().is_none());
    assert_eq!(
        root.restore_ui_selection_once(&saved.restore_request()),
        Err(KucUiSelectionRestoreError::Unavailable(
            KucUiSelectionAvailability::Hidden
        ))
    );
    frame(&context, &mut root, None);
    assert_eq!(
        root.ui_selection_before_input().availability(),
        KucUiSelectionAvailability::Unfocused
    );
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("snapshot-contract").value("日本語⭐️"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 640, 360),
    ));
    let mut text = TextSurfacePresentation::from_props(surface.props());
    text.selection_start = 3;
    text.selection_end = 3;
    text.focus_request = Some(TextSurfaceFocusRequest::new(
        TextSurfaceFocusRequestToken::new("focus-after-hidden"),
        true,
    ));
    root.synchronize(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            2,
            b"snapshot-root",
            EguiTextCommandSurfacePresentation {
                text_state_id: None,
                text,
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            TextCommandSurfaceStyle::standard().expect("style"),
        )
        .expect("token"),
    )
    .expect("focus sync");
    for _ in 0..3 {
        frame(&context, &mut root, None);
    }
    assert_eq!(
        root.ui_selection_before_input().availability(),
        KucUiSelectionAvailability::FocusedEditable
    );
    assert_eq!(root.ui_selection_before_input().snapshot(), Some(&saved));
}
