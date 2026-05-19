use super::button_options;
use super::canvas::Canvas;
use super::layout_metrics::{INSPECTOR_HEIGHT, INSPECTOR_WIDTH, INSPECTOR_X, INSPECTOR_Y};
use super::palette::VisualPalette;
use super::render_context::{RenderContext, ScenarioContext};
use super::text::TextRenderer;
use super::{inspector_rows, inspector_rows::settings_title};
use crate::catalog::StoryExample;
use katana_ui_core::render_model::UiNode;

const SECTION_X: usize = INSPECTOR_X + 18;
const SECTION_WIDTH: usize = INSPECTOR_WIDTH - 36;
const TITLE_Y: usize = INSPECTOR_Y + 18;
const SUBTITLE_Y: usize = INSPECTOR_Y + 44;
const FIRST_SECTION_Y: usize = INSPECTOR_Y + 78;
const SECTION_GAP: usize = 18;
const ROW_HEIGHT: usize = 24;
const TEXT_X: usize = SECTION_X + 10;
const TITLE_SIZE: f32 = 17.0;
const META_SIZE: f32 = 11.0;
const BODY_SIZE: f32 = 12.0;
const CODE_SIZE: f32 = 10.5;
const SECTION_HEADER_HEIGHT: usize = 38;
const SECTION_ACCENT_WIDTH: usize = 4;
const SECTION_TITLE_Y_OFFSET: usize = 12;
const FIRST_ROW_Y_OFFSET: usize = 34;

pub(super) fn draw(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    selected: Option<(&UiNode, &StoryExample)>,
    scenario: ScenarioContext<'_>,
) {
    let palette = render.palette;
    canvas.fill_rect(
        INSPECTOR_X,
        INSPECTOR_Y,
        INSPECTOR_WIDTH,
        INSPECTOR_HEIGHT,
        palette.surface,
    );
    canvas.stroke_rect(
        INSPECTOR_X,
        INSPECTOR_Y,
        INSPECTOR_WIDTH,
        INSPECTOR_HEIGHT,
        palette.border,
    );
    render.text.draw(
        canvas,
        "Inspector",
        SECTION_X,
        TITLE_Y,
        TITLE_SIZE,
        palette.text,
    );
    render.text.draw(
        canvas,
        "settings / state / event / action",
        SECTION_X,
        SUBTITLE_Y,
        META_SIZE,
        palette.muted,
    );

    let mut y = FIRST_SECTION_Y.saturating_sub(scenario.panel_scroll.inspector_y);
    let Some((node, example)) = selected else {
        draw_section(canvas, render.text, palette, "No selection", &[], y);
        return;
    };

    y = draw_settings(canvas, render, node, example, scenario, y);
    y = draw_state(canvas, render, node, scenario, y + SECTION_GAP);
    y = draw_history(canvas, render, example, scenario, y + SECTION_GAP);
    draw_quality(canvas, render.text, palette, scenario, y + SECTION_GAP);
}

fn draw_settings(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    node: &UiNode,
    example: &StoryExample,
    scenario: ScenarioContext<'_>,
    y: usize,
) -> usize {
    if button_options::is_button_page(example.page) {
        return button_options::draw_controls(canvas, render.text, render.palette, scenario, y);
    }
    draw_section(
        canvas,
        render.code_text,
        render.palette,
        settings_title(example),
        &inspector_rows::settings_rows(node, example, scenario),
        y,
    )
}

fn draw_state(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    node: &UiNode,
    scenario: ScenarioContext<'_>,
    y: usize,
) -> usize {
    draw_section(
        canvas,
        render.code_text,
        render.palette,
        "State",
        &inspector_rows::state_rows(node, scenario),
        y,
    )
}

fn draw_history(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    example: &StoryExample,
    scenario: ScenarioContext<'_>,
    y: usize,
) -> usize {
    draw_section(
        canvas,
        render.code_text,
        render.palette,
        "Event / Action",
        &inspector_rows::history_rows(example, scenario),
        y,
    )
}

fn draw_quality(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    y: usize,
) {
    draw_section(
        canvas,
        text,
        palette,
        "Quality",
        &inspector_rows::quality_rows(scenario),
        y,
    );
}

fn draw_section(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    title: &str,
    rows: &[String],
    y: usize,
) -> usize {
    let height = SECTION_HEADER_HEIGHT + rows.len() * ROW_HEIGHT;
    canvas.fill_rect(SECTION_X, y, SECTION_WIDTH, height, palette.code_background);
    canvas.stroke_rect(SECTION_X, y, SECTION_WIDTH, height, palette.border);
    canvas.fill_rect(SECTION_X, y, SECTION_ACCENT_WIDTH, height, palette.accent);
    text.draw(
        canvas,
        title,
        TEXT_X,
        y + SECTION_TITLE_Y_OFFSET,
        BODY_SIZE,
        palette.text,
    );
    let mut row_y = y + FIRST_ROW_Y_OFFSET;
    for row in rows {
        text.draw(canvas, row, TEXT_X, row_y, CODE_SIZE, palette.muted);
        row_y += ROW_HEIGHT;
    }
    y + height
}
