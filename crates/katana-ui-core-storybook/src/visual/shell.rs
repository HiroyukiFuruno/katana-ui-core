use super::canvas::Canvas;
use super::navigation;
use super::preview;
use super::render::HEIGHT;
use super::render_context::ShellContext;

const NAV_WIDTH: usize = 280;
const BRAND_X: usize = 22;
const BRAND_TITLE_Y: usize = 20;
const BRAND_THEME_Y: usize = 46;
const BRAND_TITLE_SIZE: f32 = 18.0;
const BRAND_META_SIZE: f32 = 13.0;
const THEME_CONTROL_Y: usize = 64;
const THEME_CONTROL_WIDTH: usize = 86;
const THEME_CONTROL_HEIGHT: usize = 24;
const THEME_CONTROL_GAP: usize = 8;
const THEME_CONTROL_TEXT_Y: usize = 6;
const THEME_CONTROL_TEXT_SIZE: f32 = 12.0;

pub(super) fn draw(canvas: &mut Canvas, context: ShellContext<'_>) {
    let palette = context.render.palette;
    canvas.fill_rect(0, 0, NAV_WIDTH, HEIGHT, palette.surface);
    canvas.stroke_rect(0, 0, NAV_WIDTH, HEIGHT, palette.border);
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
    navigation::draw(
        canvas,
        context.render.text,
        context.root,
        palette,
        context.scenario.selected_page,
    );
    preview::draw(canvas, context.root, context.render, context.scenario);
}

fn draw_theme_control(canvas: &mut Canvas, context: &ShellContext<'_>) {
    let selected = context.root.props().theme_id.as_str();
    draw_theme_option(canvas, context, "light", selected, BRAND_X);
    draw_theme_option(
        canvas,
        context,
        "dark",
        selected,
        BRAND_X + THEME_CONTROL_WIDTH + THEME_CONTROL_GAP,
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
    context.render.text.draw(
        canvas,
        label,
        x + THEME_CONTROL_GAP,
        THEME_CONTROL_Y + THEME_CONTROL_TEXT_Y,
        THEME_CONTROL_TEXT_SIZE,
        text_color,
    );
}
