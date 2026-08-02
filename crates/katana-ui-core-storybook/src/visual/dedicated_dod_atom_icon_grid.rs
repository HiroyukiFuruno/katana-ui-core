use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const ICON_COUNT: usize = 4;
const CONTENT_VALUE_PRESET_INDEX: usize = 0;
const VISUAL_ROLE_PRESET_INDEX: usize = 1;
const A11Y_LABEL_PRESET_INDEX: usize = 2;
const THEME_COLOR_PRESET_INDEX: usize = 3;
const SVG_SOURCE_PRESET_INDEX: usize = 4;
const SVG_ICON_PRESET_INDEX: usize = 5;
const VIEW_BOX_PRESET_INDEX: usize = 6;
const PATH_SUMMARY_PRESET_INDEX: usize = 7;
const PAINT_POLICY_PRESET_INDEX: usize = 8;
const ICON_ROLE_PRESET_INDEX: usize = 9;
const COLOR_TOKEN_PRESET_INDEX: usize = 10;
const THEME_TOKEN_PRESET_INDEX: usize = 11;

pub(super) fn icon_grid(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let status_color = if scenario.screen_state.has_settings_override() {
        common::SUCCESS
    } else {
        icon_custom_color(palette, scenario)
    };
    let blocks = [
        Block::outlined(m::PX_18, m::PX_36, m::PX_36, m::PX_36, palette.surface),
        Block::outlined(m::PX_62, m::PX_36, m::PX_36, m::PX_36, palette.surface),
        Block::outlined(m::PX_106, m::PX_36, m::PX_36, m::PX_36, palette.surface),
        Block::outlined(m::PX_150, m::PX_36, m::PX_36, m::PX_36, palette.surface),
        Block::new(
            m::PX_212,
            m::PX_38,
            active_bar_width(scenario),
            m::PX_10,
            status_color,
        ),
        Block::new(
            m::PX_212,
            m::PX_58,
            m::PX_96,
            m::PX_10,
            option_track_color(scenario),
        ),
        Block::new(
            m::PX_212,
            m::PX_78,
            m::PX_72,
            m::PX_10,
            paint_policy_color(scenario),
        ),
    ];
    let labels = [
        TextSpec::new(m::PX_22, m::PX_78, m::FONT_8, palette.muted, "12"),
        TextSpec::new(m::PX_66, m::PX_78, m::FONT_8, palette.muted, "16"),
        TextSpec::new(m::PX_110, m::PX_78, m::FONT_8, palette.muted, "20"),
        TextSpec::new(m::PX_148, m::PX_78, m::FONT_8, palette.muted, "custom"),
        TextSpec::new(
            m::PX_284,
            m::PX_36,
            m::FONT_9,
            palette.text,
            preset_label(scenario),
        ),
        TextSpec::new(
            m::PX_284,
            m::PX_56,
            m::FONT_9,
            palette.muted,
            option_label(scenario),
        ),
        TextSpec::new(
            m::PX_284,
            m::PX_76,
            m::FONT_9,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            m::PX_212,
            m::PX_98,
            m::FONT_8,
            palette.muted,
            contract_label(scenario),
        ),
    ];
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "SVG Icon props",
        &blocks,
        &labels,
    );
    draw_icons(canvas, palette, scenario, x, y, status_color);
}

fn draw_icons(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
    status_color: u32,
) {
    for (index, (size, color)) in icon_specs(palette, scenario, status_color)
        .iter()
        .enumerate()
    {
        let box_x = x + m::PX_18 + index * m::PX_44;
        let inset = (m::PX_36 - size) / m::PX_2;
        common::cross_icon(canvas, box_x + inset, y + m::PX_36 + inset, *size, *color);
    }
}

fn icon_specs(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    status_color: u32,
) -> [(usize, u32); ICON_COUNT] {
    match scenario.preset_index {
        VISUAL_ROLE_PRESET_INDEX | ICON_ROLE_PRESET_INDEX => [
            (m::PX_12, palette.muted),
            (m::PX_18, common::TOKEN),
            (m::PX_24, common::PURPLE),
            (m::PX_30, status_color),
        ],
        VIEW_BOX_PRESET_INDEX | PATH_SUMMARY_PRESET_INDEX => [
            (m::PX_26, palette.accent),
            (m::PX_20, common::TOKEN),
            (m::PX_16, common::PURPLE),
            (m::PX_12, status_color),
        ],
        PAINT_POLICY_PRESET_INDEX | COLOR_TOKEN_PRESET_INDEX | THEME_TOKEN_PRESET_INDEX => [
            (m::PX_14, status_color),
            (m::PX_18, status_color),
            (m::PX_22, status_color),
            (m::PX_26, status_color),
        ],
        _ => [
            (m::PX_12, palette.accent),
            (m::PX_16, common::TOKEN),
            (m::PX_20, common::PURPLE),
            (m::PX_24, status_color),
        ],
    }
}

fn active_bar_width(scenario: ScenarioContext<'_>) -> usize {
    m::PX_16 + scenario.preset_index.min(THEME_TOKEN_PRESET_INDEX) * m::PX_8
}

fn icon_custom_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    match scenario.preset_index {
        A11Y_LABEL_PRESET_INDEX => common::SUCCESS,
        THEME_COLOR_PRESET_INDEX | COLOR_TOKEN_PRESET_INDEX => palette.accent,
        SVG_SOURCE_PRESET_INDEX | SVG_ICON_PRESET_INDEX => common::WARN,
        VIEW_BOX_PRESET_INDEX | PATH_SUMMARY_PRESET_INDEX => common::PURPLE,
        PAINT_POLICY_PRESET_INDEX => common::TOKEN,
        THEME_TOKEN_PRESET_INDEX => palette.muted,
        _ => common::WARN,
    }
}

fn option_track_color(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index >= SVG_SOURCE_PRESET_INDEX {
        common::TOKEN
    } else {
        common::PURPLE
    }
}

fn paint_policy_color(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PAINT_POLICY_PRESET_INDEX {
        common::DANGER
    } else {
        common::WARN
    }
}

fn preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        CONTENT_VALUE_PRESET_INDEX => "content.value",
        VISUAL_ROLE_PRESET_INDEX => "visual.role",
        A11Y_LABEL_PRESET_INDEX => "a11y.label",
        THEME_COLOR_PRESET_INDEX => "theme.color",
        SVG_SOURCE_PRESET_INDEX => "icon.svg_source",
        SVG_ICON_PRESET_INDEX => "icon.svg_icon",
        VIEW_BOX_PRESET_INDEX => "icon.view_box",
        PATH_SUMMARY_PRESET_INDEX => "icon.path_summary",
        PAINT_POLICY_PRESET_INDEX => "icon.paint_policy",
        ICON_ROLE_PRESET_INDEX => "icon.role",
        COLOR_TOKEN_PRESET_INDEX => "icon.color_token",
        THEME_TOKEN_PRESET_INDEX => "icon.theme_token",
        _ => "icon.option",
    }
}

fn option_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        SVG_ICON_PRESET_INDEX => "UiIconProps object",
        VIEW_BOX_PRESET_INDEX => "viewBox 0 0 24 24",
        PATH_SUMMARY_PRESET_INDEX => "summary: search outline",
        PAINT_POLICY_PRESET_INDEX => "paint: currentColor",
        COLOR_TOKEN_PRESET_INDEX => "color token accent",
        THEME_TOKEN_PRESET_INDEX => "theme token muted",
        _ => "source string and metadata",
    }
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_settings_override() {
        "Inspector override applied"
    } else {
        "preset tab controls body"
    }
}

fn contract_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index >= SVG_SOURCE_PRESET_INDEX {
        "typed svg public API"
    } else {
        "shared atom public API"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::screen_state::StorybookScreenState;

    #[test]
    fn icon_grid_unknown_preset_uses_the_generic_option_name() {
        let state = StorybookScreenState::default();
        let scenario = ScenarioContext::for_test("icon", usize::MAX, &state);

        assert_eq!("icon.option", preset_label(scenario));
    }
}
