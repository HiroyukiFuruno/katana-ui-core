use crate::raster_host::text::{RichTextLineSpan, RichTextStyle, TextRenderer};
use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use crate::raster_host::{
    Canvas, UiTreeCanvasRenderer, UiTreeDocumentTypography, UiTreeHitRect, UiTreeRenderArea,
    UiTreeTextRoleBaselineTypography,
};
use katana_ui_core::atom::Text;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiNode, UiTextSpan, UiTextSpanStyle};
use katana_ui_core::theme::ThemeSnapshot;

const AREA_WIDTH: usize = 480;
const AREA_HEIGHT: usize = 80;
const HEADING_3_FONT_SIZE: f32 = 20.0;
const HEADING_3_LINE_BOX_HEIGHT: f32 = 28.0;
const HEADING_3_BASELINE: f32 = 20.5;
const LINK_TARGET: &str = "https://example.test/inline-code";
const CODE_TEXT: &str = "<h1 align=\"center\">";
const CODE_LINK_TEXT: &str = " <h1 align=\"center\"> ";

#[test]
fn inline_code_link_uses_the_code_measurement_for_its_glyph_and_hit() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let facade = UiCoreFacade::new(theme.clone());
    let expected_renderer = TextRenderer::load(&facade, facade.default_font_role());
    let renderer = UiTreeCanvasRenderer::with_document_typography(
        theme,
        UiTreeDocumentTypography::new().with_heading_3_baseline(
            UiTreeTextRoleBaselineTypography::new(
                HEADING_3_FONT_SIZE,
                HEADING_3_LINE_BOX_HEIGHT,
                HEADING_3_BASELINE,
            ),
        ),
    );
    let root = inline_code_link_node();
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: AREA_WIDTH,
        height: AREA_HEIGHT,
        scroll_y: 0.0,
    };
    let metrics = UiTreeTextMetrics::for_node_with_typography(&root, renderer.typography);
    let code_style = inline_code_style(metrics, palette);
    let code_width = renderer.code_text.measure_width_rich(CODE_TEXT, code_style);
    let hits = renderer.document_host_action_hit_rects(&root, area);
    let link_hit = hits
        .iter()
        .find(|hit| hit.action.payload == LINK_TARGET)
        .expect("inline-code link hit");
    let mut canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, palette.background);
    let mut expected_code = Canvas::new(AREA_WIDTH, AREA_HEIGHT, palette.background);

    renderer.render(&mut canvas, &root, area);
    expected_renderer.draw_rich_line_signed_in_line_box(
        &mut expected_code,
        &expected_rich_line(&expected_renderer, metrics, palette),
        0,
        0.0,
        HEADING_3_LINE_BOX_HEIGHT,
        HEADING_3_BASELINE,
    );

    assert_eq!(code_width, link_hit.rect.width, "metrics={metrics:?}");
    let expected_mask = link_color_mask(&expected_code, palette.link, link_hit.rect, None);
    let (_, Some((_, _, _, expected_last_row))) =
        glyph_mask_summary(&expected_mask, link_hit.rect.width)
    else {
        panic!("the code renderer must paint opaque link glyph pixels");
    };
    let actual_mask = link_color_mask(
        &canvas,
        palette.link,
        link_hit.rect,
        Some(expected_last_row),
    );
    assert_eq!(
        glyph_mask_summary(&expected_mask, link_hit.rect.width),
        glyph_mask_summary(&actual_mask, link_hit.rect.width),
        "the inline-code glyph must occupy the same bounds as the code renderer: {link_hit:?}"
    );
    assert_eq!(
        expected_mask, actual_mask,
        "the inline-code glyph pattern must use the same face as the code renderer: {link_hit:?}"
    );
}

fn inline_code_link_node() -> UiNode {
    inline_code_link_node_with_code_text(CODE_LINK_TEXT)
}

fn inline_code_link_node_with_code_text(code_link_text: &str) -> UiNode {
    Text::new(format!("prefix{code_link_text}suffix"))
        .text_role("heading-3")
        .text_spans(vec![
            UiTextSpan::plain("prefix "),
            UiTextSpan {
                text: code_link_text.to_owned(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: LINK_TARGET.to_owned(),
            },
            UiTextSpan::plain(" suffix"),
        ])
        .into()
}

fn inline_code_style(metrics: UiTreeTextMetrics, palette: UiTreeCanvasPalette) -> RichTextStyle {
    RichTextStyle::new(metrics.font_size, palette.link).monospace(true)
}

fn expected_rich_line(
    renderer: &TextRenderer,
    metrics: UiTreeTextMetrics,
    palette: UiTreeCanvasPalette,
) -> Vec<RichTextLineSpan> {
    let heading_style = RichTextStyle::new(metrics.font_size, palette.text);
    vec![
        renderer.rich_line_span("prefix ", heading_style),
        renderer.rich_line_span(CODE_LINK_TEXT, inline_code_style(metrics, palette)),
        renderer.rich_line_span(" suffix", heading_style),
    ]
}

fn link_color_mask(
    canvas: &Canvas,
    color: u32,
    rect: UiTreeHitRect,
    max_row: Option<usize>,
) -> Vec<bool> {
    (rect.y..rect.y.saturating_add(rect.height))
        .enumerate()
        .flat_map(|(row, y)| {
            (rect.x..rect.x.saturating_add(rect.width)).map(move |x| {
                x < canvas.width()
                    && y < canvas.height()
                    && max_row.is_none_or(|last_row| row <= last_row)
                    && canvas.pixels()[y * canvas.width() + x] == color
            })
        })
        .collect()
}

fn glyph_mask_summary(
    mask: &[bool],
    width: usize,
) -> (usize, Option<(usize, usize, usize, usize)>) {
    let mut count = 0;
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for (index, painted) in mask.iter().enumerate() {
        if !painted {
            continue;
        }
        count += 1;
        let x = index % width;
        let y = index / width;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    let bounds = (count > 0).then_some((min_x, min_y, max_x, max_y));
    (count, bounds)
}
