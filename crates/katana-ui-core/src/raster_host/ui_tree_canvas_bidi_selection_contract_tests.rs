use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{UiRect, UiTextSpan, UiTextSpanStyle};
use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::theme::ThemeSnapshot;

const AREA_WIDTH: usize = 480;
const AREA_HEIGHT: usize = 80;
const BIDI_RASTER_WIDTH: usize = 8_192;
const LINK_TARGET: &str = "https://example.test/arabic";

#[test]
fn combined_grapheme_keeps_atomic_canvas_selection_and_fragment_hit_geometry() {
    let theme = ThemeSnapshot::dark();
    let renderer = UiTreeCanvasRenderer::new(theme.clone()).with_code_text_raster_config(
        PlatformTextRasterConfig::default(),
        PlatformTextFaceSelection::CandidateChain,
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: BIDI_RASTER_WIDTH,
        height: AREA_HEIGHT,
        scroll_y: 0.0,
    };
    let root = Text::new("\u{05d0}e\u{0301}x")
        .text_role("heading-3")
        .text_spans(vec![
            UiTextSpan::plain("\u{05d0}"),
            UiTextSpan {
                text: "e".to_owned(),
                style: UiTextSpanStyle {
                    underline: true,
                    highlight: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
            UiTextSpan {
                text: "\u{0301}".to_owned(),
                style: UiTextSpanStyle::default(),
                link_target: LINK_TARGET.to_owned(),
            },
            UiTextSpan {
                text: "x".to_owned(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
        ])
        .into();
    let mut canvas = Canvas::new(BIDI_RASTER_WIDTH, AREA_HEIGHT, 0);

    let link_hit = renderer
        .host_action_hit_rects(&root, area)
        .into_iter()
        .find(|hit| hit.action.payload == LINK_TARGET)
        .expect("the combining-mark link must retain its own hit geometry");
    renderer.render(&mut canvas, &root, area);

    let combined_run = canvas
        .text_runs()
        .iter()
        .find(|run| run.text() == "e\u{0301}")
        .expect("the combined grapheme must stay in one canvas run");
    let atomic_rects = combined_run
        .model()
        .highlight_rects(UiTextSelectionRange::new(0, 1));
    assert_eq!(
        1,
        atomic_rects.len(),
        "the combined grapheme must remain one selectable grapheme"
    );
    let atomic_rect = atomic_rects[0];
    assert!(
        atomic_rect.x <= link_hit.rect.x as i32
            && atomic_rect.x.saturating_add(atomic_rect.width as i32)
                >= link_hit.rect.x.saturating_add(link_hit.rect.width) as i32,
        "the atomic selection bound must preserve the full linked combining-mark bound"
    );
    assert_eq!(
        Some("e\u{0301}".to_owned()),
        canvas.copy_text_in_selection(
            Some((atomic_rect.x.max(0) as usize, combined_run.y())),
            Some((
                atomic_rect
                    .x
                    .saturating_add(atomic_rect.width as i32)
                    .max(0) as usize,
                combined_run.bottom(),
            )),
        ),
        "copying the atomic selection must preserve the full combined grapheme"
    );

    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut expected_highlight = Canvas::new(1, 1, 0);
    expected_highlight.blend_rect(
        0,
        0,
        1,
        1,
        palette.text_highlight_background,
        palette.text_highlight_alpha,
    );
    let highlight_pixel = expected_highlight.pixels()[0];
    assert!(
        !rect_contains_color(
            &canvas,
            UiRect::new(
                link_hit.rect.x as i32,
                link_hit.rect.y as i32,
                link_hit.rect.width as u32,
                link_hit.rect.height as u32,
            ),
            highlight_pixel,
        ),
        "the highlighted base fragment must not leak into the linked combining-mark fragment"
    );
}

fn rect_contains_color(canvas: &Canvas, rect: UiRect, color: u32) -> bool {
    let left = rect.x.max(0) as usize;
    let top = rect.y.max(0) as usize;
    let right = (rect.x.saturating_add(rect.width as i32).max(0) as usize).min(canvas.width());
    let bottom = (rect.y.saturating_add(rect.height as i32).max(0) as usize).min(canvas.height());
    (top..bottom).any(|y| (left..right).any(|x| canvas.pixels()[y * canvas.width() + x] == color))
}

#[test]
fn overridden_single_code_fragment_preserves_resolved_direction_for_selection() -> Result<(), String>
{
    let theme = ThemeSnapshot::dark();
    let renderer = UiTreeCanvasRenderer::new(theme.clone()).with_code_text_raster_config(
        PlatformTextRasterConfig::default(),
        PlatformTextFaceSelection::CandidateChain,
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: AREA_WIDTH,
        height: AREA_HEIGHT,
        scroll_y: 0.0,
    };
    let root = Text::new("\u{202e}a\u{202c}!")
        .text_role("heading-3")
        .text_spans(vec![
            UiTextSpan::plain("\u{202e}"),
            UiTextSpan {
                text: "a".to_owned(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
            UiTextSpan::plain("\u{202c}!"),
        ])
        .into();
    let mut canvas = Canvas::new(AREA_WIDTH, AREA_HEIGHT, 0);

    renderer.render(&mut canvas, &root, area);

    let code_run = canvas
        .text_runs()
        .iter()
        .find(|run| run.text() == "a")
        .ok_or_else(|| {
            "the overridden code fragment must be emitted as a selectable run".to_owned()
        })?;
    let center_y = code_run.y().saturating_add(code_run.height() / 2) as i32;
    let left_x = code_run.x().saturating_add(1) as i32;
    let right_x = code_run.right().saturating_sub(1) as i32;

    assert_eq!(Some(true), code_run.resolved_bidi_rtl());
    assert_eq!(
        1,
        code_run.model().point_to_caret(left_x, center_y),
        "the visual left edge of the RLO fragment maps to its logical end",
    );
    assert_eq!(
        0,
        code_run.model().point_to_caret(right_x, center_y),
        "the visual right edge of the RLO fragment maps to its logical start",
    );
    Ok(())
}
