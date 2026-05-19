use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) fn loading_dots(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let active_color = if scenario.screen_state.has_settings_override() {
        common::SUCCESS
    } else {
        palette.accent
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "LoadingDots",
        &[
            Block::new(m::PX_32, m::PX_48, m::PX_6, m::PX_6, active_color),
            Block::new(m::PX_56, m::PX_45, m::PX_8, m::PX_8, common::TOKEN),
            Block::new(m::PX_80, m::PX_48, m::PX_10, m::PX_10, common::PURPLE),
            Block::new(m::PX_104, m::PX_45, m::PX_8, m::PX_8, common::WARN),
            Block::outlined(m::PX_188, m::PX_38, m::PX_96, m::PX_20, palette.surface),
        ],
        &[
            TextSpec::new(
                m::PX_198,
                m::PX_43,
                m::FONT_9,
                palette.muted,
                "reduced motion",
            ),
            TextSpec::new(
                m::PX_34,
                m::PX_82,
                m::FONT_9,
                palette.muted,
                "phase=3 speed=fast label=Loading",
            ),
        ],
    );
}
pub(super) fn spinner(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let leading_color = if scenario.screen_state.has_settings_override() {
        common::SUCCESS
    } else {
        palette.accent
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Spinner",
        &[
            Block::outlined(m::PX_34, m::PX_38, m::PX_72, m::PX_50, palette.panel),
            Block::new(m::PX_62, m::PX_40, m::PX_16, m::PX_6, leading_color),
            Block::new(m::PX_70, m::PX_47, m::PX_16, m::PX_6, common::TOKEN),
            Block::new(m::PX_78, m::PX_54, m::PX_16, m::PX_6, common::PURPLE),
            Block::new(m::PX_86, m::PX_61, m::PX_16, m::PX_6, common::WARN),
        ],
        &[
            TextSpec::new(
                m::PX_128,
                m::PX_42,
                m::FONT_9,
                palette.muted,
                "motion tick: 6/12",
            ),
            TextSpec::new(
                m::PX_128,
                m::PX_60,
                m::FONT_9,
                palette.muted,
                "reduced motion: paused",
            ),
            TextSpec::new(
                m::PX_128,
                m::PX_78,
                m::FONT_9,
                palette.muted,
                "label: Saving",
            ),
        ],
    );
}
pub(super) fn progress(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let progress_width = if scenario.screen_state.has_settings_override() {
        m::PX_204
    } else {
        m::PX_164
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "ProgressBar",
        &[
            Block::outlined(m::PX_20, m::PX_44, m::PX_244, m::PX_18, palette.panel),
            Block::new(m::PX_22, m::PX_46, progress_width, m::PX_14, palette.accent),
            Block::new(m::PX_22, m::PX_72, m::PX_244, m::PX_8, palette.surface),
        ],
        &[
            TextSpec::new(m::PX_278, m::PX_46, m::FONT_9, palette.muted, "65%"),
            TextSpec::new(
                m::PX_20,
                m::PX_90,
                m::FONT_9,
                palette.muted,
                "determinate / indeterminate",
            ),
        ],
    );
}
