use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PICKED: u32 = 0x40a6ff;
const VALUE_PRESET_INDEX: usize = 1;
const OPEN_PRESET_INDEX: usize = 2;
const HUE_PRESET_INDEX: usize = 3;
const ALPHA_PRESET_INDEX: usize = 4;
const BLEND_PRESET_INDEX: usize = 5;
const COLOR_AREA_PRESET_INDEX: usize = 6;
const TRIGGER_SIZE_PRESET_INDEX: usize = 7;
const TITLE_PRESET_INDEX: usize = 8;
const RGB_MODE_PRESET_INDEX: usize = 9;
const PANEL_SCALE_PRESET_INDEX: usize = 10;
const TRIGGER_BORDER_PRESET_INDEX: usize = 11;
const EYEDROPPER_PRESET_INDEX: usize = 12;
const READONLY_PRESET_INDEX: usize = 13;
const DISABLED_PRESET_INDEX: usize = 14;

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
    let preview_color = if scenario.screen_state.is_button_focused() {
        common::TOKEN
    } else if scenario.screen_state.preview_hovered {
        m::COLOR_HUE_CYAN
    } else if should_show_picked_color(scenario) {
        PICKED
    } else {
        palette.accent
    };
    let floating_panel_fill = if matches!(
        scenario.preset_index,
        OPEN_PRESET_INDEX | PANEL_SCALE_PRESET_INDEX
    ) {
        palette.panel
    } else {
        palette.surface
    };
    let size_marker_color = if scenario.preset_index == TRIGGER_SIZE_PRESET_INDEX {
        common::TOKEN
    } else {
        preview_color
    };
    let variant_color = color_picker_variant_color(scenario.preset_index, palette);
    [
        Block::new(m::PX_14, m::PX_34, m::PX_68, m::PX_50, preview_color),
        Block::new(m::PX_90, m::PX_34, m::PX_64, m::PX_8, common::DANGER),
        Block::new(m::PX_90, m::PX_50, m::PX_64, m::PX_8, common::WARN),
        Block::new(m::PX_90, m::PX_66, m::PX_64, m::PX_8, common::PURPLE),
        Block::new(m::PX_90, m::PX_82, m::PX_64, m::PX_8, palette.border),
        trigger_block(
            m::PX_174,
            m::PX_32,
            m::PX_22,
            m::PX_22,
            preview_color,
            scenario,
        ),
        trigger_block(
            m::PX_214,
            m::PX_32,
            m::PX_30,
            m::PX_30,
            size_marker_color,
            scenario,
        ),
        trigger_block(
            m::PX_258,
            m::PX_32,
            m::PX_38,
            m::PX_38,
            size_marker_color,
            scenario,
        ),
        Block::outlined(
            m::PX_176,
            m::PX_74,
            m::PX_134,
            m::PX_28,
            floating_panel_fill,
        ),
        Block::new(m::PX_28, m::PX_48, m::PX_18, m::PX_18, variant_color),
        Block::new(
            m::PX_50,
            m::PX_58,
            m::PX_18,
            m::PX_18,
            color_picker_variant_shadow(scenario.preset_index, palette),
        ),
        Block::new(m::PX_64, m::PX_42, m::PX_10, m::PX_10, common::TOKEN),
        Block::new(m::PX_182, m::PX_84, m::PX_10, m::PX_8, common::DANGER),
        Block::new(m::PX_192, m::PX_84, m::PX_10, m::PX_8, common::WARN),
        Block::new(m::PX_202, m::PX_84, m::PX_10, m::PX_8, common::SUCCESS),
        Block::new(m::PX_212, m::PX_84, m::PX_10, m::PX_8, common::PURPLE),
    ]
}

fn trigger_block(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
    scenario: ScenarioContext<'_>,
) -> Block {
    if scenario.preset_index == TRIGGER_BORDER_PRESET_INDEX {
        return Block::new(x, y, width, height, color);
    }
    Block::outlined(x, y, width, height, color)
}

fn color_picker_note(preset_index: usize) -> &'static str {
    match preset_index {
        VALUE_PRESET_INDEX => "string value mirrors rgba",
        OPEN_PRESET_INDEX => "open panel / close actions",
        HUE_PRESET_INDEX => "hue slider live preview",
        ALPHA_PRESET_INDEX => "alpha slider opacity",
        BLEND_PRESET_INDEX => "blend mode compare",
        COLOR_AREA_PRESET_INDEX => "color plane drag target",
        TRIGGER_SIZE_PRESET_INDEX => "xs sm mid large xlarge",
        TITLE_PRESET_INDEX => "panel title text",
        RGB_MODE_PRESET_INDEX => "rgb mode hides alpha",
        PANEL_SCALE_PRESET_INDEX => "scaled floating panel",
        TRIGGER_BORDER_PRESET_INDEX => "border off / single frame",
        EYEDROPPER_PRESET_INDEX => "eyedropper callback ready",
        READONLY_PRESET_INDEX => "readonly blocks writes",
        DISABLED_PRESET_INDEX => "disabled blocks focus",
        _ => "rgba panel / seamless hue",
    }
}

fn should_show_picked_color(scenario: ScenarioContext<'_>) -> bool {
    scenario.screen_state.color_picker.has_committed_color()
        || scenario.screen_state.has_settings_override()
        || matches!(
            scenario.preset_index,
            VALUE_PRESET_INDEX | COLOR_AREA_PRESET_INDEX | EYEDROPPER_PRESET_INDEX
        )
}

fn color_picker_variant_color(preset_index: usize, palette: &VisualPalette) -> u32 {
    match preset_index {
        VALUE_PRESET_INDEX => PICKED,
        OPEN_PRESET_INDEX => palette.panel,
        HUE_PRESET_INDEX => m::COLOR_HUE_ORANGE,
        ALPHA_PRESET_INDEX => common::PURPLE,
        BLEND_PRESET_INDEX => common::SUCCESS,
        COLOR_AREA_PRESET_INDEX => common::TOKEN,
        TRIGGER_SIZE_PRESET_INDEX => common::WARN,
        TITLE_PRESET_INDEX => palette.text,
        RGB_MODE_PRESET_INDEX => m::COLOR_HUE_CYAN,
        PANEL_SCALE_PRESET_INDEX => palette.accent,
        TRIGGER_BORDER_PRESET_INDEX => palette.border,
        EYEDROPPER_PRESET_INDEX => common::DANGER,
        READONLY_PRESET_INDEX => palette.muted,
        DISABLED_PRESET_INDEX => palette.muted,
        _ => common::DANGER,
    }
}

fn color_picker_variant_shadow(preset_index: usize, palette: &VisualPalette) -> u32 {
    match preset_index {
        ALPHA_PRESET_INDEX | DISABLED_PRESET_INDEX => palette.surface,
        OPEN_PRESET_INDEX | PANEL_SCALE_PRESET_INDEX => palette.panel,
        TRIGGER_BORDER_PRESET_INDEX | READONLY_PRESET_INDEX => palette.border,
        _ => common::WARN,
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
