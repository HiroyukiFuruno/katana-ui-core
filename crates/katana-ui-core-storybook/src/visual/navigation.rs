use super::canvas::Canvas;
use super::layout_metrics::{
    NAV_FIRST_ROW_Y, NAV_ROW_HEIGHT, NAV_ROW_STEP, NAV_ROW_WIDTH, NAV_ROW_X,
    navigation_menu_panel_rect,
};
use super::navigation_guides::{
    GROUP_TEXT_X, NavigationDepth, NavigationGuideContext, PageDepth, SECTION_TEXT_X, disclosure_x,
    draw_row_guides, page_text_x,
};
use super::navigation_icons::draw_disclosure;
pub(super) use super::navigation_render_types::NavigationRenderOptions;
use super::navigation_render_types::{
    NavigationBranchContext, NavigationGuideOptions, NavigationPageContext, NavigationRowContext,
};
use super::navigation_tree::{NavigationRow, visible_rows};
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
    options: NavigationRenderOptions<'_>,
) {
    let guides = NavigationGuideOptions {
        show_lines: options.show_lines,
        show_text_connectors: options.show_text_connectors,
    };
    let rows = visible_rows(options.expansion);
    draw_navigation_panel(canvas, palette);
    let viewport = panel_layout::region_layout(PanelScrollRegion::Navigation).content_viewport;
    canvas.with_clip(
        viewport.x,
        viewport.y,
        viewport.width,
        viewport.height,
        |canvas| {
            let first_index = options.scroll_y / NAV_ROW_STEP;
            let row_offset = options.scroll_y % NAV_ROW_STEP;
            let mut row_y = NAV_FIRST_ROW_Y.saturating_sub(row_offset);
            for (row_index, row) in rows.iter().enumerate().skip(first_index) {
                let row_context = NavigationRowContext {
                    rows: &rows,
                    row_index,
                    y: row_y,
                };
                match row {
                    NavigationRow::Group(group) => {
                        draw_group(
                            canvas,
                            text,
                            palette,
                            *group,
                            NavigationBranchContext {
                                open: options.expansion.is_open(*group),
                                row: row_context,
                                guides,
                            },
                        );
                    }
                    NavigationRow::Section { group, section } => {
                        draw_section(
                            canvas,
                            text,
                            palette,
                            *section,
                            NavigationBranchContext {
                                open: options.expansion.is_section_open(*group, *section),
                                row: row_context,
                                guides,
                            },
                        );
                    }
                    NavigationRow::Page { page, .. } => {
                        draw_page(
                            canvas,
                            text,
                            palette,
                            page,
                            NavigationPageContext {
                                selected: *page == options.selected_page,
                                depth: PageDepth::Section,
                                row: row_context,
                                guides,
                            },
                        );
                    }
                    NavigationRow::PageWithoutSection { page, .. } => {
                        draw_page(
                            canvas,
                            text,
                            palette,
                            page,
                            NavigationPageContext {
                                selected: *page == options.selected_page,
                                depth: PageDepth::Sectionless,
                                row: row_context,
                                guides,
                            },
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
    context: NavigationBranchContext<'_>,
) {
    canvas.fill_rect(
        NAV_ROW_X,
        context.row.y,
        NAV_ROW_WIDTH,
        NAV_ROW_HEIGHT,
        palette.code_background,
    );
    draw_disclosure(
        canvas,
        palette,
        disclosure_x(NavigationDepth::Group),
        context.open,
        context.row.y,
    );
    if context.guides.show_lines {
        draw_row_guides(
            canvas,
            palette,
            NavigationGuideContext {
                row_depth: NavigationDepth::Group,
                show_text_connector: context.guides.show_text_connectors,
                draw_horizontal_connector: true,
                rows: context.row.rows,
                row_index: context.row.row_index,
                row_y: context.row.y,
            },
        );
    }
    text.draw_centered(
        canvas,
        group.label(),
        GROUP_TEXT_X,
        TextVerticalBox::new(context.row.y, NAV_ROW_HEIGHT as f32),
        NAV_GROUP_TEXT_SIZE,
        palette.muted,
    );
}

fn draw_section(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    section: StorySection,
    context: NavigationBranchContext<'_>,
) {
    canvas.fill_rect(
        NAV_ROW_X,
        context.row.y,
        NAV_ROW_WIDTH,
        NAV_ROW_HEIGHT,
        palette.code_background,
    );
    draw_disclosure(
        canvas,
        palette,
        disclosure_x(NavigationDepth::Section),
        context.open,
        context.row.y,
    );
    if context.guides.show_lines {
        draw_row_guides(
            canvas,
            palette,
            NavigationGuideContext {
                row_depth: NavigationDepth::Section,
                show_text_connector: context.guides.show_text_connectors,
                draw_horizontal_connector: true,
                rows: context.row.rows,
                row_index: context.row.row_index,
                row_y: context.row.y,
            },
        );
    }
    text.draw_centered(
        canvas,
        section.label(),
        SECTION_TEXT_X,
        TextVerticalBox::new(context.row.y, NAV_ROW_HEIGHT as f32),
        NAV_GROUP_TEXT_SIZE,
        palette.muted,
    );
}

fn draw_page(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    page: &str,
    context: NavigationPageContext<'_>,
) {
    let fill = if context.selected {
        palette.selection
    } else {
        palette.surface
    };
    let text_color = if context.selected {
        palette.text
    } else {
        palette.muted
    };
    canvas.fill_rect(
        NAV_ROW_X,
        context.row.y,
        NAV_ROW_WIDTH,
        NAV_ROW_HEIGHT,
        fill,
    );
    if context.selected {
        canvas.fill_rect(
            NAV_ROW_X,
            context.row.y,
            SELECTED_ACCENT_WIDTH,
            NAV_ROW_HEIGHT,
            palette.accent,
        );
    }
    if context.guides.show_lines {
        let depth = match context.depth {
            PageDepth::Sectionless => NavigationDepth::Section,
            PageDepth::Section => NavigationDepth::Page,
        };
        draw_row_guides(
            canvas,
            palette,
            NavigationGuideContext {
                row_depth: depth,
                show_text_connector: context.guides.show_text_connectors,
                draw_horizontal_connector: false,
                rows: context.row.rows,
                row_index: context.row.row_index,
                row_y: context.row.y,
            },
        );
    }
    text.draw_centered(
        canvas,
        page,
        page_text_x(context.depth),
        TextVerticalBox::new(context.row.y, NAV_ROW_HEIGHT as f32),
        NAV_TEXT_SIZE,
        text_color,
    );
}
