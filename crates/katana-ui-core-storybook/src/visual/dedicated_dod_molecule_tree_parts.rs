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
pub(super) const DISCLOSURE_X: usize = 24;
pub(super) const INDENT_STEP: usize = 18;
pub(super) const NODE_ICON_X: usize = 36;
#[cfg(test)]
pub(super) const CHILD_ICON_X: usize = 54;
#[cfg(test)]
pub(super) const GRANDCHILD_ICON_X: usize = 72;
pub(super) const LABEL_X: usize = 50;
const CONTEXT_X: usize = 212;
const CONTEXT_Y: usize = 30;
const CONTEXT_WIDTH: usize = 112;
const CONTEXT_HEIGHT: usize = 58;
const BRANCH_MARKER_COLOR: u32 = 0x9aa4b2;
const LEAF_MARKER_COLOR: u32 = 0xb7c0cd;
const MENU_COLOR: u32 = 0x252a33;

pub(super) fn branch_marker(canvas: &mut Canvas, x: usize, y: usize) {
    common::fill(
        canvas,
        Rect::new(x + m::PX_2, y + m::PX_4, m::PX_8, m::PX_8),
        BRANCH_MARKER_COLOR,
    );
    common::fill(
        canvas,
        Rect::new(x + m::PX_4, y + m::PX_6, m::PX_4, m::PX_4),
        MENU_COLOR,
    );
}

pub(super) fn leaf_marker(canvas: &mut Canvas, x: usize, y: usize) {
    common::fill(
        canvas,
        Rect::new(x + m::PX_4, y + m::PX_6, m::PX_4, m::PX_4),
        LEAF_MARKER_COLOR,
    );
}

pub(super) fn draw_disclosure(canvas: &mut Canvas, color: u32, x: usize, y: usize, expanded: bool) {
    if expanded {
        common::fill(
            canvas,
            Rect::new(x + m::PX_3, y + m::PX_8, m::PX_6, m::PX_2),
            color,
        );
        common::fill(
            canvas,
            Rect::new(x + m::PX_4 + m::PX_1, y + m::PX_10, m::PX_2, m::PX_2),
            color,
        );
    } else {
        common::fill(
            canvas,
            Rect::new(x + m::PX_4, y + m::PX_6, m::PX_2, m::PX_6),
            color,
        );
        common::fill(
            canvas,
            Rect::new(x + m::PX_6, y + m::PX_8, m::PX_2, m::PX_2),
            color,
        );
    }
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
        "New branch",
        x + CONTEXT_X + m::PX_8,
        y + CONTEXT_Y + m::PX_24,
        m::FONT_8,
        palette.muted,
    );
    text.draw(
        canvas,
        "Open item",
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
            "{line} line / branch+leaf / default open={}",
            tree.default_open
        );
    }
    format!(
        "{line} line / icons hidden / default open={}",
        tree.default_open
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree_props() -> UiTreeProps {
        UiTreeProps {
            active_id: "tree-active".to_string(),
            line_display: true,
            line_style: UiTreeLineStyle::Solid,
            line_width: 1,
            icons_visible: true,
            directory_icon: "<svg data-icon=\"branch\"/>".to_string(),
            file_icon: "<svg data-icon=\"leaf\"/>".to_string(),
            font_role: "body".to_string(),
            theme_id: "dark".to_string(),
            default_open: true,
            nodes: Vec::new(),
            empty_area_context_menu: true,
            toggle_icon: "<svg data-icon=\"chevron\"/>".to_string(),
            toggle_trigger_area: UiTreeToggleTriggerArea::IconAndText,
        }
    }

    #[test]
    fn option_summary_tracks_marker_visibility_toggle() {
        let visible = sample_tree_props();
        let mut hidden = sample_tree_props();
        hidden.icons_visible = false;

        assert_eq!(
            "solid line / branch+leaf / default open=true",
            option_summary(&visible)
        );
        assert_eq!(
            "solid line / icons hidden / default open=true",
            option_summary(&hidden)
        );
    }

    #[test]
    fn trigger_label_covers_all_trigger_areas() {
        assert_eq!("icon", trigger_label(UiTreeToggleTriggerArea::IconOnly));
        assert_eq!(
            "icon+text",
            trigger_label(UiTreeToggleTriggerArea::IconAndText)
        );
        assert_eq!(
            "full row",
            trigger_label(UiTreeToggleTriggerArea::WholeElement)
        );
        assert_eq!("text", trigger_label(UiTreeToggleTriggerArea::TextOnly));
    }
}
