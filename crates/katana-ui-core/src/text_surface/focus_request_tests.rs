use super::{
    TextSurface, TextSurfaceFocusRequest, TextSurfaceFocusRequestAcknowledgement,
    TextSurfaceFocusRequestToken, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use crate::atom::{TextArea, TextAreaAction, TextAreaCompositionPhase};

#[test]
fn controlled_focus_request_tokens_do_not_forge_or_reset_interaction_state() {
    let text = "日本語 ⭐️";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("focus-request").value(text),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 120, 40).scroll_offset(4, 8),
    ));
    let _ = surface.apply_action(super::TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(super::TextSurfaceAction::TextArea(
        TextAreaAction::composition(TextAreaCompositionPhase::Update, "入力中⭐️", 3),
    ));
    let before = surface.state().clone();

    let mut first = TextSurfacePresentation::from_props(surface.props());
    first.focus_request = Some(TextSurfaceFocusRequest::new(
        TextSurfaceFocusRequestToken::new("focus-once"),
        true,
    ));
    assert!(surface.synchronize_presentation(first));
    assert_eq!(
        Some(TextSurfaceFocusRequestAcknowledgement {
            token: TextSurfaceFocusRequestToken::new("focus-once"),
            focused: true,
        }),
        surface.issue_controlled_focus_request()
    );
    assert_eq!(before.text_area, surface.state().text_area);
    assert_eq!(before.scroll_x, surface.state().scroll_x);
    assert_eq!(before.scroll_y, surface.state().scroll_y);
    assert_eq!(None, surface.issue_controlled_focus_request());

    let _ = surface.apply_action(super::TextSurfaceAction::SetFocus(false));
    let after_user_blur = surface.state().clone();
    assert_eq!(None, surface.issue_controlled_focus_request());
    assert_eq!(after_user_blur, surface.state().clone());

    let mut replacement = TextSurfacePresentation::from_props(surface.props());
    replacement.focus_request = Some(TextSurfaceFocusRequest::new(
        TextSurfaceFocusRequestToken::new("blur-once"),
        false,
    ));
    assert!(surface.synchronize_presentation(replacement));
    assert_eq!(
        Some(TextSurfaceFocusRequestAcknowledgement {
            token: TextSurfaceFocusRequestToken::new("blur-once"),
            focused: false,
        }),
        surface.issue_controlled_focus_request()
    );
    assert_eq!(after_user_blur.text_area, surface.state().text_area);
    assert_eq!(after_user_blur.scroll_x, surface.state().scroll_x);
    assert_eq!(after_user_blur.scroll_y, surface.state().scroll_y);
}
