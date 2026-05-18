use super::canvas::Canvas;
use super::layout_metrics::{PREVIEW_X, STORY_CARD_STEP_Y};
use super::preview_contract_rows::{contract_rows, status_rows};
use super::render_context::{RenderContext, ScenarioContext};
use crate::catalog::StoryExample;
use katana_ui_core::render_model::UiNode;

const CONTRACT_Y: usize = 930;
const CONTRACT_WIDTH: usize = 710;
const CONTRACT_HEADER_HEIGHT: usize = 42;
const CONTRACT_ROW_HEIGHT: usize = 28;
const CONTRACT_SECTION_GAP: usize = 18;
const CONTRACT_ACCENT_WIDTH: usize = 5;
const CONTRACT_TEXT_X: usize = PREVIEW_X + 18;
const CONTRACT_VALUE_X: usize = PREVIEW_X + 170;
const CONTRACT_TITLE_Y_OFFSET: usize = 13;
const CONTRACT_TITLE_SIZE: f32 = 14.0;
const CONTRACT_ROW_SIZE: f32 = 11.0;
const CONTRACT_CODE_SIZE: f32 = 10.5;
const CONTRACT_ROWS: usize = 8;
const CONTRACT_ROW_TEXT_Y_OFFSET: usize = 8;

pub(super) struct PreviewContract;

impl PreviewContract {
    pub(super) fn draw(
        canvas: &mut Canvas,
        preview: &UiNode,
        render: RenderContext<'_>,
        scenario: ScenarioContext<'_>,
    ) {
        let Some((node, example)) = selected_pair(preview, render.examples, scenario.selected_page)
        else {
            return;
        };
        let base_y = CONTRACT_Y + scenario.preset_index * STORY_CARD_STEP_Y / 2;
        draw_contract_table(canvas, render, node, example, base_y);
        draw_status_table(
            canvas,
            render,
            example,
            base_y
                + CONTRACT_HEADER_HEIGHT
                + CONTRACT_ROWS * CONTRACT_ROW_HEIGHT
                + CONTRACT_SECTION_GAP,
        );
    }
}

fn draw_contract_table(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    node: &UiNode,
    example: &StoryExample,
    y: usize,
) {
    let rows = contract_rows(node, example);
    draw_table(canvas, render, "Component contract", &rows, y);
}

fn draw_status_table(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    example: &StoryExample,
    y: usize,
) {
    let rows = status_rows(example);
    draw_table(canvas, render, "Implementation status", &rows, y);
}

fn draw_table(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    title: &str,
    rows: &[(&str, String)],
    y: usize,
) {
    let height = CONTRACT_HEADER_HEIGHT + rows.len() * CONTRACT_ROW_HEIGHT;
    canvas.fill_rect(PREVIEW_X, y, CONTRACT_WIDTH, height, render.palette.surface);
    canvas.stroke_rect(PREVIEW_X, y, CONTRACT_WIDTH, height, render.palette.border);
    canvas.fill_rect(
        PREVIEW_X,
        y,
        CONTRACT_ACCENT_WIDTH,
        height,
        render.palette.accent,
    );
    render.text.draw(
        canvas,
        title,
        CONTRACT_TEXT_X,
        y + CONTRACT_TITLE_Y_OFFSET,
        CONTRACT_TITLE_SIZE,
        render.palette.text,
    );
    let mut row_y = y + CONTRACT_HEADER_HEIGHT;
    for (label, value) in rows {
        canvas.fill_rect(
            PREVIEW_X + CONTRACT_ACCENT_WIDTH,
            row_y,
            CONTRACT_WIDTH - CONTRACT_ACCENT_WIDTH,
            1,
            render.palette.border,
        );
        render.text.draw(
            canvas,
            label,
            CONTRACT_TEXT_X,
            row_y + CONTRACT_ROW_TEXT_Y_OFFSET,
            CONTRACT_ROW_SIZE,
            render.palette.muted,
        );
        render.code_text.draw(
            canvas,
            value,
            CONTRACT_VALUE_X,
            row_y + CONTRACT_ROW_TEXT_Y_OFFSET,
            CONTRACT_CODE_SIZE,
            render.palette.text,
        );
        row_y += CONTRACT_ROW_HEIGHT;
    }
}

fn selected_pair<'a>(
    preview: &'a UiNode,
    examples: &'a [StoryExample],
    selected_page: &str,
) -> Option<(&'a UiNode, &'a StoryExample)> {
    preview
        .children()
        .iter()
        .zip(examples.iter())
        .find(|(_, example)| example.page == selected_page)
}
