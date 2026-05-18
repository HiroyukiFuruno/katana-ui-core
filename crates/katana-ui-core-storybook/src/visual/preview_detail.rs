use super::canvas::Canvas;
use super::dedicated;
use super::layout_metrics::PREVIEW_X;
use super::render_context::{RenderContext, ScenarioContext};
use katana_ui_core::render_model::UiNode;

const HERO_Y: usize = 140;
const HERO_WIDTH: usize = 710;
const HERO_HEIGHT: usize = 244;
const HERO_INSET: usize = 24;
const HERO_PREVIEW_X: usize = PREVIEW_X + 34;
const HERO_PREVIEW_Y: usize = HERO_Y + 86;
const HERO_ACCENT_WIDTH: usize = 5;
const HERO_PRESET_BAND_HEIGHT: usize = 18;
const HERO_TITLE_Y_OFFSET: usize = 24;
const HERO_META_Y_OFFSET: usize = 58;
const HERO_TITLE_SIZE: f32 = 24.0;
const PRESET_TEXT_SIZE: f32 = 12.0;
const CHIP_WIDTH: usize = 146;
const CHIP_HEIGHT: usize = 30;
const CHIP_GAP: usize = 10;
const CHIP_Y_OFFSET_FROM_BOTTOM: usize = 52;
const CHIP_TEXT_X_OFFSET: usize = 10;
const CHIP_TEXT_Y_OFFSET: usize = 9;
const CHIP_TEXT_SIZE: f32 = 10.0;
const INTERACTIVE_PRESET_INDEX: usize = 1;
const EDGE_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;

pub(super) fn draw_selected_hero(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    preview: &UiNode,
    scenario: ScenarioContext<'_>,
) {
    let Some((node, example)) = selected_pair(preview, render.examples, scenario.selected_page)
    else {
        return;
    };
    canvas.fill_rect(
        PREVIEW_X,
        HERO_Y,
        HERO_WIDTH,
        HERO_HEIGHT,
        render.palette.surface,
    );
    if scenario.preset_index > 0 {
        canvas.fill_rect(
            PREVIEW_X,
            HERO_Y,
            HERO_WIDTH,
            HERO_PRESET_BAND_HEIGHT,
            render.palette.accent,
        );
    }
    canvas.stroke_rect(
        PREVIEW_X,
        HERO_Y,
        HERO_WIDTH,
        HERO_HEIGHT,
        render.palette.border,
    );
    canvas.fill_rect(
        PREVIEW_X,
        HERO_Y,
        HERO_ACCENT_WIDTH,
        HERO_HEIGHT,
        render.palette.accent,
    );
    render.text.draw(
        canvas,
        &node.props().label,
        PREVIEW_X + HERO_INSET,
        HERO_Y + HERO_TITLE_Y_OFFSET,
        HERO_TITLE_SIZE,
        render.palette.text,
    );
    render.code_text.draw(
        canvas,
        &format!("page={} / kind={:?}", example.page, node.kind()),
        PREVIEW_X + HERO_INSET,
        HERO_Y + HERO_META_Y_OFFSET,
        PRESET_TEXT_SIZE,
        render.palette.muted,
    );
    dedicated::draw(
        canvas,
        render.text,
        node,
        render.palette,
        HERO_PREVIEW_X,
        HERO_PREVIEW_Y,
    );
    draw_option_chips(canvas, render, node, scenario.preset_index);
}

fn draw_option_chips(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    node: &UiNode,
    preset_index: usize,
) {
    let chips = [
        format!("preset {}", preset_label(preset_index)),
        format!("state {}", node.props().state_id.as_str()),
        format!("variant {:?}", node.props().variant),
        format!("tone {:?}", node.props().tone),
    ];
    let mut x = PREVIEW_X + HERO_INSET;
    let y = HERO_Y + HERO_HEIGHT - CHIP_Y_OFFSET_FROM_BOTTOM;
    for (index, chip) in chips.iter().enumerate() {
        let active_preset = index == 0 && preset_index > 0;
        let fill = if active_preset {
            render.palette.accent
        } else {
            render.palette.panel
        };
        let text_color = if active_preset {
            render.palette.background
        } else {
            render.palette.muted
        };
        canvas.fill_rect(x, y, CHIP_WIDTH, CHIP_HEIGHT, fill);
        canvas.stroke_rect(x, y, CHIP_WIDTH, CHIP_HEIGHT, render.palette.border);
        render.code_text.draw(
            canvas,
            chip,
            x + CHIP_TEXT_X_OFFSET,
            y + CHIP_TEXT_Y_OFFSET,
            CHIP_TEXT_SIZE,
            text_color,
        );
        x += CHIP_WIDTH + CHIP_GAP;
    }
}

fn preset_label(index: usize) -> &'static str {
    match index {
        INTERACTIVE_PRESET_INDEX => "interactive",
        EDGE_PRESET_INDEX => "edge",
        THEME_PRESET_INDEX => "theme",
        _ => "default",
    }
}

fn selected_pair<'a>(
    preview: &'a UiNode,
    examples: &'a [crate::catalog::StoryExample],
    selected_page: &str,
) -> Option<(&'a UiNode, &'a crate::catalog::StoryExample)> {
    preview
        .children()
        .iter()
        .zip(examples.iter())
        .find(|(_, example)| example.page == selected_page)
}
