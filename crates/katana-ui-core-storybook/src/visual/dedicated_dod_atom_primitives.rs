use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const CODE: u32 = 0x2d2d30;
const MUTED_TEXT: u32 = 0x8f98a8;
const THEME_LIGHT_PRESET_INDEX: usize = 1;
const THEME_CONTRAST_PRESET_INDEX: usize = 2;
const THEME_ACCENT_PRESET_INDEX: usize = 3;
const LIGHT_BACKGROUND: u32 = 0xf8fafc;
const LIGHT_SURFACE: u32 = 0xe2e8f0;
const LIGHT_PANEL: u32 = 0xffffff;
const LIGHT_BORDER: u32 = 0x94a3b8;
const CONTRAST_BACKGROUND: u32 = 0x111827;
const CONTRAST_SURFACE: u32 = 0x4338ca;
const CONTRAST_PANEL: u32 = 0xf59e0b;
const CONTRAST_BORDER: u32 = 0xef4444;
const TEXT_MIXED_SCRIPT_PRESET_INDEX: usize = 1;
const TEXT_EMPTY_PRESET_INDEX: usize = 2;
const TEXT_THEME_COLOR_PRESET_INDEX: usize = 3;
const ICON_ACCENT_PRESET_INDEX: usize = 1;
const ICON_CUSTOM_PRESET_INDEX: usize = 2;
const ICON_MUTED_PRESET_INDEX: usize = 3;
const ICON_COUNT: usize = 4;

pub(super) fn theme(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let accent = if scenario.screen_state.has_settings_override() {
        common::SUCCESS
    } else if scenario.preset_index == THEME_ACCENT_PRESET_INDEX {
        common::WARN
    } else {
        palette.accent
    };
    let (background, surface, panel, border, token, label) =
        theme_preview_tokens(palette, scenario);
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Theme / Panel theme",
        &[
            Block::outlined(m::PX_16, m::PX_34, m::PX_72, m::PX_38, background),
            Block::outlined(m::PX_98, m::PX_34, m::PX_72, m::PX_38, surface),
            Block::outlined(m::PX_180, m::PX_34, m::PX_72, m::PX_38, panel),
            Block::new(m::PX_16, m::PX_84, m::PX_18, m::PX_12, accent),
            Block::new(m::PX_40, m::PX_84, m::PX_18, m::PX_12, border),
            Block::new(m::PX_64, m::PX_84, m::PX_18, m::PX_12, token),
            Block::new(m::PX_88, m::PX_84, m::PX_18, m::PX_12, common::PURPLE),
            Block::new(m::PX_112, m::PX_84, m::PX_18, m::PX_12, common::WARN),
        ],
        &[
            TextSpec::new(m::PX_24, m::PX_46, m::FONT_9, palette.text, "nav"),
            TextSpec::new(m::PX_110, m::PX_46, m::FONT_9, palette.text, "preview"),
            TextSpec::new(m::PX_190, m::PX_46, m::FONT_9, palette.text, "inspector"),
            TextSpec::new(m::PX_174, m::PX_84, m::FONT_9, palette.text, label),
        ],
    );
}

fn theme_preview_tokens(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> (u32, u32, u32, u32, u32, &'static str) {
    match scenario.preset_index {
        THEME_LIGHT_PRESET_INDEX => (
            LIGHT_BACKGROUND,
            LIGHT_SURFACE,
            LIGHT_PANEL,
            LIGHT_BORDER,
            common::TOKEN,
            "preset: light token ramp",
        ),
        THEME_CONTRAST_PRESET_INDEX => (
            CONTRAST_BACKGROUND,
            CONTRAST_SURFACE,
            CONTRAST_PANEL,
            CONTRAST_BORDER,
            common::PURPLE,
            "preset: contrast surface",
        ),
        THEME_ACCENT_PRESET_INDEX => (
            palette.background,
            palette.surface,
            palette.panel,
            common::WARN,
            common::SUCCESS,
            "preset: accent override",
        ),
        _ => (
            palette.background,
            palette.surface,
            palette.panel,
            palette.border,
            common::TOKEN,
            "theme action: light -> dark",
        ),
    }
}
pub(super) fn text_grid(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let heading_color = if scenario.screen_state.has_settings_override() {
        palette.accent
    } else {
        palette.text
    };
    let body_value = match scenario.preset_index {
        TEXT_MIXED_SCRIPT_PRESET_INDEX => "日本語 Body 🔷",
        TEXT_EMPTY_PRESET_INDEX => "",
        TEXT_THEME_COLOR_PRESET_INDEX => "Theme color",
        _ => "本文 Body",
    };
    let body_color = if scenario.preset_index == TEXT_THEME_COLOR_PRESET_INDEX {
        palette.accent
    } else {
        palette.text
    };
    let code_value = if scenario.preset_index == TEXT_EMPTY_PRESET_INDEX {
        "empty string is visible"
    } else {
        "let value = \"日本語\";"
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Text roles / mixed script",
        &[
            Block::new(m::PX_16, m::PX_30, m::PX_66, m::PX_12, CODE),
            Block::new(m::PX_16, m::PX_44, m::PX_66, m::PX_12, CODE),
            Block::new(m::PX_16, m::PX_58, m::PX_66, m::PX_12, CODE),
            Block::new(m::PX_16, m::PX_72, m::PX_66, m::PX_12, CODE),
        ],
        &[
            TextSpec::new(m::PX_22, m::PX_32, m::FONT_8, common::WARN, "heading"),
            TextSpec::new(m::PX_92, m::PX_31, m::FONT_13, heading_color, "Heading"),
            TextSpec::new(m::PX_22, m::PX_46, m::FONT_8, common::WARN, "body"),
            TextSpec::new(m::PX_92, m::PX_45, m::FONT_10, body_color, body_value),
            TextSpec::new(m::PX_22, m::PX_60, m::FONT_8, common::WARN, "code"),
            TextSpec::new(m::PX_92, m::PX_59, m::FONT_9, common::TOKEN, code_value),
            TextSpec::new(m::PX_22, m::PX_74, m::FONT_8, common::WARN, "muted"),
            TextSpec::new(m::PX_92, m::PX_73, m::FONT_9, MUTED_TEXT, "Muted UI 🔷"),
            TextSpec::new(
                m::PX_16,
                m::PX_96,
                m::FONT_8,
                palette.muted,
                "English / 日本語 / emoji aligned",
            ),
        ],
    );
}
pub(super) fn icon_grid(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let custom_color = if scenario.screen_state.has_settings_override() {
        palette.accent
    } else {
        icon_custom_color(palette, scenario)
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "SVG Icon grid",
        &[
            Block::outlined(m::PX_18, m::PX_36, m::PX_36, m::PX_36, palette.surface),
            Block::outlined(m::PX_62, m::PX_36, m::PX_36, m::PX_36, palette.surface),
            Block::outlined(m::PX_106, m::PX_36, m::PX_36, m::PX_36, palette.surface),
            Block::outlined(m::PX_150, m::PX_36, m::PX_36, m::PX_36, palette.surface),
        ],
        &[
            TextSpec::new(m::PX_22, m::PX_78, m::FONT_8, palette.muted, "12"),
            TextSpec::new(m::PX_66, m::PX_78, m::FONT_8, palette.muted, "16"),
            TextSpec::new(m::PX_110, m::PX_78, m::FONT_8, palette.muted, "20"),
            TextSpec::new(m::PX_148, m::PX_78, m::FONT_8, palette.muted, "custom"),
            TextSpec::new(m::PX_212, m::PX_42, m::FONT_9, palette.muted, "a11y label"),
            TextSpec::new(m::PX_212, m::PX_60, m::FONT_9, palette.muted, "color token"),
            TextSpec::new(
                m::PX_212,
                m::PX_78,
                m::FONT_9,
                palette.muted,
                "typed svg props",
            ),
        ],
    );
    for (index, (size, color)) in icon_specs(palette, scenario, custom_color)
        .iter()
        .enumerate()
    {
        let origin = Rect::new(
            x + m::PX_18 + index * m::PX_44,
            y + m::PX_36,
            m::PX_36,
            m::PX_36,
        );
        let inset = (m::PX_36 - size) / m::PX_2;
        common::cross_icon(canvas, origin.x + inset, origin.y + inset, *size, *color);
    }
}

fn icon_custom_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ICON_MUTED_PRESET_INDEX {
        return palette.muted;
    }
    common::WARN
}

fn icon_specs(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    custom_color: u32,
) -> [(usize, u32); ICON_COUNT] {
    match scenario.preset_index {
        ICON_ACCENT_PRESET_INDEX => [
            (m::PX_12, palette.accent),
            (m::PX_16, palette.accent),
            (m::PX_20, palette.accent),
            (m::PX_24, palette.accent),
        ],
        ICON_CUSTOM_PRESET_INDEX => [
            (m::PX_18, palette.accent),
            (m::PX_22, common::TOKEN),
            (m::PX_26, common::PURPLE),
            (m::PX_30, custom_color),
        ],
        ICON_MUTED_PRESET_INDEX => [
            (m::PX_12, palette.muted),
            (m::PX_16, palette.muted),
            (m::PX_20, palette.muted),
            (m::PX_24, custom_color),
        ],
        _ => [
            (m::PX_12, palette.accent),
            (m::PX_16, common::TOKEN),
            (m::PX_20, common::PURPLE),
            (m::PX_24, custom_color),
        ],
    }
}
