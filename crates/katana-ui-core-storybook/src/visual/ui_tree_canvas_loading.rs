use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas_hit_metrics::TEXT_HEIGHT;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::render_model::{UiAnimationState, UiNode};

const TEXT_SIZE: f32 = 14.0;
const LOADING_DOT_CENTER_Y_OFFSET: usize = 6;
const LOADING_LABEL_X_OFFSET: usize = 24;
const LOADING_PHASE_COUNT: usize = 3;
const LOADING_DOT_GAP: usize = 7;
const LOADING_DOT_SIZE: usize = 4;

pub(super) struct UiTreeLoadingRenderer;

impl UiTreeLoadingRenderer {
    pub(super) fn draw(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
    ) {
        let center_y = (*y).saturating_add(LOADING_DOT_CENTER_Y_OFFSET);
        draw_loading_dots(canvas, node, x, center_y, palette);
        if !node.props().label.is_empty() {
            text.draw(
                canvas,
                &node.props().label,
                x + LOADING_LABEL_X_OFFSET,
                *y,
                TEXT_SIZE,
                palette.text,
            );
        }
        *y = y.saturating_add(TEXT_HEIGHT);
    }
}

fn draw_loading_dots(
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    center_y: usize,
    palette: UiTreeCanvasPalette,
) {
    let active = loading_phase(node) % LOADING_PHASE_COUNT;
    for index in 0..LOADING_PHASE_COUNT {
        let color = if index == active {
            palette.text
        } else {
            palette.selection
        };
        canvas.fill_rect(
            x + index.saturating_mul(LOADING_DOT_GAP),
            center_y,
            LOADING_DOT_SIZE,
            LOADING_DOT_SIZE,
            color,
        );
    }
}

fn loading_phase(node: &UiNode) -> usize {
    let indicator = &node.props().loading_indicator;
    let animated = !indicator.reduced_motion
        && !matches!(
            indicator.animation_state,
            UiAnimationState::Idle | UiAnimationState::Paused
        );
    usize::from(node.props().interaction.animation_phase) * usize::from(animated)
}
