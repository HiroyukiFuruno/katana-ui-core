use super::{
    TextSurface, TextSurfaceGraphemeBox, TextSurfaceLayout, TextSurfaceLayoutAction,
    TextSurfacePoint, TextSurfaceProps, TextSurfaceViewport,
};
use crate::atom::TextArea;
use crate::render_model::{UiRect, UiTextSpan};

#[test]
fn pointer_selection_preserves_the_chosen_bidi_caret_affinity_in_the_frame() {
    let text = "a\u{05d0}\u{05d1}b";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("bidi-editor").value(text),
        vec![UiTextSpan::plain(text)],
        TextSurfaceViewport::new(0, 0, 40, 10),
    ));
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "raster:bidi:1",
        UiRect::new(0, 0, 40, 10),
        text,
        vec![
            TextSurfaceGraphemeBox {
                grapheme_index: 0,
                byte_start: 0,
                byte_end: 1,
                bounds: UiRect::new(0, 0, 10, 10),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 1,
                byte_start: 1,
                byte_end: 3,
                bounds: UiRect::new(20, 0, 10, 10),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 2,
                byte_start: 3,
                byte_end: 5,
                bounds: UiRect::new(10, 0, 10, 10),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 3,
                byte_start: 5,
                byte_end: 6,
                bounds: UiRect::new(30, 0, 10, 10),
            },
        ],
    );

    let outcome = surface.apply_layout_action(
        &layout,
        TextSurfaceLayoutAction::PointerPress {
            point: TextSurfacePoint::new(11, 5),
            extend_selection: false,
        },
    );

    assert!(outcome.handled);
    assert_eq!(5, outcome.state.text_area.caret);
    assert_eq!(
        UiRect::new(10, 0, 1, 10),
        surface.frame(&layout).selection.caret
    );
}

#[test]
fn a_single_rtl_grapheme_uses_resolved_direction_for_hit_testing() {
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "raster:bidi:single",
        UiRect::new(10, 0, 10, 10),
        "\u{05d0}",
        vec![TextSurfaceGraphemeBox {
            grapheme_index: 0,
            byte_start: 0,
            byte_end: 2,
            bounds: UiRect::new(10, 0, 10, 10),
        }],
    );

    assert_eq!(1, layout.hit_test(11, 5).caret_position());
    assert_eq!(0, layout.hit_test(19, 5).caret_position());
}

#[test]
fn pointer_drag_keeps_its_final_bidi_caret_affinity_after_release() {
    let text = "a\u{05d0}\u{05d1}b";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("bidi-drag-editor").value(text),
        vec![UiTextSpan::plain(text)],
        TextSurfaceViewport::new(0, 0, 40, 10),
    ));
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "raster:bidi:drag",
        UiRect::new(0, 0, 40, 10),
        text,
        vec![
            TextSurfaceGraphemeBox {
                grapheme_index: 0,
                byte_start: 0,
                byte_end: 1,
                bounds: UiRect::new(0, 0, 10, 10),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 1,
                byte_start: 1,
                byte_end: 3,
                bounds: UiRect::new(20, 0, 10, 10),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 2,
                byte_start: 3,
                byte_end: 5,
                bounds: UiRect::new(10, 0, 10, 10),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 3,
                byte_start: 5,
                byte_end: 6,
                bounds: UiRect::new(30, 0, 10, 10),
            },
        ],
    );

    let press = surface.apply_layout_action(
        &layout,
        TextSurfaceLayoutAction::PointerPress {
            point: TextSurfacePoint::new(1, 5),
            extend_selection: false,
        },
    );
    let drag = surface.apply_layout_action(
        &layout,
        TextSurfaceLayoutAction::PointerDrag {
            point: TextSurfacePoint::new(11, 5),
        },
    );
    let release = surface.apply_layout_action(&layout, TextSurfaceLayoutAction::PointerRelease);

    assert!(press.handled);
    assert!(drag.handled);
    assert!(release.handled);
    assert_eq!(5, drag.state.text_area.caret);
    assert_eq!(
        UiRect::new(10, 0, 1, 10),
        surface.frame(&layout).selection.caret
    );
}
