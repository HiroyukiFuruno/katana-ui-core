use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::text::TextRenderer;
use katana_ui_core::render_model::{UiTreeLineStyle, UiTreeProps, UiTreeToggleTriggerArea};

pub(super) const TREE_PANEL_X: usize = 14;
pub(super) const TREE_PANEL_Y: usize = 30;
pub(super) const TREE_PANEL_WIDTH: usize = 178;
pub(super) const TREE_PANEL_HEIGHT: usize = 68;
pub(super) const ROW_HEIGHT: usize = 17;
pub(super) const LINE_X: usize = 31;
pub(super) const NODE_ICON_X: usize = 48;
#[cfg(test)]
pub(super) const CHILD_ICON_X: usize = 68;
pub(super) const LABEL_X: usize = 82;
const CONTEXT_X: usize = 212;
const CONTEXT_Y: usize = 30;
const CONTEXT_WIDTH: usize = 112;
const CONTEXT_HEIGHT: usize = 58;
const FOLDER_COLOR: u32 = 0xd7ba7d;
const FILE_COLOR: u32 = 0x9cdcfe;
const MENU_COLOR: u32 = 0x252a33;

pub(super) fn folder_icon(canvas: &mut Canvas, x: usize, y: usize) {
    common::fill(
        canvas,
        Rect::new(x, y + m::PX_2 + m::PX_2, m::PX_14, m::PX_10),
        FOLDER_COLOR,
    );
    common::fill(
        canvas,
        Rect::new(x + m::PX_2, y + m::PX_1, m::PX_8, m::PX_2 + m::PX_2),
        FOLDER_COLOR,
    );
}

pub(super) fn file_icon(canvas: &mut Canvas, x: usize, y: usize) {
    common::fill(
        canvas,
        Rect::new(x + m::PX_2, y + m::PX_1, m::PX_10, m::PX_13),
        FILE_COLOR,
    );
    common::fill(
        canvas,
        Rect::new(x + m::PX_8 + m::PX_1, y + m::PX_1, m::PX_3, m::PX_3),
        MENU_COLOR,
    );
}

pub(super) fn draw_context_menu(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    let rect = Rect::new(x + CONTEXT_X, y + CONTEXT_Y, CONTEXT_WIDTH, CONTEXT_HEIGHT);
    common::fill(canvas, rect, MENU_COLOR);
    common::outline(canvas, palette, rect);
    text.draw(
        canvas,
        "context menu",
        x + CONTEXT_X + m::PX_8,
        y + CONTEXT_Y + m::PX_8,
        m::FONT_8,
        palette.text,
    );
    text.draw(
        canvas,
        "New folder",
        x + CONTEXT_X + m::PX_8,
        y + CONTEXT_Y + m::PX_24,
        m::FONT_8,
        palette.muted,
    );
    text.draw(
        canvas,
        "Reveal file",
        x + CONTEXT_X + m::PX_8,
        y + CONTEXT_Y + m::PX_40,
        m::FONT_8,
        palette.muted,
    );
}

pub(super) fn draw_option_strip(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    tree: &UiTreeProps,
    x: usize,
    y: usize,
) {
    common::chip(
        canvas,
        text,
        palette,
        Rect::new(x + CONTEXT_X, y + m::PX_92, m::PX_54, m::PX_14),
        trigger_label(tree.toggle_trigger_area),
        palette.accent,
    );
    text.draw(
        canvas,
        &option_summary(tree),
        x + TREE_PANEL_X,
        y + m::PX_98 + m::PX_2,
        m::FONT_7,
        palette.muted,
    );
}

fn trigger_label(value: UiTreeToggleTriggerArea) -> &'static str {
    match value {
        UiTreeToggleTriggerArea::IconOnly => "icon",
        UiTreeToggleTriggerArea::IconAndText => "icon+text",
        UiTreeToggleTriggerArea::WholeElement => "full row",
        UiTreeToggleTriggerArea::TextOnly => "text",
    }
}

fn option_summary(tree: &UiTreeProps) -> String {
    let line = match tree.line_style {
        UiTreeLineStyle::Solid => "solid",
        UiTreeLineStyle::Dotted => "dotted",
        UiTreeLineStyle::Dashed => "dashed",
    };
    if tree.icons_visible {
        return format!(
            "{line} line / folder+file / default open={}",
            tree.default_open
        );
    }
    format!(
        "{line} line / icons hidden / default open={}",
        tree.default_open
    )
}
