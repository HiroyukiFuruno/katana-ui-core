use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_dod_status;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PANEL: u32 = 0x20242c;
const REMOVED: u32 = 0x5a2328;
const ADDED: u32 = 0x244d31;
const UNCHANGED: u32 = 0x2d2d30;
const CODE_DIFF_SLOT_COUNT: usize = 7;
const WHITESPACE_PRESET_INDEX: usize = 1;
const DIRECTION_PRESET_INDEX: usize = 2;
const CONTEXT_PRESET_INDEX: usize = 3;
const ITEM_COUNT_PRESET_INDEX: usize = 4;
const SCROLL_SYNC_PRESET_INDEX: usize = 5;
const LANGUAGE_PRESET_INDEX: usize = 6;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;

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
        &code_diff_blocks(scenario.preset_index, mode_fill),
        &code_diff_texts(palette, scenario.preset_index),
    );
    draw_surface_token(canvas, palette, x, y);
    dedicated_dod_status::draw(canvas, text, palette, scenario, x, y);
}

fn code_diff_blocks(preset_index: usize, mode_fill: u32) -> [Block; CODE_DIFF_SLOT_COUNT] {
    match preset_index {
        WHITESPACE_PRESET_INDEX => horizontal_split_blocks(common::TOKEN),
        DIRECTION_PRESET_INDEX => vertical_split_blocks(mode_fill),
        CONTEXT_PRESET_INDEX => horizontal_split_blocks(common::WARN),
        ITEM_COUNT_PRESET_INDEX => item_count_blocks(mode_fill),
        SCROLL_SYNC_PRESET_INDEX => horizontal_split_blocks(common::SUCCESS),
        LANGUAGE_PRESET_INDEX => horizontal_split_blocks(common::DANGER),
        _ => horizontal_split_blocks(mode_fill),
    }
}

fn draw_surface_token(canvas: &mut Canvas, palette: &VisualPalette, x: usize, y: usize) {
    let token = Rect::new(
        x + SURFACE_TOKEN_X,
        y + SURFACE_TOKEN_Y,
        SURFACE_TOKEN_WIDTH,
        SURFACE_TOKEN_HEIGHT,
    );
    common::fill(canvas, token, palette.surface);
    common::outline(canvas, palette, token);
}

fn horizontal_split_blocks(mode_fill: u32) -> [Block; CODE_DIFF_SLOT_COUNT] {
    [
        Block::new(m::PX_18, m::PX_34, m::PX_136, m::PX_13, UNCHANGED),
        Block::new(m::PX_160, m::PX_34, m::PX_136, m::PX_13, UNCHANGED),
        Block::new(m::PX_18, m::PX_50, m::PX_136, m::PX_13, REMOVED),
        Block::new(m::PX_160, m::PX_50, m::PX_136, m::PX_13, ADDED),
        Block::new(m::PX_18, m::PX_66, m::PX_136, m::PX_13, REMOVED),
        Block::new(m::PX_160, m::PX_66, m::PX_136, m::PX_13, ADDED),
        Block::new(m::PX_18, m::PX_84, m::PX_278, m::PX_12, mode_fill),
    ]
}

fn vertical_split_blocks(mode_fill: u32) -> [Block; CODE_DIFF_SLOT_COUNT] {
    [
        Block::new(m::PX_18, m::PX_32, m::PX_278, m::PX_12, UNCHANGED),
        Block::new(m::PX_18, m::PX_46, m::PX_278, m::PX_12, REMOVED),
        Block::new(m::PX_18, m::PX_60, m::PX_278, m::PX_3, PANEL),
        Block::new(m::PX_18, m::PX_66, m::PX_278, m::PX_12, UNCHANGED),
        Block::new(m::PX_18, m::PX_80, m::PX_278, m::PX_12, ADDED),
        Block::new(m::PX_18, m::PX_96, m::PX_120, m::PX_8, mode_fill),
        Block::new(m::PX_148, m::PX_96, m::PX_148, m::PX_8, PANEL),
    ]
}

fn item_count_blocks(mode_fill: u32) -> [Block; CODE_DIFF_SLOT_COUNT] {
    [
        Block::new(m::PX_18, m::PX_34, m::PX_278, m::PX_12, UNCHANGED),
        Block::new(m::PX_18, m::PX_48, m::PX_278, m::PX_12, REMOVED),
        Block::new(m::PX_18, m::PX_62, m::PX_278, m::PX_12, ADDED),
        Block::new(m::PX_18, m::PX_76, m::PX_278, m::PX_8, mode_fill),
        Block::new(m::PX_18, m::PX_88, m::PX_84, m::PX_8, REMOVED),
        Block::new(m::PX_108, m::PX_88, m::PX_84, m::PX_8, ADDED),
        Block::new(m::PX_198, m::PX_88, m::PX_98, m::PX_8, PANEL),
    ]
}

fn code_diff_texts(
    palette: &VisualPalette,
    preset_index: usize,
) -> [TextSpec; CODE_DIFF_SLOT_COUNT] {
    match preset_index {
        DIRECTION_PRESET_INDEX => vertical_split_texts(palette),
        ITEM_COUNT_PRESET_INDEX => item_count_texts(palette),
        _ => horizontal_split_texts(palette, preset_index),
    }
}

fn horizontal_split_texts(
    palette: &VisualPalette,
    preset_index: usize,
) -> [TextSpec; CODE_DIFF_SLOT_COUNT] {
    let note = match preset_index {
        WHITESPACE_PRESET_INDEX => "whitespace=visible / space=· / tab=→",
        CONTEXT_PRESET_INDEX => "context_lines=0 / collapsed block",
        SCROLL_SYNC_PRESET_INDEX => "scroll_sync=false / independent panes",
        LANGUAGE_PRESET_INDEX => "language=markdown / 日本語 diff",
        _ => "mode=Split / left-right",
    };
    [
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
        TextSpec::new(m::PX_26, m::PX_84, m::FONT_8, palette.muted, note),
    ]
}

fn vertical_split_texts(palette: &VisualPalette) -> [TextSpec; CODE_DIFF_SLOT_COUNT] {
    [
        TextSpec::new(
            m::PX_24,
            m::PX_34,
            m::FONT_7,
            palette.text,
            "before: fn render()",
        ),
        TextSpec::new(m::PX_24, m::PX_48, m::FONT_7, palette.text, "- old line"),
        TextSpec::new(
            m::PX_24,
            m::PX_60,
            m::FONT_7,
            palette.muted,
            "split boundary",
        ),
        TextSpec::new(
            m::PX_24,
            m::PX_68,
            m::FONT_7,
            palette.text,
            "after: fn render()",
        ),
        TextSpec::new(m::PX_24, m::PX_82, m::FONT_7, palette.text, "+ new line"),
        TextSpec::new(m::PX_24, m::PX_96, m::FONT_7, palette.muted, "top-bottom"),
        TextSpec::new(
            m::PX_152,
            m::PX_96,
            m::FONT_7,
            palette.muted,
            "direction=vertical",
        ),
    ]
}

fn item_count_texts(palette: &VisualPalette) -> [TextSpec; CODE_DIFF_SLOT_COUNT] {
    [
        TextSpec::new(m::PX_24, m::PX_36, m::FONT_7, palette.text, " fn render()"),
        TextSpec::new(m::PX_24, m::PX_50, m::FONT_7, palette.text, "- old line"),
        TextSpec::new(m::PX_24, m::PX_64, m::FONT_7, palette.text, "+ new line"),
        TextSpec::new(
            m::PX_24,
            m::PX_76,
            m::FONT_8,
            palette.muted,
            "item_count=3 compact preview",
        ),
        TextSpec::new(m::PX_24, m::PX_88, m::FONT_7, palette.text, "old"),
        TextSpec::new(m::PX_112, m::PX_88, m::FONT_7, palette.text, "new"),
        TextSpec::new(m::PX_204, m::PX_88, m::FONT_7, palette.muted, "char range"),
    ]
}
