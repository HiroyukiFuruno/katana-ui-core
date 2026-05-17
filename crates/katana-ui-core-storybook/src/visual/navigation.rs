use super::canvas::Canvas;
use super::palette::VisualPalette;
use super::text::TextRenderer;
use katana_ui_core::render_model::{UiNode, UiNodeKind};

const NAV_FIRST_ROW_Y: usize = 104;
const NAV_VISIBLE_ROWS: usize = 36;
const NAV_ROW_X: usize = 14;
const NAV_ROW_Y_OFFSET: usize = 5;
const NAV_ROW_WIDTH: usize = 248;
const NAV_ROW_HEIGHT: usize = 24;
const NAV_TEXT_X: usize = 24;
const NAV_TEXT_SIZE: f32 = 12.0;
const NAV_ROW_STEP: usize = 28;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    root: &UiNode,
    palette: &VisualPalette,
    selected_page: &str,
) {
    let Some(nav) = panel_child(root, "Navigation") else {
        return;
    };
    let mut y = NAV_FIRST_ROW_Y;
    for child in nav.children().iter().take(NAV_VISIBLE_ROWS) {
        draw_row(canvas, text, palette, selected_page, child, y);
        y += NAV_ROW_STEP;
    }
}

fn draw_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    selected_page: &str,
    child: &UiNode,
    y: usize,
) {
    let selected = child.props().label == selected_page;
    let fill = if selected {
        palette.accent
    } else {
        palette.panel
    };
    let color = if selected {
        palette.background
    } else {
        palette.text
    };
    canvas.fill_rect(
        NAV_ROW_X,
        y - NAV_ROW_Y_OFFSET,
        NAV_ROW_WIDTH,
        NAV_ROW_HEIGHT,
        fill,
    );
    text.draw(
        canvas,
        &child.props().label,
        NAV_TEXT_X,
        y,
        NAV_TEXT_SIZE,
        color,
    );
}

fn panel_child<'a>(root: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == label)
}
