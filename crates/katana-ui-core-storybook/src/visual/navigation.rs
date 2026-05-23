use super::canvas::Canvas;
use super::layout_metrics::{
    NAV_FIRST_ROW_Y, NAV_ROW_HEIGHT, NAV_ROW_STEP, NAV_ROW_WIDTH, NAV_ROW_X,
    navigation_menu_panel_rect,
};
use super::navigation_icons::draw_disclosure;
use super::navigation_tree::{NavigationRow, TreeExpansionState, visible_rows};
use super::palette::VisualPalette;
use super::panel_layout;
use super::panel_scroll_state::PanelScrollRegion;
use super::text::{TextRenderer, TextVerticalBox};
use crate::catalog::story_map::{StoryGroup, StorySection};

const SECTION_LINE_X: usize = 68;
const PAGE_LINE_X: usize = 84;
const GROUP_LINE_X: usize = 54;
const GROUP_TEXT_X: usize = 62;
const SECTION_TEXT_X: usize = 78;
const PAGE_TEXT_X: usize = 98;
const DISCLOSURE_SIZE: usize = 7;
const CONNECTOR_LABEL_GAP: usize = 1;
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
    scroll_y: usize,
    show_navigation_lines: bool,
    show_navigation_text_connectors: bool,
) {
    let rows = visible_rows(expansion);
    draw_navigation_panel(canvas, palette);
    let viewport = panel_layout::region_layout(PanelScrollRegion::Navigation).content_viewport;
    canvas.with_clip(
        viewport.x,
        viewport.y,
        viewport.width,
        viewport.height,
        |canvas| {
            let first_index = scroll_y / NAV_ROW_STEP;
            let row_offset = scroll_y % NAV_ROW_STEP;
            let mut row_y = NAV_FIRST_ROW_Y.saturating_sub(row_offset);
            for (row_index, row) in rows.iter().enumerate().skip(first_index) {
                match row {
                    NavigationRow::Group(group) => {
                        draw_group(
                            canvas,
                            text,
                            palette,
                            *group,
                            expansion.is_open(*group),
                            show_navigation_lines,
                            show_navigation_text_connectors,
                            &rows,
                            row_index,
                            row_y,
                        );
                    }
                    NavigationRow::Section { group, section } => {
                        draw_section(
                            canvas,
                            text,
                            palette,
                            *section,
                            expansion.is_section_open(*group, *section),
                            show_navigation_lines,
                            show_navigation_text_connectors,
                            &rows,
                            row_index,
                            row_y,
                        );
                    }
                    NavigationRow::Page { page, .. } => {
                        draw_page(
                            canvas,
                            text,
                            palette,
                            page,
                            *page == selected_page,
                            show_navigation_lines,
                            show_navigation_text_connectors,
                            &rows,
                            row_index,
                            row_y,
                            PageDepth::Section,
                        );
                    }
                    NavigationRow::PageWithoutSection { page, .. } => {
                        draw_page(
                            canvas,
                            text,
                            palette,
                            page,
                            *page == selected_page,
                            show_navigation_lines,
                            show_navigation_text_connectors,
                            &rows,
                            row_index,
                            row_y,
                            PageDepth::Sectionless,
                        );
                    }
                }
                row_y += NAV_ROW_STEP;
            }
        },
    );
}

fn draw_navigation_panel(canvas: &mut Canvas, palette: &VisualPalette) {
    let panel = navigation_menu_panel_rect();
    canvas.fill_rect(panel.x, panel.y, panel.width, panel.height, palette.panel);
    canvas.stroke_rect(panel.x, panel.y, panel.width, panel.height, palette.border);
}

fn draw_group(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    group: StoryGroup,
    open: bool,
    show_lines: bool,
    show_text_connectors: bool,
    rows: &[NavigationRow],
    row_index: usize,
    y: usize,
) {
    canvas.fill_rect(
        NAV_ROW_X,
        y,
        NAV_ROW_WIDTH,
        NAV_ROW_HEIGHT,
        palette.code_background,
    );
    draw_disclosure(
        canvas,
        palette,
        disclosure_x(NavigationDepth::Group),
        open,
        y,
    );
    if show_lines {
        draw_row_guides(
            canvas,
            palette,
            NavigationDepth::Group,
            show_text_connectors,
            rows,
            row_index,
            y,
        );
    }
    text.draw_centered(
        canvas,
        group.label(),
        GROUP_TEXT_X,
        TextVerticalBox::new(y, NAV_ROW_HEIGHT as f32),
        NAV_GROUP_TEXT_SIZE,
        palette.muted,
    );
}

fn draw_section(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    section: StorySection,
    open: bool,
    show_lines: bool,
    show_text_connectors: bool,
    rows: &[NavigationRow],
    row_index: usize,
    y: usize,
) {
    canvas.fill_rect(
        NAV_ROW_X,
        y,
        NAV_ROW_WIDTH,
        NAV_ROW_HEIGHT,
        palette.code_background,
    );
    draw_disclosure(
        canvas,
        palette,
        disclosure_x(NavigationDepth::Section),
        open,
        y,
    );
    if show_lines {
        draw_row_guides(
            canvas,
            palette,
            NavigationDepth::Section,
            show_text_connectors,
            rows,
            row_index,
            y,
        );
    }
    text.draw_centered(
        canvas,
        section.label(),
        SECTION_TEXT_X,
        TextVerticalBox::new(y, NAV_ROW_HEIGHT as f32),
        NAV_GROUP_TEXT_SIZE,
        palette.muted,
    );
}

#[derive(Clone, Copy)]
enum NavigationDepth {
    Group,
    Section,
    Page,
}

fn draw_page(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    page: &str,
    selected: bool,
    show_lines: bool,
    show_text_connectors: bool,
    rows: &[NavigationRow],
    row_index: usize,
    y: usize,
    page_depth: PageDepth,
) {
    let fill = if selected {
        palette.selection
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
    if show_lines {
        let depth = match page_depth {
            PageDepth::Sectionless => NavigationDepth::Section,
            PageDepth::Section => NavigationDepth::Page,
        };
        draw_row_guides(
            canvas,
            palette,
            depth,
            show_text_connectors,
            rows,
            row_index,
            y,
        );
    }
    text.draw_centered(
        canvas,
        page,
        page_text_x(page_depth),
        TextVerticalBox::new(y, NAV_ROW_HEIGHT as f32),
        NAV_TEXT_SIZE,
        text_color,
    );
}

fn draw_row_guides(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    row_depth: NavigationDepth,
    show_text_connector: bool,
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

    for level in 0..=current_depth {
        let has_up = level_continues_up(level, current_depth, previous_depth);
        let has_down = level_continues_down(level, current_depth, next_depth);
        let start_y = if has_up { row_y } else { row_center_y };
        let end_y = if has_down {
            row_y + NAV_ROW_STEP
        } else {
            row_center_y + 1
        };
        if has_up || has_down {
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
                1,
                palette.border,
            );
        }
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

fn disclosure_x(depth: NavigationDepth) -> usize {
    match depth {
        NavigationDepth::Group => GROUP_LINE_X.saturating_sub(4),
        NavigationDepth::Section => SECTION_LINE_X.saturating_sub(4),
        NavigationDepth::Page => PAGE_LINE_X.saturating_sub(4),
    }
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

#[derive(Clone, Copy)]
enum PageDepth {
    Section,
    Sectionless,
}

fn page_text_x(depth: PageDepth) -> usize {
    match depth {
        PageDepth::Section => PAGE_TEXT_X,
        PageDepth::Sectionless => SECTION_TEXT_X,
    }
}

fn text_x(depth: NavigationDepth) -> usize {
    match depth {
        NavigationDepth::Group => GROUP_TEXT_X,
        NavigationDepth::Section => SECTION_TEXT_X,
        NavigationDepth::Page => PAGE_TEXT_X,
    }
}
