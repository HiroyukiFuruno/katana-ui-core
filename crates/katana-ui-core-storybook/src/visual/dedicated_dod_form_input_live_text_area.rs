use super::{
    Canvas, FIELD_X, LABEL_SIZE, STATUS_GAP, STATUS_HEIGHT, STATUS_TEXT_X, STATUS_TEXT_Y, STATUS_Y,
    ScenarioContext, TEXT_AREA_HEIGHT, TEXT_AREA_LINE_FIRST_Y, TEXT_AREA_LINE_STEP,
    TEXT_AREA_LINE_X, TEXT_AREA_STATUS_WIDTH, TEXT_AREA_STATUS_X, TEXT_AREA_WIDTH, TEXT_AREA_Y,
    TextRenderer, VisualPalette, common, m,
};

const TEXT_AREA_SCROLLBAR_THUMB_X_OFFSET: usize = 5;
const TEXT_AREA_LINE_COUNT: usize = 4;
const TEXT_AREA_STATUS_ROW_COUNT: usize = 3;
const SEARCH_PRESET_INDEX: usize = 1;
const WRAP_PRESET_INDEX: usize = 2;
const AUTO_GROW_PRESET_INDEX: usize = 3;
const OVERFLOW_PRESET_INDEX: usize = 4;
const IME_PRESET_INDEX: usize = 5;
const EMOJI_PRESET_INDEX: usize = 6;

pub(super) fn text_area(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "TextArea");
    let border = if scenario.screen_state.has_settings_override() {
        palette.accent
    } else {
        palette.border
    };
    canvas.fill_rect(
        x + FIELD_X,
        y + TEXT_AREA_Y,
        TEXT_AREA_WIDTH,
        TEXT_AREA_HEIGHT,
        palette.surface,
    );
    canvas.stroke_rect(
        x + FIELD_X,
        y + TEXT_AREA_Y,
        TEXT_AREA_WIDTH,
        TEXT_AREA_HEIGHT,
        border,
    );
    for (index, line) in text_area_lines(scenario).into_iter().enumerate() {
        text.draw(
            canvas,
            line,
            x + TEXT_AREA_LINE_X,
            y + TEXT_AREA_LINE_FIRST_Y + index * TEXT_AREA_LINE_STEP,
            LABEL_SIZE,
            palette.text,
        );
    }
    canvas.fill_rect(
        x + FIELD_X + TEXT_AREA_WIDTH - m::PX_4,
        y + TEXT_AREA_Y + m::PX_8,
        m::PX_2,
        TEXT_AREA_HEIGHT - m::PX_16,
        palette.panel,
    );
    canvas.fill_rect(
        x + FIELD_X + TEXT_AREA_WIDTH - TEXT_AREA_SCROLLBAR_THUMB_X_OFFSET,
        y + TEXT_AREA_Y + text_area_thumb_y(scenario),
        m::PX_4,
        m::PX_24,
        palette.accent,
    );
    draw_text_area_status(canvas, text, palette, scenario, x, y);
}

fn draw_text_area_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (index, row) in text_area_status_rows(scenario).into_iter().enumerate() {
        let row_y = y + STATUS_Y + index * (STATUS_HEIGHT + STATUS_GAP);
        canvas.fill_rect(
            x + TEXT_AREA_STATUS_X,
            row_y,
            TEXT_AREA_STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            x + TEXT_AREA_STATUS_X,
            row_y,
            TEXT_AREA_STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            x + TEXT_AREA_STATUS_X + STATUS_TEXT_X,
            row_y + STATUS_TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn text_area_lines(scenario: ScenarioContext<'_>) -> [&'static str; TEXT_AREA_LINE_COUNT] {
    match scenario.preset_index {
        SEARCH_PRESET_INDEX => [
            "検索 query",
            "path: src/**/*.rs",
            "日本語 mixed",
            "Shift+Enter",
        ],
        WRAP_PRESET_INDEX => ["長文 line 1", "line 2 wraps", "line 3 keeps", "line 4"],
        AUTO_GROW_PRESET_INDEX => ["auto grow", "rows 2 -> 4", "resize event", "scroll=false"],
        OVERFLOW_PRESET_INDEX => ["max rows", "overflow line", "internal scroll", "value kept"],
        IME_PRESET_INDEX => [
            "IME preedit",
            "かな -> 日本語",
            "caret stable",
            "commit once",
        ],
        EMOJI_PRESET_INDEX => ["emoji input", "👩‍💻 is one", "delete once", "caret grapheme"],
        _ => ["chat composer", "English", "日本語 🔷", "Cmd+Enter"],
    }
}

fn text_area_status_rows(
    scenario: ScenarioContext<'_>,
) -> [&'static str; TEXT_AREA_STATUS_ROW_COUNT] {
    match scenario.preset_index {
        OVERFLOW_PRESET_INDEX => ["rows 4/4", "scroll true", "value full"],
        IME_PRESET_INDEX => ["IME update", "preedit on", "commit once"],
        EMOJI_PRESET_INDEX => ["emoji event", "grapheme 1", "caret ok"],
        _ if scenario.screen_state.has_widget_action() => ["type event", "change", "resize"],
        _ => ["submit Enter", "newline Shift", "auto grow"],
    }
}

fn text_area_thumb_y(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == OVERFLOW_PRESET_INDEX {
        return m::PX_34;
    }
    m::PX_12
}
