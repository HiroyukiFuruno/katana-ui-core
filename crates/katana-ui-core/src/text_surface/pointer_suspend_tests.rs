use super::{TextSurface, TextSurfacePoint, TextSurfaceProps, TextSurfaceViewport};
use crate::atom::{TextArea, TextAreaSelection};
use crate::render_model::UiTextSpan;

#[test]
fn suspending_pointer_interaction_clears_only_the_pointer_anchor() {
    let value = "pointer state";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("editor").value(value),
        vec![UiTextSpan::plain(value)],
        TextSurfaceViewport::new(0, 0, 320, 120),
    ));
    surface.state.text_area.focused = true;
    surface.state.text_area.selection = TextAreaSelection { start: 2, end: 7 };
    surface.state.pointer_anchor = Some(TextSurfacePoint::new(12, 8));
    let before = surface.state.text_area.clone();

    surface.suspend_pointer_interaction();

    assert_eq!(None, surface.state.pointer_anchor);
    assert_eq!(before, surface.state.text_area);
    assert_eq!(value, surface.state.text_area.value);
    assert_eq!(before.selection, surface.state.text_area.selection);

    surface.suspend_pointer_interaction();
    assert_eq!(None, surface.state.pointer_anchor);
    assert_eq!(before, surface.state.text_area);
}
