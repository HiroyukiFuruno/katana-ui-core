use super::*;
use crate::render_model::UiImageSurfaceProps;

fn lease() -> EditorViewportProjectionLease {
    EditorViewportProjectionLease::new(
        UiImageSurfaceProps::new("preview", 2, 1, vec![1, 2, 3, 255, 4, 5, 6, 255])
            .expect("valid preview")
            .accessibility_label("Preview"),
    )
}

#[test]
fn split_layout_is_non_overlapping_and_preview_paints_real_rgba() {
    let context = egui::Context::default();
    let mut lease = lease();
    let mut texture = None;
    let mut observed = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        let body = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 200.0));
        let layout = layout(ui, body, &mut lease);
        assert!(layout.document.max.x < layout.preview.min.x);
        assert_eq!(layout.document.min, body.min);
        assert_eq!(layout.preview.max, body.max);
        observed = Some(render_preview(
            ui,
            layout.preview,
            &lease,
            &mut texture,
            [7, 8, 9, 255],
        ));
    });
    output.textures_delta.clear();
    let observed = observed.expect("preview output");
    assert_eq!(observed.paint_plan.operations.len(), 2);
    let TextSurfacePaintOperationKind::Texture { texture, .. } =
        &observed.paint_plan.operations[1].kind
    else {
        panic!("preview must retain a texture operation");
    };
    assert_eq!(texture.rgba_pixels, vec![1, 2, 3, 255, 4, 5, 6, 255]);
}

#[test]
fn fit_modes_preserve_their_generic_geometry_contract() {
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 100.0));
    assert_eq!(
        fitted_rect(bounds, 200, 100, UiImageSurfaceFit::Stretch),
        bounds
    );
    assert_eq!(
        fitted_rect(bounds, 200, 100, UiImageSurfaceFit::Contain).size(),
        egui::vec2(100.0, 50.0)
    );
    assert_eq!(
        fitted_rect(bounds, 200, 100, UiImageSurfaceFit::Cover).size(),
        egui::vec2(200.0, 100.0)
    );
    assert_eq!(
        fitted_rect(bounds, 40, 20, UiImageSurfaceFit::Original).size(),
        egui::vec2(40.0, 20.0)
    );
}

#[test]
fn split_handle_routes_drag_and_keyboard_updates_through_the_real_egui_frame() {
    let context = egui::Context::default();
    let body = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 200.0));
    let mut lease = lease();
    let handle = egui::pos2(200.0, 50.0);

    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            events: vec![egui::Event::PointerMoved(handle)],
            screen_rect: Some(body),
            ..egui::RawInput::default()
        },
        |ui| {
            let _ = layout(ui, body, &mut lease);
        },
    );

    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            events: vec![egui::Event::PointerButton {
                pos: handle,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
            screen_rect: Some(body),
            ..egui::RawInput::default()
        },
        |ui| {
            let _ = layout(ui, body, &mut lease);
        },
    );
    let before_drag = lease.split_ratio_percent;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            events: vec![egui::Event::PointerMoved(egui::pos2(240.0, 50.0))],
            ..egui::RawInput::default()
        },
        |ui| {
            let _ = layout(ui, body, &mut lease);
        },
    );
    assert_ne!(lease.split_ratio_percent, before_drag);

    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            events: vec![egui::Event::PointerButton {
                pos: egui::pos2(240.0, 50.0),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            let _ = layout(ui, body, &mut lease);
        },
    );
    let before_keyboard = lease.split_ratio_percent;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::ArrowLeft,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            let _ = layout(ui, body, &mut lease);
        },
    );
    assert_eq!(lease.split_ratio_percent, before_keyboard - 1);
}

#[test]
fn split_handle_arrow_right_from_focused_handle_increments_ratio_in_real_egui_input() {
    let context = egui::Context::default();
    let body = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 200.0));
    let mut lease = lease();
    let handle = egui::pos2(200.0, 50.0);

    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            events: vec![egui::Event::PointerMoved(handle)],
            screen_rect: Some(body),
            ..egui::RawInput::default()
        },
        |ui| {
            let _ = layout(ui, body, &mut lease);
        },
    );
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            events: vec![
                egui::Event::PointerButton {
                    pos: handle,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos: handle,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            screen_rect: Some(body),
            ..egui::RawInput::default()
        },
        |ui| {
            let _ = layout(ui, body, &mut lease);
        },
    );

    let before_focus = lease.split_ratio_percent;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(body),
            events: vec![egui::Event::Key {
                key: egui::Key::ArrowRight,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            let _ = layout(ui, body, &mut lease);
        },
    );

    assert_eq!(lease.split_ratio_percent, before_focus + 1);
}

#[test]
fn preview_texture_is_freed_when_the_real_preview_identity_changes() {
    let context = egui::Context::default();
    let mut first = lease();
    let mut texture = None;
    crate::egui::run_ui_discard(&context, egui::RawInput::default(), |ui| {
        let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 100.0));
        let _ = render_preview(ui, rect, &first, &mut texture, [0, 0, 0, 255]);
    });
    let old = texture.as_ref().expect("first texture").1;
    let changed =
        crate::render_model::UiImageSurfaceProps::new("changed-preview", 1, 1, vec![9, 8, 7, 255])
            .expect("changed preview");
    first.preview = changed;
    crate::egui::run_ui_discard(&context, egui::RawInput::default(), |ui| {
        let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 100.0));
        let _ = render_preview(ui, rect, &first, &mut texture, [0, 0, 0, 255]);
    });
    assert_ne!(texture.as_ref().expect("changed texture").1, old);
}
