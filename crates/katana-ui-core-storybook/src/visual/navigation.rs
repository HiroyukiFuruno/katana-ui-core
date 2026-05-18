use super::canvas::Canvas;
use super::layout_metrics::{
    NAV_FIRST_ROW_Y, NAV_ROW_HEIGHT, NAV_ROW_STEP, NAV_ROW_WIDTH, NAV_ROW_X,
};
use super::navigation_icons::{draw_disclosure, draw_file_icon, draw_folder_icon};
use super::navigation_tree::{NavigationGroup, NavigationRow, TreeExpansionState, visible_rows};
use super::palette::VisualPalette;
use super::text::{TextRenderer, TextVerticalBox};

const PAGE_LINE_X: usize = 44;
const GROUP_TEXT_X: usize = 62;
const PAGE_TEXT_X: usize = 78;
const NAV_TEXT_SIZE: f32 = 12.0;
const NAV_GROUP_TEXT_SIZE: f32 = 11.0;
const TREE_LINE_WIDTH: usize = 1;
const SELECTED_ACCENT_WIDTH: usize = 3;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    selected_page: &str,
    expansion: TreeExpansionState,
) {
    let mut row_y = NAV_FIRST_ROW_Y;
    for row in visible_rows(expansion) {
        match row {
            NavigationRow::Group(group) => {
                draw_group(
                    canvas,
                    text,
                    palette,
                    group,
                    expansion.is_open(group),
                    row_y,
                );
            }
            NavigationRow::Page { page, .. } => {
                draw_page(canvas, text, palette, page, page == selected_page, row_y);
            }
        }
        row_y += NAV_ROW_STEP;
    }
}

fn draw_group(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    group: NavigationGroup,
    open: bool,
    y: usize,
) {
    canvas.fill_rect(NAV_ROW_X, y, NAV_ROW_WIDTH, NAV_ROW_HEIGHT, palette.surface);
    draw_disclosure(canvas, palette, open, y);
    draw_folder_icon(canvas, palette, y);
    text.draw_centered(
        canvas,
        group.label(),
        GROUP_TEXT_X,
        TextVerticalBox::new(y, NAV_ROW_HEIGHT as f32),
        NAV_GROUP_TEXT_SIZE,
        palette.muted,
    );
}

fn draw_page(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    page: &str,
    selected: bool,
    y: usize,
) {
    let fill = if selected {
        palette.panel
    } else {
        palette.surface
    };
    let text_color = if selected {
        palette.text
    } else {
        palette.muted
    };
    canvas.fill_rect(NAV_ROW_X, y, NAV_ROW_WIDTH, NAV_ROW_HEIGHT, fill);
    if selected {
        canvas.fill_rect(
            NAV_ROW_X,
            y,
            SELECTED_ACCENT_WIDTH,
            NAV_ROW_HEIGHT,
            palette.accent,
        );
    }
    canvas.fill_rect(
        PAGE_LINE_X,
        y,
        TREE_LINE_WIDTH,
        NAV_ROW_HEIGHT + NAV_ROW_STEP / 2,
        palette.border,
    );
    draw_file_icon(canvas, palette, y, selected);
    text.draw_centered(
        canvas,
        page,
        PAGE_TEXT_X,
        TextVerticalBox::new(y, NAV_ROW_HEIGHT as f32),
        NAV_TEXT_SIZE,
        text_color,
    );
}
