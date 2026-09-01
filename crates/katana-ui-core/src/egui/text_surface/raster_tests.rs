use super::*;
use crate::atom::{TextArea, TextAreaAction, TextAreaCompositionPhase};
use crate::component::ComponentAction;
use crate::interaction::UiAction;
use crate::text_surface::{TextSurface, TextSurfaceAction, TextSurfaceProps, TextSurfaceViewport};

const TEST_VIEWPORT_HEIGHT: u32 = 20;

fn composing_surface(value: &str, start: usize, end: usize) -> TextSurface {
    let mut text_area = TextArea::new("test").value(value);
    let target = text_area.state_id().clone();
    let selection = text_area.apply_action(&UiAction::cursor_selection(target, end, start, end));
    assert!(selection.handled);
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        text_area,
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_VIEWPORT_HEIGHT),
    ));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::composition(
        TextAreaCompositionPhase::Update,
        "X",
        1,
    )));
    surface
}

#[test]
fn composed_text_orders_reversed_selection_bounds_before_replacing_text() {
    let surface = composing_surface("abcd", 3, 1);
    assert_eq!("aXd", composed_text(&surface));
}

#[test]
fn composed_text_fails_closed_for_non_character_boundary_before_and_after() {
    let before_invalid = composing_surface("あ", 1, 2);
    assert_eq!("あ", composed_text(&before_invalid));

    let after_invalid = composing_surface("あ", 0, 1);
    assert_eq!("あ", composed_text(&after_invalid));
}
