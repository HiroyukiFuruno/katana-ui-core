use super::canvas::Canvas;
use super::dedicated_context_menu_metrics as cm;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::text::TextRenderer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ContextMenuPreviewCommand {
    OpenInsertSubmenu,
    SelectLink,
}

#[derive(Debug, Clone, Copy)]
struct SubmenuRowSpec<'a> {
    x: usize,
    y: usize,
    index: usize,
    label: &'a str,
    active: bool,
}

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    let menu = Rect::new(
        x + cm::SUBMENU_X,
        y + cm::SUBMENU_Y,
        cm::SUBMENU_WIDTH,
        cm::SUBMENU_HEIGHT,
    );
    common::fill(canvas, menu, palette.panel);
    common::outline(canvas, palette, menu);
    draw_row(
        canvas,
        text,
        palette,
        SubmenuRowSpec {
            x,
            y,
            index: cm::SUBMENU_ROW_TABLE,
            label: "Table",
            active: false,
        },
    );
    draw_row(
        canvas,
        text,
        palette,
        SubmenuRowSpec {
            x,
            y,
            index: cm::SUBMENU_ROW_LINK,
            label: "Link",
            active: true,
        },
    );
}

pub(super) fn command_at(
    component_x: usize,
    component_y: usize,
    x: usize,
    y: usize,
    submenu_open: bool,
) -> Option<ContextMenuPreviewCommand> {
    if submenu_open && submenu_link_rect(component_x, component_y).contains(x, y) {
        return Some(ContextMenuPreviewCommand::SelectLink);
    }
    if insert_row_rect(component_x, component_y).contains(x, y) {
        return Some(ContextMenuPreviewCommand::OpenInsertSubmenu);
    }
    None
}

pub(super) fn insert_row_rect(component_x: usize, component_y: usize) -> LayoutRect {
    menu_row_rect(component_x, component_y, cm::ROW_INSERT)
}

pub(super) fn submenu_link_rect(component_x: usize, component_y: usize) -> LayoutRect {
    submenu_row_rect(component_x, component_y, cm::SUBMENU_ROW_LINK)
}

fn draw_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    spec: SubmenuRowSpec<'_>,
) {
    let rect = submenu_row_rect(spec.x, spec.y, spec.index);
    common::fill(
        canvas,
        Rect::new(rect.x, rect.y, rect.width, rect.height),
        row_fill(spec.active, palette),
    );
    text.draw(
        canvas,
        spec.label,
        rect.x + cm::SUBMENU_ROW_LABEL_X_OFFSET,
        rect.y + cm::MENU_ROW_ICON_Y_OFFSET,
        m::FONT_8,
        row_text(spec.active, palette),
    );
}

fn row_fill(active: bool, palette: &VisualPalette) -> u32 {
    if active {
        palette.accent
    } else {
        palette.panel
    }
}

fn row_text(active: bool, palette: &VisualPalette) -> u32 {
    if active {
        palette.background
    } else {
        palette.text
    }
}

fn menu_row_rect(component_x: usize, component_y: usize, row: usize) -> LayoutRect {
    LayoutRect::new(
        component_x + cm::MENU_X + cm::MENU_ROW_FILL_X_OFFSET,
        component_y + cm::MENU_Y + cm::MENU_ROW_TOP_OFFSET + row * cm::MENU_ROW_HEIGHT,
        cm::MENU_WIDTH - cm::MENU_ROW_FILL_WIDTH_INSET,
        cm::MENU_ROW_HEIGHT,
    )
}

fn submenu_row_rect(component_x: usize, component_y: usize, row: usize) -> LayoutRect {
    LayoutRect::new(
        component_x + cm::SUBMENU_X + cm::MENU_ROW_FILL_X_OFFSET,
        component_y + cm::SUBMENU_Y + cm::SUBMENU_ROW_TOP_OFFSET + row * cm::SUBMENU_ROW_HEIGHT,
        cm::SUBMENU_WIDTH - cm::MENU_ROW_FILL_WIDTH_INSET,
        cm::SUBMENU_ROW_HEIGHT,
    )
}
