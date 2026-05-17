use super::canvas::Canvas;
use super::render::{ACCENT, BACKGROUND, BORDER, MUTED, PANEL, SURFACE, TEXT};
use super::text::TextRenderer;
use katana_ui_core::{atom, molecule};

pub(super) const MODAL_WIDTH: usize = 420;
pub(super) const MODAL_HEIGHT: usize = 260;
pub(super) const MODAL_BACKGROUND: u32 = BACKGROUND;

const PADDING: usize = 24;
const TITLE_Y: usize = 24;
const TITLE_SIZE: f32 = 20.0;
const META_Y: usize = 58;
const META_SIZE: f32 = 12.0;
const BODY_Y: usize = 96;
const BODY_HEIGHT: usize = 56;
const BODY_TEXT_X: usize = 16;
const BODY_TEXT_Y: usize = 18;
const BODY_TEXT_SIZE: f32 = 13.0;
const BUTTON_Y: usize = 176;
const BUTTON_WIDTH: usize = 140;
const BUTTON_HEIGHT: usize = 36;
const BUTTON_TEXT_X: usize = 18;
const BUTTON_TEXT_Y: usize = 11;
const BUTTON_TEXT_SIZE: f32 = 13.0;
const MIN_RENDERED_PIXELS: usize = 1000;

pub(super) fn render_modal_canvas() -> Canvas {
    let text = TextRenderer::load();
    let mut canvas = Canvas::new(MODAL_WIDTH, MODAL_HEIGHT, MODAL_BACKGROUND);
    canvas.fill_rect(0, 0, MODAL_WIDTH, MODAL_HEIGHT, SURFACE);
    canvas.stroke_rect(0, 0, MODAL_WIDTH, MODAL_HEIGHT, ACCENT);
    text.draw(
        &mut canvas,
        "Modal window",
        PADDING,
        TITLE_Y,
        TITLE_SIZE,
        TEXT,
    );
    text.draw(
        &mut canvas,
        "same display / frontmost",
        PADDING,
        META_Y,
        META_SIZE,
        MUTED,
    );
    canvas.fill_rect(
        PADDING,
        BODY_Y,
        MODAL_WIDTH - PADDING * 2,
        BODY_HEIGHT,
        PANEL,
    );
    canvas.stroke_rect(
        PADDING,
        BODY_Y,
        MODAL_WIDTH - PADDING * 2,
        BODY_HEIGHT,
        BORDER,
    );
    text.draw(
        &mut canvas,
        "state: open",
        PADDING + BODY_TEXT_X,
        BODY_Y + BODY_TEXT_Y,
        BODY_TEXT_SIZE,
        TEXT,
    );
    canvas.fill_rect(PADDING, BUTTON_Y, BUTTON_WIDTH, BUTTON_HEIGHT, ACCENT);
    text.draw(
        &mut canvas,
        "close",
        PADDING + BUTTON_TEXT_X,
        BUTTON_Y + BUTTON_TEXT_Y,
        BUTTON_TEXT_SIZE,
        BACKGROUND,
    );
    canvas
}

pub(super) fn state_reflected_after_operation() -> bool {
    let node: katana_ui_core::render_model::UiNode = molecule::Modal::new("Runtime modal")
        .open(true)
        .value("opened")
        .child(atom::Text::new("State applied"))
        .into();
    node.props().interaction.open
        && node.props().interaction.value == "opened"
        && node.children().len() == 1
}

pub(super) fn overlay_rendered() -> bool {
    render_modal_canvas().non_background_pixels(MODAL_BACKGROUND) > MIN_RENDERED_PIXELS
}
