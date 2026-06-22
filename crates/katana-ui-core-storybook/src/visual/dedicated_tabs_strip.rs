use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_tabs_icons::draw_pin_icon;
use super::dedicated_tabs_layout::{TabsLayoutItem, layout_items};
use super::dedicated_tabs_metrics::{
    CLOSE_ICON_X_OFFSET, CLOSE_ICON_Y_OFFSET, DIRTY_RIGHT_OFFSET, DIRTY_SIZE, DIRTY_Y_OFFSET,
    GROUP_DOT_SIZE, GROUP_DOT_X, GROUP_DOT_Y, GROUP_TEXT_X, GROUP_UNDERLINE_HEIGHT,
    PIN_ICON_X_OFFSET, PIN_ICON_Y_OFFSET, STRIP_HEIGHT, STRIP_WIDTH, STRIP_X, STRIP_Y,
    TAB_CLOSE_AREA, TAB_CLOSE_SIZE, TAB_HEIGHT, TAB_LABEL_X,
};
use super::palette::VisualPalette;
use super::screen_state_tabs::{TabsScreenState, TabsScreenTab};
use super::text::{TextBox, TextRenderer};

const TAB_ICON_SIZE: usize = 8;
const TAB_ICON_X_OFFSET: usize = 7;
const TAB_ICON_Y_OFFSET: usize = 9;
const TAB_ICON_LABEL_GAP: usize = 4;
const TAB_METADATA_MARKER_SIZE: usize = 10;
const TAB_TOOLTIP_MARKER_RIGHT_OFFSET: usize = 44;
const TAB_A11Y_MARKER_RIGHT_OFFSET: usize = 58;
const TAB_METADATA_MARKER_Y: usize = 5;
const TONE_WARNING: u32 = 0xd9904a;

pub(super) fn draw_strip(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    state: &TabsScreenState,
    hovered: bool,
    x: usize,
    y: usize,
) {
    let strip = Rect::new(x + STRIP_X, y + STRIP_Y, STRIP_WIDTH, STRIP_HEIGHT);
    common::fill(canvas, strip, palette.surface);
    common::outline(canvas, palette, strip);
    if hovered {
        canvas.stroke_rect(
            strip.x,
            strip.y,
            strip.width,
            strip.height,
            palette.hover_border,
        );
    }
    canvas.with_clip(strip.x, strip.y, strip.width, strip.height, |canvas| {
        for item in layout_items(x, y, state) {
            match item {
                TabsLayoutItem::GroupHeader {
                    title, color, rect, ..
                } => {
                    draw_group_header(canvas, text, palette, title, color, rect);
                }
                TabsLayoutItem::Tab { tab, rect } => {
                    draw_tab(canvas, text, palette, state, tab, rect);
                }
            }
        }
    });
}

fn draw_tab(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    state: &TabsScreenState,
    tab: &TabsScreenTab,
    rect: super::layout_metrics::LayoutRect,
) {
    let active = state.active_tab_id == tab.id;
    common::fill(canvas, rect_to_common(rect), palette.panel);
    common::outline(canvas, palette, rect_to_common(rect));
    if active {
        common::fill(
            canvas,
            Rect::new(rect.x, rect.y + TAB_HEIGHT - 2, rect.width, 2),
            palette.accent,
        );
    }
    if state.focused_tab_id.as_deref() == Some(tab.id.as_str()) {
        canvas.stroke_rect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            palette.hover_border,
        );
    }
    draw_tab_tone(canvas, tab, rect.x, rect.y, rect.width);
    draw_tab_icon(canvas, palette, tab, rect.x, rect.y);
    draw_tab_action_icon(canvas, palette, tab, rect.x, rect.y, rect.width);
    draw_tab_metadata_markers(canvas, palette, tab, rect.x, rect.y, rect.width);
    draw_tab_label(canvas, text, tab, rect.x, rect.y, rect.width, palette.text);
    draw_dirty_dot(canvas, tab, rect.x, rect.y, rect.width);
    draw_group_underline(canvas, tab, rect.x, rect.y, rect.width);
}

fn draw_tab_action_icon(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    tab: &TabsScreenTab,
    x: usize,
    y: usize,
    width: usize,
) {
    let icon_x = x + width - TAB_CLOSE_AREA;
    if tab.pinned {
        draw_pin_icon(
            canvas,
            icon_x + PIN_ICON_X_OFFSET,
            y + PIN_ICON_Y_OFFSET,
            palette.muted,
        );
        return;
    }
    if !tab.closeable {
        return;
    }
    common::cross_icon(
        canvas,
        icon_x + CLOSE_ICON_X_OFFSET,
        y + CLOSE_ICON_Y_OFFSET,
        TAB_CLOSE_SIZE,
        palette.muted,
    );
}

fn draw_tab_metadata_markers(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    tab: &TabsScreenTab,
    x: usize,
    y: usize,
    width: usize,
) {
    if tab.tooltip.is_some() {
        common::fill(
            canvas,
            Rect::new(
                x + width - TAB_TOOLTIP_MARKER_RIGHT_OFFSET,
                y + TAB_METADATA_MARKER_Y,
                TAB_METADATA_MARKER_SIZE,
                TAB_METADATA_MARKER_SIZE,
            ),
            palette.muted,
        );
    }
    if tab.accessibility_label.is_some() {
        common::fill(
            canvas,
            Rect::new(
                x + width - TAB_A11Y_MARKER_RIGHT_OFFSET,
                y + TAB_METADATA_MARKER_Y,
                TAB_METADATA_MARKER_SIZE,
                TAB_METADATA_MARKER_SIZE,
            ),
            palette.accent,
        );
    }
}

fn draw_tab_icon(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    tab: &TabsScreenTab,
    x: usize,
    y: usize,
) {
    if !tab.icon_visible {
        return;
    }
    common::outline(
        canvas,
        palette,
        Rect::new(
            x + TAB_ICON_X_OFFSET,
            y + TAB_ICON_Y_OFFSET,
            TAB_ICON_SIZE,
            TAB_ICON_SIZE,
        ),
    );
}

fn draw_tab_tone(canvas: &mut Canvas, tab: &TabsScreenTab, x: usize, y: usize, width: usize) {
    if tab.tone != "warning" {
        return;
    }
    common::fill(canvas, Rect::new(x, y, width, 2), TONE_WARNING);
}

fn draw_tab_label(
    canvas: &mut Canvas,
    text: &TextRenderer,
    tab: &TabsScreenTab,
    x: usize,
    y: usize,
    width: usize,
    color: u32,
) {
    let label_x = if tab.icon_visible {
        TAB_LABEL_X + TAB_ICON_SIZE + TAB_ICON_LABEL_GAP
    } else {
        TAB_LABEL_X
    };
    let label_width = width.saturating_sub(TAB_CLOSE_AREA + label_x);
    canvas.with_clip(x + label_x, y, label_width, TAB_HEIGHT, |canvas| {
        text.draw_in_box(
            canvas,
            tab.title.as_str(),
            TextBox::new(x + label_x, y, label_width, TAB_HEIGHT),
            m::FONT_8,
            color,
        );
    });
}

fn draw_dirty_dot(canvas: &mut Canvas, tab: &TabsScreenTab, x: usize, y: usize, width: usize) {
    if !tab.dirty {
        return;
    }
    common::fill(
        canvas,
        Rect::new(
            x + width - DIRTY_RIGHT_OFFSET,
            y + DIRTY_Y_OFFSET,
            DIRTY_SIZE,
            DIRTY_SIZE,
        ),
        common::DANGER,
    );
}

fn draw_group_underline(
    canvas: &mut Canvas,
    tab: &TabsScreenTab,
    x: usize,
    y: usize,
    width: usize,
) {
    if tab.group_id.is_none() {
        return;
    }
    common::fill(
        canvas,
        Rect::new(
            x,
            y + TAB_HEIGHT - GROUP_UNDERLINE_HEIGHT,
            width,
            GROUP_UNDERLINE_HEIGHT,
        ),
        common::TOKEN,
    );
}

fn draw_group_header(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    title: &str,
    color: u32,
    rect: super::layout_metrics::LayoutRect,
) {
    common::fill(canvas, rect_to_common(rect), palette.panel);
    common::outline(canvas, palette, rect_to_common(rect));
    common::fill(
        canvas,
        Rect::new(
            rect.x + GROUP_DOT_X,
            rect.y + GROUP_DOT_Y,
            GROUP_DOT_SIZE,
            GROUP_DOT_SIZE,
        ),
        color,
    );
    text.draw_in_box(
        canvas,
        title,
        TextBox::new(
            rect.x + GROUP_TEXT_X,
            rect.y,
            rect.width.saturating_sub(GROUP_TEXT_X),
            rect.height,
        ),
        m::FONT_7,
        palette.text,
    );
}

fn rect_to_common(rect: super::layout_metrics::LayoutRect) -> Rect {
    Rect::new(rect.x, rect.y, rect.width, rect.height)
}
