use super::*;
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

const VIEWPORT_WIDTH: u32 = 320;
const VIEWPORT_HEIGHT: u32 = 180;

pub(super) fn presentation() -> EguiTextCommandSurfacePresentation {
    let surface = TextSurface::new(TextSurfaceProps::new(
        katana_ui_core::atom::TextArea::new("host-root-unit").value("opaque text"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
    ));
    EguiTextCommandSurfacePresentation {
        text_state_id: None,
        text: TextSurfacePresentation::from_props(surface.props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    }
}

#[path = "host_root_factory_tests.rs"]
mod factory_tests;
#[path = "host_root_lease_tests.rs"]
mod lease_tests;
