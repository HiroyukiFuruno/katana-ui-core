use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle};
use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
use katana_ui_core::theme::ThemeSnapshot;

const AREA_WIDTH: usize = 480;
const AREA_HEIGHT: usize = 80;

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
