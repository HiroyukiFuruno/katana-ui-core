use super::canvas::Canvas;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::render_model::{UiBorder, UiNode};

pub(super) struct UiTreeCanvasHover;

impl UiTreeCanvasHover {
    pub(super) fn draw_node_border(
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        palette: UiTreeCanvasPalette,
    ) {
        if !node.props().interaction.hovered || node.props().disabled {
            return;
        }
        Self::draw_border(
            canvas,
            &node.props().common.hover_border,
            x,
            y,
            width,
            height,
            palette,
        );
    }

    pub(super) fn draw_border(
        canvas: &mut Canvas,
        border: &UiBorder,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        palette: UiTreeCanvasPalette,
    ) {
        if !border.visible || border.width_px == 0 || width == 0 || height == 0 {
            return;
        }
        for inset in 0..usize::from(border.width_px) {
            let inset_width = width.saturating_sub(inset * 2);
            let inset_height = height.saturating_sub(inset * 2);
            canvas.stroke_rect(
                x.saturating_add(inset),
                y.saturating_add(inset),
                inset_width,
                inset_height,
                Self::border_color(border, palette),
            );
        }
    }

    fn border_color(border: &UiBorder, palette: UiTreeCanvasPalette) -> u32 {
        match border.color_token.as_str() {
            "border" | "control.border" => palette.muted_border,
            _ => palette.visual.hover_border,
        }
    }
}
