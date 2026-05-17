use super::canvas::Canvas;
use super::render::{ACCENT, BACKGROUND, BORDER, MUTED, PANEL, SURFACE, TEXT};
use super::text::TextRenderer;
use katana_ui_core::render_model::{UiNode, UiNodeKind};

const STORY_CARD_WIDTH: usize = 206;
const STORY_CARD_HEIGHT: usize = 122;
const STORY_TEXT_X: usize = 12;
const STORY_TITLE_Y: usize = 12;
const STORY_KIND_Y: usize = 36;
const STORY_HINT_Y: usize = 66;
const STORY_TITLE_SIZE: f32 = 14.0;
const STORY_KIND_SIZE: f32 = 11.0;
const BUTTON_HINT_WIDTH: usize = 82;
const INPUT_HINT_WIDTH: usize = 130;
const DEFAULT_HINT_WIDTH: usize = 58;
const HINT_HEIGHT: usize = 28;
const HINT_TEXT_X: usize = 12;
const INPUT_HINT_TEXT_X: usize = 10;
const HINT_TEXT_Y: usize = 7;
const HINT_TEXT_SIZE: f32 = 11.0;

pub(super) fn draw_story_card(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    style_sheet: &katana_ui_core::style::StyleSheet,
    x: usize,
    y: usize,
) {
    let resolved = style_sheet.resolve(node);
    let edge = if resolved.declarations().is_empty() {
        BORDER
    } else {
        ACCENT
    };
    canvas.fill_rect(x, y, STORY_CARD_WIDTH, STORY_CARD_HEIGHT, PANEL);
    canvas.stroke_rect(x, y, STORY_CARD_WIDTH, STORY_CARD_HEIGHT, edge);
    text.draw(
        canvas,
        &node.props().label,
        x + STORY_TEXT_X,
        y + STORY_TITLE_Y,
        STORY_TITLE_SIZE,
        TEXT,
    );
    text.draw(
        canvas,
        &format!("{:?}", node.kind()),
        x + STORY_TEXT_X,
        y + STORY_KIND_Y,
        STORY_KIND_SIZE,
        MUTED,
    );
    draw_node_hint(canvas, text, node, x + STORY_TEXT_X, y + STORY_HINT_Y);
}

fn draw_node_hint(canvas: &mut Canvas, text: &TextRenderer, node: &UiNode, x: usize, y: usize) {
    match node.kind() {
        UiNodeKind::Button | UiNodeKind::TextButton | UiNodeKind::IconTextButton => {
            canvas.fill_rect(x, y, BUTTON_HINT_WIDTH, HINT_HEIGHT, ACCENT);
            text.draw(
                canvas,
                "button",
                x + HINT_TEXT_X,
                y + HINT_TEXT_Y,
                HINT_TEXT_SIZE,
                BACKGROUND,
            );
        }
        UiNodeKind::Input | UiNodeKind::SearchBox | UiNodeKind::ComboBox => {
            canvas.stroke_rect(x, y, INPUT_HINT_WIDTH, HINT_HEIGHT, BORDER);
            text.draw(
                canvas,
                "input value",
                x + INPUT_HINT_TEXT_X,
                y + HINT_TEXT_Y,
                HINT_TEXT_SIZE,
                MUTED,
            );
        }
        _ => {
            canvas.fill_rect(x, y, DEFAULT_HINT_WIDTH, HINT_HEIGHT, SURFACE);
            text.draw(
                canvas,
                "node",
                x + HINT_TEXT_X,
                y + HINT_TEXT_Y,
                HINT_TEXT_SIZE,
                TEXT,
            );
        }
    }
}
