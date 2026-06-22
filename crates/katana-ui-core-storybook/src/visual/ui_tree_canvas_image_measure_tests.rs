use super::*;
use crate::test_assert::KucTestExpect;
use crate::visual::text::TextRenderer;
use crate::visual::ui_tree_canvas_scroll_measure;
use crate::visual::ui_tree_canvas_text::UiTreeTextContext;
use crate::visual::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;
use katana_ui_core::facade::UiCoreFacade;

#[test]
fn image_surface_scroll_measure_uses_logical_extent() {
    let theme = ThemeSnapshot::dark();
    let image: UiNode = ImageSurface::from_rgba(
        "retina",
        "fingerprint",
        80,
        40,
        [255, 0, 0, 255].repeat(80 * 40),
    )
    .kuc_expect("valid image surface")
    .content_scale(200)
    .into();

    let measured = ui_tree_canvas_scroll_measure::measured_node_height(
        &image,
        UiTreeTextContext {
            text: text_renderer("body"),
            export_text: text_renderer("body"),
            code_text: text_renderer("code"),
            palette: UiTreeCanvasPalette::from_theme(&theme),
            typography: UiTreeDocumentTypography::default(),
        },
        0,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 80,
            height: 40,
            scroll_y: 0.0,
        },
    );

    assert_eq!(20, measured);
}

fn text_renderer(role: &str) -> &'static TextRenderer {
    let facade = Box::leak(Box::new(UiCoreFacade::default()));
    Box::leak(Box::new(TextRenderer::load(facade, role)))
}
