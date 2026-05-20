#[path = "dedicated_dod_atom_button_live_surface_common_props.rs"]
mod common_props;
#[path = "dedicated_dod_atom_button_live_surface_label.rs"]
mod label;
#[path = "dedicated_dod_atom_button_live_surface_layout.rs"]
mod layout;
#[path = "dedicated_dod_atom_button_live_surface_material.rs"]
mod material;
use super::canvas::Canvas;
use super::dedicated_dod_atom_button_live::ButtonLiveKind;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
pub(super) use label::draw_button_label;
pub(in crate::visual) use layout::button_layout;
use material::draw_material_surface;

const CODE: u32 = 0x2d2d30;
const MODERN_HEIGHT: usize = 40;
const CLASSIC_HEIGHT: usize = 38;
const BASIC_HEIGHT: usize = 34;
const DENSE_HEIGHT: usize = 28;
const CUSTOM_WIDTH: usize = 220;
const PERCENT_WIDTH: usize = 248;
const FILL_WIDTH: usize = 304;
const BUTTON_LABEL_SIZE: f32 = 11.5;
const BUTTON_LABEL_AVG_WIDTH: usize = 6;
const BUTTON_LABEL_ICON_OFFSET: usize = 20;
const BUTTON_PADDING_X: usize = 34;
const BUTTON_ICON_GAP: usize = 18;
const BUTTON_RADIUS: usize = 6;
const BUTTON_BORDER: usize = 1;
const MODERN_MIN_WIDTH: usize = 96;
const CLASSIC_MIN_WIDTH: usize = 90;
const BASIC_MIN_WIDTH: usize = 82;
const DENSE_MIN_WIDTH: usize = 70;
const ICON_SIZE: usize = 12;
const ICON_ONLY_SIZE: usize = 14;
const INNER_STROKE_INSET: usize = 2;
const INNER_STROKE_REDUCTION: usize = 3;
const SHADOW_OFFSET: usize = 4;
const CLASSIC_PRESET_INDEX: usize = 1;
const BASIC_PRESET_INDEX: usize = 2;
const DENSE_PRESET_INDEX: usize = 3;
const HOVER_BLEND_ALPHA: u8 = 48;
const PRESSED_BLEND_ALPHA: u8 = 68;

pub(super) fn draw_button_surface(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
    kind: ButtonLiveKind,
) {
    let options = scenario.screen_state.button_options;
    common_props::draw_z_index_shadow(canvas, palette, rect, options.z_index);
    if !options.visible {
        common_props::draw_invisible_placeholder(canvas, palette, rect);
        return;
    }
    let fill = button_fill(palette, scenario, kind);
    draw_material_surface(canvas, palette, scenario, rect, kind, fill);
    draw_text_button_underline(canvas, palette, scenario, rect, kind);
    common_props::draw_setting_outline(canvas, palette, scenario, rect);
}

fn button_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    kind: ButtonLiveKind,
) -> u32 {
    if scenario.screen_state.button_options.disabled {
        return CODE;
    }
    if matches!(kind, ButtonLiveKind::TextButton) {
        return if scenario.screen_state.has_widget_action() {
            palette.surface
        } else {
            palette.panel
        };
    }
    if scenario.screen_state.has_settings_override() {
        return palette.surface;
    }
    if scenario.screen_state.has_widget_action() {
        return common::SUCCESS;
    }
    match scenario.preset_index {
        CLASSIC_PRESET_INDEX => palette.surface,
        BASIC_PRESET_INDEX => palette.background,
        _ => palette.accent,
    }
}

fn draw_text_button_underline(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    rect: Rect,
    kind: ButtonLiveKind,
) {
    if !matches!(kind, ButtonLiveKind::TextButton) || !scenario.screen_state.button_options.border {
        return;
    }
    let color = if scenario.screen_state.has_widget_action() {
        common::SUCCESS
    } else {
        palette.accent
    };
    canvas.fill_rect(
        rect.x,
        rect.y + rect.height - m::PX_3,
        rect.width,
        m::PX_2,
        color,
    );
}
