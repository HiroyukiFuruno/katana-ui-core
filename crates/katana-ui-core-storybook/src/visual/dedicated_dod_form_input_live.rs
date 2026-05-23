use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, ChipSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::{TextRenderer, TextVerticalBox};

#[path = "dedicated_dod_form_input_live_text_area.rs"]
mod dedicated_dod_form_input_live_text_area;

const FIELD: u32 = 0x1f242d;
const CODE: u32 = 0x2d2d30;
const FIELD_X: usize = 18;
const FIELD_Y: usize = 36;
const FIELD_WIDTH: usize = 210;
const FIELD_HEIGHT: usize = 34;
const FIELD_TEXT_X: usize = 42;
const FIELD_ICON_X: usize = 28;
const FIELD_ICON_Y: usize = 47;
const FIELD_CURSOR_X: usize = 206;
const FIELD_CURSOR_Y: usize = 44;
const FIELD_CURSOR_WIDTH: usize = 2;
const FIELD_CURSOR_HEIGHT: usize = 18;
const CLEAR_X: usize = 208;
const CLEAR_Y: usize = 46;
const CLEAR_SIZE: usize = 14;
const SEARCH_ICON_STEM_OFFSET: usize = 4;
const STATUS_X: usize = 246;
const STATUS_Y: usize = 76;
const STATUS_WIDTH: usize = 84;
const STATUS_HEIGHT: usize = 20;
const STATUS_GAP: usize = 8;
const STATUS_TEXT_X: usize = 7;
const STATUS_TEXT_Y: usize = 6;
const CHIP_Y: usize = 84;
const CHIP_WIDTH: usize = 68;
const CHIP_HEIGHT: usize = 18;
const CHIP_GAP: usize = 8;
const CHIP_LABEL_COUNT: usize = 3;
const LABEL_SIZE: f32 = 10.0;
const TEXT_AREA_Y: usize = 32;
const TEXT_AREA_WIDTH: usize = 236;
const TEXT_AREA_HEIGHT: usize = 92;
const TEXT_AREA_LINE_X: usize = 30;
const TEXT_AREA_LINE_FIRST_Y: usize = 54;
const TEXT_AREA_LINE_STEP: usize = 18;
const TEXT_AREA_STATUS_X: usize = 272;
const TEXT_AREA_STATUS_WIDTH: usize = 68;

pub(super) fn input(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "Input / TextInput");
    draw_input_field(canvas, text, palette, scenario, x, y, input_value(scenario));
    draw_status(canvas, text, palette, scenario, x, y);
    draw_input_chips(canvas, text, palette, x, y);
}

pub(super) fn text_area(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_form_input_live_text_area::text_area(canvas, text, palette, scenario, x, y);
}

pub(super) fn search(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "SearchBox");
    draw_search_icon(canvas, palette, x, y);
    draw_input_field(
        canvas,
        text,
        palette,
        scenario,
        x,
        y,
        search_value(scenario),
    );
    draw_clear_button(canvas, x, y);
    draw_status(canvas, text, palette, scenario, x, y);
    draw_search_chips(canvas, text, palette, x, y);
}

pub(super) fn draw_input_field(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
    value: &str,
) {
    let border = if scenario.screen_state.has_widget_action() {
        palette.accent
    } else {
        palette.border
    };
    canvas.fill_rect(x + FIELD_X, y + FIELD_Y, FIELD_WIDTH, FIELD_HEIGHT, FIELD);
    canvas.stroke_rect(x + FIELD_X, y + FIELD_Y, FIELD_WIDTH, FIELD_HEIGHT, border);
    text.draw_centered(
        canvas,
        value,
        x + FIELD_TEXT_X,
        TextVerticalBox::new(y + FIELD_Y, FIELD_HEIGHT as f32),
        LABEL_SIZE,
        palette.text,
    );
    canvas.fill_rect(
        x + FIELD_CURSOR_X,
        y + FIELD_CURSOR_Y,
        FIELD_CURSOR_WIDTH,
        FIELD_CURSOR_HEIGHT,
        palette.accent,
    );
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let rows = [
        status_action(scenario),
        status_event(scenario),
        status_state(scenario),
    ];
    for (index, row) in rows.into_iter().enumerate() {
        let row_y = y + STATUS_Y + index * (STATUS_HEIGHT + STATUS_GAP);
        canvas.fill_rect(
            x + STATUS_X,
            row_y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            x + STATUS_X,
            row_y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            x + STATUS_X + STATUS_TEXT_X,
            row_y + STATUS_TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn draw_input_chips(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    draw_chips(canvas, text, palette, x, y, ["IME", "emoji", "invalid"]);
}

fn draw_search_chips(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    draw_chips(canvas, text, palette, x, y, ["regex", "word", "case"]);
}

fn draw_chips(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    labels: [&'static str; CHIP_LABEL_COUNT],
) {
    common::draw_chips(
        canvas,
        text,
        palette,
        x,
        y,
        &[
            ChipSpec::new(
                FIELD_X,
                CHIP_Y,
                CHIP_WIDTH,
                CHIP_HEIGHT,
                labels[0],
                palette.accent,
            ),
            ChipSpec::new(
                FIELD_X + CHIP_WIDTH + CHIP_GAP,
                CHIP_Y,
                CHIP_WIDTH,
                CHIP_HEIGHT,
                labels[1],
                palette.panel,
            ),
            ChipSpec::new(
                FIELD_X + (CHIP_WIDTH + CHIP_GAP) * m::PX_2,
                CHIP_Y,
                CHIP_WIDTH,
                CHIP_HEIGHT,
                labels[2],
                CODE,
            ),
        ],
    );
}

fn draw_search_icon(canvas: &mut Canvas, palette: &VisualPalette, x: usize, y: usize) {
    canvas.fill_rect(
        x + FIELD_ICON_X,
        y + FIELD_ICON_Y,
        m::PX_10,
        m::PX_2,
        palette.accent,
    );
    canvas.fill_rect(
        x + FIELD_ICON_X + SEARCH_ICON_STEM_OFFSET,
        y + FIELD_ICON_Y - SEARCH_ICON_STEM_OFFSET,
        m::PX_2,
        m::PX_10,
        palette.accent,
    );
}

fn draw_clear_button(canvas: &mut Canvas, x: usize, y: usize) {
    canvas.fill_rect(
        x + CLEAR_X,
        y + CLEAR_Y,
        CLEAR_SIZE,
        CLEAR_SIZE,
        common::DANGER,
    );
}

fn input_value(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_widget_action() {
        return "typed 日本語 🔷";
    }
    "日本語 value 🔷"
}

fn search_value(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_widget_action() {
        return "Query submitted";
    }
    "Query: error"
}

fn status_action(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

fn status_event(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn status_state(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "value idle";
    }
    scenario.screen_state.state_label
}
