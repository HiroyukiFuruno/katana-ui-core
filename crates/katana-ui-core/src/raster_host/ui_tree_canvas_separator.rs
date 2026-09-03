use super::canvas::Canvas;
use super::ui_tree_canvas_hit_metrics::{TEXT_HEIGHT, dimension_px};
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::UiNode;

const DIVIDER_DEFAULT_RIGHT_INSET: usize = 8;
const MIN_STROKE_WIDTH: usize = 1;
const DIVIDER_VERTICAL_CENTER_DIVISOR: usize = 2;

pub(super) struct UiTreeSeparatorRenderer;

impl UiTreeSeparatorRenderer {
    pub(super) fn draw_divider(
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let requested_height = dimension_px(&node.props().common.height);
        let height = requested_height.max(TEXT_HEIGHT);
        let requested_width = dimension_px(&node.props().common.width);
        let width = if requested_width > 0 {
            requested_width.min(area.width)
        } else {
            area.width.saturating_sub(DIVIDER_DEFAULT_RIGHT_INSET)
        };
        let border = &node.props().common.border;
        let stroke_width = if border.visible {
            usize::from(border.width_px).max(MIN_STROKE_WIDTH)
        } else {
            MIN_STROKE_WIDTH
        };
        let color = if border.visible {
            palette.border_color(&border.color_token)
        } else {
            palette.text
        };
        let line_offset = height
            .saturating_div(DIVIDER_VERTICAL_CENTER_DIVISOR)
            .saturating_add(dimension_px(&node.props().common.padding.top))
            .min(height.saturating_sub(stroke_width));
        canvas.fill_rect(
            x,
            (*y).saturating_add(line_offset),
            width,
            stroke_width,
            color,
        );
        *y = y.saturating_add(height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::render_model::{
        UiBorder, UiCommonProps, UiDimension, UiEdgeInsets, UiNodeKind,
    };
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn divider_uses_requested_geometry_and_visible_border_contract() {
        let palette = UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark());
        let node = UiNode::new(UiNodeKind::Divider, "divider").common(
            UiCommonProps::default()
                .width(UiDimension::Px(200))
                .height(UiDimension::Px(4))
                .padding(UiEdgeInsets {
                    top: UiDimension::Px(20),
                    ..UiEdgeInsets::default()
                })
                .border(UiBorder::solid(3, 0, "document.rule.border")),
        );
        let mut canvas = Canvas::new(120, 80, palette.background);
        let mut y = 5;

        UiTreeSeparatorRenderer::draw_divider(
            &mut canvas,
            &node,
            7,
            &mut y,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 100,
                height: 80,
                scroll_y: 0.0,
            },
            palette,
        );

        assert_eq!(TEXT_HEIGHT, y - 5);
        let line_y = 5 + (TEXT_HEIGHT / 2 + 20).min(TEXT_HEIGHT - 3);
        assert_eq!(
            palette.document_rule_border,
            canvas.pixels()[line_y * 120 + 7]
        );
        assert_eq!(
            palette.document_rule_border,
            canvas.pixels()[line_y * 120 + 9]
        );
    }
}
