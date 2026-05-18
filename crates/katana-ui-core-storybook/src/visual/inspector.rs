use super::canvas::Canvas;
use super::layout_metrics::{INSPECTOR_HEIGHT, INSPECTOR_WIDTH, INSPECTOR_X, INSPECTOR_Y};
use super::palette::VisualPalette;
use super::render_context::RenderContext;
use super::text::TextRenderer;
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
const HISTORY_ROW_LIMIT: usize = 3;

pub(super) fn draw(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    selected: Option<(&UiNode, &StoryExample)>,
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

    let mut y = FIRST_SECTION_Y;
    let Some((node, example)) = selected else {
        draw_section(canvas, render.text, palette, "No selection", &[], y);
        return;
    };

    y = draw_settings(canvas, render, node, example, y);
    y = draw_state(canvas, render, node, y + SECTION_GAP);
    y = draw_history(canvas, render, example, y + SECTION_GAP);
    draw_quality(canvas, render.text, palette, y + SECTION_GAP);
}

fn draw_settings(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    node: &UiNode,
    example: &StoryExample,
    y: usize,
) -> usize {
    if example.page == "tree-view" {
        return draw_tree_view_settings(canvas, render, y);
    }
    let props = node.props();
    let rows = [
        format!("variant: {:?}", props.variant),
        format!("tone: {:?}", props.tone),
        format!("size: {:?}", props.size),
        format!("font: {}", props.font_role),
    ];
    draw_section(
        canvas,
        render.code_text,
        render.palette,
        "Settings",
        &rows,
        y,
    )
}

fn draw_tree_view_settings(canvas: &mut Canvas, render: RenderContext<'_>, y: usize) -> usize {
    let rows = [
        "lines: solid / 1px".to_string(),
        "icons: directory + file".to_string(),
        "context menu: enabled".to_string(),
        "default open: true".to_string(),
        "trigger: icon + text".to_string(),
    ];
    draw_section(
        canvas,
        render.code_text,
        render.palette,
        "Tree settings",
        &rows,
        y,
    )
}

fn draw_state(canvas: &mut Canvas, render: RenderContext<'_>, node: &UiNode, y: usize) -> usize {
    let props = node.props();
    let rows = [
        format!("state: {}", props.state_id.as_str()),
        format!("open: {}", props.interaction.open),
        format!("selected: {}", props.interaction.has_selection),
        format!("value: {}", visible_value(props.interaction.value.as_str())),
    ];
    draw_section(canvas, render.code_text, render.palette, "State", &rows, y)
}

fn draw_history(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    example: &StoryExample,
    y: usize,
) -> usize {
    let rows = history_rows(example);
    draw_section(
        canvas,
        render.code_text,
        render.palette,
        "Event / Action",
        &rows,
        y,
    )
}

fn draw_quality(canvas: &mut Canvas, text: &TextRenderer, palette: &VisualPalette, y: usize) {
    let rows = [
        "preview: rendered".to_string(),
        "settings: visible".to_string(),
        "preset: tabs".to_string(),
        "visual gate: required".to_string(),
    ];
    draw_section(canvas, text, palette, "Quality", &rows, y);
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
    canvas.fill_rect(SECTION_X, y, SECTION_WIDTH, height, palette.panel);
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

fn history_rows(example: &StoryExample) -> Vec<String> {
    if example.callback_logs.is_empty() {
        return vec![
            "action: none".to_string(),
            "event: passive component".to_string(),
            "log: visible".to_string(),
        ];
    }
    example
        .callback_logs
        .iter()
        .take(HISTORY_ROW_LIMIT)
        .map(|it| format!("{} -> {}", it.action, it.target.as_str()))
        .collect()
}

fn visible_value(value: &str) -> &str {
    if value.is_empty() {
        return "-";
    }
    value
}
