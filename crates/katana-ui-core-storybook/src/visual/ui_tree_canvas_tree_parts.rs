use super::canvas::Canvas;
use katana_ui_core::molecule::TreeView;
use katana_ui_core::render_model::UiTreeNodeKind;

pub(super) const ROW_HEIGHT: usize = TreeView::row_height() as usize;
pub(super) const TEXT_SIZE: f32 = 13.0;
pub(super) const NODE_GAP: usize = 8;
pub(super) const INDENT: usize = TreeView::indent_width() as usize;

const DISCLOSURE_WIDTH: usize = TreeView::disclosure_width() as usize;
const DISCLOSURE_GAP: usize = TreeView::disclosure_gap() as usize;
const ICON_WIDTH: usize = TreeView::icon_width() as usize;
const ICON_GAP: usize = TreeView::icon_gap() as usize;
const ICON_TOP_OFFSET: usize = 3;
const LINE_OFFSET: usize = 6;
const DISCLOSURE_EXPANDED_BAR_X: usize = 3;
const DISCLOSURE_EXPANDED_BAR_Y: usize = 10;
const DISCLOSURE_EXPANDED_BAR_WIDTH: usize = 7;
const DISCLOSURE_EXPANDED_BAR_HEIGHT: usize = 2;
const DISCLOSURE_EXPANDED_STEM_X: usize = 5;
const DISCLOSURE_EXPANDED_STEM_Y: usize = 12;
const DISCLOSURE_EXPANDED_STEM_WIDTH: usize = 3;
const DISCLOSURE_COLLAPSED_STEM_Y: usize = 6;
const DISCLOSURE_COLLAPSED_STEM_HEIGHT: usize = 8;
const DISCLOSURE_COLLAPSED_TICK_X: usize = 7;
const DISCLOSURE_COLLAPSED_UPPER_TICK_Y: usize = 8;
const DISCLOSURE_COLLAPSED_LOWER_TICK_Y: usize = 10;
const FOLDER_BODY_X: usize = 1;
const FOLDER_BODY_Y: usize = 5;
const FOLDER_BODY_WIDTH: usize = 14;
const FOLDER_BODY_HEIGHT: usize = 10;
const FOLDER_TAB_X: usize = 3;
const FOLDER_TAB_Y: usize = 2;
const FOLDER_TAB_WIDTH: usize = 8;
const FOLDER_TAB_HEIGHT: usize = 5;
const FOLDER_OPEN_LIP_X: usize = 2;
const FOLDER_OPEN_LIP_Y: usize = 13;
const FOLDER_OPEN_LIP_WIDTH: usize = 12;
const FOLDER_OPEN_LIP_HEIGHT: usize = 2;
const FILE_BODY_X: usize = 3;
const FILE_BODY_Y: usize = 1;
const FILE_BODY_WIDTH: usize = 10;
const FILE_BODY_HEIGHT: usize = 14;
const FILE_CORNER_X: usize = 10;
const FILE_CORNER_Y: usize = 2;
const FILE_CORNER_SIZE: usize = 3;
const MARKDOWN_LEFT_STEM_X: usize = 5;
const MARKDOWN_RIGHT_STEM_X: usize = 8;
const MARKDOWN_STEM_Y: usize = 6;
const MARKDOWN_STEM_WIDTH: usize = 2;
const MARKDOWN_STEM_HEIGHT: usize = 6;
const MARKDOWN_BRIDGE_X: usize = 6;
const MARKDOWN_BRIDGE_Y: usize = 7;
const MARKDOWN_BRIDGE_WIDTH: usize = 3;
const MARKDOWN_BRIDGE_HEIGHT: usize = 2;
const IMAGE_FRAME_X: usize = 2;
const IMAGE_FRAME_Y: usize = 2;
const IMAGE_FRAME_SIZE: usize = 13;
const IMAGE_DOT_X: usize = 5;
const IMAGE_DOT_Y: usize = 5;
const IMAGE_DOT_SIZE: usize = 2;
const IMAGE_BASE_X: usize = 4;
const IMAGE_BASE_Y: usize = 11;
const IMAGE_BASE_WIDTH: usize = 10;
const IMAGE_BASE_HEIGHT: usize = 2;
const IMAGE_PEAK_X: usize = 7;
const IMAGE_PEAK_Y: usize = 9;
const IMAGE_PEAK_WIDTH: usize = 4;
const IMAGE_PEAK_HEIGHT: usize = 2;

pub(super) fn row_background_width(area_width: usize, area_x: usize, x: usize) -> usize {
    area_width.saturating_sub(x.saturating_sub(area_x))
}

pub(super) fn content_x(x: usize, depth: usize) -> usize {
    x.saturating_add(depth.saturating_mul(INDENT))
}

pub(super) fn line_x(x: usize, depth: usize) -> usize {
    x.saturating_add(depth.saturating_mul(INDENT))
        .saturating_add(LINE_OFFSET)
}

pub(super) fn label_x(x: usize, depth: usize, icons_visible: bool) -> usize {
    let start = content_x(x, depth);
    if !icons_visible {
        return start;
    }
    start
        .saturating_add(DISCLOSURE_WIDTH)
        .saturating_add(DISCLOSURE_GAP)
        .saturating_add(ICON_WIDTH)
        .saturating_add(ICON_GAP)
}

pub(super) fn draw_affordance(
    canvas: &mut Canvas,
    kind: UiTreeNodeKind,
    expanded: bool,
    directory_icon: &str,
    file_icon: &str,
    x: usize,
    y: usize,
    color: u32,
) {
    match kind {
        UiTreeNodeKind::Directory => {
            draw_directory_affordance(canvas, expanded, directory_icon, x, y, color);
        }
        UiTreeNodeKind::File => draw_file_affordance(canvas, file_icon, x, y, color),
    }
}

fn draw_directory_affordance(
    canvas: &mut Canvas,
    expanded: bool,
    directory_icon: &str,
    x: usize,
    y: usize,
    color: u32,
) {
    draw_disclosure(canvas, expanded, x, y, color);
    draw_directory_icon(
        canvas,
        x.saturating_add(DISCLOSURE_WIDTH)
            .saturating_add(DISCLOSURE_GAP),
        y,
        expanded || directory_icon.contains("open"),
        color,
    );
}

fn draw_file_affordance(canvas: &mut Canvas, icon: &str, x: usize, y: usize, color: u32) {
    let icon_x = x
        .saturating_add(DISCLOSURE_WIDTH)
        .saturating_add(DISCLOSURE_GAP);
    if icon.contains("markdown") || icon == "md" {
        draw_markdown_icon(canvas, icon_x, y, color);
        return;
    }
    if icon.contains("image") {
        draw_image_icon(canvas, icon_x, y, color);
        return;
    }
    draw_file_icon(canvas, icon_x, y, color);
}

fn draw_disclosure(canvas: &mut Canvas, expanded: bool, x: usize, y: usize, color: u32) {
    if expanded {
        canvas.fill_rect(
            x.saturating_add(DISCLOSURE_EXPANDED_BAR_X),
            y.saturating_add(DISCLOSURE_EXPANDED_BAR_Y),
            DISCLOSURE_EXPANDED_BAR_WIDTH,
            DISCLOSURE_EXPANDED_BAR_HEIGHT,
            color,
        );
        canvas.fill_rect(
            x.saturating_add(DISCLOSURE_EXPANDED_STEM_X),
            y.saturating_add(DISCLOSURE_EXPANDED_STEM_Y),
            DISCLOSURE_EXPANDED_STEM_WIDTH,
            DISCLOSURE_EXPANDED_BAR_HEIGHT,
            color,
        );
        return;
    }
    canvas.fill_rect(
        x.saturating_add(DISCLOSURE_EXPANDED_STEM_X),
        y.saturating_add(DISCLOSURE_COLLAPSED_STEM_Y),
        DISCLOSURE_EXPANDED_BAR_HEIGHT,
        DISCLOSURE_COLLAPSED_STEM_HEIGHT,
        color,
    );
    canvas.fill_rect(
        x.saturating_add(DISCLOSURE_COLLAPSED_TICK_X),
        y.saturating_add(DISCLOSURE_COLLAPSED_UPPER_TICK_Y),
        DISCLOSURE_EXPANDED_STEM_WIDTH,
        DISCLOSURE_EXPANDED_BAR_HEIGHT,
        color,
    );
    canvas.fill_rect(
        x.saturating_add(DISCLOSURE_COLLAPSED_TICK_X),
        y.saturating_add(DISCLOSURE_COLLAPSED_LOWER_TICK_Y),
        DISCLOSURE_EXPANDED_STEM_WIDTH,
        DISCLOSURE_EXPANDED_BAR_HEIGHT,
        color,
    );
}

fn draw_directory_icon(canvas: &mut Canvas, x: usize, y: usize, open: bool, color: u32) {
    let icon_y = y.saturating_add(ICON_TOP_OFFSET);
    canvas.stroke_rect(
        x.saturating_add(FOLDER_BODY_X),
        icon_y.saturating_add(FOLDER_BODY_Y),
        FOLDER_BODY_WIDTH,
        FOLDER_BODY_HEIGHT,
        color,
    );
    canvas.stroke_rect(
        x.saturating_add(FOLDER_TAB_X),
        icon_y.saturating_add(FOLDER_TAB_Y),
        FOLDER_TAB_WIDTH,
        FOLDER_TAB_HEIGHT,
        color,
    );
    if open {
        canvas.fill_rect(
            x.saturating_add(FOLDER_OPEN_LIP_X),
            icon_y.saturating_add(FOLDER_OPEN_LIP_Y),
            FOLDER_OPEN_LIP_WIDTH,
            FOLDER_OPEN_LIP_HEIGHT,
            color,
        );
    }
}

fn draw_file_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    let icon_y = y.saturating_add(ICON_TOP_OFFSET);
    canvas.stroke_rect(
        x.saturating_add(FILE_BODY_X),
        icon_y.saturating_add(FILE_BODY_Y),
        FILE_BODY_WIDTH,
        FILE_BODY_HEIGHT,
        color,
    );
    canvas.fill_rect(
        x.saturating_add(FILE_CORNER_X),
        icon_y.saturating_add(FILE_CORNER_Y),
        FILE_CORNER_SIZE,
        FILE_CORNER_SIZE,
        color,
    );
}

fn draw_markdown_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    draw_file_icon(canvas, x, y, color);
    let icon_y = y.saturating_add(ICON_TOP_OFFSET);
    canvas.fill_rect(
        x.saturating_add(MARKDOWN_LEFT_STEM_X),
        icon_y.saturating_add(MARKDOWN_STEM_Y),
        MARKDOWN_STEM_WIDTH,
        MARKDOWN_STEM_HEIGHT,
        color,
    );
    canvas.fill_rect(
        x.saturating_add(MARKDOWN_RIGHT_STEM_X),
        icon_y.saturating_add(MARKDOWN_STEM_Y),
        MARKDOWN_STEM_WIDTH,
        MARKDOWN_STEM_HEIGHT,
        color,
    );
    canvas.fill_rect(
        x.saturating_add(MARKDOWN_BRIDGE_X),
        icon_y.saturating_add(MARKDOWN_BRIDGE_Y),
        MARKDOWN_BRIDGE_WIDTH,
        MARKDOWN_BRIDGE_HEIGHT,
        color,
    );
}

fn draw_image_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    let icon_y = y.saturating_add(ICON_TOP_OFFSET);
    canvas.stroke_rect(
        x.saturating_add(IMAGE_FRAME_X),
        icon_y.saturating_add(IMAGE_FRAME_Y),
        IMAGE_FRAME_SIZE,
        IMAGE_FRAME_SIZE,
        color,
    );
    canvas.fill_rect(
        x.saturating_add(IMAGE_DOT_X),
        icon_y.saturating_add(IMAGE_DOT_Y),
        IMAGE_DOT_SIZE,
        IMAGE_DOT_SIZE,
        color,
    );
    canvas.fill_rect(
        x.saturating_add(IMAGE_BASE_X),
        icon_y.saturating_add(IMAGE_BASE_Y),
        IMAGE_BASE_WIDTH,
        IMAGE_BASE_HEIGHT,
        color,
    );
    canvas.fill_rect(
        x.saturating_add(IMAGE_PEAK_X),
        icon_y.saturating_add(IMAGE_PEAK_Y),
        IMAGE_PEAK_WIDTH,
        IMAGE_PEAK_HEIGHT,
        color,
    );
}
