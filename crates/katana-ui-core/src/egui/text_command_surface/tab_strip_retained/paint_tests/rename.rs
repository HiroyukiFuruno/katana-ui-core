use super::*;
use crate::egui::text_command_surface::TabStripGroupPopupPresentation;

#[test]
fn requested_inline_rename_is_consumed_per_target() {
    let mut state = build_state();
    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"rename-once"),
        TabStripText::new("before"),
    )
    .capabilities(TabStripGroupCapabilities::new().renamable(true))
    .popup(
        TabStripGroupPopupPresentation::new()
            .rename_placeholder(TabStripText::new("名前"))
            .request_inline_rename(true),
    );
    let context = egui::Context::default();
    let mut operations = Vec::new();
    let mut x = 0.0;
    let mut reveal = false;
    let mut first_frame = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_group(
                ui,
                &group,
                "root-group-0".to_owned(),
                &mut x,
                ui.available_rect_before_wrap(),
                &mut operations,
                &mut reveal,
            )
            .expect("requested rename should render");
    });
    first_frame.textures_delta.clear();
    assert!(matches!(
        state.overlay,
        TabStripOverlayState::GroupPopup {
            rename: Some(_),
            ..
        }
    ));
    state.overlay = TabStripOverlayState::Closed;
    operations.clear();
    x = 0.0;
    let mut second_frame = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .render_group(
                ui,
                &group,
                "root-group-0".to_owned(),
                &mut x,
                ui.available_rect_before_wrap(),
                &mut operations,
                &mut reveal,
            )
            .expect("second frame should render");
    });
    second_frame.textures_delta.clear();
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
}

#[test]
fn invalid_inline_rename_request_is_rejected_before_overlay_creation() {
    let mut state = build_state();
    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"invalid-rename"),
        TabStripText::new("group"),
    )
    .popup(TabStripGroupPopupPresentation::new().request_inline_rename(true));
    let context = egui::Context::default();
    let mut operations = Vec::new();
    let mut x = 0.0;
    let mut reveal = false;
    let mut frame = context.run_ui(egui::RawInput::default(), |ui| {
        let error = state
            .render_group(
                ui,
                &group,
                "root-group-0".to_owned(),
                &mut x,
                ui.available_rect_before_wrap(),
                &mut operations,
                &mut reveal,
            )
            .expect_err("rename without placeholder must be rejected");
        assert!(matches!(
            error,
            TabStripRetainedError::InvalidInlineRenameRequest
        ));
    });
    frame.textures_delta.clear();
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
}
