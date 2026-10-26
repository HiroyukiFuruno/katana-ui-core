use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootFactory, TabStripProjection, TabStripProjectionLease,
    TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

fn token() -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("raster-host").value("host"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"raster-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("raster-host-text")),
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().unwrap(),
    )
    .unwrap()
}

pub fn root_with_projection(projection: TabStripProjection) -> EguiTextCommandSurfaceHostRoot {
    EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            EguiTextCommandSurfaceHostProjectionLease::new(token(), |_context| Ok(None))
                .with_tab_strip(TabStripProjectionLease::new(projection)),
        )
        .unwrap()
}
