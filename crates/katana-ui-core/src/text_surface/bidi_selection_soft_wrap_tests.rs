use super::{TextSurfaceGraphemeBox, TextSurfaceLayout};
use crate::render_model::UiRect;

#[test]
fn soft_wrapped_rtl_run_keeps_trailing_space_ltr_caret_affinity() {
    let text = "a\u{05d0}\u{05d1} ";
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "raster:bidi:soft-wrap-trailing-space",
        UiRect::new(0, 0, 30, 20),
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
                bounds: UiRect::new(10, 10, 10, 10),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 2,
                byte_start: 3,
                byte_end: 5,
                bounds: UiRect::new(0, 10, 10, 10),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 3,
                byte_start: 5,
                byte_end: 6,
                bounds: UiRect::new(20, 10, 10, 10),
            },
        ],
    );

    let hit = layout.hit_test_with_affinity(25, 15);

    assert_eq!(4, hit.caret_position());
    assert_eq!(UiRect::new(30, 10, 1, 10), layout.caret_rect_for_hit(hit));
}
