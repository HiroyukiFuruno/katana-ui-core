use super::{
    TextSurface, TextSurfaceAction, TextSurfaceAnnotation, TextSurfaceAnnotationStyle,
    TextSurfaceEvent, TextSurfaceLayout, TextSurfaceProps, TextSurfaceViewport,
};
use crate::atom::{TextArea, TextAreaAction, TextAreaCompositionPhase, TextAreaSelection};
use crate::render_model::{UiRect, UiTextSpan};

const TEST_VIEWPORT_WIDTH: u32 = 640;
const TEST_VIEWPORT_HEIGHT: u32 = 320;
const TEST_GRAPHEME_WIDTH: u32 = 10;
const TEST_LINE_HEIGHT: u32 = 20;

fn surface(readonly: bool) -> TextSurface {
    let text_area = TextArea::new("editor")
        .stable_state_id("surface.editor")
        .value("日本語 ⭐️")
        .readonly(readonly);
    TextSurface::new(
        TextSurfaceProps::new(
            text_area,
            vec![UiTextSpan::plain("日本語 ⭐️")],
            TextSurfaceViewport::new(0, 0, TEST_VIEWPORT_WIDTH, TEST_VIEWPORT_HEIGHT),
        )
        .accessibility_label("Editor"),
    )
}

#[test]
fn text_area_actions_are_forwarded_without_replacing_the_existing_state_machine() {
    let mut surface = surface(false);

    let outcome = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Type(
        " input".to_string(),
    )));

    assert!(outcome.handled);
    assert!(matches!(
        outcome.events.as_slice(),
        [TextSurfaceEvent::TextArea(crate::atom::TextAreaEvent::TextInput(value)), ..]
            if value == " input"
    ));
    assert_eq!("日本語 ⭐️ input", outcome.state.text_area.value);
}

#[test]
fn frame_record_preserves_the_single_layout_identity_and_accessibility_state() {
    let mut surface = surface(false);
    let focus = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let layout = TextSurfaceLayout::new("raster:editor:1", UiRect::new(4, 8, 320, 120));

    let frame = surface.frame(&layout);

    assert!(focus.handled);
    assert_eq!("raster:editor:1", frame.layout_identity);
    assert_eq!(layout.content_bounds, frame.content_bounds);
    assert!(frame.accessibility.root.focused);
    assert!(frame.accessibility.root.editable);
    assert_eq!("Editor", frame.accessibility.root.label.as_str());
}

#[test]
fn preedit_frame_requires_matching_composed_raster_layout_and_uses_its_geometry() {
    let text_area = TextArea::new("editor")
        .stable_state_id("surface.editor")
        .value("A日本")
        .ime_enabled(true);
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        text_area,
        vec![UiTextSpan::plain("A日本")],
        TextSurfaceViewport::new(0, 0, TEST_VIEWPORT_WIDTH, TEST_VIEWPORT_HEIGHT),
    ));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 1, end: 1 },
    )));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::composition(
        TextAreaCompositionPhase::Update,
        "⭐️",
        "⭐️".len(),
    )));
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "raster:composition:1",
        UiRect::new(0, 0, 80, 20),
        "A⭐️日本",
        vec![
            grapheme_box(0, 0, 1, 0),
            grapheme_box(1, 1, "⭐️".len() + 1, 10),
            grapheme_box(2, "⭐️".len() + 1, "A⭐️日".len(), 30),
            grapheme_box(3, "A⭐️日".len(), "A⭐️日本".len(), 40),
        ],
    )
    .with_composition(1, 1, "⭐️", "⭐️".len());

    let frame = surface.frame(&layout);

    assert!(matches!(
        frame.preedit,
        Some(ref preedit)
            if preedit.text == "⭐️"
                && preedit.range == crate::text_selection::UiTextSelectionRange::new(1, 2)
                && !preedit.rects.is_empty()
    ));
}

#[test]
fn annotations_use_one_layout_and_sort_by_priority_then_input_order() {
    let text_area = TextArea::new("editor").value("ab");
    let surface = TextSurface::new(
        TextSurfaceProps::new(
            text_area,
            vec![UiTextSpan::plain("ab")],
            TextSurfaceViewport::new(0, 0, TEST_VIEWPORT_WIDTH, TEST_VIEWPORT_HEIGHT),
        )
        .annotation(
            TextSurfaceAnnotation::new(
                "second",
                crate::text_selection::UiTextSelectionRange::new(1, 2),
                "secondary",
                TextSurfaceAnnotationStyle::Underline,
            )
            .priority(2),
        )
        .annotation(
            TextSurfaceAnnotation::new(
                "first",
                crate::text_selection::UiTextSelectionRange::new(0, 1),
                "primary",
                TextSurfaceAnnotationStyle::Fill,
            )
            .priority(2),
        )
        .annotation(TextSurfaceAnnotation::new(
            "low",
            crate::text_selection::UiTextSelectionRange::new(0, 2),
            "low",
            TextSurfaceAnnotationStyle::Outline,
        )),
    );
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "raster:annotation:1",
        UiRect::new(0, 0, 20, TEST_LINE_HEIGHT),
        "ab",
        vec![grapheme_box(0, 0, 1, 0), grapheme_box(1, 1, 2, 10)],
    );

    let frame = surface.frame(&layout);

    assert_eq!(
        vec!["second", "first", "low"],
        frame
            .annotations
            .iter()
            .map(|annotation| annotation.id.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![UiRect::new(10, 0, 10, TEST_LINE_HEIGHT)],
        frame.annotations[0].rects
    );
}

#[test]
fn controlled_value_synchronization_updates_readonly_surface_without_user_events() {
    let mut surface = surface(true);

    assert!(surface.synchronize_value("更新後の日本語 ⭐️"));
    assert_eq!("更新後の日本語 ⭐️", surface.state().text_area.value);
    assert!(surface.state().text_area.readonly);
    assert!(!surface.synchronize_value("更新後の日本語 ⭐️"));
}

fn grapheme_box(
    index: usize,
    byte_start: usize,
    byte_end: usize,
    x: i32,
) -> crate::text_surface::TextSurfaceGraphemeBox {
    crate::text_surface::TextSurfaceGraphemeBox {
        grapheme_index: index,
        byte_start,
        byte_end,
        bounds: UiRect::new(x, 0, TEST_GRAPHEME_WIDTH, TEST_LINE_HEIGHT),
    }
}
