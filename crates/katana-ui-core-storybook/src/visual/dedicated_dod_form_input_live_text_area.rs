use super::{
    Canvas, LABEL_SIZE, STATUS_TEXT_X, STATUS_TEXT_Y, ScenarioContext, TEXT_AREA_LINE_FIRST_Y,
    TEXT_AREA_LINE_STEP, TEXT_AREA_LINE_X, TextRenderer, VisualPalette, common, m,
    text_area_caret_rect,
};

use super::dedicated_dod_form_input_live_text_area_chrome as chrome;
use chrome::{
    AUTO_GROW_PRESET_INDEX, EMOJI_PRESET_INDEX, HORIZONTAL_SCROLL_PRESET_INDEX, IME_PRESET_INDEX,
    RESIZE_PRESET_INDEX, VERTICAL_SCROLL_PRESET_INDEX,
};

const TEXT_AREA_LINE_COUNT: usize = 4;
const TEXT_AREA_STATUS_ROW_COUNT: usize = 3;
const SEARCH_PRESET_INDEX: usize = 1;
const WRAP_PRESET_INDEX: usize = 2;
const HORIZONTAL_SCROLL_MAX_OFFSET: usize = 96;

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
    let rect = chrome::text_area_rect_for_state(x, y, scenario.screen_state);
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.surface);
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, border);
    draw_text_area_lines(canvas, text, palette, scenario, x, y);
    chrome::draw_vertical_scrollbar(canvas, palette, scenario, x, y);
    chrome::draw_horizontal_scrollbar(canvas, palette, scenario, x, y);
    chrome::draw_resize_grip(canvas, palette, scenario, x, y);
    draw_text_area_caret(canvas, text, palette, scenario, x, y);
    draw_text_area_status(canvas, text, palette, scenario, x, y);
}

fn draw_text_area_lines(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let rect = chrome::text_area_rect_for_state(x, y, scenario.screen_state);
    canvas.with_clip(
        rect.x + 1,
        rect.y + 1,
        rect.width - 2,
        rect.height - 2,
        |canvas| {
            for (index, line) in text_area_lines(scenario).iter().enumerate() {
                text.draw(
                    canvas,
                    line.as_str(),
                    (x + TEXT_AREA_LINE_X)
                        .saturating_sub(chrome::horizontal_scroll_offset(scenario)),
                    y + TEXT_AREA_LINE_FIRST_Y + index * TEXT_AREA_LINE_STEP,
                    LABEL_SIZE,
                    palette.text,
                );
            }
        },
    );
}

fn draw_text_area_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (rect, row) in chrome::text_area_status_rects_for_state(x, y, scenario.screen_state)
        .into_iter()
        .zip(text_area_status_rows(scenario))
    {
        canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.panel);
        canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
        text.draw(
            canvas,
            row,
            rect.x + STATUS_TEXT_X,
            rect.y + STATUS_TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn draw_text_area_caret(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !scenario.screen_state.text_area_focused()
        || !scenario.screen_state.text_area_caret_visible()
    {
        return;
    }
    let value = scenario.screen_state.text_area_value();
    let line = value.lines().last().unwrap_or_default();
    let line_index = value.lines().count().saturating_sub(1);
    let value_width = text.measure_width(line, LABEL_SIZE);
    let rect = text_area_caret_rect(x, y, value_width, line_index);
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.accent);
}

fn text_area_lines(scenario: ScenarioContext<'_>) -> [String; TEXT_AREA_LINE_COUNT] {
    visible_text_area_lines(
        text_area_content_lines(scenario),
        chrome::vertical_scroll_offset(scenario),
    )
}

pub(in crate::visual) fn vertical_scroll_max_offset_for(
    preset_index: usize,
    screen_state: &crate::visual::screen_state::StorybookScreenState,
) -> usize {
    text_area_line_count(preset_index, screen_state).saturating_sub(TEXT_AREA_LINE_COUNT)
}

pub(in crate::visual) fn horizontal_scroll_max_offset_for(
    preset_index: usize,
    screen_state: &crate::visual::screen_state::StorybookScreenState,
) -> usize {
    if chrome::horizontal_scroll_enabled_for(preset_index, screen_state) {
        return HORIZONTAL_SCROLL_MAX_OFFSET;
    }
    0
}

fn text_area_line_count(
    preset_index: usize,
    screen_state: &crate::visual::screen_state::StorybookScreenState,
) -> usize {
    if screen_state.text_area_uses_live_value() {
        return screen_state.text_area_value().lines().count().max(1);
    }
    static_text_area_rows(preset_index).len()
}

fn text_area_content_lines(scenario: ScenarioContext<'_>) -> Vec<String> {
    if scenario.screen_state.text_area_uses_live_value() {
        return scenario
            .screen_state
            .text_area_value()
            .lines()
            .map(str::to_string)
            .collect();
    }
    static_text_area_rows(scenario.preset_index)
        .iter()
        .map(|line| (*line).to_string())
        .collect()
}

fn static_text_area_rows(preset_index: usize) -> &'static [&'static str] {
    match preset_index {
        SEARCH_PRESET_INDEX => &[
            "検索 query",
            "path: src/**/*.rs",
            "日本語 mixed",
            "Shift+Enter",
        ],
        WRAP_PRESET_INDEX => &["長文 line 1", "line 2 wraps", "line 3 keeps", "line 4"],
        RESIZE_PRESET_INDEX => &[
            "resize option",
            "default false",
            "handle visible",
            "corner grip",
        ],
        AUTO_GROW_PRESET_INDEX => &["auto grow", "rows 2 -> 4", "resize event", "scroll=false"],
        VERTICAL_SCROLL_PRESET_INDEX => &[
            "line 01", "line 02", "line 03", "line 04", "line 05", "line 06", "line 07", "line 08",
        ],
        HORIZONTAL_SCROLL_PRESET_INDEX => &[
            "long unwrapped line keeps horizontal scroll",
            "wrap=false",
            "scroll-x enabled",
            "bar visible",
        ],
        IME_PRESET_INDEX => &[
            "IME preedit",
            "かな -> 日本語",
            "caret stable",
            "commit once",
        ],
        EMOJI_PRESET_INDEX => &["emoji input", "👩‍💻 is one", "delete once", "caret grapheme"],
        _ => &["chat composer", "English", "日本語 🔷", "Cmd+Enter"],
    }
}

fn visible_text_area_lines(lines: Vec<String>, offset: usize) -> [String; TEXT_AREA_LINE_COUNT] {
    let mut rows = [String::new(), String::new(), String::new(), String::new()];
    for (index, line) in lines
        .into_iter()
        .skip(offset)
        .take(TEXT_AREA_LINE_COUNT)
        .enumerate()
    {
        rows[index] = line;
    }
    rows
}

fn text_area_status_rows(
    scenario: ScenarioContext<'_>,
) -> [&'static str; TEXT_AREA_STATUS_ROW_COUNT] {
    if matches!(
        scenario.screen_state.last_event,
        "text_area_scroll_changed" | "text_area_resized"
    ) {
        return [
            scenario.screen_state.last_action,
            scenario.screen_state.last_event,
            scenario.screen_state.state_label,
        ];
    }
    match scenario.preset_index {
        RESIZE_PRESET_INDEX => ["resize=true", "default false", "option on"],
        VERTICAL_SCROLL_PRESET_INDEX => ["scroll-y on", "bar visible", "wrap=true"],
        HORIZONTAL_SCROLL_PRESET_INDEX => ["scroll-x on", "bar visible", "wrap=false"],
        IME_PRESET_INDEX => ["IME update", "preedit on", "commit once"],
        EMOJI_PRESET_INDEX => ["emoji event", "grapheme 1", "caret ok"],
        _ if scenario.screen_state.text_area_uses_live_value() => [
            scenario.screen_state.last_action,
            scenario.screen_state.last_event,
            scenario.screen_state.state_label,
        ],
        _ => ["wrap=true", "resize=false", "scroll=false"],
    }
}
