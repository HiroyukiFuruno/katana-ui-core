use super::draw_span_background;
use crate::raster_host::Canvas;
use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use katana_ui_core::render_model::{UiNode, UiNodeKind, UiTextSpanStyle};
use katana_ui_core::theme::{ColorToken, ThemeSnapshot};

#[test]
fn text_highlight_theme_token_blends_its_rgba_over_light_and_dark_canvas_pixels() {
    for (mut theme, background, expected_alpha_60) in [
        (ThemeSnapshot::light(), 0xe0e8f0, 0xad_b6_be),
        (ThemeSnapshot::dark(), 0x102030, 0x0e_1d_2b),
    ] {
        let fallback = UiTreeCanvasPalette::from_theme(&theme);
        assert_eq!(0x4a4620, fallback.text_highlight_background);
        let metrics = UiTreeTextMetrics::for_node(&UiNode::new(UiNodeKind::Text, "mark"));
        for (alpha, expected) in [(0, background), (60, expected_alpha_60), (255, 0x0a141e)] {
            theme
                .colors
                .retain(|token| token.name != "text-highlight-background");
            theme.colors.push(ColorToken {
                name: "text-highlight-background".to_owned(),
                rgba: [10, 20, 30, alpha],
            });
            let palette = UiTreeCanvasPalette::from_theme(&theme);
            assert_eq!(0x0a141e, palette.text_highlight_background);
            assert_eq!(alpha, palette.text_highlight_alpha);
            let mut canvas = Canvas::new(24, 32, background);
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
            assert_eq!(expected, canvas.pixels()[0]);
            assert_eq!(background, canvas.pixels()[23]);
        }
    }
}
