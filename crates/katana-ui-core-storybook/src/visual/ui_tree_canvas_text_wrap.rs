use super::super::super::text::TextRenderer;
use super::super::super::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use super::super::super::ui_tree_canvas_types::UiTreeRenderArea;
use super::wrap_state::SpanWrapState;
pub(super) use super::wrap_state::span_segments;
use crate::visual::ui_tree_canvas_text_line_width::SpanTextRenderers;
#[cfg(test)]
use crate::visual::ui_tree_canvas_text_line_width::span_part_width;
use katana_ui_core::render_model::{UiNode, UiTextSpan, UiTextWrapMode};
use std::cell::RefCell;
use std::collections::HashMap;

const MIN_TEXT_WIDTH: usize = 120;
const COMPACT_DOCUMENT_FONT_SIZE: f32 = 14.0;
const COMPACT_DOCUMENT_RASTER_WRAP_SCALE: f32 = 1.10;
const ALERT_TITLE_MAX_CHARS: usize = 12;
const ALERT_BODY_MAX_CHARS: usize = 58;
const MAX_PLAIN_LINE_CACHE_ENTRIES: usize = 4_096;

thread_local! {
    static PLAIN_LINE_CACHE: RefCell<HashMap<PlainLineCacheKey, Vec<String>>> =
        RefCell::new(HashMap::new());
}

pub(super) struct UiTreeTextWrap;

impl UiTreeTextWrap {
    pub(super) fn plain_lines(
        renderer: &TextRenderer,
        node: &UiNode,
        x: usize,
        area: UiTreeRenderArea,
        metrics: UiTreeTextMetrics,
    ) -> Vec<String> {
        let max_width = available_width(node, x, area);
        let wrap_metrics = wrap_metrics_for_node(node, metrics);
        let cache_key = PlainLineCacheKey::from_node(node, max_width, wrap_metrics);
        if let Some(lines) = cached_plain_lines(&cache_key) {
            return lines;
        }
        if !should_wrap(node) {
            let lines: Vec<String> = node.props().label.split('\n').map(str::to_string).collect();
            remember_plain_lines(cache_key, &lines);
            return lines;
        }
        let lines = if node.props().text.role == "alert" {
            alert_plain_lines(&node.props().label)
        } else {
            node.props()
                .label
                .split('\n')
                .flat_map(|line| wrap_plain_line(renderer, line, max_width, wrap_metrics))
                .collect()
        };
        remember_plain_lines(cache_key, &lines);
        lines
    }

    pub(super) fn span_lines(
        renderers: SpanTextRenderers<'_>,
        node: &UiNode,
        x: usize,
        area: UiTreeRenderArea,
        metrics: UiTreeTextMetrics,
    ) -> Vec<Vec<UiTextSpan>> {
        if !should_wrap(node) {
            return span_no_wrap_lines(&node.props().text.spans);
        }
        let mut state =
            SpanWrapState::new(available_width(node, x, area), preserves_whitespace(node));
        let wrap_metrics = wrap_metrics_for_node(node, metrics);
        for span in &node.props().text.spans {
            for segment in span_segments(span) {
                state.push(renderers, segment, wrap_metrics);
            }
        }
        state.finish()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct PlainLineCacheKey {
    node_id: String,
    label: String,
    role: String,
    font_role: String,
    wrap: bool,
    width: usize,
    font_size_bits: u32,
    line_height: usize,
}

impl PlainLineCacheKey {
    fn from_node(node: &UiNode, width: usize, metrics: UiTreeTextMetrics) -> Self {
        Self {
            node_id: node.id().as_str().to_string(),
            label: node.props().label.clone(),
            role: node.props().text.role.clone(),
            font_role: node.props().font_role.clone(),
            wrap: should_wrap(node),
            width,
            font_size_bits: metrics.font_size.to_bits(),
            line_height: metrics.line_height,
        }
    }
}

fn cached_plain_lines(key: &PlainLineCacheKey) -> Option<Vec<String>> {
    PLAIN_LINE_CACHE.with(|cache| cache.borrow().get(key).cloned())
}

fn remember_plain_lines(key: PlainLineCacheKey, lines: &[String]) {
    PLAIN_LINE_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache.len() >= MAX_PLAIN_LINE_CACHE_ENTRIES {
            cache.clear();
        }
        cache.insert(key, lines.to_vec());
    });
}

fn should_wrap(node: &UiNode) -> bool {
    matches!(node.props().text.wrap, UiTextWrapMode::Wrap)
}

fn alert_plain_lines(label: &str) -> Vec<String> {
    let mut lines = label.split('\n');
    let Some(title) = lines.next() else {
        return vec![String::new()];
    };
    let body_lines = lines.collect::<Vec<_>>();
    let title_max_chars = if body_lines.is_empty() {
        ALERT_TITLE_MAX_CHARS
    } else {
        ALERT_BODY_MAX_CHARS
    };
    let mut wrapped = fixed_char_chunks(title, title_max_chars);
    for body in body_lines {
        wrapped.extend(fixed_char_chunks(body, ALERT_BODY_MAX_CHARS));
    }
    wrapped
}

fn fixed_char_chunks(text: &str, max_chars: usize) -> Vec<String> {
    let characters = text.chars().collect::<Vec<_>>();
    let mut chunks = Vec::new();
    for chunk in characters.chunks(max_chars) {
        chunks.push(chunk.iter().collect());
    }
    if chunks.is_empty() {
        chunks.push(String::new());
    }
    chunks
}

fn span_no_wrap_lines(spans: &[UiTextSpan]) -> Vec<Vec<UiTextSpan>> {
    let mut lines = vec![Vec::new()];
    for span in spans {
        push_no_wrap_span(&mut lines, span);
    }
    lines
}

fn push_no_wrap_span(lines: &mut Vec<Vec<UiTextSpan>>, span: &UiTextSpan) {
    for (index, part) in span.text.split('\n').enumerate() {
        if index > 0 {
            lines.push(Vec::new());
        }
        if part.is_empty() {
            continue;
        }
        let mut segment = span.clone();
        segment.text = part.to_string();
        if let Some(line) = lines.last_mut() {
            line.push(segment);
        }
    }
}

fn wrap_plain_line(
    renderer: &TextRenderer,
    line: &str,
    max_width: usize,
    metrics: UiTreeTextMetrics,
) -> Vec<String> {
    let spans = vec![UiTextSpan {
        text: line.to_string(),
        style: Default::default(),
        link_target: String::new(),
    }];
    SpanWrapState::new(max_width, false)
        .tap_segments(SpanTextRenderers::new(renderer, renderer), spans, metrics)
        .into_iter()
        .map(|segments| segments.into_iter().map(|segment| segment.text).collect())
        .collect()
}

trait SpanWrapStateExt {
    fn tap_segments(
        self,
        renderers: SpanTextRenderers<'_>,
        spans: Vec<UiTextSpan>,
        metrics: UiTreeTextMetrics,
    ) -> Vec<Vec<UiTextSpan>>;
}

impl SpanWrapStateExt for SpanWrapState {
    fn tap_segments(
        mut self,
        renderers: SpanTextRenderers<'_>,
        spans: Vec<UiTextSpan>,
        metrics: UiTreeTextMetrics,
    ) -> Vec<Vec<UiTextSpan>> {
        for span in &spans {
            for segment in span_segments(span) {
                self.push(renderers, segment, metrics);
            }
        }
        self.finish()
    }
}

#[cfg(test)]
fn span_width(
    renderers: SpanTextRenderers<'_>,
    span: &UiTextSpan,
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
) -> usize {
    span_part_width(renderers, span, metrics, preserve_whitespace)
}

fn available_width(node: &UiNode, x: usize, area: UiTreeRenderArea) -> usize {
    let explicit_width = dimension_px(&node.props().common.width);
    if explicit_width > 0 {
        return explicit_width
            .saturating_sub(dimension_px(&node.props().common.margin.left))
            .saturating_sub(dimension_px(&node.props().common.margin.right))
            .saturating_sub(dimension_px(&node.props().common.padding.right))
            .max(MIN_TEXT_WIDTH);
    }
    area.width
        .saturating_sub(x.saturating_sub(area.x))
        .saturating_sub(dimension_px(&node.props().common.padding.right))
        .max(MIN_TEXT_WIDTH)
}

fn dimension_px(value: &katana_ui_core::render_model::UiDimension) -> usize {
    match value {
        katana_ui_core::render_model::UiDimension::Px(value) => usize::from(*value),
        _ => 0,
    }
}

fn preserves_whitespace(node: &UiNode) -> bool {
    node.props().text.role == "code" || node.props().font_role == "code"
}

fn wrap_metrics_for_node(node: &UiNode, mut metrics: UiTreeTextMetrics) -> UiTreeTextMetrics {
    if is_compact_document_text(node, metrics) {
        metrics.font_size *= COMPACT_DOCUMENT_RASTER_WRAP_SCALE;
    }
    metrics
}

fn is_compact_document_text(node: &UiNode, metrics: UiTreeTextMetrics) -> bool {
    metrics.font_size <= COMPACT_DOCUMENT_FONT_SIZE
        && matches!(
            node.props().text.role.as_str(),
            "body"
                | "document-export-body"
                | "html-centered"
                | "html-right"
                | "html-left"
                | "html-block"
                | "html-accordion"
                | "html-accordion-body"
                | "html-centered-preview"
                | "html-right-preview"
                | "html-left-preview"
                | "html-block-preview"
                | "html-accordion-preview"
                | "html-accordion-body-preview"
                | "list"
                | "list-item"
                | "blockquote"
                | "footnote"
        )
}

#[cfg(test)]
mod tests {
    use super::{UiTreeTextWrap, available_width, span_segments, span_width};
    use crate::visual::text::TextRenderer;
    use crate::visual::ui_tree_canvas_text_line_width::{SpanTextRenderers, whitespace_width};
    use crate::visual::ui_tree_canvas_text_metrics::{UiTreeDocumentTypography, UiTreeTextMetrics};
    use crate::visual::ui_tree_canvas_types::UiTreeRenderArea;
    use katana_ui_core::atom::Text;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::render_model::{
        UiCommonProps, UiDimension, UiEdgeInsets, UiNode, UiNodeKind, UiTextProps, UiTextSpan,
        UiTextSpanStyle, UiTextWrapMode,
    };
    use katana_ui_core::theme::{FontFamily, FontToken, ThemeSnapshot};

    #[test]
    fn span_segments_preserve_whitespace_only_syntax_spans() {
        let span = UiTextSpan {
            text: " ".to_string(),
            style: UiTextSpanStyle::default(),
            link_target: String::new(),
        };

        let segments = span_segments(&span);

        assert_eq!(1, segments.len());
        assert_eq!(" ", segments[0].text);
    }

    #[test]
    fn code_role_span_wrapping_preserves_leading_whitespace() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "code");
        let node = UiNode::new(UiNodeKind::Text, "    println")
            .font_role("code")
            .text(UiTextProps {
                role: "code".to_string(),
                spans: vec![
                    UiTextSpan {
                        text: "    ".to_string(),
                        style: UiTextSpanStyle::default(),
                        link_target: String::new(),
                    },
                    UiTextSpan {
                        text: "println".to_string(),
                        style: UiTextSpanStyle::default(),
                        link_target: String::new(),
                    },
                ],
                ..UiTextProps::default()
            });

        let metrics = UiTreeTextMetrics::for_node(&node);
        let lines = UiTreeTextWrap::span_lines(
            SpanTextRenderers::new(&renderer, &renderer),
            &node,
            0,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 400,
                height: 80,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert_eq!("    ", lines[0][0].text);
        assert_eq!("println", lines[0][1].text);
    }

    #[test]
    fn wrap_width_uses_same_whitespace_contract_as_span_drawing() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "body");
        let node: UiNode = Text::new("H1 Heading").text_role("body").into();
        let metrics = UiTreeTextMetrics::for_node(&node);
        let space = UiTextSpan::plain(" ");

        let width = span_width(
            SpanTextRenderers::new(&renderer, &renderer),
            &space,
            metrics,
            false,
        );

        assert_eq!(whitespace_width(metrics, false), width);
    }

    #[test]
    fn katana_sample_centering_note_stays_single_line_at_preview_font_14() {
        let text = "↑ \"English | 日本語\" should appear on the same line, centered.";
        let mut theme = ThemeSnapshot::light();
        theme.fonts.push(FontToken {
            name: "document-body".to_string(),
            family: FontFamily::Proportional,
            size: 14.0,
            weight: 400,
        });
        let facade = UiCoreFacade::new(theme.clone());
        let renderer = TextRenderer::load(&facade, "body");
        let node = UiNode::new(UiNodeKind::Text, text).text(UiTextProps {
            role: "body".to_string(),
            spans: vec![UiTextSpan {
                text: text.to_string(),
                style: UiTextSpanStyle::default(),
                link_target: String::new(),
            }],
            wrap: UiTextWrapMode::Wrap,
            ..UiTextProps::default()
        });
        let metrics = UiTreeTextMetrics::for_node_with_typography(
            &node,
            UiTreeDocumentTypography::from_theme(&theme),
        );

        let lines = UiTreeTextWrap::span_lines(
            SpanTextRenderers::new(&renderer, &renderer),
            &node,
            16,
            UiTreeRenderArea {
                x: 16,
                y: 0,
                width: 1248,
                height: 120,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert_eq!(1, lines.len(), "lines={lines:?}");
        let rendered = lines[0]
            .iter()
            .map(|segment| segment.text.as_str())
            .collect::<String>();
        assert_eq!(text, rendered);
    }

    #[test]
    fn katana_sample_intro_wraps_like_preview_font_14() {
        let text = "This document is a comprehensive sample that exercises every rendering feature of KatanA. Open it in KatanA's preview pane to visually verify that all elements render correctly.";
        let mut theme = ThemeSnapshot::light();
        theme.fonts.push(FontToken {
            name: "document-body".to_string(),
            family: FontFamily::Proportional,
            size: 14.0,
            weight: 400,
        });
        let facade = UiCoreFacade::new(theme.clone());
        let renderer = TextRenderer::load(&facade, "document-body");
        let node = UiNode::new(UiNodeKind::Text, text).text(UiTextProps {
            role: "body".to_string(),
            spans: vec![UiTextSpan {
                text: text.to_string(),
                style: UiTextSpanStyle::default(),
                link_target: String::new(),
            }],
            wrap: UiTextWrapMode::Wrap,
            ..UiTextProps::default()
        });
        let metrics = UiTreeTextMetrics::for_node_with_typography(
            &node,
            UiTreeDocumentTypography::from_theme(&theme),
        );

        let lines = UiTreeTextWrap::span_lines(
            SpanTextRenderers::new(&renderer, &renderer),
            &node,
            0,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 1280,
                height: 120,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert_eq!(2, lines.len(), "lines={lines:?}");
        let first = lines[0]
            .iter()
            .map(|segment| segment.text.as_str())
            .collect::<String>();
        let second = lines[1]
            .iter()
            .map(|segment| segment.text.as_str())
            .collect::<String>();
        assert!(first.ends_with("all elements render"), "first={first:?}");
        assert_eq!("correctly.", second);
    }

    #[test]
    fn explicit_width_controls_document_text_wrap_area() {
        let node = UiNode::new(UiNodeKind::Text, "body")
            .common(UiCommonProps::default().width(UiDimension::Px(320)));
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1280,
            height: 120,
            scroll_y: 0.0,
        };

        let width = available_width(&node, 56, area);

        assert_eq!(320, width);
    }

    #[test]
    fn compact_inline_code_wraps_like_viewer_surface() {
        assert_compact_inline_code_wraps_like_viewer_surface("code", "body");
    }

    #[test]
    fn compact_document_code_wraps_like_viewer_surface() {
        assert_compact_inline_code_wraps_like_viewer_surface("document-code", "body");
    }

    #[test]
    fn compact_export_document_code_wraps_like_viewer_surface() {
        assert_compact_inline_code_wraps_like_viewer_surface(
            "document-code",
            "document-export-body",
        );
    }

    fn assert_compact_inline_code_wraps_like_viewer_surface(font_role: &str, text_role: &str) {
        let text = concat!(
            "This is a very long line to verify horizontal scrolling or word wrapping. ",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 repeated. ",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        );
        let mut theme = ThemeSnapshot::light();
        theme.fonts.push(FontToken {
            name: "document-body".to_string(),
            family: FontFamily::Proportional,
            size: 14.0,
            weight: 400,
        });
        theme.fonts.push(FontToken {
            name: "code".to_string(),
            family: FontFamily::Monospace,
            size: 12.0,
            weight: 400,
        });
        let facade = UiCoreFacade::new(theme.clone());
        let body_renderer = TextRenderer::load(&facade, "body");
        let code_renderer = TextRenderer::load(&facade, "code");
        let node = UiNode::new(UiNodeKind::Text, text)
            .font_role(font_role)
            .common(UiCommonProps::default().width(UiDimension::Px(1168)))
            .text(UiTextProps {
                role: text_role.to_string(),
                wrap: UiTextWrapMode::Wrap,
                spans: vec![UiTextSpan {
                    text: text.to_string(),
                    style: UiTextSpanStyle {
                        monospace: true,
                        inline_code: true,
                        ..UiTextSpanStyle::default()
                    },
                    link_target: String::new(),
                }],
                ..UiTextProps::default()
            });
        let metrics = UiTreeTextMetrics::for_node_with_typography(
            &node,
            UiTreeDocumentTypography::from_theme(&theme),
        );

        let lines = UiTreeTextWrap::span_lines(
            SpanTextRenderers::new(&body_renderer, &code_renderer),
            &node,
            56,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 1280,
                height: 160,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert_eq!(3, lines.len(), "lines={lines:?}");
        for line in &lines {
            let width = line
                .iter()
                .map(|span| {
                    span_width(
                        SpanTextRenderers::new(&body_renderer, &code_renderer),
                        span,
                        metrics,
                        false,
                    )
                })
                .sum::<usize>();
            assert!(
                width <= 1168,
                "wrapped inline code line must fit export surface width: width={width} line={line:?}"
            );
        }
    }

    #[test]
    fn alert_plain_text_wraps_body_like_export_surface() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "body");
        let node = UiNode::new(
            UiNodeKind::Text,
            concat!(
                "Note\n",
                "Highlights information that users should take into account, even when skimming."
            ),
        )
        .text(UiTextProps {
            role: "alert".to_string(),
            wrap: UiTextWrapMode::Wrap,
            ..UiTextProps::default()
        });
        let metrics = UiTreeTextMetrics::for_node(&node);

        let lines = UiTreeTextWrap::plain_lines(
            &renderer,
            &node,
            56,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 1280,
                height: 160,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert_eq!(3, lines.len(), "lines={lines:?}");
        assert_eq!("Note", lines[0]);
        assert_eq!(
            "Highlights information that users should take into account",
            lines[1]
        );
        assert_eq!(", even when skimming.", lines[2]);
    }

    #[test]
    fn plain_no_wrap_text_keeps_single_line_when_width_is_small() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "body");
        let node: UiNode = Text::new("This is a very long line that must not wrap")
            .text_role("body")
            .wrap(UiTextWrapMode::NoWrap)
            .into();
        let metrics = UiTreeTextMetrics::for_node(&node);

        let lines = UiTreeTextWrap::plain_lines(
            &renderer,
            &node,
            0,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 120,
                height: 80,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert_eq!(1, lines.len());
        assert_eq!("This is a very long line that must not wrap", lines[0]);
    }

    #[test]
    fn plain_wrap_text_uses_available_width() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "body");
        let node: UiNode = Text::new("This is a very long line that must wrap")
            .text_role("body")
            .wrap(UiTextWrapMode::Wrap)
            .into();
        let metrics = UiTreeTextMetrics::for_node(&node);

        let lines = UiTreeTextWrap::plain_lines(
            &renderer,
            &node,
            0,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 120,
                height: 80,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert!(lines.len() > 1);
    }

    #[test]
    fn plain_wrap_text_subtracts_right_padding_from_available_width() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "body");
        let text = "This line wraps more aggressively when right padding reserves controls.";
        let no_padding: UiNode = Text::new(text)
            .text_role("body")
            .wrap(UiTextWrapMode::Wrap)
            .into();
        let padded: UiNode = Text::new(text)
            .text_role("body")
            .wrap(UiTextWrapMode::Wrap)
            .common(UiCommonProps::default().padding(UiEdgeInsets {
                right: UiDimension::Px(180),
                ..UiEdgeInsets::default()
            }))
            .into();
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 160,
            scroll_y: 0.0,
        };

        let no_padding_lines = UiTreeTextWrap::plain_lines(
            &renderer,
            &no_padding,
            0,
            area,
            UiTreeTextMetrics::for_node(&no_padding),
        );
        let padded_lines = UiTreeTextWrap::plain_lines(
            &renderer,
            &padded,
            0,
            area,
            UiTreeTextMetrics::for_node(&padded),
        );

        assert!(padded_lines.len() > no_padding_lines.len());
    }

    #[test]
    fn explicit_width_subtracts_left_margin_from_available_width() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "body");
        let text = "This line wraps when html margin-left consumes row width.";
        let no_margin: UiNode = Text::new(text)
            .text_role("html-left")
            .wrap(UiTextWrapMode::Wrap)
            .common(UiCommonProps::default().width(UiDimension::Px(320)))
            .into();
        let with_margin: UiNode = Text::new(text)
            .text_role("html-left")
            .wrap(UiTextWrapMode::Wrap)
            .common(
                UiCommonProps::default()
                    .width(UiDimension::Px(320))
                    .margin(UiEdgeInsets {
                        left: UiDimension::Px(180),
                        ..UiEdgeInsets::default()
                    }),
            )
            .into();
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 640,
            height: 160,
            scroll_y: 0.0,
        };

        let no_margin_lines = UiTreeTextWrap::plain_lines(
            &renderer,
            &no_margin,
            0,
            area,
            UiTreeTextMetrics::for_node(&no_margin),
        );
        let with_margin_lines = UiTreeTextWrap::plain_lines(
            &renderer,
            &with_margin,
            0,
            area,
            UiTreeTextMetrics::for_node(&with_margin),
        );

        assert!(
            with_margin_lines.len() > no_margin_lines.len(),
            "html margin-left must reserve width before wrapping: no_margin={no_margin_lines:?} with_margin={with_margin_lines:?}"
        );
    }

    #[test]
    fn html_centered_katana_fixture_sentence_stays_on_one_line_at_preview_width() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "body");
        let text = "Second centered paragraph — should NOT overlap with the first one.";
        let node: UiNode = Text::new(text)
            .text_role("html-centered")
            .wrap(UiTextWrapMode::Wrap)
            .into();
        let metrics = UiTreeTextMetrics::for_node(&node);

        let lines = UiTreeTextWrap::plain_lines(
            &renderer,
            &node,
            14,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 1280,
                height: 120,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert_eq!(vec![text.to_string()], lines);
    }

    #[test]
    fn html_centered_katana_fixture_spans_stay_on_one_line_at_preview_width() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "body");
        let text = "Second centered paragraph — should NOT overlap with the first one.";
        let node = UiNode::new(UiNodeKind::Text, text).text(UiTextProps {
            role: "html-centered".to_string(),
            wrap: UiTextWrapMode::Wrap,
            spans: vec![UiTextSpan::plain(text)],
            ..UiTextProps::default()
        });
        let metrics = UiTreeTextMetrics::for_node(&node);

        let lines = UiTreeTextWrap::span_lines(
            SpanTextRenderers::new(&renderer, &renderer),
            &node,
            14,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 1280,
                height: 120,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert_eq!(1, lines.len());
        let rendered = lines[0]
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>();
        assert_eq!(text, rendered);
    }

    #[test]
    fn span_no_wrap_text_keeps_single_visual_line() {
        let theme = ThemeSnapshot::light();
        let facade = UiCoreFacade::new(theme);
        let renderer = TextRenderer::load(&facade, "body");
        let node = UiNode::new(UiNodeKind::Text, "link text").text(UiTextProps {
            role: "body".to_string(),
            wrap: UiTextWrapMode::NoWrap,
            spans: vec![UiTextSpan {
                text: "This is a very long linked text that must not wrap".to_string(),
                style: UiTextSpanStyle::default(),
                link_target: "https://example.com".to_string(),
            }],
            ..UiTextProps::default()
        });
        let metrics = UiTreeTextMetrics::for_node(&node);

        let lines = UiTreeTextWrap::span_lines(
            SpanTextRenderers::new(&renderer, &renderer),
            &node,
            0,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 120,
                height: 80,
                scroll_y: 0.0,
            },
            metrics,
        );

        assert_eq!(1, lines.len());
        assert_eq!(
            "This is a very long linked text that must not wrap",
            lines[0][0].text
        );
    }
}
