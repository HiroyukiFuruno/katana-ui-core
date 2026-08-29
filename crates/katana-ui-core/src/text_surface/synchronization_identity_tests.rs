use super::{TextSurface, TextSurfaceAction, TextSurfaceProps, TextSurfaceViewport};
use crate::atom::{TextArea, TextAreaAction, TextAreaCompositionPhase, TextAreaSelection};

#[test]
fn controlled_identity_sync_preserves_text_surface_interaction_state_without_events() {
    let value = "\u{65e5}\u{672c}\u{8a9e} \u{2b50}\u{fe0f}";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("surface")
            .stable_state_id("surface.before")
            .value(value),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 48, 16),
    ));
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection {
            start: 3,
            end: value.len(),
        },
    )));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::composition(
        TextAreaCompositionPhase::Update,
        "\u{5165}\u{529b}\u{4e2d} \u{2b50}\u{fe0f}",
        "\u{5165}".len(),
    )));
    let _ = surface.apply_action(TextSurfaceAction::ScrollBy {
        delta_x: 3,
        delta_y: 7,
    });
    let before_state = surface.state().clone();
    let before_events = surface.props().text_area.events().to_vec();

    assert!(surface.synchronize_state_id("surface.after"));
    assert_eq!(
        surface.props().text_area.state_id().as_str(),
        "surface.after"
    );
    assert_eq!(surface.state().text_area.state_id.as_str(), "surface.after");
    assert_eq!(surface.state().text_area.value, value);
    assert_eq!(
        surface.state().text_area.selection,
        before_state.text_area.selection
    );
    assert_eq!(
        surface.state().text_area.caret,
        before_state.text_area.caret
    );
    assert_eq!(
        surface.state().text_area.focused,
        before_state.text_area.focused
    );
    assert_eq!(
        surface.state().text_area.composition,
        before_state.text_area.composition
    );
    assert_eq!(surface.state().scroll_x, before_state.scroll_x);
    assert_eq!(surface.state().scroll_y, before_state.scroll_y);
    assert_eq!(surface.props().text_area.events(), before_events);
}

#[test]
fn controlled_identity_sync_is_a_noop_for_the_same_identity() {
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("surface").stable_state_id("surface.identity"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 1, 1),
    ));
    let before_state = surface.state().clone();
    let before_events = surface.props().text_area.events().to_vec();

    assert!(!surface.synchronize_state_id("surface.identity"));
    assert_eq!(surface.state(), &before_state);
    assert_eq!(surface.props().text_area.events(), before_events);
}
