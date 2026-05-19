use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PANEL: u32 = 0x20242c;
const REMOVED: u32 = 0x5a2328;
const ADDED: u32 = 0x244d31;
const UNCHANGED: u32 = 0x2d2d30;
const PICKED: u32 = 0x40a6ff;
const STATUS_X: usize = 18;
const STATUS_Y: usize = 102;
const STATUS_WIDTH: usize = 96;
const STATUS_HEIGHT: usize = 18;
const STATUS_GAP: usize = 8;
const STATUS_TEXT_X: usize = 7;
const STATUS_TEXT_Y: usize = 5;

pub(super) fn color_picker(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "ColorPicker 22 RGBA + 23 parity",
        &color_picker_blocks(palette, scenario),
        &[
            TextSpec::new(
                m::PX_14,
                m::PX_96,
                m::FONT_8,
                palette.muted,
                "R=64 G=128 B=255 A=204",
            ),
            TextSpec::new(
                m::PX_176,
                m::PX_96,
                m::FONT_8,
                palette.muted,
                "floating panel / seamless hue",
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
    draw_hue_bar(canvas, x + m::PX_182, y + m::PX_84);
}
pub(super) fn code_diff(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let mode_fill = if scenario.screen_state.has_widget_action() {
        common::TOKEN
    } else {
        PANEL
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "CodeDiff",
        &[
            Block::new(m::PX_18, m::PX_34, m::PX_136, m::PX_13, UNCHANGED),
            Block::new(m::PX_160, m::PX_34, m::PX_136, m::PX_13, UNCHANGED),
            Block::new(m::PX_18, m::PX_50, m::PX_136, m::PX_13, REMOVED),
            Block::new(m::PX_160, m::PX_50, m::PX_136, m::PX_13, ADDED),
            Block::new(m::PX_18, m::PX_66, m::PX_136, m::PX_13, REMOVED),
            Block::new(m::PX_160, m::PX_66, m::PX_136, m::PX_13, ADDED),
            Block::new(m::PX_18, m::PX_84, m::PX_278, m::PX_12, mode_fill),
        ],
        &[
            TextSpec::new(m::PX_24, m::PX_36, m::FONT_7, palette.text, " fn render()"),
            TextSpec::new(m::PX_166, m::PX_36, m::FONT_7, palette.text, " fn render()"),
            TextSpec::new(m::PX_24, m::PX_52, m::FONT_7, palette.text, "- old line"),
            TextSpec::new(m::PX_166, m::PX_52, m::FONT_7, palette.text, "+ new line"),
            TextSpec::new(m::PX_24, m::PX_68, m::FONT_7, palette.text, "- 空白  "),
            TextSpec::new(
                m::PX_166,
                m::PX_68,
                m::FONT_7,
                palette.text,
                "+ 空白 trimmed",
            ),
            TextSpec::new(
                m::PX_26,
                m::PX_84,
                m::FONT_8,
                palette.muted,
                "inline / collapsed / long line / 日本語",
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
}
fn color_picker_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [Block; m::PX_16] {
    let preview_color = if scenario.screen_state.has_widget_action() {
        PICKED
    } else {
        palette.accent
    };
    [
        Block::new(m::PX_14, m::PX_34, m::PX_68, m::PX_50, preview_color),
        Block::new(m::PX_90, m::PX_34, m::PX_64, m::PX_8, common::DANGER),
        Block::new(m::PX_90, m::PX_50, m::PX_64, m::PX_8, common::WARN),
        Block::new(m::PX_90, m::PX_66, m::PX_64, m::PX_8, common::PURPLE),
        Block::new(m::PX_90, m::PX_82, m::PX_64, m::PX_8, palette.border),
        Block::outlined(m::PX_174, m::PX_32, m::PX_22, m::PX_22, preview_color),
        Block::outlined(m::PX_214, m::PX_32, m::PX_30, m::PX_30, preview_color),
        Block::outlined(m::PX_258, m::PX_32, m::PX_38, m::PX_38, preview_color),
        Block::outlined(m::PX_176, m::PX_74, m::PX_134, m::PX_28, palette.surface),
        Block::new(m::PX_28, m::PX_48, m::PX_18, m::PX_18, common::DANGER),
        Block::new(m::PX_50, m::PX_58, m::PX_18, m::PX_18, common::WARN),
        Block::new(m::PX_64, m::PX_42, m::PX_10, m::PX_10, common::TOKEN),
        Block::new(m::PX_182, m::PX_84, m::PX_10, m::PX_8, common::DANGER),
        Block::new(m::PX_192, m::PX_84, m::PX_10, m::PX_8, common::WARN),
        Block::new(m::PX_202, m::PX_84, m::PX_10, m::PX_8, common::SUCCESS),
        Block::new(m::PX_212, m::PX_84, m::PX_10, m::PX_8, common::PURPLE),
    ]
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
        action_label(scenario),
        event_label(scenario),
        state_label(scenario),
    ];
    for (index, row) in rows.into_iter().enumerate() {
        let row_x = x + STATUS_X + index * (STATUS_WIDTH + STATUS_GAP);
        canvas.fill_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            row_x + STATUS_TEXT_X,
            y + STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
        );
    }
}

fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

fn event_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "state=ready";
    }
    scenario.screen_state.state_label
}
fn draw_hue_bar(canvas: &mut Canvas, x: usize, y: usize) {
    for (index, color) in [
        common::DANGER,
        m::COLOR_HUE_ORANGE,
        common::WARN,
        common::SUCCESS,
        common::TOKEN,
        m::COLOR_HUE_CYAN,
        m::COLOR_HUE_BLUE,
        common::PURPLE,
    ]
    .iter()
    .enumerate()
    {
        common::fill(
            canvas,
            Rect::new(x + index * m::PX_10, y, m::PX_10, m::PX_8),
            *color,
        );
    }
}
