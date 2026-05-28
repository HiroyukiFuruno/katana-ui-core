use crate::visual::layout_metrics::LayoutRect;
use crate::visual::layout_metrics::PREVIEW_X;
use crate::visual::{
    canvas::Canvas,
    panel_scroll_state::{self, PanelScrollRegion},
    render_context::{RenderContext, ScenarioContext},
};

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
const GENERIC_ACTION_HEIGHT: usize = 132;
const TABS_ACTION_WIDTH: usize = 520;
const ACTION_MARKER_HEIGHT: usize = 4;
const ACTION_LABEL_X_OFFSET: usize = 18;
const ACTION_LABEL_Y_OFFSET: usize = 18;

const HERO_Y: usize = 136;
const HERO_WIDTH: usize = 710;
const HERO_HEIGHT: usize = 244;
const HERO_PREVIEW_X: usize = PREVIEW_X + 34;
const HERO_PREVIEW_Y: usize = HERO_Y + 86;
const PANEL_ACTION_WIDTH: usize = 676;
const PANEL_ACTION_HEIGHT: usize = 344;

pub(super) fn preview_scroll_y(scenario: ScenarioContext<'_>) -> usize {
    let max_scroll = panel_scroll_state::PanelScrollOverflowModel::max_scroll_y_for(
        PanelScrollRegion::Preview,
        scenario.selected_page,
        scenario.tree_expansion,
    );
    if max_scroll > 0 {
        scenario
            .panel_scroll
            .offset_with_max(PanelScrollRegion::Preview, max_scroll)
    } else {
        0
    }
}

pub(super) fn preview_scroll_x(scenario: ScenarioContext<'_>) -> usize {
    let max_scroll = panel_scroll_state::PanelScrollOverflowModel::max_scroll_x_for(
        PanelScrollRegion::Preview,
        scenario.selected_page,
        scenario.tree_expansion,
    );
    if max_scroll > 0 {
        scenario
            .panel_scroll
            .offset_x_with_max(PanelScrollRegion::Preview, max_scroll)
    } else {
        0
    }
}

pub(super) fn draw_runtime_state(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
) {
    if is_button_page(scenario.selected_page) {
        return;
    }
    if scenario.selected_page == "panel" {
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
    if !should_draw_runtime_label(scenario.selected_page) {
        return;
    }
    render.code_text.draw(
        canvas,
        &format!("clicked {}", scenario.screen_state.action_count),
        rect.x + ACTION_LABEL_X_OFFSET,
        rect.bottom().saturating_sub(ACTION_LABEL_Y_OFFSET),
        ACTION_TEXT_SIZE,
        render.palette.text,
    );
}

fn should_draw_runtime_label(page: &str) -> bool {
    !matches!(page, "text-input" | "text-area")
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
    if page == "panel" {
        return LayoutRect::new(
            HERO_PREVIEW_X,
            HERO_PREVIEW_Y,
            PANEL_ACTION_WIDTH,
            PANEL_ACTION_HEIGHT,
        );
    }
    if page == "toggle" {
        return LayoutRect::new(
            HERO_PREVIEW_X + TOGGLE_REL_X,
            HERO_PREVIEW_Y + TOGGLE_REL_Y,
            TOGGLE_ROW_WIDTH,
            TOGGLE_ROW_HEIGHT,
        );
    }
    if page == "tabs" {
        return LayoutRect::new(
            HERO_PREVIEW_X,
            HERO_PREVIEW_Y,
            TABS_ACTION_WIDTH,
            GENERIC_ACTION_HEIGHT,
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
