use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PICKED: u32 = 0x40a6ff;
const COLOR_TRIGGER_PRESET_INDEX: usize = 1;
const SIZE_PRESET_INDEX: usize = 2;
const BORDERLESS_PRESET_INDEX: usize = 3;
const FLOATING_PANEL_PRESET_INDEX: usize = 4;

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
                color_picker_note(scenario.preset_index),
            ),
        ],
    );
    super::dedicated_dod_status::draw(canvas, text, palette, scenario, x, y);
    draw_hue_bar(canvas, x + m::PX_182, y + m::PX_84);
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

fn color_picker_note(preset_index: usize) -> &'static str {
    match preset_index {
        COLOR_TRIGGER_PRESET_INDEX => "color-only trigger / value outside",
        SIZE_PRESET_INDEX => "xs sm mid large xlarge",
        BORDERLESS_PRESET_INDEX => "border off / single frame",
        FLOATING_PANEL_PRESET_INDEX => "floating panel / close actions",
        _ => "rgba panel / seamless hue",
    }
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
