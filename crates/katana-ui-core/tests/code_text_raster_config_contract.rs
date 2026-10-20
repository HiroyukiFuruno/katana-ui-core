#![cfg(feature = "raster-host")]

use katana_ui_core::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::render_model::{
    UiNode, UiNodeKind, UiScrollAreaProps, UiTextProps, UiTextSpan, UiTextSpanStyle, UiTextWrapMode,
};
use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
use katana_ui_core::theme::ThemeSnapshot;

fn config() -> PlatformTextRasterConfig {
    let mut config = PlatformTextRasterConfig::default();
    config.monospace_candidates = config.proportional_candidates.clone();
    config
}

fn text(monospace: bool) -> UiNode {
    UiNode::new(UiNodeKind::Text, "")
        .stable_node_id("body")
        .text(UiTextProps {
            role: "markdown-paragraph".into(),
            wrap: UiTextWrapMode::Wrap,
            spans: vec![UiTextSpan {
                text: "Wide WWW and narrow iii body text ".repeat(12),
                style: UiTextSpanStyle {
                    monospace,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            }],
            ..UiTextProps::default()
        })
}

fn area() -> UiTreeRenderArea {
    UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 180,
        height: 140,
        scroll_y: 0.0,
    }
}

fn pixels(renderer: &UiTreeCanvasRenderer, node: &UiNode) -> Vec<u32> {
    let mut canvas = Canvas::new(180, 140, 0);
    renderer.render(&mut canvas, node, area());
    assert!(canvas.pixels().iter().any(|pixel| *pixel != 0));
    canvas.pixels().to_vec()
}

#[test]
fn code_face_override_preserves_system_body_renderer() {
    let default = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let configured = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let body = text(false);
    assert_eq!(pixels(&default, &body), pixels(&configured, &body));
    assert_eq!(
        default.document_node_hit_rects(&body, area()),
        configured.document_node_hit_rects(&body, area())
    );
}

#[test]
fn code_override_matches_shared_config_for_draw_wrap_measurement_and_hits() {
    let configured = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let shared = UiTreeCanvasRenderer::with_text_raster_config(
        ThemeSnapshot::dark(),
        config(),
        PlatformTextFaceSelection::CandidateChain,
    );
    let code = text(true);
    assert!(code.props().text.spans[0].style.monospace);
    assert!(pixels(&shared, &code) == pixels(&configured, &code));
    assert_eq!(
        shared.document_node_hit_rects(&code, area()),
        configured.document_node_hit_rects(&code, area())
    );
    assert_eq!(
        shared.viewport_node_hit_rects(&code, area()),
        configured.viewport_node_hit_rects(&code, area())
    );
}

#[test]
fn code_override_invalidates_existing_scroll_measurements() {
    let code = text(true);
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 180,
            viewport_height: 140,
            content_height: 1000,
            offset_y: 120,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(code.clone())
                .child(code),
        );
    let previous = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let _ = pixels(&previous, &root);
    let configured =
        previous.with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let fresh = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    assert_eq!(pixels(&fresh, &root), pixels(&configured, &root));
    assert_eq!(
        fresh.viewport_node_hit_rects(&root, area()),
        configured.viewport_node_hit_rects(&root, area())
    );
}

#[test]
fn mixed_plain_and_code_runs_use_the_configured_code_face() {
    let default = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let configured = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let body = UiTextSpan {
        text: "A ".into(),
        style: UiTextSpanStyle {
            color_rgba: [255, 0, 0, 255],
            ..UiTextSpanStyle::default()
        },
        link_target: String::new(),
    };
    let code = UiTextSpan {
        text: "WWWiii".into(),
        style: UiTextSpanStyle {
            monospace: true,
            color_rgba: [0, 255, 0, 255],
            ..UiTextSpanStyle::default()
        },
        link_target: String::new(),
    };
    let mixed = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        spans: vec![body.clone(), code.clone()],
        ..UiTextProps::default()
    });
    let configured_pixels = pixels(&configured, &mixed);
    let body_only = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        spans: vec![body],
        ..UiTextProps::default()
    });
    let code_only = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        spans: vec![code.clone()],
        ..UiTextProps::default()
    });

    let mixed_body_pixels = red_pixels(&configured_pixels);
    let body_only_pixels = red_pixels(&pixels(&default, &body_only));
    let mixed_code_pixels = green_pixels(&configured_pixels);
    let code_only_pixels = green_pixels(&pixels(&configured, &code_only));
    assert!(!mixed_body_pixels.is_empty());
    assert!(!mixed_code_pixels.is_empty());
    assert_eq!(mixed_body_pixels, body_only_pixels);
    assert_eq!(mixed_code_pixels, code_only_pixels);
    assert!(mixed.props().text.spans[1].style.monospace);
}

fn green_pixels(pixels: &[u32]) -> Vec<(usize, usize, u32)> {
    let points = pixels
        .iter()
        .enumerate()
        .filter(|(_, pixel)| **pixel & 0xff00 != 0 && **pixel & 0xff00ff == 0)
        .map(|(index, pixel)| (index % 180, index / 180, *pixel))
        .collect::<Vec<_>>();
    let left = points.iter().map(|(x, _, _)| *x).min().unwrap_or(0);
    points
        .into_iter()
        .map(|(x, y, color)| (x - left, y, color))
        .collect()
}

#[test]
fn code_first_run_preserves_following_body_pixels_and_decoration_baseline() {
    use katana_ui_core::raster_host::{UiTreeDocumentTypography, UiTreeTextRoleBaselineTypography};
    let typography = UiTreeDocumentTypography::default()
        .with_body_baseline(UiTreeTextRoleBaselineTypography::new(20.0, 32.5, 24.25));
    let configured =
        UiTreeCanvasRenderer::with_document_typography(ThemeSnapshot::dark(), typography)
            .with_code_text_raster_config(
                PlatformTextRasterConfig::default(),
                PlatformTextFaceSelection::CandidateChain,
            );
    let body = UiTextSpan {
        text: "Body".into(),
        style: UiTextSpanStyle {
            color_rgba: [255, 0, 0, 255],
            underline: true,
            ..UiTextSpanStyle::default()
        },
        link_target: String::new(),
    };
    let code = UiTextSpan {
        text: "ii".into(),
        style: UiTextSpanStyle {
            color_rgba: [0, 255, 0, 255],
            monospace: true,
            ..UiTextSpanStyle::default()
        },
        link_target: String::new(),
    };
    let node = |spans| {
        UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
            role: "markdown-paragraph".into(),
            spans,
            ..UiTextProps::default()
        })
    };
    let expected = red_pixels(&pixels(&configured, &node(vec![body.clone()])));
    let actual = red_pixels(&pixels(&configured, &node(vec![code, body])));
    assert!(!expected.is_empty());
    assert_eq!(expected, actual);
}

fn red_pixels(pixels: &[u32]) -> Vec<(usize, usize, u32)> {
    let points = pixels
        .iter()
        .enumerate()
        .filter(|(_, pixel)| **pixel & 0x00ffff == 0 && **pixel & 0xff0000 != 0)
        .map(|(index, pixel)| (index % 180, index / 180, *pixel))
        .collect::<Vec<_>>();
    let left = points.iter().map(|(x, _, _)| *x).min().unwrap_or(0);
    points
        .into_iter()
        .map(|(x, y, color)| (x - left, y, color))
        .collect()
}
