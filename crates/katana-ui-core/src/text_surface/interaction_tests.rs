use super::{
    TextSurface, TextSurfaceAction, TextSurfaceEvent, TextSurfaceGraphemeBox, TextSurfaceLayout,
    TextSurfaceLayoutAction, TextSurfacePoint, TextSurfaceProps, TextSurfaceViewport,
};
use crate::atom::{
    TextArea, TextAreaAction, TextAreaCompositionPhase, TextAreaEvent, TextAreaKeyChord,
};
use crate::render_model::{UiRect, UiTextSpan};
use crate::text_selection::UiTextSelectionRange;

const VIEWPORT_WIDTH: u32 = 320;
const VIEWPORT_HEIGHT: u32 = 120;
const GRAPHEME_WIDTH: u32 = 12;
const LINE_HEIGHT: u32 = 20;
const GRAPHEME_COUNT: u32 = 4;

#[test]
fn layout_converts_collapsed_and_selected_grapheme_ranges_without_scalar_boundaries() {
    let layout = layout();
    let star_start = "日本".len();
    let star_end = "日本⭐️".len();

    assert_eq!(
        UiTextSelectionRange::caret(2),
        layout.grapheme_range_for_byte_offsets(star_start, star_start)
    );
    assert_eq!(
        (star_start, star_end),
        layout.byte_offsets_for_grapheme_range(UiTextSelectionRange::new(2, 3))
    );
    assert_eq!(
        (star_end, star_start),
        layout.byte_offsets_for_grapheme_range(UiTextSelectionRange::new(3, 2))
    );
}

#[test]
fn pointer_drag_uses_layout_geometry_to_select_the_full_star_grapheme() {
    let mut surface = surface();
    let layout = layout();

    let press = surface.apply_layout_action(
        &layout,
        TextSurfaceLayoutAction::PointerPress {
            point: TextSurfacePoint::new(26, 8),
            extend_selection: false,
        },
    );
    let drag = surface.apply_layout_action(
        &layout,
        TextSurfaceLayoutAction::PointerDrag {
            point: TextSurfacePoint::new(34, 8),
        },
    );
    let release = surface.apply_layout_action(&layout, TextSurfaceLayoutAction::PointerRelease);

    assert!(press.handled);
    assert!(drag.handled);
    assert!(release.handled);
    assert_eq!("日本".len(), drag.state.text_area.selection.start);
    assert_eq!("日本⭐️".len(), drag.state.text_area.selection.end);
    let frame = surface.frame(&layout);
    assert_eq!(UiTextSelectionRange::new(2, 3), frame.selection.range);
    assert_eq!(
        vec![UiRect::new(
            GRAPHEME_WIDTH as i32 * 2,
            0,
            GRAPHEME_WIDTH,
            LINE_HEIGHT
        )],
        frame.selection.rects
    );
    assert!(matches!(
        drag.events.as_slice(),
        [TextSurfaceEvent::SelectionChanged {
            selection_start,
            selection_end,
        }] if *selection_start == "日本".len() && *selection_end == "日本⭐️".len()
    ));
}

#[test]
fn scroll_context_and_composition_cancel_stay_in_the_text_surface_contract() {
    let mut surface = surface();
    let layout = layout();
    let scroll = surface.apply_action(TextSurfaceAction::ScrollBy {
        delta_x: 3,
        delta_y: 17,
    });
    let context = surface.apply_action(TextSurfaceAction::RequestContextTarget {
        selection: UiTextSelectionRange::new(2, 3),
    });
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::composition(
        TextAreaCompositionPhase::Update,
        "かな",
        "かな".len(),
    )));
    let cancel = surface.apply_action(TextSurfaceAction::CancelComposition);
    let frame = surface.frame(&layout);

    assert!(matches!(
        scroll.events.as_slice(),
        [TextSurfaceEvent::Scrolled {
            scroll_x: 3,
            scroll_y: 17,
        }]
    ));
    assert!(matches!(
        context.events.as_slice(),
        [TextSurfaceEvent::ContextTargetRequested { selection }]
            if *selection == UiTextSelectionRange::new(2, 3)
    ));
    assert!(cancel.handled);
    assert!(matches!(
        cancel.events.as_slice(),
        [TextSurfaceEvent::CompositionCancelled]
    ));
    assert!(cancel.state.text_area.composition.is_none());
    assert_eq!(3, frame.viewport.scroll_x);
    assert_eq!(17, frame.viewport.scroll_y);
}

#[test]
fn synchronized_scroll_bounds_clamp_wheel_offsets_and_preserve_typed_events() {
    let mut surface = surface();
    surface.synchronize_scroll_bounds(UiRect::new(0, 0, 128, 72), UiRect::new(32, 0, 96, 40));

    let lower = surface.apply_action(TextSurfaceAction::ScrollBy {
        delta_x: 400,
        delta_y: 400,
    });
    let upper = surface.apply_action(TextSurfaceAction::ScrollBy {
        delta_x: -400,
        delta_y: -400,
    });

    assert!(matches!(
        lower.events.as_slice(),
        [TextSurfaceEvent::Scrolled {
            scroll_x: 32,
            scroll_y: 32,
        }]
    ));
    assert!(matches!(
        upper.events.as_slice(),
        [TextSurfaceEvent::Scrolled {
            scroll_x: 0,
            scroll_y: 0,
        }]
    ));
    assert_eq!(0, upper.state.scroll_x);
    assert_eq!(0, upper.state.scroll_y);
}

#[test]
fn key_chords_preserve_the_embedded_text_area_submit_and_newline_contract() {
    let mut surface = surface();

    let submit = surface.apply_action(TextSurfaceAction::Key(TextAreaKeyChord::enter()));
    let newline = surface.apply_action(TextSurfaceAction::Key(TextAreaKeyChord::shift_enter()));

    assert!(submit.handled);
    assert!(
        submit
            .events
            .contains(&TextSurfaceEvent::TextArea(TextAreaEvent::Submit(
                "日本⭐️A".to_string()
            )))
    );
    assert!(newline.handled);
    assert!(
        newline
            .events
            .contains(&TextSurfaceEvent::TextArea(TextAreaEvent::InsertNewline))
    );
    assert_eq!("日本⭐️A\n", surface.state().text_area.value);
}

fn surface() -> TextSurface {
    let text = "日本⭐️A";
    TextSurface::new(TextSurfaceProps::new(
        TextArea::new("editor").value(text).ime_enabled(true),
        vec![UiTextSpan::plain(text)],
        TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
    ))
}

fn layout() -> TextSurfaceLayout {
    let text = "日本⭐️A";
    let byte_offsets = [0, "日".len(), "日本".len(), "日本⭐️".len(), text.len()];
    TextSurfaceLayout::from_grapheme_boxes(
        "raster:interaction:1",
        UiRect::new(0, 0, GRAPHEME_WIDTH * GRAPHEME_COUNT, LINE_HEIGHT),
        text,
        byte_offsets
            .windows(2)
            .enumerate()
            .map(|(index, offsets)| TextSurfaceGraphemeBox {
                grapheme_index: index,
                byte_start: offsets[0],
                byte_end: offsets[1],
                bounds: UiRect::new(
                    i32::try_from(index).map_or(0, |value| value) * GRAPHEME_WIDTH as i32,
                    0,
                    GRAPHEME_WIDTH,
                    LINE_HEIGHT,
                ),
            })
            .collect(),
    )
}
