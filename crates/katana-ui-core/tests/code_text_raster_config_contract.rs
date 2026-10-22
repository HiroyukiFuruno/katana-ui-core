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
fn code_override_recomputes_plain_text_wrapping_and_measurement() {
    let plain_code = UiNode::new(
        UiNodeKind::Text,
        "Wide WWW and narrow iii code text ".repeat(12),
    )
    .font_role("code")
    .text(UiTextProps {
        role: "code".into(),
        wrap: UiTextWrapMode::Wrap,
        ..UiTextProps::default()
    });
    let previous = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let previous_pixels = pixels(&previous, &plain_code);
    let configured =
        previous.with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let fresh = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);

    let configured_pixels = pixels(&configured, &plain_code);
    let configured_hits = configured.document_node_hit_rects(&plain_code, area());
    assert_ne!(previous_pixels, configured_pixels);
    assert_eq!(pixels(&fresh, &plain_code), configured_pixels);
    assert_eq!(
        fresh.document_node_hit_rects(&plain_code, area()),
        configured_hits
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

#[test]
fn mixed_inline_code_selection_copies_a_single_logical_line() {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let code = UiTextSpan {
        text: "code".into(),
        style: UiTextSpanStyle {
            monospace: true,
            ..UiTextSpanStyle::default()
        },
        link_target: String::new(),
    };
    let node = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        spans: vec![UiTextSpan::plain("A "), code, UiTextSpan::plain(" B")],
        ..UiTextProps::default()
    });
    let mut canvas = Canvas::new(180, 140, 0);
    renderer.render(&mut canvas, &node, area());
    let runs = canvas.text_runs();
    assert_eq!(3, runs.len());
    assert!(runs.iter().all(|run| run.y() == runs[0].y()));
    let start = (runs[0].x(), runs[0].y() + runs[0].height() / 2);
    let last = &runs[2];
    let end = (last.right(), last.y() + last.height() / 2);
    assert_eq!(
        Some("A code B".to_owned()),
        canvas.copy_text_in_selection(Some(start), Some(end))
    );
}

#[test]
fn mixed_rtl_inline_code_preserves_visual_order_at_a_document_baseline() {
    use katana_ui_core::raster_host::{UiTreeDocumentTypography, UiTreeTextRoleBaselineTypography};

    let typography = UiTreeDocumentTypography::default()
        .with_body_baseline(UiTreeTextRoleBaselineTypography::new(20.0, 32.5, 24.25));
    let renderer =
        UiTreeCanvasRenderer::with_document_typography(ThemeSnapshot::dark(), typography)
            .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let node = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        role: "paragraph".into(),
        spans: vec![
            styled_span("אבג ", false),
            styled_span("code", true),
            styled_span(" דהו", false),
        ],
        ..UiTextProps::default()
    });
    let mut canvas = Canvas::new(180, 140, 0);
    renderer.render(&mut canvas, &node, area());

    let runs = canvas.text_runs();
    assert_eq!(
        ["אבג ", "code", " דהו"],
        runs.iter()
            .map(|run| run.text())
            .collect::<Vec<_>>()
            .as_slice()
    );
    assert!(
        runs[2].x() < runs[1].x(),
        "trailing RTL body must be left of inline code"
    );
    assert!(
        runs[1].x() < runs[0].x(),
        "inline code must be left of leading RTL body"
    );
    let y = runs[0].y() + runs[0].height() / 2;
    let right = runs.iter().map(|run| run.right()).max().unwrap_or(0);
    assert_eq!(
        Some("אבג code דהו".to_owned()),
        canvas.copy_text_in_selection(Some((0, y)), Some((right, y)))
    );
}

#[test]
fn mixed_rtl_neutral_inline_code_keeps_its_resolved_rtl_context() {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let node = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        spans: vec![
            styled_span("אבג ", false),
            styled_span("[]", true),
            styled_span(" דהו", false),
        ],
        ..UiTextProps::default()
    });
    let mut canvas = Canvas::new(180, 140, 0);
    renderer.render(&mut canvas, &node, area());

    let runs = canvas.text_runs();
    assert_eq!(
        ["אבג ", "[]", " דהו"],
        runs.iter()
            .map(|run| run.text())
            .collect::<Vec<_>>()
            .as_slice()
    );
    assert!(
        runs[2].x() < runs[1].x() && runs[1].x() < runs[0].x(),
        "a neutral code fragment must remain in the full line's RTL visual order"
    );
    let y = runs[0].y() + runs[0].height() / 2;
    let right = runs.iter().map(|run| run.right()).max().unwrap_or(0);
    assert_eq!(
        Some("אבג [] דהו".to_owned()),
        canvas.copy_text_in_selection(Some((0, y)), Some((right, y)))
    );
}

#[test]
fn adjacent_rtl_plain_and_code_runs_reverse_within_one_visual_run() {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let node = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        spans: vec![styled_span("אבג ", false), styled_span("דהו", true)],
        ..UiTextProps::default()
    });
    let mut canvas = Canvas::new(180, 140, 0);
    renderer.render(&mut canvas, &node, area());

    let runs = canvas.text_runs();
    assert_eq!(
        ["אבג ", "דהו"],
        runs.iter()
            .map(|run| run.text())
            .collect::<Vec<_>>()
            .as_slice()
    );
    assert!(
        runs[1].x() < runs[0].x(),
        "RTL code must be left of preceding RTL body"
    );
    let y = runs[0].y() + runs[0].height() / 2;
    let right = runs.iter().map(|run| run.right()).max().unwrap_or(0);
    assert_eq!(
        Some("אבג דהו".to_owned()),
        canvas.copy_text_in_selection(Some((0, y)), Some((right, y)))
    );
}

#[test]
fn mixed_direction_plain_run_splits_at_visual_boundaries_around_inline_code() {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let node = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        spans: vec![
            styled_span("אבג abc ", false),
            styled_span("CODE", true),
            styled_span(" דהו", false),
        ],
        ..UiTextProps::default()
    });
    let mut canvas = Canvas::new(180, 140, 0);
    renderer.render(&mut canvas, &node, area());

    let runs = canvas.text_runs();
    assert_eq!(
        ["אבג ", "abc ", "CODE", " דהו"],
        runs.iter()
            .map(|run| run.text())
            .collect::<Vec<_>>()
            .as_slice()
    );
    assert!(
        runs[3].x() < runs[1].x() && runs[1].x() < runs[2].x() && runs[2].x() < runs[0].x(),
        "each visual-range fragment must surround inline code in visual order"
    );
    let y = runs[0].y() + runs[0].height() / 2;
    let right = runs.iter().map(|run| run.right()).max().unwrap_or(0);
    assert_eq!(
        Some("אבג abc CODE דהו".to_owned()),
        canvas.copy_text_in_selection(Some((0, y)), Some((right, y)))
    );
}

#[test]
fn mixed_rtl_inline_code_link_hit_tracks_its_visual_raster_run() {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let node = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        spans: vec![
            styled_span("\u{202e}ABCDEFGHIJ", false),
            UiTextSpan {
                text: "CODE".into(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    monospace: true,
                    underline: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: "https://example.test/code".into(),
            },
            styled_span("Z\u{202c}", false),
        ],
        ..UiTextProps::default()
    });
    let single_line_area = UiTreeRenderArea {
        width: 10_000,
        ..area()
    };
    let mut canvas = Canvas::new(single_line_area.width, single_line_area.height, 0);
    renderer.render(&mut canvas, &node, single_line_area);

    let code_run = canvas
        .text_runs()
        .iter()
        .find(|run| run.text() == "CODE")
        .expect("code raster run");
    let link_hit = renderer
        .document_host_action_hit_rects(&node, single_line_area)
        .into_iter()
        .find(|hit| hit.action.payload == "https://example.test/code")
        .expect("code link hit");
    assert_eq!(code_run.x(), link_hit.rect.x);
    assert!(
        code_run.right() <= link_hit.rect.x + link_hit.rect.width + 1,
        "code={:?}, hit={:?}",
        (code_run.x(), code_run.right()),
        (link_hit.rect.x, link_hit.rect.x + link_hit.rect.width),
    );
}

#[test]
fn mixed_rtl_inline_code_keeps_split_rlo_context_without_exposing_controls() {
    use katana_ui_core::raster_host::{UiTreeDocumentTypography, UiTreeTextRoleBaselineTypography};

    let typography = UiTreeDocumentTypography::default()
        .with_body_baseline(UiTreeTextRoleBaselineTypography::new(20.0, 32.5, 24.25));
    let renderer =
        UiTreeCanvasRenderer::with_document_typography(ThemeSnapshot::dark(), typography)
            .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let code_style = UiTextSpanStyle {
        color_rgba: [0, 255, 0, 255],
        inline_code: true,
        monospace: true,
        ..UiTextSpanStyle::default()
    };
    let actual = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        role: "paragraph".into(),
        spans: vec![
            styled_span("\u{202e}A", false),
            UiTextSpan {
                text: "CODE".into(),
                style: code_style,
                link_target: String::new(),
            },
            styled_span("Z\u{202c}", false),
        ],
        ..UiTextProps::default()
    });
    let mut actual_canvas = Canvas::new(180, 140, 0);
    renderer.render(&mut actual_canvas, &actual, area());
    let actual_code_run = actual_canvas
        .text_runs()
        .iter()
        .find(|run| run.text() == "CODE")
        .expect("actual code raster run");

    let y = actual_code_run
        .y()
        .saturating_add(actual_code_run.height() / 2);
    for (start, end) in [
        (actual_code_run.x(), actual_code_run.right()),
        (actual_code_run.right(), actual_code_run.x()),
    ] {
        assert_eq!(
            Some("CODE".to_owned()),
            actual_canvas.copy_text_in_selection(Some((start, y)), Some((end, y))),
            "the visible code fragment must remain selectable without its shaping controls"
        );
    }
}

#[test]
fn mixed_bidi_link_keeps_internal_fragment_whitespace_clickable() {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark())
        .with_code_text_raster_config(config(), PlatformTextFaceSelection::CandidateChain);
    let link_target = "https://example.test/mixed";
    let node = UiNode::new(UiNodeKind::Text, "").text(UiTextProps {
        spans: vec![
            UiTextSpan {
                text: "abc אבג def".into(),
                style: UiTextSpanStyle::default(),
                link_target: link_target.into(),
            },
            styled_span("CODE", true),
        ],
        ..UiTextProps::default()
    });
    let single_line_area = UiTreeRenderArea {
        width: 10_000,
        ..area()
    };
    let mut canvas = Canvas::new(single_line_area.width, single_line_area.height, 0);
    renderer.render(&mut canvas, &node, single_line_area);
    let hits = renderer
        .document_host_action_hit_rects(&node, single_line_area)
        .into_iter()
        .filter(|hit| hit.action.payload == link_target)
        .collect::<Vec<_>>();

    for text in ["abc ", " def"] {
        let run = canvas
            .text_runs()
            .iter()
            .find(|run| run.text() == text)
            .expect("split link raster run");
        assert!(
            hits.iter().any(|hit| {
                hit.rect.x <= run.x() && run.right() <= hit.rect.x + hit.rect.width + 1
            }),
            "link hit must cover the full visual fragment including internal whitespace: {text:?}",
        );
    }
}

fn styled_span(text: &str, monospace: bool) -> UiTextSpan {
    UiTextSpan {
        text: text.into(),
        style: UiTextSpanStyle {
            monospace,
            ..UiTextSpanStyle::default()
        },
        link_target: String::new(),
    }
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
