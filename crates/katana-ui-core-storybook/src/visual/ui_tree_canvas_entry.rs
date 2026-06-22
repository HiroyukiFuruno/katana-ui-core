use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas_hover::UiTreeCanvasHover;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::render_model::UiNode;

const TEXT_HEIGHT: usize = 20;
const TEXT_SIZE: f32 = 14.0;
const INPUT_WIDTH: usize = 132;
const CONTROL_HEIGHT: usize = 18;
const INPUT_TEXT_X_PADDING: usize = 6;
const INPUT_TEXT_Y_PADDING: usize = 3;
const SELECT_ARROW_RIGHT_INSET: usize = 14;
const SELECT_ARROW_Y_OFFSET_FROM_NEXT_ROW: usize = 15;
const SELECT_ARROW_SIZE: usize = 6;
const LABEL_COLUMN_WIDTH: usize = 112;

pub(super) struct UiTreeEntryRenderer;

impl UiTreeEntryRenderer {
    pub(super) fn draw_text_entry(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
        show_label: bool,
    ) {
        let text_value = entry_value(node);
        let input_x = input_x(canvas, text, node, x, *y, palette, show_label);
        canvas.fill_rect(
            input_x,
            *y,
            INPUT_WIDTH,
            CONTROL_HEIGHT,
            palette.code_background,
        );
        canvas.fill_rect(input_x, *y, INPUT_WIDTH, 1, palette.muted_border);
        canvas.fill_rect(
            input_x,
            *y + CONTROL_HEIGHT - 1,
            INPUT_WIDTH,
            1,
            palette.muted_border,
        );
        UiTreeCanvasHover::draw_node_border(
            canvas,
            node,
            input_x,
            *y,
            INPUT_WIDTH,
            CONTROL_HEIGHT,
            palette,
        );
        text.draw(
            canvas,
            &text_value,
            input_x + INPUT_TEXT_X_PADDING,
            (*y).saturating_add(INPUT_TEXT_Y_PADDING),
            TEXT_SIZE,
            palette.text,
        );
        *y = y.saturating_add(TEXT_HEIGHT);
    }

    pub(super) fn draw_select(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
        show_label: bool,
    ) {
        Self::draw_text_entry(canvas, text, node, x, y, palette, show_label);
        canvas.fill_rect(
            x + INPUT_WIDTH.saturating_sub(SELECT_ARROW_RIGHT_INSET),
            *y - SELECT_ARROW_Y_OFFSET_FROM_NEXT_ROW,
            SELECT_ARROW_SIZE,
            SELECT_ARROW_SIZE,
            palette.selection,
        );
    }
}

fn input_x(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    x: usize,
    y: usize,
    palette: UiTreeCanvasPalette,
    show_label: bool,
) -> usize {
    if !show_label {
        return x;
    }
    text.draw(canvas, &node.props().label, x, y, TEXT_SIZE, palette.text);
    x + LABEL_COLUMN_WIDTH
}

fn entry_value(node: &UiNode) -> String {
    if !node.props().interaction.value.is_empty() {
        return node.props().interaction.value.clone();
    }
    if !node.props().placeholder.is_empty() {
        return node.props().placeholder.clone();
    }
    node.props().label.clone()
}
