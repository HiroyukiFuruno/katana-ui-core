use super::canvas::Canvas;
use super::layout_metrics::NAV_ROW_HEIGHT;
use super::palette::VisualPalette;

const DISCLOSURE_X: usize = 22;
const GROUP_ICON_X: usize = 38;
const PAGE_ICON_X: usize = 54;
const NAV_ICON_SIZE: usize = 12;
const FOLDER_TAB_X_OFFSET: usize = 3;
const FOLDER_TAB_Y_OFFSET: usize = 2;
const FOLDER_TAB_WIDTH: usize = 7;
const FOLDER_TAB_HEIGHT: usize = 2;
const FILE_MARK_X_OFFSET: usize = 3;
const FILE_MARK_Y_OFFSET: usize = 3;
const FILE_MARK_WIDTH_INSET: usize = 6;
const DISCLOSURE_SIZE: usize = 7;

pub(super) fn draw_disclosure(canvas: &mut Canvas, palette: &VisualPalette, open: bool, y: usize) {
    let top = centered_icon_y(y, DISCLOSURE_SIZE);
    if open {
        draw_open_disclosure(canvas, palette, top);
        return;
    }
    draw_closed_disclosure(canvas, palette, top);
}

pub(super) fn draw_folder_icon(canvas: &mut Canvas, palette: &VisualPalette, y: usize) {
    let icon_y = centered_icon_y(y, NAV_ICON_SIZE);
    canvas.stroke_rect(
        GROUP_ICON_X,
        icon_y + FOLDER_TAB_Y_OFFSET,
        NAV_ICON_SIZE,
        NAV_ICON_SIZE,
        palette.text,
    );
    canvas.fill_rect(
        GROUP_ICON_X + FOLDER_TAB_X_OFFSET,
        icon_y,
        FOLDER_TAB_WIDTH,
        FOLDER_TAB_HEIGHT,
        palette.text,
    );
}

pub(super) fn draw_file_icon(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    y: usize,
    selected: bool,
) {
    let color = if selected {
        palette.accent
    } else {
        palette.border
    };
    let icon_y = centered_icon_y(y, NAV_ICON_SIZE);
    canvas.stroke_rect(PAGE_ICON_X, icon_y, NAV_ICON_SIZE - 2, NAV_ICON_SIZE, color);
    canvas.fill_rect(
        PAGE_ICON_X + FILE_MARK_X_OFFSET,
        icon_y + FILE_MARK_Y_OFFSET,
        NAV_ICON_SIZE - FILE_MARK_WIDTH_INSET,
        FOLDER_TAB_HEIGHT,
        color,
    );
}

fn draw_open_disclosure(canvas: &mut Canvas, palette: &VisualPalette, top: usize) {
    for offset in 0..DISCLOSURE_SIZE / 2 {
        canvas.set(DISCLOSURE_X + offset, top + offset, palette.text);
        canvas.set(
            DISCLOSURE_X + DISCLOSURE_SIZE - offset - 1,
            top + offset,
            palette.text,
        );
    }
    canvas.set(
        DISCLOSURE_X + DISCLOSURE_SIZE / 2,
        top + DISCLOSURE_SIZE / 2,
        palette.text,
    );
}

fn draw_closed_disclosure(canvas: &mut Canvas, palette: &VisualPalette, top: usize) {
    for offset in 0..DISCLOSURE_SIZE / 2 {
        canvas.set(DISCLOSURE_X + offset, top + offset, palette.text);
        canvas.set(
            DISCLOSURE_X + offset,
            top + DISCLOSURE_SIZE - offset - 1,
            palette.text,
        );
    }
    canvas.set(
        DISCLOSURE_X + DISCLOSURE_SIZE / 2,
        top + DISCLOSURE_SIZE / 2,
        palette.text,
    );
}

fn centered_icon_y(row_y: usize, icon_size: usize) -> usize {
    row_y + (NAV_ROW_HEIGHT - icon_size) / 2
}

#[cfg(test)]
mod tests {
    use super::{NAV_ICON_SIZE, centered_icon_y};
    use crate::visual::layout_metrics::{NAV_FIRST_ROW_Y, NAV_ROW_HEIGHT};

    #[test]
    fn tree_icons_are_vertically_centered_in_row_box() {
        let group_icon_y = centered_icon_y(NAV_FIRST_ROW_Y, NAV_ICON_SIZE);
        let page_icon_y = centered_icon_y(NAV_FIRST_ROW_Y, NAV_ICON_SIZE);
        let expected = NAV_FIRST_ROW_Y + (NAV_ROW_HEIGHT - NAV_ICON_SIZE) / 2;

        assert_eq!(expected, group_icon_y);
        assert_eq!(expected, page_icon_y);
    }
}
