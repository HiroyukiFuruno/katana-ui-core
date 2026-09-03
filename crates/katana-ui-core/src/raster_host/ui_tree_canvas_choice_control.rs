use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas_hover::UiTreeCanvasHover;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::render_model::UiNode;

const TEXT_SIZE: f32 = 14.0;
const ROW_HEIGHT: usize = 22;
const CONTROL_SIZE: usize = 14;
const LABEL_GAP: usize = 8;
const SWATCH_SIZE: usize = 18;
const SLIDE_WIDTH: usize = 72;
const SLIDE_HEIGHT: usize = 22;
const TRACK_Y_OFFSET: usize = 10;
const THUMB_WIDTH: usize = 8;
const RADIO_DOT_INSET: usize = 4;
const RADIO_DOT_SIZE: usize = 6;
const SLIDER_TRACK_HEIGHT: usize = 2;
const SLIDER_THUMB_Y_OFFSET: usize = 4;
const SLIDER_THUMB_HEIGHT: usize = 14;

pub(super) struct UiTreeChoiceControlRenderer;

impl UiTreeChoiceControlRenderer {
    pub(super) fn draw_radio(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
    ) {
        canvas.stroke_rect(x, *y, CONTROL_SIZE, CONTROL_SIZE, palette.text);
        if node.props().checked {
            canvas.fill_rect(
                x + RADIO_DOT_INSET,
                y.saturating_add(RADIO_DOT_INSET),
                RADIO_DOT_SIZE,
                RADIO_DOT_SIZE,
                palette.selection,
            );
        }
        draw_label(
            canvas,
            text,
            node,
            x + CONTROL_SIZE + LABEL_GAP,
            *y,
            palette,
        );
        UiTreeCanvasHover::draw_node_border(
            canvas,
            node,
            x,
            *y,
            row_width(text, node),
            ROW_HEIGHT,
            palette,
        );
        *y = y.saturating_add(ROW_HEIGHT);
    }

    pub(super) fn draw_color_swatch(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
    ) {
        canvas.fill_rect(x, *y, SWATCH_SIZE, SWATCH_SIZE, palette.selection);
        canvas.stroke_rect(x, *y, SWATCH_SIZE, SWATCH_SIZE, palette.muted_border);
        draw_label(canvas, text, node, x + SWATCH_SIZE + LABEL_GAP, *y, palette);
        UiTreeCanvasHover::draw_node_border(
            canvas,
            node,
            x,
            *y,
            row_width(text, node),
            ROW_HEIGHT,
            palette,
        );
        *y = y.saturating_add(ROW_HEIGHT);
    }

    pub(super) fn draw_slide_control(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
    ) {
        canvas.fill_rect(
            x,
            y.saturating_add(TRACK_Y_OFFSET),
            SLIDE_WIDTH,
            SLIDER_TRACK_HEIGHT,
            palette.muted_border,
        );
        canvas.fill_rect(
            x + SLIDE_WIDTH / 2,
            y.saturating_add(SLIDER_THUMB_Y_OFFSET),
            THUMB_WIDTH,
            SLIDER_THUMB_HEIGHT,
            palette.selection,
        );
        draw_label(canvas, text, node, x + SLIDE_WIDTH + LABEL_GAP, *y, palette);
        UiTreeCanvasHover::draw_node_border(
            canvas,
            node,
            x,
            *y,
            row_width(text, node),
            SLIDE_HEIGHT,
            palette,
        );
        *y = y.saturating_add(SLIDE_HEIGHT);
    }
}

fn draw_label(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    x: usize,
    y: usize,
    palette: UiTreeCanvasPalette,
) {
    if node.props().label.is_empty() {
        return;
    }
    text.draw(canvas, &node.props().label, x, y, TEXT_SIZE, palette.text);
}

fn row_width(text: &TextRenderer, node: &UiNode) -> usize {
    if node.props().label.is_empty() {
        return SLIDE_WIDTH;
    }
    SLIDE_WIDTH
        .saturating_add(LABEL_GAP)
        .saturating_add(text.measure_width(&node.props().label, TEXT_SIZE))
        .max(CONTROL_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::atom::Radio;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn checked_radio_draws_dot_and_empty_label_uses_control_width() {
        let palette = UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark());
        let text = TextRenderer::load(&UiCoreFacade::default(), "body");
        let checked: UiNode = Radio::new("").selected(true).into();
        let mut canvas = Canvas::new(100, 40, palette.background);
        let mut y = 4;

        UiTreeChoiceControlRenderer::draw_radio(&mut canvas, &text, &checked, 4, &mut y, palette);

        assert_eq!(4 + ROW_HEIGHT, y);
        assert_eq!(
            palette.selection,
            canvas.pixels()[(4 + RADIO_DOT_INSET) * canvas.width() + 4 + RADIO_DOT_INSET]
        );
        assert_eq!(SLIDE_WIDTH, row_width(&text, &checked));
        assert!(canvas.text_runs().is_empty());
    }
}
