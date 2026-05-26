#[path = "dedicated_foundation_panel/body.rs"]
mod body;
#[path = "dedicated_foundation_panel/model.rs"]
mod model;
#[path = "dedicated_foundation_panel/scrollbars.rs"]
mod scrollbars;
#[path = "dedicated_foundation_panel/status.rs"]
mod status;

use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;
use katana_ui_core::render_model::UiNode;
pub(in crate::visual) use model::panel_at;
use model::{DETAILS_SLOT, NAV_SLOT, PREVIEW_SLOT, ROOT_HEIGHT, ROOT_WIDTH, ROOT_X, ROOT_Y};

const SURFACE_EXTRA_HEIGHT: usize = 18;
const TITLE_X: usize = 16;
const TITLE_Y: usize = 14;
const TITLE_SIZE: f32 = 13.0;
const SUBTITLE_Y: usize = 34;
const SUBTITLE_SIZE: f32 = 9.0;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    draw_surface(canvas, text, palette, x, y);
    draw_root_panel(canvas, text, node, palette, scenario, x, y);
    status::draw(canvas, text, palette, node, scenario, x, y);
}

fn draw_surface(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    canvas.fill_rect(
        x,
        y,
        ROOT_WIDTH + ROOT_X * 2,
        ROOT_HEIGHT + ROOT_Y + SURFACE_EXTRA_HEIGHT,
        palette.panel,
    );
    canvas.stroke_rect(
        x,
        y,
        ROOT_WIDTH + ROOT_X * 2,
        ROOT_HEIGHT + ROOT_Y + SURFACE_EXTRA_HEIGHT,
        palette.border,
    );
    canvas.fill_rect(
        x,
        y,
        m::PX_4,
        ROOT_HEIGHT + ROOT_Y + SURFACE_EXTRA_HEIGHT,
        palette.accent,
    );
    text.draw(
        canvas,
        "Panel root foundation",
        x + TITLE_X,
        y + TITLE_Y,
        TITLE_SIZE,
        palette.text,
    );
    text.draw(
        canvas,
        "viewport / clipping / local scroll state",
        x + TITLE_X,
        y + SUBTITLE_Y,
        SUBTITLE_SIZE,
        palette.muted,
    );
}

fn draw_root_panel(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::draw_blocks(
        canvas,
        palette,
        x,
        y,
        &[Block::outlined(
            ROOT_X,
            ROOT_Y,
            ROOT_WIDTH,
            ROOT_HEIGHT,
            palette.surface,
        )],
    );
    draw_child_panel_frame(canvas, text, palette, scenario, x, y, NAV_SLOT);
    draw_child_panel_frame(canvas, text, palette, scenario, x, y, PREVIEW_SLOT);
    draw_child_panel_frame(canvas, text, palette, scenario, x, y, DETAILS_SLOT);
    body::draw(canvas, text, palette, scenario, x, y);
    draw_child_panel_overlay(canvas, palette, scenario, x, y, NAV_SLOT, node);
    draw_child_panel_overlay(canvas, palette, scenario, x, y, PREVIEW_SLOT, node);
    draw_child_panel_overlay(canvas, palette, scenario, x, y, DETAILS_SLOT, node);
}

fn draw_child_panel_frame(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
    slot: model::PanelSlot,
) {
    let rect = slot.rect(x, y);
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.panel);
    let border = if scenario.screen_state.panel.active_panel == slot.key {
        palette.accent
    } else {
        palette.border
    };
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, border);
    if scenario.screen_state.panel.active_panel == slot.key {
        canvas.fill_rect(rect.x, rect.y, m::PX_4, rect.height, palette.accent);
    }
    text.draw(
        canvas,
        slot.label,
        rect.x + model::TEXT_X_OFFSET,
        rect.y + model::TEXT_Y_OFFSET,
        model::LABEL_SIZE,
        palette.text,
    );
}

fn draw_child_panel_overlay(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
    slot: model::PanelSlot,
    root: &UiNode,
) {
    let rect = slot.rect(x, y);
    if let Some(panel) = model::child_panel(root, slot.node_label) {
        let props = model::panel_props_for_slot(slot, &panel.props().panel, scenario);
        scrollbars::draw(canvas, palette, scenario, rect, &props);
    }
}
