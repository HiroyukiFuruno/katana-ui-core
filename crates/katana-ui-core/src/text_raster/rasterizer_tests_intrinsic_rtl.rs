use crate::text_raster::{
    PlatformTextFaceSelection, PlatformTextRasterConfig, PlatformTextRasterRequest,
    PlatformTextRasterizer,
};
use crate::theme::{FontFamily, FontToken};

const TEXT_COLOR: [u8; 4] = [245, 245, 245, 255];
const FONT_SIZE_PX: f32 = 18.0;
const FONT_WEIGHT: u16 = 400;
const MAX_INTRINSIC_WIDTH: usize = 128;
#[cfg(feature = "raster-host")]
const VIEWPORT_WIDTH: usize = 640;
#[cfg(feature = "raster-host")]
const VIEWPORT_HEIGHT: usize = 96;

fn font(family: FontFamily) -> FontToken {
    FontToken {
        name: "intrinsic-rtl".to_owned(),
        family,
        size: FONT_SIZE_PX,
        weight: FONT_WEIGHT,
    }
}

#[test]
fn width_unspecified_rtl_raster_uses_intrinsic_glyph_coordinates()
-> Result<(), Box<dyn std::error::Error>> {
    for selection in [
        PlatformTextFaceSelection::System,
        PlatformTextFaceSelection::CandidateChain,
    ] {
        let mut rasterizer = PlatformTextRasterizer::new_with_face_selection(
            PlatformTextRasterConfig::default(),
            selection,
        );
        for text in ["سلا", "אבג", "سلاx"] {
            let request = PlatformTextRasterRequest::from_text(
                text,
                font(FontFamily::Proportional),
                TEXT_COLOR,
            );
            let raster = rasterizer.rasterize(&request)?;
            assert!(
                raster.width > 0 && raster.width < MAX_INTRINSIC_WIDTH,
                "short intrinsic RTL text {text:?} must not inherit a fallback width: {}",
                raster.width
            );
            assert!(
                raster
                    .grapheme_bounds
                    .iter()
                    .all(|bounds| bounds.x.round() >= 0.0
                        && (bounds.x + bounds.width).round() <= raster.width as f32),
                "intrinsic bounds for {selection:?}/{text:?}: {:?} (width {})",
                raster.grapheme_bounds,
                raster.width
            );
            assert!(raster.rgba_pixels.iter().any(|pixel| pixel[3] > 0));
        }
    }
    Ok(())
}

#[cfg(feature = "raster-host")]
#[test]
fn mixed_rtl_code_link_stays_inside_a_regular_viewport() -> Result<(), Box<dyn std::error::Error>> {
    use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
    use crate::theme::ThemeSnapshot;
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark()).with_code_text_raster_config(
        PlatformTextRasterConfig::default(),
        PlatformTextFaceSelection::CandidateChain,
    );
    let root = mixed_rtl_code_node();
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
        scroll_y: 0.0,
    };
    let hit = renderer
        .document_host_action_hit_rects(&root, area)
        .into_iter()
        .find(|hit| hit.action.payload == "https://example.test/rtl")
        .ok_or("missing Arabic link hit")?;
    assert!(
        hit.rect.x + hit.rect.width < MAX_INTRINSIC_WIDTH,
        "short Arabic fragment must have intrinsic hit bounds: {:?}",
        hit.rect
    );
    let mut canvas = Canvas::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, 0);
    renderer.render(&mut canvas, &root, area);
    assert!(
        canvas
            .text_runs()
            .iter()
            .all(|run| run.right() < MAX_INTRINSIC_WIDTH)
    );
    assert_eq!(
        Some("سلاx".to_owned()),
        canvas.copy_text_in_selection(Some((0, 0)), Some((VIEWPORT_WIDTH, VIEWPORT_HEIGHT)))
    );
    Ok(())
}

#[cfg(feature = "raster-host")]
fn mixed_rtl_code_node() -> crate::render_model::UiNode {
    use crate::atom::Text;
    use crate::render_model::{UiTextSpan, UiTextSpanStyle, UiTextWrapMode};
    Text::new("سلاx")
        .text_role("body")
        .wrap(UiTextWrapMode::Wrap)
        .text_spans(vec![
            UiTextSpan::plain("س"),
            UiTextSpan {
                text: "ل".to_owned(),
                style: UiTextSpanStyle {
                    underline: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: "https://example.test/rtl".to_owned(),
            },
            UiTextSpan::plain("ا"),
            UiTextSpan {
                text: "x".to_owned(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    monospace: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
        ])
        .into()
}
