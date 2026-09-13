use super::draw_span_background;
use crate::raster_host::Canvas;
use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use katana_ui_core::render_model::{UiNode, UiNodeKind, UiTextSpanStyle};
use katana_ui_core::theme::{ColorToken, ThemeSnapshot};

#[test]
fn text_highlight_theme_token_preserves_legacy_default_and_changes_actual_paint() {
    for mut theme in [ThemeSnapshot::light(), ThemeSnapshot::dark()] {
        let fallback = UiTreeCanvasPalette::from_theme(&theme);
        assert_eq!(0x4a4620, fallback.text_highlight_background);
        theme.colors.push(ColorToken {
            name: "text-highlight-background".to_owned(),
            rgba: [10, 20, 30, 255],
        });
        let custom = UiTreeCanvasPalette::from_theme(&theme);
        assert_eq!(0x0a141e, custom.text_highlight_background);
        let metrics = UiTreeTextMetrics::for_node(&UiNode::new(UiNodeKind::Text, "mark"));
        for palette in [fallback, custom] {
            let mut canvas = Canvas::new(24, 32, palette.background);
            draw_span_background(
                &mut canvas,
                0,
                0.0,
                20,
                UiTextSpanStyle {
                    highlight: true,
                    ..UiTextSpanStyle::default()
                },
                palette,
                metrics,
                0.0,
            );
            assert_eq!(palette.text_highlight_background, canvas.pixels()[0]);
            assert_eq!(palette.background, canvas.pixels()[23]);
        }
    }
}
