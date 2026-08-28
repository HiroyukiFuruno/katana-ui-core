use super::{
    TextSurface, TextSurfaceAction, TextSurfaceClipboardOperation, TextSurfaceEvent,
    TextSurfaceHistoryOperation, TextSurfaceProps, TextSurfaceViewport,
};
use crate::atom::{TextArea, TextAreaAction, TextAreaSelection};
use crate::render_model::UiTextSpan;

const VIEWPORT_WIDTH: u32 = 640;
const VIEWPORT_HEIGHT: u32 = 320;

#[test]
fn clipboard_and_history_are_typed_requests_without_text_payload() {
    let mut surface = surface(false);
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 0, end: 3 },
    )));

    let copy = surface.apply_action(TextSurfaceAction::ClipboardRequest(
        TextSurfaceClipboardOperation::Copy,
    ));
    let undo = surface.apply_action(TextSurfaceAction::HistoryRequest(
        TextSurfaceHistoryOperation::Undo,
    ));

    assert!(matches!(
        copy.events.as_slice(),
        [TextSurfaceEvent::ClipboardRequested {
            operation: TextSurfaceClipboardOperation::Copy,
            selection_start: 0,
            selection_end: 3,
        }]
    ));
    assert!(matches!(
        undo.events.as_slice(),
        [TextSurfaceEvent::HistoryRequested(
            TextSurfaceHistoryOperation::Undo
        )]
    ));
}

#[test]
fn every_clipboard_and_history_operation_uses_typed_requests_and_readonly_rules() {
    let mut writable = surface(false);
    let mut readonly = surface(true);
    for surface in [&mut writable, &mut readonly] {
        let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
            TextAreaSelection { start: 0, end: 3 },
        )));
    }

    for operation in [
        TextSurfaceClipboardOperation::Copy,
        TextSurfaceClipboardOperation::Cut,
        TextSurfaceClipboardOperation::Paste,
    ] {
        let outcome = writable.apply_action(TextSurfaceAction::ClipboardRequest(operation));
        assert!(outcome.handled);
        assert!(matches!(
            outcome.events.as_slice(),
            [TextSurfaceEvent::ClipboardRequested { operation: actual, .. }] if *actual == operation
        ));
    }
    for operation in [
        TextSurfaceHistoryOperation::Undo,
        TextSurfaceHistoryOperation::Redo,
    ] {
        let outcome = writable.apply_action(TextSurfaceAction::HistoryRequest(operation));
        assert!(outcome.handled);
        assert!(matches!(
            outcome.events.as_slice(),
            [TextSurfaceEvent::HistoryRequested(actual)] if *actual == operation
        ));
    }
    for operation in [
        TextSurfaceClipboardOperation::Cut,
        TextSurfaceClipboardOperation::Paste,
    ] {
        assert!(
            !readonly
                .apply_action(TextSurfaceAction::ClipboardRequest(operation))
                .handled
        );
    }
    for operation in [
        TextSurfaceHistoryOperation::Undo,
        TextSurfaceHistoryOperation::Redo,
    ] {
        assert!(
            !readonly
                .apply_action(TextSurfaceAction::HistoryRequest(operation))
                .handled
        );
    }
    assert!(
        readonly
            .apply_action(TextSurfaceAction::ClipboardRequest(
                TextSurfaceClipboardOperation::Copy,
            ))
            .handled
    );
}

#[test]
fn copy_and_cut_require_a_nonempty_selection() {
    let mut surface = surface(false);

    for operation in [
        TextSurfaceClipboardOperation::Copy,
        TextSurfaceClipboardOperation::Cut,
    ] {
        let outcome = surface.apply_action(TextSurfaceAction::ClipboardRequest(operation));
        assert!(!outcome.handled);
        assert!(outcome.events.is_empty());
    }
}

#[test]
fn readonly_surface_keeps_selection_but_rejects_mutation_requests() {
    let mut surface = surface(true);

    let selection = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 0, end: 3 },
    )));
    let edit = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Type(
        "blocked".to_string(),
    )));
    let paste = surface.apply_action(TextSurfaceAction::ClipboardRequest(
        TextSurfaceClipboardOperation::Paste,
    ));
    let undo = surface.apply_action(TextSurfaceAction::HistoryRequest(
        TextSurfaceHistoryOperation::Undo,
    ));

    assert!(selection.handled);
    assert_eq!(3, selection.state.text_area.selection.end);
    assert!(!edit.handled);
    assert_eq!("日本語 ⭐️", edit.state.text_area.value);
    assert!(!paste.handled);
    assert!(!undo.handled);
}

fn surface(readonly: bool) -> TextSurface {
    let text_area = TextArea::new("editor")
        .stable_state_id("surface.editor")
        .value("日本語 ⭐️")
        .readonly(readonly);
    TextSurface::new(
        TextSurfaceProps::new(
            text_area,
            vec![UiTextSpan::plain("日本語 ⭐️")],
            TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        )
        .accessibility_label("Editor"),
    )
}
