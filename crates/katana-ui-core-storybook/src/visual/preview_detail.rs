use super::canvas::Canvas;
use super::dedicated;
use super::layout_metrics::{LayoutRect, PREVIEW_X};
use super::panel_scroll_state::PanelScrollRegion;
use super::panel_scrollbars;
use super::preview_effects;
use super::render_context::{RenderContext, ScenarioContext};
use katana_ui_core::render_model::UiNode;

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
const ACTION_TEXT_SIZE: f32 = 10.0;
const TEXT_BUTTON_REL_X: usize = 16;
const TEXT_BUTTON_REL_Y: usize = 50;
const BUTTON_WIDTH: usize = 106;
const TEXT_BUTTON_WIDTH: usize = 106;
const TEXT_BUTTON_HEIGHT: usize = 40;
const SVG_BUTTON_REL_X: usize = 22;
const SVG_BUTTON_REL_Y: usize = 50;
const SVG_BUTTON_WIDTH: usize = 44;
const SVG_BUTTON_HEIGHT: usize = 40;
const ICON_TEXT_BUTTON_REL_X: usize = 20;
const ICON_TEXT_BUTTON_REL_Y: usize = 50;
const ICON_TEXT_BUTTON_WIDTH: usize = 138;
const ICON_TEXT_BUTTON_HEIGHT: usize = 40;
const TOGGLE_REL_X: usize = 18;
const TOGGLE_REL_Y: usize = 36;
const TOGGLE_ROW_WIDTH: usize = 294;
const TOGGLE_ROW_HEIGHT: usize = 34;
const GENERIC_ACTION_WIDTH: usize = 344;
const GENERIC_ACTION_HEIGHT: usize = 108;
const ACTION_MARKER_HEIGHT: usize = 4;
const ACTION_LABEL_X_OFFSET: usize = 18;
const ACTION_LABEL_Y_OFFSET: usize = 18;
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

fn preview_scroll_y(scenario: ScenarioContext<'_>) -> usize {
    if panel_scrollbars::vertical_region_scrollable_for(
        PanelScrollRegion::Preview,
        scenario.selected_page,
        scenario.tree_expansion,
    ) {
        return scenario.panel_scroll.preview_y;
    }
    0
}

fn preview_scroll_x(scenario: ScenarioContext<'_>) -> usize {
    if panel_scrollbars::horizontal_region_scrollable_for(
        PanelScrollRegion::Preview,
        scenario.selected_page,
        scenario.tree_expansion,
    ) {
        return scenario.panel_scroll.preview_x;
    }
    0
}

fn draw_runtime_state(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
) {
    if is_button_page(scenario.selected_page) {
        return;
    }
    let rect = component_action_hit_rect(scenario.selected_page);
    if rect.width == 0 {
        return;
    }
    if scenario.screen_state.has_settings_override() {
        canvas.stroke_rect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            render.palette.accent,
        );
    }
    if !scenario.screen_state.has_widget_action() {
        return;
    }
    canvas.fill_rect(
        rect.x,
        rect.bottom() - ACTION_MARKER_HEIGHT,
        rect.width,
        ACTION_MARKER_HEIGHT,
        render.palette.accent,
    );
    render.code_text.draw(
        canvas,
        &format!("clicked {}", scenario.screen_state.action_count),
        rect.x + ACTION_LABEL_X_OFFSET,
        rect.bottom().saturating_sub(ACTION_LABEL_Y_OFFSET),
        ACTION_TEXT_SIZE,
        render.palette.text,
    );
}

fn is_button_page(page: &str) -> bool {
    matches!(
        page,
        "button" | "text-button" | "svg-button" | "icon-text-button"
    )
}

pub(super) fn button_action_hit_rect(page: &str) -> LayoutRect {
    match page {
        "svg-button" => LayoutRect::new(
            HERO_PREVIEW_X + SVG_BUTTON_REL_X,
            HERO_PREVIEW_Y + SVG_BUTTON_REL_Y,
            SVG_BUTTON_WIDTH,
            SVG_BUTTON_HEIGHT,
        ),
        "icon-text-button" => LayoutRect::new(
            HERO_PREVIEW_X + ICON_TEXT_BUTTON_REL_X,
            HERO_PREVIEW_Y + ICON_TEXT_BUTTON_REL_Y,
            ICON_TEXT_BUTTON_WIDTH,
            ICON_TEXT_BUTTON_HEIGHT,
        ),
        "text-button" => LayoutRect::new(
            HERO_PREVIEW_X + TEXT_BUTTON_REL_X,
            HERO_PREVIEW_Y + TEXT_BUTTON_REL_Y,
            TEXT_BUTTON_WIDTH,
            TEXT_BUTTON_HEIGHT,
        ),
        "button" => LayoutRect::new(
            HERO_PREVIEW_X + TEXT_BUTTON_REL_X,
            HERO_PREVIEW_Y + TEXT_BUTTON_REL_Y,
            BUTTON_WIDTH,
            TEXT_BUTTON_HEIGHT,
        ),
        _ => LayoutRect::new(0, 0, 0, 0),
    }
}

pub(super) fn component_action_hit_rect(page: &str) -> LayoutRect {
    let button = button_action_hit_rect(page);
    if button.width > 0 {
        return button;
    }
    if page == "toggle" {
        return LayoutRect::new(
            HERO_PREVIEW_X + TOGGLE_REL_X,
            HERO_PREVIEW_Y + TOGGLE_REL_Y,
            TOGGLE_ROW_WIDTH,
            TOGGLE_ROW_HEIGHT,
        );
    }
    LayoutRect::new(
        HERO_PREVIEW_X,
        HERO_PREVIEW_Y,
        GENERIC_ACTION_WIDTH,
        GENERIC_ACTION_HEIGHT,
    )
}

pub(super) const fn selected_hero_y() -> usize {
    HERO_Y
}

#[cfg(test)]
pub(super) const HERO_PREVIEW_X_FOR_TEST: usize = HERO_PREVIEW_X;

#[cfg(test)]
pub(super) const HERO_PREVIEW_Y_FOR_TEST: usize = HERO_PREVIEW_Y;

pub(super) const fn selected_hero_rect() -> (usize, usize, usize, usize) {
    (PREVIEW_X, HERO_Y, HERO_WIDTH, HERO_HEIGHT)
}

fn selected_example<'a>(
    examples: &'a [crate::catalog::StoryExample],
    selected_page: &str,
) -> Option<&'a crate::catalog::StoryExample> {
    examples
        .iter()
        .find(|example| example.page == selected_page)
}
