use super::span_style::span_color;
use super::{UiTreeTextLineContext, UiTreeTextLines};
use crate::raster_host::canvas::Canvas;
use crate::raster_host::text::TextRenderer;
use crate::raster_host::ui_tree_canvas_text_line_width::{
    SpanTextRenderers, preserves_whitespace, span_part_width, span_visible_part_bounds,
    whitespace_width,
};
use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use crate::raster_host::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::atom::Text;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiDimension, UiNode, UiTextSpan, UiTextSpanStyle};
use katana_ui_core::theme::{ColorToken, ThemeSnapshot};

const TEST_BACKGROUND: u32 = 0x151515;
const CURRENT_HIGHLIGHT_BACKGROUND: u32 = 0x654100;
const STRIKE_COLOR: u32 = 0xcc6633;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;

#[path = "ui_tree_canvas_text_lines_decoration_tests.rs"]
mod decoration_tests;
#[path = "ui_tree_canvas_text_lines_fractional_tests.rs"]
mod fractional_tests;
#[path = "ui_tree_canvas_text_lines_html_tests.rs"]
mod html_tests;
#[path = "ui_tree_canvas_text_lines_spacing_tests.rs"]
mod spacing_tests;
#[path = "ui_tree_canvas_text_lines_test_support.rs"]
mod support;
#[cfg(test)]
pub(super) use support::*;

#[test]
fn span_lines_cover_bold_and_clipped_rendering_paths() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node: UiNode = Text::new("Important")
        .text_role("alert")
        .text_spans(vec![UiTextSpan::plain("Important")])
        .into();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let palette = crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );
    let mut canvas = Canvas::new(320, 120, TEST_BACKGROUND);

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area: UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 320,
                height: 120,
                scroll_y: 0.0,
            },
            palette,
            metrics,
        },
        0,
        8,
    );
    assert!(
        canvas
            .pixels()
            .iter()
            .any(|pixel| *pixel != TEST_BACKGROUND),
        "bold alert span should draw visible pixels"
    );

    let mut clipped = Canvas::new(320, 120, TEST_BACKGROUND);
    UiTreeTextLines::draw_spans(
        &mut clipped,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area: UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 320,
                height: 1,
                scroll_y: metrics.line_height as f32,
            },
            palette,
            metrics,
        },
        0,
        8,
    );
    assert!(
        clipped
            .pixels()
            .iter()
            .all(|pixel| *pixel == TEST_BACKGROUND),
        "a fully clipped span line must not draw"
    );
}

#[test]
fn document_baseline_metrics_render_plain_and_spans_with_fractional_line_boxes() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node: UiNode = Text::new("Baseline")
        .text_role("body")
        .text_spans(vec![UiTextSpan::plain("Baseline")])
        .into();
    let mut metrics = UiTreeTextMetrics::for_node(&node);
    metrics.line_height = 32;
    metrics.line_box_height = 31.5;
    metrics.baseline_from_line_box_top = Some(18.5);
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 320,
        height: 120,
        scroll_y: 0.0,
    };
    let palette = crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );
    let mut canvas = Canvas::new(320, 120, TEST_BACKGROUND);

    UiTreeTextLines::draw_plain(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        0,
        0,
        8,
    );
    UiTreeTextLines::draw_spans_at_logical_y(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        0,
        48.0,
    );

    assert!(
        canvas
            .pixels()
            .iter()
            .any(|pixel| *pixel != TEST_BACKGROUND),
        "a fractional document line box must draw plain and span text"
    );
}

#[test]
fn fractional_logical_origin_quantizes_at_the_scaled_paint_boundary() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node: UiNode = Text::new("Current")
        .text_role("body")
        .text_spans(vec![UiTextSpan {
            text: "Current".into(),
            style: UiTextSpanStyle {
                current_highlight: true,
                ..UiTextSpanStyle::default()
            },
            link_target: String::new(),
        }])
        .into();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let palette = crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 160,
        height: 120,
        scroll_y: 0.0,
    };
    let mut canvas = Canvas::new_scaled(160, 120, 2.0, TEST_BACKGROUND);

    UiTreeTextLines::draw_spans_at_logical_y(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        0,
        31.5,
    );

    assert_eq!(
        TEST_BACKGROUND,
        canvas.pixels()[62 * canvas.width()],
        "31.5 logical px must not floor to physical row 62 at scale 2"
    );
    assert_ne!(
        TEST_BACKGROUND,
        canvas.pixels()[63 * canvas.width()],
        "31.5 logical px must quantize to physical row 63 at scale 2"
    );
}

#[test]
fn span_backgrounds_are_painted_before_their_glyphs() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let renderer = TextRenderer::load(&facade, "body");
    let node: UiNode = Text::new("Current")
        .text_role("body")
        .text_spans(vec![UiTextSpan {
            text: "Current".into(),
            style: UiTextSpanStyle {
                current_highlight: true,
                color_rgba: [255, 0, 0, 255],
                ..UiTextSpanStyle::default()
            },
            link_target: String::new(),
        }])
        .into();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let palette = crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(
        &ThemeSnapshot::dark(),
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 160,
        height: 120,
        scroll_y: 0.0,
    };
    let mut canvas = Canvas::new(160, 120, TEST_BACKGROUND);

    UiTreeTextLines::draw_spans_at_logical_y(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        0,
        16.0,
    );

    assert!(canvas.pixels().contains(&CURRENT_HIGHLIGHT_BACKGROUND));
    assert!(
        canvas
            .pixels()
            .iter()
            .any(|pixel| *pixel != TEST_BACKGROUND && *pixel != CURRENT_HIGHLIGHT_BACKGROUND),
        "glyph pixels must remain visible over a current-highlight background"
    );
}
