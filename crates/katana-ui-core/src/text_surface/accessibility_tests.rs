use super::{
    TextSurface, TextSurfaceAccessibilityActionKind, TextSurfaceAccessibilityLabels,
    TextSurfaceAccessibilityTarget, TextSurfaceAction, TextSurfaceGraphemeBox, TextSurfaceGutter,
    TextSurfaceGutterRow, TextSurfaceLayout, TextSurfaceProps, TextSurfaceViewport,
};
use crate::accessibility::AccessibilityRole;
use crate::atom::{TextArea, TextAreaAction, TextAreaSelection};
use crate::render_model::{UiRect, UiTextSpan};
use crate::text_selection::UiTextSelectionRange;
use unicode_segmentation::UnicodeSegmentation;

const VIEWPORT_WIDTH: u32 = 320;
const VIEWPORT_HEIGHT: u32 = 120;
const LINE_HEIGHT: u32 = 20;
const GRAPHEME_WIDTH: u32 = 12;
const GUTTER_WIDTH: u32 = 24;
const TEXT_ORIGIN_X: i32 = GUTTER_WIDTH as i32;
const CONTENT_WIDTH: u32 = 96;

#[test]
fn accessibility_tree_projects_selection_gutter_context_and_readonly_actions() {
    let text = "日本⭐️A";
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("editor").value(text).readonly(true),
            vec![UiTextSpan::plain(text)],
            TextSurfaceViewport::new(TEXT_ORIGIN_X, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        )
        .accessibility_label("編集領域")
        .context_target_label("選択範囲のコンテキスト")
        .accessibility_actions(action_labels())
        .gutter(
            TextSurfaceGutter::new(GUTTER_WIDTH).row(
                TextSurfaceGutterRow::new(0, "1")
                    .marker_id("breakpoint")
                    .accessibility_label("1 行目"),
            ),
        ),
    );
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection {
            start: "日本".len(),
            end: "日本⭐️".len(),
        },
    )));

    let frame = surface.frame(&layout(text));
    let root = &frame.accessibility.root;

    assert_eq!(AccessibilityRole::Input, root.role);
    assert_eq!("編集領域", root.label.as_str());
    assert!(root.focused);
    assert!(!root.editable);
    assert!(root.readonly);
    assert_eq!(Some(UiTextSelectionRange::new(2, 3)), root.selection);
    assert_eq!(2, frame.accessibility.gutter_targets.len());
    assert!(frame.accessibility.gutter_targets.iter().all(|node| {
        node.bounds == UiRect::new(0, 0, GUTTER_WIDTH, LINE_HEIGHT)
            && node.role == AccessibilityRole::Button
    }));
    assert!(matches!(
        frame.accessibility.gutter_targets[1].target,
        TextSurfaceAccessibilityTarget::GutterMarker {
            logical_row: 0,
            ref marker_id
        } if marker_id == "breakpoint"
    ));
    let context_target = frame.accessibility.context_target.as_ref();
    assert!(context_target.is_some());
    let Some(context_target) = context_target else {
        return;
    };
    assert_eq!(
        TextSurfaceAccessibilityTarget::ContextSelection,
        context_target.target
    );
    assert_eq!(root.selection, context_target.selection);
    assert_eq!(root.bounds, context_target.bounds);
    assert_eq!(
        vec![
            (TextSurfaceAccessibilityActionKind::Copy, true),
            (TextSurfaceAccessibilityActionKind::Cut, false),
            (TextSurfaceAccessibilityActionKind::Paste, false),
            (TextSurfaceAccessibilityActionKind::Undo, false),
            (TextSurfaceAccessibilityActionKind::Redo, false),
        ],
        frame
            .accessibility
            .actions
            .iter()
            .map(|action| (action.kind, action.enabled))
            .collect::<Vec<_>>(),
    );
}

#[test]
fn accessibility_tree_exposes_a_disabled_reason_without_inventing_labels() {
    let text = "⭐️";
    let surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("editor").value(text).disabled(true),
            vec![UiTextSpan::plain(text)],
            TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        )
        .accessibility_label("編集領域")
        .context_target_label("選択範囲のコンテキスト")
        .accessibility_actions(action_labels())
        .disabled_reason("読み込み中"),
    );

    let frame = surface.frame(&layout(text));

    assert!(frame.accessibility.root.disabled);
    assert_eq!(
        Some("読み込み中"),
        frame.accessibility.root.disabled_reason.as_deref()
    );
    assert!(
        frame
            .accessibility
            .actions
            .iter()
            .all(|action| !action.enabled)
    );
    assert!(
        frame
            .accessibility
            .context_target
            .as_ref()
            .is_some_and(|target| target.disabled)
    );
}

#[test]
fn explicit_surface_bounds_drive_root_and_context_accessibility_bounds() {
    let text = "日本語 ⭐️";
    let surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("editor").value(text),
            vec![UiTextSpan::plain(text)],
            TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        )
        .accessibility_label("編集領域")
        .context_target_label("選択範囲のコンテキスト"),
    );
    let surface_bounds = UiRect::new(8, 12, 240, 96);
    let viewport_bounds = UiRect::new(32, 12, 216, 96);
    let frame = surface.frame_with_bounds(&layout(text), surface_bounds, viewport_bounds);

    assert_eq!(surface_bounds, frame.surface_bounds);
    assert_eq!(viewport_bounds, frame.viewport_bounds);
    assert_eq!(surface_bounds, frame.accessibility.root.bounds);
    assert_eq!(
        Some(surface_bounds),
        frame
            .accessibility
            .context_target
            .map(|target| target.bounds),
    );
}

fn action_labels() -> TextSurfaceAccessibilityLabels {
    TextSurfaceAccessibilityLabels::new()
        .with_label(TextSurfaceAccessibilityActionKind::Copy, "コピー")
        .with_label(TextSurfaceAccessibilityActionKind::Cut, "切り取り")
        .with_label(TextSurfaceAccessibilityActionKind::Paste, "貼り付け")
        .with_label(TextSurfaceAccessibilityActionKind::Undo, "元に戻す")
        .with_label(TextSurfaceAccessibilityActionKind::Redo, "やり直す")
}

fn layout(text: &str) -> TextSurfaceLayout {
    TextSurfaceLayout::from_grapheme_boxes(
        "raster:accessibility:1",
        UiRect::new(TEXT_ORIGIN_X, 0, CONTENT_WIDTH, LINE_HEIGHT),
        text,
        text.grapheme_indices(true)
            .enumerate()
            .map(|(index, (byte_start, grapheme))| TextSurfaceGraphemeBox {
                grapheme_index: index,
                byte_start,
                byte_end: byte_start + grapheme.len(),
                bounds: UiRect::new(
                    TEXT_ORIGIN_X
                        + i32::try_from(index).map_or(0, |value| value) * GRAPHEME_WIDTH as i32,
                    0,
                    GRAPHEME_WIDTH,
                    LINE_HEIGHT,
                ),
            })
            .collect(),
    )
}
