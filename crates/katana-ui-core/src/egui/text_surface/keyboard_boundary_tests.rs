use super::*;
use crate::render_model::UiRect;
use crate::text_surface::TextSurfaceGraphemeBox;

#[test]
fn home_moves_a_nonempty_selection_to_the_logical_line_start() {
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "keyboard-boundary",
        UiRect::new(0, 0, 100, 20),
        "ab",
        (0..2)
            .map(|index| TextSurfaceGraphemeBox {
                grapheme_index: index,
                byte_start: index,
                byte_end: index + 1,
                bounds: UiRect::new(index as i32 * 10, 0, 10, 20),
            })
            .collect(),
    );
    assert_eq!(
        target_grapheme(&layout, UiTextSelectionRange::new(0, 2), egui::Key::Home),
        0
    );
}

#[test]
fn vertical_navigation_keeps_the_caret_when_no_visual_row_exists() {
    let layout = TextSurfaceLayout::new("no-rows", UiRect::new(0, 0, 100, 20));
    for down in [false, true] {
        assert_eq!(
            vertical_target_grapheme(&layout, 0, UiRect::new(0, 0, 1, 20), down),
            0
        );
    }
}
