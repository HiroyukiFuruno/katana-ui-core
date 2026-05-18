use super::canvas::Canvas;
use super::dedicated;
use super::layout_metrics::{STORY_CARD_HEIGHT, STORY_CARD_WIDTH};
use super::palette::VisualPalette;
use super::text::TextRenderer;
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::UiNode;

const STORY_TEXT_X: usize = 12;
const STORY_TITLE_Y: usize = 12;
const STORY_KIND_Y: usize = 36;
const STORY_PROPS_Y: usize = 58;
const STORY_LOG_Y: usize = 78;
const STORY_HINT_Y: usize = 92;
const STORY_TITLE_SIZE: f32 = 14.0;
const STORY_KIND_SIZE: f32 = 11.0;
const STORY_META_SIZE: f32 = 10.0;
const HEADER_HEIGHT: usize = 42;
const ACCENT_STRIP_WIDTH: usize = 4;
const CODE_FONT_ROLE: &str = "code";

pub(super) struct StoryCardContext<'a> {
    pub(super) text: &'a TextRenderer,
    pub(super) code_text: &'a TextRenderer,
    pub(super) style_sheet: &'a katana_ui_core::style::StyleSheet,
    pub(super) palette: &'a VisualPalette,
}

pub(super) struct StoryCardFrame {
    pub(super) x: usize,
    pub(super) y: usize,
}

pub(super) fn draw_story_card(
    canvas: &mut Canvas,
    context: &StoryCardContext<'_>,
    node: &UiNode,
    callback_logs: &[UiCallbackLog],
    frame: StoryCardFrame,
) {
    let resolved = context.style_sheet.resolve(node);
    let edge = if resolved.declarations().is_empty() {
        context.palette.border
    } else {
        context.palette.accent
    };
    canvas.fill_rect(
        frame.x,
        frame.y,
        STORY_CARD_WIDTH,
        STORY_CARD_HEIGHT,
        context.palette.panel,
    );
    canvas.fill_rect(
        frame.x,
        frame.y,
        STORY_CARD_WIDTH,
        HEADER_HEIGHT,
        context.palette.surface,
    );
    canvas.fill_rect(
        frame.x,
        frame.y,
        ACCENT_STRIP_WIDTH,
        STORY_CARD_HEIGHT,
        edge,
    );
    canvas.stroke_rect(frame.x, frame.y, STORY_CARD_WIDTH, STORY_CARD_HEIGHT, edge);
    renderer_for(node, context).draw(
        canvas,
        &node.props().label,
        frame.x + STORY_TEXT_X,
        frame.y + STORY_TITLE_Y,
        STORY_TITLE_SIZE,
        context.palette.text,
    );
    context.text.draw(
        canvas,
        &format!("{:?}", node.kind()),
        frame.x + STORY_TEXT_X,
        frame.y + STORY_KIND_Y,
        STORY_KIND_SIZE,
        context.palette.muted,
    );
    context.code_text.draw(
        canvas,
        &props_label(node),
        frame.x + STORY_TEXT_X,
        frame.y + STORY_PROPS_Y,
        STORY_META_SIZE,
        context.palette.muted,
    );
    draw_callback_log(canvas, context, callback_logs, frame.x, frame.y);
    draw_node_hint(
        canvas,
        context.text,
        node,
        context.palette,
        frame.x + STORY_TEXT_X,
        frame.y + STORY_HINT_Y,
    );
}

fn renderer_for<'a>(node: &UiNode, context: &'a StoryCardContext<'_>) -> &'a TextRenderer {
    if node.props().font_role == CODE_FONT_ROLE {
        return context.code_text;
    }
    context.text
}

fn props_label(node: &UiNode) -> String {
    format!(
        "{:?}/{:?}/{:?}/{:?}",
        node.props().visual_role,
        node.props().variant,
        node.props().tone,
        node.props().size
    )
}

fn draw_callback_log(
    canvas: &mut Canvas,
    context: &StoryCardContext<'_>,
    callback_logs: &[UiCallbackLog],
    x: usize,
    y: usize,
) {
    if let Some(log) = callback_logs.first() {
        context.code_text.draw(
            canvas,
            &format!("log: {}", log.action),
            x + STORY_TEXT_X,
            y + STORY_LOG_Y,
            STORY_META_SIZE,
            context.palette.accent,
        );
    }
}

fn draw_node_hint(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    dedicated::draw(canvas, text, node, palette, x, y);
}
