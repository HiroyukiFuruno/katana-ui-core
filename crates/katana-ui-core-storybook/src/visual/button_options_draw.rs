use super::button_options::{
    CONTROL_COUNT, FIRST_ROW_Y_OFFSET, ROW_GAP, ROW_HEIGHT, ROW_WIDTH, ROW_X, SECTION_WIDTH,
    SECTION_X, StorybookButtonOptionControl, StorybookButtonOptions, control_index,
};
use super::canvas::Canvas;
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::switch_control;
use super::text::{TextRenderer, TextVerticalBox};

const SECTION_HEADER_HEIGHT: usize = 38;
const SECTION_ACCENT_WIDTH: usize = 4;
const SECTION_TITLE_X_OFFSET: usize = 10;
const SECTION_TITLE_Y_OFFSET: usize = 12;
const LABEL_X_OFFSET: usize = 8;
const LABEL_SIZE: f32 = 10.5;
const CONTROL_WIDTH: usize = 106;
const CONTROL_HEIGHT: usize = 22;
const CONTROL_X_OFFSET: usize = ROW_WIDTH - CONTROL_WIDTH - 8;
const CONTROL_Y_OFFSET: usize = 4;
const TOGGLE_WIDTH: usize = 48;
const TOGGLE_HEIGHT: usize = 22;
const TOGGLE_X_OFFSET: usize = ROW_WIDTH - TOGGLE_WIDTH - 12;
const TOGGLE_Y_OFFSET: usize = 4;
const TEXT_X_OFFSET: usize = 10;
const TEXT_SIZE: f32 = 10.0;

pub(super) fn draw_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    y: usize,
) -> usize {
    let height = SECTION_HEADER_HEIGHT + CONTROL_COUNT * (ROW_HEIGHT + ROW_GAP);
    canvas.fill_rect(SECTION_X, y, SECTION_WIDTH, height, palette.code_background);
    canvas.stroke_rect(SECTION_X, y, SECTION_WIDTH, height, palette.border);
    canvas.fill_rect(SECTION_X, y, SECTION_ACCENT_WIDTH, height, palette.accent);
    text.draw(
        canvas,
        "Button options",
        SECTION_X + SECTION_TITLE_X_OFFSET,
        y + SECTION_TITLE_Y_OFFSET,
        LABEL_SIZE,
        palette.text,
    );
    for control in StorybookButtonOptionControl::all() {
        draw_control(canvas, text, palette, scenario, y, control);
    }
    y + height
}

fn draw_control(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    section_y: usize,
    control: StorybookButtonOptionControl,
) {
    let index = control_index(control);
    let row_y = section_y + FIRST_ROW_Y_OFFSET + index * (ROW_HEIGHT + ROW_GAP);
    let row = LayoutRect::new(ROW_X, row_y, ROW_WIDTH, ROW_HEIGHT);
    canvas.fill_rect(row.x, row.y, row.width, row.height, palette.panel);
    canvas.stroke_rect(row.x, row.y, row.width, row.height, palette.border);
    text.draw_centered(
        canvas,
        control.setting_name(),
        row.x + LABEL_X_OFFSET,
        TextVerticalBox::new(row.y, row.height as f32),
        LABEL_SIZE,
        palette.text,
    );
    draw_value(canvas, text, palette, scenario, row, control);
}

fn draw_value(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    row: LayoutRect,
    control: StorybookButtonOptionControl,
) {
    let options = scenario.screen_state.button_options;
    match control {
        StorybookButtonOptionControl::Visible
        | StorybookButtonOptionControl::Disabled
        | StorybookButtonOptionControl::Focusable
        | StorybookButtonOptionControl::Border => {
            draw_toggle(canvas, palette, options, row, control)
        }
        StorybookButtonOptionControl::Label
        | StorybookButtonOptionControl::Width
        | StorybookButtonOptionControl::Height
        | StorybookButtonOptionControl::TabIndex
        | StorybookButtonOptionControl::ZIndex => {
            draw_value_button(canvas, text, palette, options, row, control);
        }
    }
}

fn draw_toggle(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    options: StorybookButtonOptions,
    row: LayoutRect,
    control: StorybookButtonOptionControl,
) {
    let enabled = match control {
        StorybookButtonOptionControl::Visible => options.visible,
        StorybookButtonOptionControl::Disabled => options.disabled,
        StorybookButtonOptionControl::Focusable => options.focusable,
        StorybookButtonOptionControl::Border => options.border,
        StorybookButtonOptionControl::Label
        | StorybookButtonOptionControl::Width
        | StorybookButtonOptionControl::Height
        | StorybookButtonOptionControl::TabIndex
        | StorybookButtonOptionControl::ZIndex => false,
    };
    let x = row.x + TOGGLE_X_OFFSET;
    let y = row.y + TOGGLE_Y_OFFSET;
    switch_control::draw_switch(canvas, palette, x, y, TOGGLE_WIDTH, TOGGLE_HEIGHT, enabled);
}

fn draw_value_button(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    options: StorybookButtonOptions,
    row: LayoutRect,
    control: StorybookButtonOptionControl,
) {
    let x = row.x + CONTROL_X_OFFSET;
    let y = row.y + CONTROL_Y_OFFSET;
    canvas.fill_rect(x, y, CONTROL_WIDTH, CONTROL_HEIGHT, palette.surface);
    canvas.stroke_rect(x, y, CONTROL_WIDTH, CONTROL_HEIGHT, palette.accent);
    text.draw_centered(
        canvas,
        control.setting_value(options),
        x + TEXT_X_OFFSET,
        TextVerticalBox::new(y, CONTROL_HEIGHT as f32),
        TEXT_SIZE,
        palette.text,
    );
}
