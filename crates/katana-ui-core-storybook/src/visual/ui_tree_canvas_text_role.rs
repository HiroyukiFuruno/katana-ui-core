use super::canvas::Canvas;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
#[cfg(test)]
use super::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;
use super::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiDimension, UiNode, UiTone};

const ALERT_STRIPE_WIDTH: usize = 5;
const ALERT_PANEL_PADDING_Y: usize = 16;
const QUOTE_STRIPE_WIDTH: usize = 4;
const QUOTE_INDENT: usize = 32;
const CODE_BLOCK_VERTICAL_MARGIN: usize = 14;
const LIST_DEPTH_INDENT: usize = 40;
const LIST_BULLET_X_OFFSET: usize = 14;
const LIST_BULLET_RADIUS: isize = 4;
const LIST_SQUARE_SIZE: usize = 8;
const ALERT_TITLE_TEXT_X_OFFSET: usize = 58;
const ALERT_BODY_TEXT_X_OFFSET: usize = ALERT_ICON_X_OFFSET;
const ALERT_ICON_X_OFFSET: usize = 28;
const ALERT_ICON_Y_OFFSET: usize = 18;
const ALERT_ICON_CENTER: usize = 10;
const NOTE_ICON_RADIUS: usize = 8;
const TIP_ICON_RADIUS: usize = 5;
const DIAGRAM_CONTROL_SPACER_Y_OFFSET: usize = 9;
const DIAGRAM_CONTROL_SPACER_WIDTH: usize = 12;
const DIAGRAM_CONTROL_SPACER_HEIGHT: usize = 1;
const NOTE_ICON_STEM_TOP: usize = 8;
const NOTE_ICON_STEM_BOTTOM: usize = 14;
const NOTE_ICON_DOT_Y: usize = 5;
const NOTE_ICON_DOT_RADIUS: usize = 1;
const TIP_ICON_CENTER_Y: usize = 7;
const TIP_ICON_LINE_START_X: usize = 7;
const TIP_ICON_LINE_END_X: usize = 13;
const TIP_ICON_LINE_Y: usize = 12;
const TIP_ICON_BASE_START_X: usize = 8;
const TIP_ICON_BASE_END_X: usize = 12;
const TIP_ICON_BASE_Y: usize = 15;
const IMPORTANT_ICON_OUTLINE: [(usize, usize); 5] = [(3, 4), (17, 4), (17, 14), (6, 18), (3, 4)];
const WARNING_ICON_OUTLINE: [(usize, usize); 4] =
    [(2, 17), (ALERT_ICON_CENTER, 2), (18, 17), (2, 17)];
const CAUTION_ICON_OUTLINE: [(usize, usize); 7] =
    [(7, 2), (13, 2), (18, 7), (18, 13), (13, 18), (2, 7), (7, 2)];
const IMPORTANT_ICON_STEM_TOP: usize = 7;
const IMPORTANT_ICON_STEM_BOTTOM: usize = 10;
const IMPORTANT_ICON_DOT_Y: usize = 13;
const WARNING_ICON_STEM_TOP: usize = 7;
const WARNING_ICON_STEM_BOTTOM: usize = 11;
const WARNING_ICON_DOT_Y: usize = 14;
const CAUTION_ICON_STEM_TOP: usize = 6;
const CAUTION_ICON_STEM_BOTTOM: usize = 11;
const CAUTION_ICON_DOT_Y: usize = 14;
const ALERT_ICON_DOT_RADIUS: usize = 1;
const CIRCLE_ARC_CLEAR_Y_OFFSET: usize = 7;
const CIRCLE_ARC_CLEAR_EXTRA_WIDTH: usize = 2;
const CIRCLE_ARC_CLEAR_EXTRA_HEIGHT: usize = 3;
const SQUARE_BULLET_X_INSET: usize = 3;
const CIRCLE_SPAN_WIDE_MAX_DY: isize = 2;
const CIRCLE_SPAN_WIDE: isize = 4;
const CIRCLE_SPAN_MID_DY: isize = 3;
const CIRCLE_SPAN_MID: isize = 3;
const CIRCLE_SPAN_NARROW: isize = 1;
const HTML_ALIGNMENT_ORIGIN_OFFSET_PX: isize = 4;

#[path = "ui_tree_canvas_text_role_alert.rs"]
mod alert;
#[path = "ui_tree_canvas_text_role_block.rs"]
mod block;
#[path = "ui_tree_canvas_text_role_geometry.rs"]
mod geometry;
#[path = "ui_tree_canvas_text_role_list.rs"]
mod list;
use alert::alert_accent;
use alert::draw_alert;
use block::{draw_code, draw_heading, draw_media_error, draw_quote, draw_table};
use geometry::{
    code_text_padding_left, dimension_px, list_depth, quote_depth, quote_text_padding_left,
    remaining_width,
};
use list::{draw_filled_bullet, draw_list_marker};

pub(super) struct UiTreeTextRoleRenderer;

impl UiTreeTextRoleRenderer {
    pub(super) fn draw_background(
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
        metrics: UiTreeTextMetrics,
    ) {
        match node.props().text.role.as_str() {
            role if role.starts_with("heading") => {
                draw_heading(canvas, node, x, y, area, palette, metrics)
            }
            "code" => draw_code(canvas, node, x, y, area, palette, metrics),
            "table" => draw_table(canvas, x, y, area, palette, metrics),
            "alert" => draw_alert(canvas, node, x, y, area, palette, metrics),
            "blockquote" => draw_quote(canvas, node, x, y, area, palette, metrics),
            "footnote" => {}
            "list-marker" => draw_list_marker(canvas, node, x, y, palette, metrics),
            "media-pending" => {
                canvas.fill_rect(
                    x,
                    y,
                    remaining_width(area, x),
                    metrics.background_height,
                    palette.pending_background,
                );
            }
            "media-error" => draw_media_error(canvas, x, y, area, palette, metrics),
            "diagram-control-spacer" => {
                canvas.fill_rect(
                    x,
                    y.saturating_add(DIAGRAM_CONTROL_SPACER_Y_OFFSET),
                    DIAGRAM_CONTROL_SPACER_WIDTH,
                    DIAGRAM_CONTROL_SPACER_HEIGHT,
                    palette.muted_border,
                );
            }
            _ => {}
        }
    }

    pub(super) fn aligned_x(
        node: &UiNode,
        x: usize,
        area: UiTreeRenderArea,
        width: usize,
    ) -> isize {
        match node.props().text.role.as_str() {
            "code" if code_text_padding_left(node) > 0 => {
                x.saturating_add(quote_depth(node).saturating_mul(QUOTE_INDENT))
                    .saturating_add(code_text_padding_left(node)) as isize
            }
            "blockquote" if quote_text_padding_left(node) > 0 => {
                x.saturating_add(quote_depth(node).saturating_mul(QUOTE_INDENT))
                    .saturating_add(quote_text_padding_left(node)) as isize
            }
            "blockquote" => {
                x.saturating_add(quote_depth(node).saturating_mul(QUOTE_INDENT)) as isize
            }
            role if is_center_aligned_role(role) => {
                let alignment_width = alignment_width(node, area);
                x as isize
                    + (alignment_width as isize - width as isize) / 2
                    + html_alignment_origin_offset(role)
            }
            role if is_right_aligned_role(role) => {
                let alignment_width = alignment_width(node, area);
                x as isize + alignment_width as isize - width as isize
                    + html_alignment_origin_offset(role)
            }
            _ => x as isize,
        }
    }

    pub(super) fn line_color(
        node: &UiNode,
        palette: UiTreeCanvasPalette,
        line_index: usize,
    ) -> u32 {
        if node.props().text.role == "alert" && line_index == 0 {
            return alert_accent(node, palette);
        }
        palette.text
    }

    pub(super) fn line_bold(node: &UiNode, line_index: usize) -> bool {
        let role = node.props().text.role.as_str();
        (role == "alert" && line_index == 0) || is_strong_heading_text_role(role)
    }

    pub(super) fn line_x(
        node: &UiNode,
        origin_x: usize,
        x: usize,
        area: UiTreeRenderArea,
        width: usize,
        line_index: usize,
    ) -> isize {
        if node.props().text.role == "alert" {
            return origin_x.saturating_add(if line_index == 0 {
                ALERT_TITLE_TEXT_X_OFFSET
            } else {
                ALERT_BODY_TEXT_X_OFFSET
            }) as isize;
        }
        Self::aligned_x(node, x, area, width)
    }
}

fn is_center_aligned_role(role: &str) -> bool {
    matches!(role, "html-centered" | "html-centered-preview") || role.ends_with("-html-centered")
}

fn is_right_aligned_role(role: &str) -> bool {
    matches!(role, "html-right" | "html-right-preview") || role.ends_with("-html-right")
}

fn is_strong_heading_text_role(role: &str) -> bool {
    matches!(
        role,
        "heading-export" | "heading-2-export" | "heading-3-export"
    )
}

fn html_alignment_origin_offset(role: &str) -> isize {
    if role.starts_with("html-") || role.contains("-html-") {
        return HTML_ALIGNMENT_ORIGIN_OFFSET_PX;
    }
    0
}

fn alignment_width(node: &UiNode, area: UiTreeRenderArea) -> usize {
    let requested = dimension_px(&node.props().common.width);
    if requested > 0 {
        return requested;
    }
    area.width
}

#[cfg(test)]
mod tests {
    use super::list::list_marker_center_y;
    use super::{UiTreeDocumentTypography, UiTreeTextMetrics, UiTreeTextRoleRenderer};
    use crate::visual::canvas::Canvas;
    use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use crate::visual::ui_tree_canvas_types::UiTreeRenderArea;
    use katana_ui_core::atom::Text;
    use katana_ui_core::render_model::{UiNode, UiNodeKind, UiTextProps};
    use katana_ui_core::theme::{ColorToken, FontFamily, FontToken, ThemeSnapshot};

    #[test]
    fn list_marker_center_uses_scaled_document_line_middle() {
        let node: UiNode = Text::new("  ").text_role("list-marker").into();
        let compact = UiTreeTextMetrics::for_node_with_typography(&node, compact_typography());
        let full = UiTreeTextMetrics::for_node(&node);

        assert_eq!(11, list_marker_center_y(0, compact));
        assert_eq!(17, list_marker_center_y(0, full));
    }

    fn compact_typography() -> UiTreeDocumentTypography {
        let mut theme = ThemeSnapshot::dark();
        theme.fonts.push(FontToken {
            name: "document-body".to_string(),
            family: FontFamily::Proportional,
            size: 14.0,
            weight: 400,
        });
        UiTreeDocumentTypography::from_theme(&theme)
    }

    #[test]
    fn alert_body_text_starts_under_icon_column_like_katana_preview() {
        let node = UiNode::new(UiNodeKind::Text, "Tip\nbody").text(UiTextProps {
            role: "alert".to_string(),
            ..UiTextProps::default()
        });
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 200,
            height: 120,
            scroll_y: 0.0,
        };

        assert_eq!(66, UiTreeTextRoleRenderer::line_x(&node, 8, 8, area, 0, 0));
        assert_eq!(36, UiTreeTextRoleRenderer::line_x(&node, 8, 8, area, 0, 1));
        assert!(UiTreeTextRoleRenderer::line_bold(&node, 0));
        assert!(!UiTreeTextRoleRenderer::line_bold(&node, 1));
    }

    #[test]
    fn markdown_preview_heading_roles_use_heading_size_without_font_weight() {
        for role in [
            "heading",
            "heading-2",
            "heading-2-long",
            "heading-3",
            "heading-html-centered",
            "heading-2-html-centered",
            "heading-3-html-centered",
        ] {
            let node = UiNode::new(UiNodeKind::Text, "KatanA Heading").text(UiTextProps {
                role: role.to_string(),
                ..UiTextProps::default()
            });

            assert!(
                !UiTreeTextRoleRenderer::line_bold(&node, 0),
                "{role} should match KatanA preview: egui strong changes color, not font weight"
            );
        }
    }

    #[test]
    fn export_heading_roles_keep_bold_font_weight_for_surface_parity() {
        for role in ["heading-export", "heading-2-export", "heading-3-export"] {
            let node = UiNode::new(UiNodeKind::Text, "KatanA Heading").text(UiTextProps {
                role: role.to_string(),
                ..UiTextProps::default()
            });

            assert!(
                UiTreeTextRoleRenderer::line_bold(&node, 0),
                "{role} should render as bold for export surface headings"
            );
        }
    }

    #[test]
    fn footnote_role_does_not_paint_background_or_rule() {
        let mut theme = ThemeSnapshot::dark();
        theme
            .colors
            .push(color_token("footnote-background", 0x243041));
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let mut canvas = Canvas::new(180, 40, palette.background);
        let node = UiNode::new(UiNodeKind::Text, "1. footnote").text(UiTextProps {
            role: "footnote".to_string(),
            ..UiTextProps::default()
        });
        let metrics = UiTreeTextMetrics::for_node_with_typography(&node, compact_typography());
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 180,
            height: 40,
            scroll_y: 0.0,
        };

        UiTreeTextRoleRenderer::draw_background(&mut canvas, &node, 8, 4, area, palette, metrics);

        assert_eq!(0, canvas.non_background_pixels(palette.background));
    }

    fn color_token(name: &str, rgb: u32) -> ColorToken {
        ColorToken {
            name: name.to_string(),
            rgba: [
                ((rgb >> 16) & 0xff) as u8,
                ((rgb >> 8) & 0xff) as u8,
                (rgb & 0xff) as u8,
                255,
            ],
        }
    }
}
