use crate::raster_host::ui_tree_canvas_text_line_width::preserves_whitespace;
use crate::raster_host::ui_tree_canvas_types::CanvasBlitRequest;
use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{UiNode, UiTextSpan, UiTextSpanStyle, UiTextWrapMode};
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::theme::ThemeSnapshot;

const COPY_AREA_HEIGHT: usize = 1_200;
const BLIT_COPY_WIDTH: usize = 120;

#[test]
fn soft_wrapped_plain_prose_copy_preserves_source_whitespace() {
    let node = wrapped_plain_prose_node();
    for width in [120, 140] {
        assert_prose_copy_contract(&node, width, "xxxxxxxxxxxx alpha beta");
    }
}

#[test]
fn soft_wrapped_rich_prose_copy_preserves_source_whitespace() {
    let node = wrapped_rich_prose_node();
    for width in [120, 140] {
        assert_prose_copy_contract(&node, width, "xxxxxxxxxxxx alpha beta gamma");
    }
}

#[test]
fn prose_copy_preserves_explicit_newlines() {
    let node: UiNode = Text::new("alpha\nbeta")
        .text_role("markdown-paragraph")
        .wrap(UiTextWrapMode::Wrap)
        .into();
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let width = 120;
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width,
        height: COPY_AREA_HEIGHT,
        scroll_y: 0.0,
    };
    let mut canvas = Canvas::new(width, area.height, 0);

    renderer.render(&mut canvas, &node, area);

    assert_eq!(
        Some("alpha\nbeta".to_owned()),
        canvas.copy_text_in_selection(Some((0, 0)), Some((width, area.height)))
    );
}

#[test]
fn aligned_blit_preserves_soft_wrap_copy_whitespace() {
    assert_soft_wrap_copy_after_blit(1.0);
}

#[test]
fn scaled_blit_preserves_soft_wrap_copy_whitespace() {
    assert_soft_wrap_copy_after_blit(2.0);
}

#[test]
fn scaled_blit_preserves_rich_continuation_line_grouping() {
    let root = wrapped_rich_prose_node();
    assert_copy_after_blit(&root, BLIT_COPY_WIDTH, 2.0, "xxxxxxxxxxxx alpha beta gamma");
}

fn wrapped_plain_prose_node() -> UiNode {
    Text::new("xxxxxxxxxxxx alpha beta")
        .text_role("markdown-paragraph")
        .wrap(UiTextWrapMode::Wrap)
        .into()
}

fn wrapped_rich_prose_node() -> UiNode {
    Text::new("xxxxxxxxxxxx alpha beta gamma")
        .text_role("markdown-paragraph")
        .wrap(UiTextWrapMode::Wrap)
        .text_spans(vec![
            UiTextSpan::plain("xxxxxxxxxxxx alpha"),
            UiTextSpan {
                text: " beta".to_owned(),
                style: UiTextSpanStyle {
                    bold: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
            UiTextSpan {
                text: " gamma".to_owned(),
                style: UiTextSpanStyle {
                    italic: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
        ])
        .into()
}

fn assert_prose_copy_contract(root: &UiNode, width: usize, expected: &str) {
    assert!(!preserves_whitespace(root));
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width,
        height: COPY_AREA_HEIGHT,
        scroll_y: 0.0,
    };
    for _ in 0..2 {
        let mut canvas = Canvas::new(width, area.height, 0);
        renderer.render(&mut canvas, root, area);
        assert!(
            canvas.text_runs().len() > 1,
            "prose must wrap at width {width}"
        );
        assert_full_prose_selection(&canvas, width, area, expected);
        assert_beta_only_selection(&canvas);
    }
}

fn assert_soft_wrap_copy_after_blit(scale_factor: f32) {
    let root = wrapped_plain_prose_node();
    assert_copy_after_blit(
        &root,
        BLIT_COPY_WIDTH,
        scale_factor,
        "xxxxxxxxxxxx alpha beta",
    );
}

fn assert_copy_after_blit(root: &UiNode, width: usize, scale_factor: f32, expected: &str) {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width,
        height: COPY_AREA_HEIGHT,
        scroll_y: 0.0,
    };
    let mut source = Canvas::new(area.width, area.height, 0);
    renderer.render(&mut source, root, area);

    let mut target = Canvas::new_scaled(area.width, area.height, scale_factor, 0);
    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: area.width,
            height: area.height,
            source_y: 0,
        },
    );

    assert_eq!(
        Some(expected.to_owned()),
        target.copy_text_in_selection(Some((0, 0)), Some((area.width, area.height))),
        "blit at scale {scale_factor} must preserve source whitespace across visual lines"
    );
}

fn assert_full_prose_selection(
    canvas: &Canvas,
    width: usize,
    area: UiTreeRenderArea,
    expected: &str,
) {
    assert_eq!(
        Some(expected.to_owned()),
        canvas.copy_text_in_selection(Some((0, 0)), Some((width, area.height))),
        "full selection must retain source whitespace at width {width}"
    );
    assert_eq!(
        Some(expected.to_owned()),
        canvas.copy_text_in_selection(Some((width, area.height)), Some((0, 0))),
        "reversed full selection must retain source whitespace at width {width}"
    );
}

fn assert_beta_only_selection(canvas: &Canvas) {
    const SELECTED_WORD: &str = "beta";
    let beta_index = canvas
        .text_runs()
        .iter()
        .position(|run| run.text().contains(SELECTED_WORD));
    assert!(
        beta_index.is_some(),
        "wrapped beta run must be selectable; runs: {:?}",
        canvas
            .text_runs()
            .iter()
            .map(|run| run.text())
            .collect::<Vec<_>>()
    );
    if let Some(beta_index) = beta_index {
        let run = &canvas.text_runs()[beta_index];
        let beta_start = run.text().find(SELECTED_WORD).unwrap_or_default();
        let beta_start = run.text()[..beta_start].chars().count();
        let beta_rects = run.model().highlight_rects(UiTextSelectionRange::new(
            beta_start,
            beta_start + SELECTED_WORD.chars().count(),
        ));
        let first = beta_rects.first().expect("beta must map to glyphs");
        let last = beta_rects.last().expect("beta must map to glyphs");
        /* WHY: glyph下端は次の行と重なるfontがあるため、語の中央の高さで選択する。 */
        let selection_y = first.y.saturating_add_unsigned(first.height / 2).max(0) as usize;
        let start = (first.x as usize, selection_y);
        let end = (
            last.x.saturating_add_unsigned(last.width) as usize,
            selection_y,
        );
        assert_eq!(
            Some(SELECTED_WORD.to_owned()),
            canvas.copy_text_in_selection(Some(start), Some(end)),
            "beta glyph bounds {start:?}..{end:?}, runs {:?}",
            canvas
                .text_runs()
                .iter()
                .map(|run| (run.text(), run.x(), run.y(), run.right(), run.bottom()))
                .collect::<Vec<_>>()
        );
    }
}
