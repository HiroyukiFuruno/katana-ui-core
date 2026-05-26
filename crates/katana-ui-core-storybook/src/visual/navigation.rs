use super::canvas::Canvas;
use super::layout_metrics::{
    NAV_FIRST_ROW_Y, NAV_ROW_HEIGHT, NAV_ROW_STEP, NAV_ROW_WIDTH, NAV_ROW_X,
    navigation_menu_panel_rect,
};
use super::navigation_guides::{
    GROUP_TEXT_X, NavigationDepth, PageDepth, SECTION_TEXT_X, disclosure_x, draw_row_guides,
    page_text_x,
};
use super::navigation_icons::draw_disclosure;
use super::navigation_tree::{NavigationRow, TreeExpansionState, visible_rows};
use super::palette::VisualPalette;
use super::panel_layout;
use super::panel_scroll_state::PanelScrollRegion;
use super::text::{TextRenderer, TextVerticalBox};
use crate::catalog::story_map::{StoryGroup, StorySection};

const NAV_TEXT_SIZE: f32 = 12.0;
const NAV_GROUP_TEXT_SIZE: f32 = 11.0;
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
            true,
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
            true,
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
            false,
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
