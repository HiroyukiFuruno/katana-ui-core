use super::canvas::Canvas;
use super::dedicated;
use super::layout_metrics::PREVIEW_X;
use super::preview_effects;
use super::render_context::{RenderContext, ScenarioContext};
use katana_ui_core::render_model::UiNode;

#[path = "preview_detail_interaction.rs"]
mod preview_detail_interaction;

fn preview_scroll_y(scenario: ScenarioContext<'_>) -> usize {
    preview_detail_interaction::preview_scroll_y(scenario)
}

fn preview_scroll_x(scenario: ScenarioContext<'_>) -> usize {
    preview_detail_interaction::preview_scroll_x(scenario)
}

fn draw_runtime_state(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
) {
    preview_detail_interaction::draw_runtime_state(canvas, render, scenario)
}

pub(super) fn button_action_hit_rect(page: &str) -> super::layout_metrics::LayoutRect {
    preview_detail_interaction::button_action_hit_rect(page)
}

pub(super) fn component_action_hit_rect(page: &str) -> super::layout_metrics::LayoutRect {
    preview_detail_interaction::component_action_hit_rect(page)
}

pub(super) const fn selected_hero_y() -> usize {
    preview_detail_interaction::selected_hero_y()
}

pub(super) const fn selected_hero_rect() -> (usize, usize, usize, usize) {
    preview_detail_interaction::selected_hero_rect()
}

#[cfg(test)]
pub(super) const HERO_PREVIEW_X_FOR_TEST: usize =
    preview_detail_interaction::HERO_PREVIEW_X_FOR_TEST;
#[cfg(test)]
pub(super) const HERO_PREVIEW_Y_FOR_TEST: usize =
    preview_detail_interaction::HERO_PREVIEW_Y_FOR_TEST;

const HERO_Y: usize = 136;
const HERO_WIDTH: usize = 710;
const HERO_HEIGHT: usize = 244;
const HERO_INSET: usize = 24;
const HERO_PREVIEW_X: usize = PREVIEW_X + 34;
const HERO_PREVIEW_Y: usize = HERO_Y + 86;
const HERO_ACCENT_WIDTH: usize = 5;
const HERO_TITLE_Y_OFFSET: usize = 24;
const HERO_META_Y_OFFSET: usize = 58;
const HERO_TITLE_SIZE: f32 = 24.0;
const PRESET_TEXT_SIZE: f32 = 12.0;
pub(super) fn draw_selected_hero(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    _preview: &UiNode,
    scenario: ScenarioContext<'_>,
) {
    let Some(example) = selected_example(render.examples, scenario.selected_page) else {
        return;
    };
    let node = example.tree.root();
    let preview_y = preview_scroll_y(scenario);
    let preview_x = preview_scroll_x(scenario);
    let hero_y = HERO_Y.saturating_sub(preview_y);
    canvas.fill_rect(
        PREVIEW_X,
        hero_y,
        HERO_WIDTH,
        HERO_HEIGHT,
        render.palette.surface,
    );
    canvas.stroke_rect(
        PREVIEW_X,
        hero_y,
        HERO_WIDTH,
        HERO_HEIGHT,
        render.palette.border,
    );
    canvas.fill_rect(
        PREVIEW_X,
        hero_y,
        HERO_ACCENT_WIDTH,
        HERO_HEIGHT,
        render.palette.accent,
    );
    render.text.draw(
        canvas,
        &node.props().label,
        PREVIEW_X + HERO_INSET,
        hero_y + HERO_TITLE_Y_OFFSET,
        HERO_TITLE_SIZE,
        render.palette.text,
    );
    render.code_text.draw(
        canvas,
        &format!("page={} / kind={:?}", example.page, node.kind()),
        PREVIEW_X + HERO_INSET,
        hero_y + HERO_META_Y_OFFSET,
        PRESET_TEXT_SIZE,
        render.palette.muted,
    );
    let component_x = HERO_PREVIEW_X.saturating_sub(preview_x);
    canvas.with_clip(PREVIEW_X, hero_y, HERO_WIDTH, HERO_HEIGHT, |canvas| {
        dedicated::draw_page(
            canvas,
            dedicated::DedicatedPageRequest {
                text: render.text,
                page: example.page,
                node,
                palette: render.palette,
                scenario,
                x: component_x,
                y: hero_y + (HERO_PREVIEW_Y - HERO_Y),
            },
        );
        preview_effects::draw(
            canvas,
            render,
            scenario,
            component_action_hit_rect(scenario.selected_page),
        );
        draw_runtime_state(canvas, render, scenario);
    });
}

fn selected_example<'a>(
    examples: &'a [crate::catalog::StoryExample],
    selected_page: &str,
) -> Option<&'a crate::catalog::StoryExample> {
    examples
        .iter()
        .find(|example| example.page == selected_page)
}
