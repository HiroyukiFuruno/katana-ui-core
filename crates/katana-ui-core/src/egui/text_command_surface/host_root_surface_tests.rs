use super::*;
use crate::atom::TextArea;
use crate::egui::text_command_surface::{EguiTextCommandSurfaceAdapter, TextCommandSurfaceStyle};
use crate::molecule::command_chrome::{
    CommandChromeFamilyId, CommandChromeToolbar, FloatingCommandToolbarVisibility,
};
use crate::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

#[test]
fn materialized_floating_toolbar_accepts_a_new_opaque_family_after_a_real_frame() {
    let value = "opaque family";
    let props = TextSurfaceProps::new(
        TextArea::new("host-floating-family").value(value),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 320, 180),
    );
    let mut presentation = TextSurfacePresentation::from_props(&props);
    presentation.selection_start = 0;
    presentation.selection_end = value.len();
    let mut text = TextSurface::new(props);
    assert!(text.synchronize_presentation(presentation));
    let mut surface = EguiTextCommandSurface::new(text).with_floating_toolbar(
        CommandChromeToolbar::new(),
        FloatingCommandToolbarVisibility::Visible,
    );
    let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
        crate::text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("adapter");
    let style = TextCommandSurfaceStyle::standard().expect("style");
    let context = egui::Context::default();
    let mut output = None;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 180.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            output =
                Some(adapter.show_with_tab_strip(ui, &mut surface, &style, None, None, None, None));
        },
    );
    output
        .expect("real frame ran")
        .expect("real frame materialized the floating toolbar");
    assert!(surface.floating_toolbar().is_some());

    let family = CommandChromeFamilyId::new("host-floating-family-next");
    apply_command_families(
        &mut surface,
        &EguiTextCommandSurfaceCommandFamilyProjection::new(None, Some(family.clone())),
    );

    assert_eq!(
        surface
            .floating_toolbar()
            .expect("materialized floating toolbar")
            .command_family_id(),
        &family
    );
}
