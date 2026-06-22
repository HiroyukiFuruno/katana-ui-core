use super::canvas::Canvas;
use super::dedicated_dod_atom_button_live_status::draw_status_rows;
use super::dedicated_dod_atom_button_live_surface::{
    button_layout, draw_button_label, draw_button_surface, measure_button_label_width,
};
use super::dedicated_dod_common::{self as common, Rect};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const BUTTON_X: usize = 16;
const BUTTON_Y: usize = 50;
const SVG_BUTTON_X: usize = 22;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
    title: &str,
) {
    let kind = ButtonLiveKind::from_title(title);
    common::frame(canvas, text, palette, x, y, title);
    let label = button_label_for(scenario, kind);
    let label_width = measure_button_label_width(text, label);
    let layout = button_layout(
        scenario
            .screen_state
            .button_options
            .effective_preset_index(scenario.preset_index),
        scenario.screen_state.button_options.width_mode,
        scenario.screen_state.button_options.height_mode,
        label_width,
        kind.has_icon(),
        kind.has_visible_label(),
    );
    let rect = Rect::new(
        x + button_x(kind),
        y + BUTTON_Y,
        layout.width,
        layout.height,
    );
    draw_button_surface(canvas, palette, scenario, rect, kind);
    draw_button_label(canvas, text, palette, scenario, rect, label, kind);
    draw_status_rows(canvas, text, palette, scenario, x, y);
}

fn button_label_for(scenario: ScenarioContext<'_>, kind: ButtonLiveKind) -> &'static str {
    let fallback = if scenario.screen_state.is_button_pressed() {
        "Pressed"
    } else if scenario.screen_state.has_settings_override() {
        "Outline"
    } else {
        kind.default_label()
    };
    scenario.screen_state.button_options.label(fallback)
}

const fn button_x(kind: ButtonLiveKind) -> usize {
    match kind {
        ButtonLiveKind::SvgButton => SVG_BUTTON_X,
        ButtonLiveKind::Button | ButtonLiveKind::TextButton | ButtonLiveKind::IconTextButton => {
            BUTTON_X
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ButtonLiveKind {
    Button,
    TextButton,
    SvgButton,
    IconTextButton,
}

impl ButtonLiveKind {
    pub(super) fn from_title(title: &str) -> Self {
        match title {
            "TextButton" => Self::TextButton,
            "SvgButton" => Self::SvgButton,
            "IconTextButton" => Self::IconTextButton,
            _ => Self::Button,
        }
    }

    pub(super) const fn has_icon(self) -> bool {
        matches!(self, Self::SvgButton | Self::IconTextButton)
    }

    pub(super) const fn has_visible_label(self) -> bool {
        !matches!(self, Self::SvgButton)
    }

    const fn default_label(self) -> &'static str {
        match self {
            Self::Button => "Save changes",
            Self::TextButton => "Text action",
            Self::SvgButton => "Svg action",
            Self::IconTextButton => "Open folder",
        }
    }
}
