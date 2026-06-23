use crate::visual::canvas::Canvas;
use crate::visual::layout_metrics::{
    LayoutRect, button_setting_hit_rect, panel_active_details_rect, panel_active_nav_rect,
    panel_active_preview_rect, panel_scrollbar_off_rect, panel_scrollbar_on_rect,
};
use crate::visual::palette::VisualPalette;
use crate::visual::panel_screen_state::PanelChildKey;
use crate::visual::render_context::ScenarioContext;
use crate::visual::text::{TextRenderer, TextVerticalBox};

const SECTION_HEIGHT: usize = 158;
const ROW_HEIGHT: usize = 24;
const ROW_GAP: usize = 6;
const TITLE_OFFSET_Y: usize = 12;
const SECTION_X_OFFSET: usize = 10;
const SECTION_WIDTH_INSET: usize = 36;
const SECTION_ACCENT_WIDTH: usize = 4;
const CONTROL_VALUE_X: usize = 94;
const SCROLL_VALUE_WIDTH: usize = 168;
const LABEL_ROW_WIDTH: usize = 274;
const TEXT_PADDING_X: usize = 8;
const SEGMENT_TEXT_OFFSET_X: usize = 6;
const LABEL_SIZE: f32 = 10.5;
const VALUE_SIZE: f32 = 10.0;

pub(in crate::visual) fn draw_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    section_y: usize,
) -> usize {
    let base = button_setting_hit_rect();
    let section_x = base.x - SECTION_X_OFFSET;
    let width = crate::visual::layout_metrics::INSPECTOR_WIDTH - SECTION_WIDTH_INSET;
    canvas.fill_rect(
        section_x,
        section_y,
        width,
        SECTION_HEIGHT,
        palette.code_background,
    );
    canvas.stroke_rect(section_x, section_y, width, SECTION_HEIGHT, palette.border);
    canvas.fill_rect(
        section_x,
        section_y,
        SECTION_ACCENT_WIDTH,
        SECTION_HEIGHT,
        palette.accent,
    );
    text.draw(
        canvas,
        "Panel settings",
        section_x + SECTION_X_OFFSET,
        section_y + TITLE_OFFSET_Y,
        LABEL_SIZE,
        palette.text,
    );
    draw_active_row(canvas, text, palette, scenario, base.y);
    draw_scrollbar_row(
        canvas,
        text,
        palette,
        scenario,
        base.y + ROW_HEIGHT + ROW_GAP,
    );
    draw_scroll_row(
        canvas,
        text,
        palette,
        scenario,
        base.y + (ROW_HEIGHT + ROW_GAP) * 2,
    );
    section_y + SECTION_HEIGHT
}

fn draw_active_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    y: usize,
) {
    draw_label(canvas, text, palette, "active panel", y);
    for (panel, rect) in [
        (PanelChildKey::Navigation, panel_active_nav_rect()),
        (PanelChildKey::Preview, panel_active_preview_rect()),
        (PanelChildKey::Details, panel_active_details_rect()),
    ] {
        draw_segment(
            canvas,
            text,
            palette,
            rect,
            panel.label(),
            scenario.screen_state.panel.active_panel == panel,
        );
    }
}

fn draw_scrollbar_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    y: usize,
) {
    let active = scenario.screen_state.panel.active_panel;
    let visible = scenario.screen_state.panel.child(active).scrollbar_visible;
    draw_label(canvas, text, palette, "scrollbar", y);
    draw_segment(
        canvas,
        text,
        palette,
        panel_scrollbar_on_rect(),
        "on",
        visible,
    );
    draw_segment(
        canvas,
        text,
        palette,
        panel_scrollbar_off_rect(),
        "off",
        !visible,
    );
}

fn draw_scroll_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    y: usize,
) {
    let active = scenario
        .screen_state
        .panel
        .child(scenario.screen_state.panel.active_panel);
    draw_label(canvas, text, palette, "scroll", y);
    let row = LayoutRect::new(
        button_setting_hit_rect().x + CONTROL_VALUE_X,
        y,
        SCROLL_VALUE_WIDTH,
        ROW_HEIGHT,
    );
    canvas.fill_rect(row.x, row.y, row.width, row.height, palette.panel);
    canvas.stroke_rect(row.x, row.y, row.width, row.height, palette.border);
    text.draw_centered(
        canvas,
        &format!("x={} y={}", active.scroll_x, active.scroll_y),
        row.x + TEXT_PADDING_X,
        TextVerticalBox::new(row.y, row.height as f32),
        VALUE_SIZE,
        palette.text,
    );
}

fn draw_label(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    label: &str,
    y: usize,
) {
    let row = LayoutRect::new(button_setting_hit_rect().x, y, LABEL_ROW_WIDTH, ROW_HEIGHT);
    canvas.fill_rect(row.x, row.y, row.width, row.height, palette.panel);
    canvas.stroke_rect(row.x, row.y, row.width, row.height, palette.border);
    text.draw_centered(
        canvas,
        label,
        row.x + TEXT_PADDING_X,
        TextVerticalBox::new(row.y, row.height as f32),
        LABEL_SIZE,
        palette.text,
    );
}

fn draw_segment(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    rect: LayoutRect,
    label: &str,
    active: bool,
) {
    canvas.fill_rect(
        rect.x,
        rect.y,
        rect.width,
        rect.height,
        if active {
            palette.accent
        } else {
            palette.surface
        },
    );
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
    text.draw_centered(
        canvas,
        label,
        rect.x + SEGMENT_TEXT_OFFSET_X,
        TextVerticalBox::new(rect.y, rect.height as f32),
        VALUE_SIZE,
        if active {
            palette.background
        } else {
            palette.text
        },
    );
}
