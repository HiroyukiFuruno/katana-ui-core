use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

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
const TEXT_WRAP_PRESET_INDEX: usize = 4;
const TEXT_COLOR_TOKEN_PRESET_INDEX: usize = 5;
const TEXT_LINE_METRICS_PRESET_INDEX: usize = 6;
const TEXT_VERTICAL_CENTER_PRESET_INDEX: usize = 7;
const TEXT_RICH_SPANS_PRESET_INDEX: usize = 8;

pub(super) fn theme(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let uses_success_accent = scenario.screen_state.has_settings_override()
        || scenario.screen_state.theme_tokens.hovered()
        || scenario.screen_state.theme_tokens.focused()
        || scenario.screen_state.theme_tokens.keyboard_selected_light()
        || scenario.screen_state.theme_tokens.resized();
    let accent = if uses_success_accent {
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
        _ if scenario.screen_state.theme_tokens.keyboard_selected_light() => (
            LIGHT_BACKGROUND,
            LIGHT_SURFACE,
            LIGHT_PANEL,
            LIGHT_BORDER,
            common::SUCCESS,
            "keyboard: light token",
        ),
        _ if scenario.screen_state.theme_tokens.resized() => (
            palette.background,
            palette.surface,
            palette.panel,
            palette.border,
            common::WARN,
            "resize: spacing token",
        ),
        _ if scenario.screen_state.theme_tokens.focused() => (
            palette.background,
            palette.surface,
            palette.panel,
            common::SUCCESS,
            common::TOKEN,
            "focus: token swatch",
        ),
        _ if scenario.screen_state.theme_tokens.hovered() => (
            palette.background,
            palette.surface,
            palette.panel,
            palette.border,
            common::SUCCESS,
            "hover: accent token",
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
        TEXT_WRAP_PRESET_INDEX => "Wrapped body line",
        TEXT_COLOR_TOKEN_PRESET_INDEX => "Accent token",
        TEXT_LINE_METRICS_PRESET_INDEX => "Line height 18",
        TEXT_VERTICAL_CENTER_PRESET_INDEX => "Centered text",
        TEXT_RICH_SPANS_PRESET_INDEX => "Bold + code span",
        _ => "本文 Body",
    };
    let body_color = if scenario.preset_index == TEXT_THEME_COLOR_PRESET_INDEX
        || scenario.preset_index == TEXT_COLOR_TOKEN_PRESET_INDEX
    {
        palette.accent
    } else {
        palette.text
    };
    let code_value = match scenario.preset_index {
        TEXT_EMPTY_PRESET_INDEX => "empty string is visible",
        TEXT_WRAP_PRESET_INDEX => "wrap: soft -> hard",
        TEXT_LINE_METRICS_PRESET_INDEX => "baseline: +2 px",
        TEXT_VERTICAL_CENTER_PRESET_INDEX => "centered: true",
        TEXT_RICH_SPANS_PRESET_INDEX => "span[0]=strong span[1]=code",
        _ => "let value = \"日本語\";",
    };
    let code_fill = if scenario.preset_index == TEXT_WRAP_PRESET_INDEX
        || scenario.preset_index == TEXT_RICH_SPANS_PRESET_INDEX
    {
        palette.accent
    } else {
        palette.code_background
    };
    let label_color = palette.muted;
    let code_color = if scenario.preset_index == TEXT_COLOR_TOKEN_PRESET_INDEX {
        palette.accent
    } else {
        palette.text
    };
    let body_y = if scenario.preset_index == TEXT_LINE_METRICS_PRESET_INDEX {
        m::PX_48
    } else {
        m::PX_45
    };
    let body_size = if scenario.preset_index == TEXT_LINE_METRICS_PRESET_INDEX {
        m::FONT_13
    } else {
        m::FONT_10
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Text roles / mixed script",
        &[
            Block::new(m::PX_16, m::PX_30, m::PX_66, m::PX_12, code_fill),
            Block::new(m::PX_16, m::PX_44, m::PX_66, m::PX_12, code_fill),
            Block::new(m::PX_16, m::PX_58, m::PX_66, m::PX_12, code_fill),
            Block::new(m::PX_16, m::PX_72, m::PX_66, m::PX_12, code_fill),
        ],
        &[
            TextSpec::new(m::PX_22, m::PX_32, m::FONT_8, label_color, "heading"),
            TextSpec::new(m::PX_92, m::PX_31, m::FONT_13, heading_color, "Heading"),
            TextSpec::new(m::PX_22, m::PX_46, m::FONT_8, label_color, "body"),
            TextSpec::new(m::PX_92, body_y, body_size, body_color, body_value),
            TextSpec::new(m::PX_22, m::PX_60, m::FONT_8, label_color, "code"),
            TextSpec::new(m::PX_92, m::PX_59, m::FONT_9, code_color, code_value),
            TextSpec::new(m::PX_22, m::PX_74, m::FONT_8, label_color, "muted"),
            TextSpec::new(m::PX_92, m::PX_73, m::FONT_9, palette.muted, "Muted UI 🔷"),
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
