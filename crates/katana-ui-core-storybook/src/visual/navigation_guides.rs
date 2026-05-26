use super::canvas::Canvas;
use super::layout_metrics::{NAV_ROW_HEIGHT, NAV_ROW_STEP};
use super::navigation_tree::NavigationRow;
use super::palette::VisualPalette;

const SECTION_LINE_X: usize = 68;
const PAGE_LINE_X: usize = 84;
const GROUP_LINE_X: usize = 54;
pub(super) const GROUP_TEXT_X: usize = 62;
pub(super) const SECTION_TEXT_X: usize = 78;
const PAGE_TEXT_X: usize = 98;
const DISCLOSURE_SIZE: usize = 7;
const CONNECTOR_LABEL_GAP: usize = 1;
const TREE_LINE_WIDTH: usize = 1;
const DISCLOSURE_LINE_INSET: usize = 4;

#[derive(Clone, Copy)]
pub(super) enum NavigationDepth {
    Group,
    Section,
    Page,
}

#[derive(Clone, Copy)]
pub(super) enum PageDepth {
    Section,
    Sectionless,
}

pub(super) fn draw_row_guides(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    row_depth: NavigationDepth,
    show_text_connector: bool,
    draw_horizontal_connector: bool,
    rows: &[NavigationRow],
    row_index: usize,
    row_y: usize,
) {
    let current_depth = navigation_depth(row_depth);
    let previous_depth = row_index
        .checked_sub(1)
        .and_then(|index| rows.get(index).map(depth_from_row));
    let next_depth = rows.get(row_index + 1).map(depth_from_row);
    let row_center_y = row_y + NAV_ROW_HEIGHT / 2;
    draw_vertical_guides(
        canvas,
        palette,
        current_depth,
        previous_depth,
        next_depth,
        row_y,
    );
    draw_horizontal_guide(
        canvas,
        palette,
        row_depth,
        show_text_connector,
        draw_horizontal_connector,
        current_depth,
        row_center_y,
    );
}

pub(super) fn disclosure_x(depth: NavigationDepth) -> usize {
    match depth {
        NavigationDepth::Group => GROUP_LINE_X.saturating_sub(DISCLOSURE_LINE_INSET),
        NavigationDepth::Section => SECTION_LINE_X.saturating_sub(DISCLOSURE_LINE_INSET),
        NavigationDepth::Page => PAGE_LINE_X.saturating_sub(DISCLOSURE_LINE_INSET),
    }
}

pub(super) fn page_text_x(depth: PageDepth) -> usize {
    match depth {
        PageDepth::Section => PAGE_TEXT_X,
        PageDepth::Sectionless => SECTION_TEXT_X,
    }
}

fn draw_vertical_guides(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    current_depth: usize,
    previous_depth: Option<usize>,
    next_depth: Option<usize>,
    row_y: usize,
) {
    let row_center_y = row_y + NAV_ROW_HEIGHT / 2;
    for level in 0..=current_depth {
        let has_up = level_continues_up(level, current_depth, previous_depth);
        let has_down = level_continues_down(level, current_depth, next_depth);
        let start_y = if has_up { row_y } else { row_center_y };
        let end_y = if has_down {
            row_y + NAV_ROW_STEP
        } else {
            row_center_y + 1
        };
        draw_vertical_guide(
            canvas,
            palette,
            level,
            current_depth,
            start_y,
            end_y,
            row_center_y,
            has_up || has_down,
        );
    }
}

fn draw_vertical_guide(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    level: usize,
    current_depth: usize,
    start_y: usize,
    end_y: usize,
    row_center_y: usize,
    connected: bool,
) {
    if connected {
        canvas.fill_rect(
            guide_x(level),
            start_y,
            TREE_LINE_WIDTH,
            end_y.saturating_sub(start_y),
            palette.border,
        );
    } else if level == current_depth {
        canvas.fill_rect(
            guide_x(level),
            row_center_y,
            TREE_LINE_WIDTH,
            TREE_LINE_WIDTH,
            palette.border,
        );
    }
}

fn draw_horizontal_guide(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    row_depth: NavigationDepth,
    show_text_connector: bool,
    draw_horizontal_connector: bool,
    current_depth: usize,
    row_center_y: usize,
) {
    if !draw_horizontal_connector {
        return;
    }
    let current_guide_x = guide_x(current_depth);
    let connector_target_x = guide_target_x(row_depth, show_text_connector);
    let connector_width = connector_target_x.saturating_sub(current_guide_x);
    if connector_width > 0 {
        canvas.fill_rect(
            current_guide_x,
            row_center_y,
            connector_width,
            TREE_LINE_WIDTH,
            palette.border,
        );
    }
}

fn guide_x(level: usize) -> usize {
    match level {
        0 => GROUP_LINE_X,
        1 => SECTION_LINE_X,
        _ => PAGE_LINE_X,
    }
}

fn level_continues_up(level: usize, current_depth: usize, previous_depth: Option<usize>) -> bool {
    if current_depth == 0 {
        return false;
    }
    previous_depth.is_some_and(|depth| depth >= level)
}

fn level_continues_down(level: usize, current_depth: usize, next_depth: Option<usize>) -> bool {
    if current_depth == 0 {
        return next_depth.is_some_and(|depth| depth > current_depth);
    }
    if level < current_depth {
        return next_depth.is_some_and(|depth| depth > level);
    }
    next_depth.is_some_and(|depth| depth >= level)
}

fn guide_target_x(depth: NavigationDepth, show_text_connector: bool) -> usize {
    if show_text_connector {
        return text_x(depth).saturating_sub(CONNECTOR_LABEL_GAP);
    }
    disclosure_x(depth) + DISCLOSURE_SIZE
}

fn navigation_depth(row: NavigationDepth) -> usize {
    match row {
        NavigationDepth::Group => 0,
        NavigationDepth::Section => 1,
        NavigationDepth::Page => 2,
    }
}

fn depth_from_row(row: &NavigationRow) -> usize {
    match row {
        NavigationRow::Group { .. } => 0,
        NavigationRow::Section { .. } => 1,
        NavigationRow::Page { .. } => 2,
        NavigationRow::PageWithoutSection { .. } => 1,
    }
}

fn text_x(depth: NavigationDepth) -> usize {
    match depth {
        NavigationDepth::Group => GROUP_TEXT_X,
        NavigationDepth::Section => SECTION_TEXT_X,
        NavigationDepth::Page => PAGE_TEXT_X,
    }
}
