use super::canvas::Canvas;
use super::inspector;
use super::layout_metrics::{
    BRAND_X, NAV_WIDTH, SCROLLBAR_CONTROL_HEIGHT, SCROLLBAR_CONTROL_WIDTH, SCROLLBAR_CONTROL_Y,
    THEME_CONTROL_HEIGHT, THEME_CONTROL_WIDTH, THEME_CONTROL_Y, dark_theme_rect, light_theme_rect,
    scrollbar_off_rect, scrollbar_on_rect,
};
use super::navigation;
use super::panel_scrollbars;
use super::preview;
use super::render::CANVAS_HEIGHT;
use super::render_context::ShellContext;
use super::text::TextVerticalBox;

const BRAND_TITLE_Y: usize = 20;
const BRAND_THEME_Y: usize = 46;
const BRAND_TITLE_SIZE: f32 = 18.0;
const BRAND_META_SIZE: f32 = 13.0;
const CONTROL_TEXT_X_PADDING: usize = 14;
const THEME_CONTROL_TEXT_SIZE: f32 = 12.0;
const SCROLLBAR_CONTROL_TEXT_SIZE: f32 = 11.0;

pub(super) fn draw(canvas: &mut Canvas, context: ShellContext<'_>) {
    let palette = context.render.palette;
    canvas.fill_rect(0, 0, NAV_WIDTH, CANVAS_HEIGHT, palette.surface);
    canvas.stroke_rect(0, 0, NAV_WIDTH, CANVAS_HEIGHT, palette.border);
    context.render.text.draw(
        canvas,
        "katana-ui-core",
        BRAND_X,
        BRAND_TITLE_Y,
        BRAND_TITLE_SIZE,
        palette.text,
    );
    context.render.text.draw(
        canvas,
        &format!("panel theme: {}", context.root.props().theme_id),
        BRAND_X,
        BRAND_THEME_Y,
        BRAND_META_SIZE,
        palette.muted,
    );
    draw_theme_control(canvas, &context);
    draw_scrollbar_control(canvas, &context);
    navigation::draw(
        canvas,
        context.render.text,
        palette,
        context.scenario.selected_page,
        context.scenario.tree_expansion,
        context.scenario.panel_scroll.navigation_y,
        context.scenario.show_navigation_lines,
    );
    preview::draw(canvas, context.root, context.render, context.scenario);
    inspector::draw(
        canvas,
        context.render,
        selected_story(context.root, context.render, context.scenario),
        context.scenario,
    );
    panel_scrollbars::draw(canvas, palette, context.scenario);
    preview::draw_overlay(canvas, context.render, context.scenario);
}

fn draw_theme_control(canvas: &mut Canvas, context: &ShellContext<'_>) {
    let selected = context.root.props().theme_id.as_str();
    draw_theme_option(canvas, context, "light", selected, light_theme_rect().x);
    draw_theme_option(canvas, context, "dark", selected, dark_theme_rect().x);
}

fn draw_scrollbar_control(canvas: &mut Canvas, context: &ShellContext<'_>) {
    draw_scrollbar_option(
        canvas,
        context,
        "scroll on",
        context.scenario.scrollbar_visible,
        scrollbar_on_rect().x,
    );
    draw_scrollbar_option(
        canvas,
        context,
        "off",
        !context.scenario.scrollbar_visible,
        scrollbar_off_rect().x,
    );
}

fn draw_scrollbar_option(
    canvas: &mut Canvas,
    context: &ShellContext<'_>,
    label: &str,
    active: bool,
    x: usize,
) {
    let palette = context.render.palette;
    let fill = if active {
        palette.accent
    } else {
        palette.panel
    };
    let text_color = if active {
        palette.background
    } else {
        palette.text
    };
    canvas.fill_rect(
        x,
        SCROLLBAR_CONTROL_Y,
        SCROLLBAR_CONTROL_WIDTH,
        SCROLLBAR_CONTROL_HEIGHT,
        fill,
    );
    canvas.stroke_rect(
        x,
        SCROLLBAR_CONTROL_Y,
        SCROLLBAR_CONTROL_WIDTH,
        SCROLLBAR_CONTROL_HEIGHT,
        palette.border,
    );
    context.render.text.draw_centered(
        canvas,
        label,
        x + CONTROL_TEXT_X_PADDING,
        TextVerticalBox::new(SCROLLBAR_CONTROL_Y, SCROLLBAR_CONTROL_HEIGHT as f32),
        SCROLLBAR_CONTROL_TEXT_SIZE,
        text_color,
    );
}

fn draw_theme_option(
    canvas: &mut Canvas,
    context: &ShellContext<'_>,
    label: &str,
    selected: &str,
    x: usize,
) {
    let active = selected == label;
    let palette = context.render.palette;
    let fill = if active {
        palette.accent
    } else {
        palette.panel
    };
    let text_color = if active {
        palette.background
    } else {
        palette.text
    };
    canvas.fill_rect(
        x,
        THEME_CONTROL_Y,
        THEME_CONTROL_WIDTH,
        THEME_CONTROL_HEIGHT,
        fill,
    );
    canvas.stroke_rect(
        x,
        THEME_CONTROL_Y,
        THEME_CONTROL_WIDTH,
        THEME_CONTROL_HEIGHT,
        palette.border,
    );
    context.render.text.draw_centered(
        canvas,
        label,
        x + CONTROL_TEXT_X_PADDING,
        TextVerticalBox::new(THEME_CONTROL_Y, THEME_CONTROL_HEIGHT as f32),
        THEME_CONTROL_TEXT_SIZE,
        text_color,
    );
}

fn selected_story<'a>(
    _root: &'a katana_ui_core::render_model::UiNode,
    render: super::render_context::RenderContext<'a>,
    scenario: super::render_context::ScenarioContext<'_>,
) -> Option<(
    &'a katana_ui_core::render_model::UiNode,
    &'a crate::catalog::StoryExample,
)> {
    render
        .examples
        .iter()
        .find(|example| example.page == scenario.selected_page)
        .map(|example| (example.tree.root(), example))
}
