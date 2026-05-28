use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_tabs_metrics::{
    CLOSE_ICON_X_OFFSET, CLOSE_ICON_Y_OFFSET, DIRTY_RIGHT_OFFSET, DIRTY_SIZE, DIRTY_Y_OFFSET,
    GROUP_DOT_SIZE, GROUP_DOT_X, GROUP_DOT_Y, GROUP_HEADER_WIDTH, GROUP_TEXT_X,
    GROUP_UNDERLINE_HEIGHT, PIN_CROSS_Y_OFFSET, PIN_HEAD_WIDTH, PIN_HEAD_X_OFFSET, PIN_ICON_SIZE,
    PIN_ICON_X_OFFSET, PIN_ICON_Y_OFFSET, PIN_STEM_HEIGHT, PIN_STEM_WIDTH, PIN_STEM_X_OFFSET,
    STRIP_HEIGHT, STRIP_LEADING_INSET, STRIP_WIDTH, STRIP_X, STRIP_Y, TAB_CLOSE_AREA,
    TAB_CLOSE_SIZE, TAB_GAP, TAB_HEIGHT, TAB_LABEL_X, TAB_Y, tab_width,
};
use super::palette::VisualPalette;
use super::screen_state_tabs::{TabsScreenState, TabsScreenTab};
use super::text::{TextBox, TextRenderer};

pub(super) fn draw_strip(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    state: &TabsScreenState,
    x: usize,
    y: usize,
) {
    let strip = Rect::new(x + STRIP_X, y + STRIP_Y, STRIP_WIDTH, STRIP_HEIGHT);
    common::fill(canvas, strip, palette.surface);
    common::outline(canvas, palette, strip);
    let mut cursor_x = x + STRIP_X + STRIP_LEADING_INSET;
    draw_pinned_tabs(canvas, text, palette, state, &mut cursor_x, x, y);
    draw_grouped_tabs(canvas, text, palette, state, &mut cursor_x, y);
    draw_ungrouped_tabs(canvas, text, palette, state, &mut cursor_x, y);
}

fn draw_pinned_tabs(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    state: &TabsScreenState,
    cursor_x: &mut usize,
    x: usize,
    y: usize,
) {
    for tab in state.tabs.iter().filter(|tab| tab.pinned) {
        draw_tab(canvas, text, palette, state, tab, *cursor_x, y + TAB_Y);
        *cursor_x += tab_width(tab) + TAB_GAP;
    }
    if *cursor_x > x + STRIP_X + STRIP_LEADING_INSET {
        *cursor_x += TAB_GAP;
    }
}

fn draw_grouped_tabs(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    state: &TabsScreenState,
    cursor_x: &mut usize,
    y: usize,
) {
    for group in &state.groups {
        if !has_visible_group_tabs(state, group.id.as_str()) {
            continue;
        }
        draw_group_header(
            canvas,
            text,
            palette,
            group.title.as_str(),
            group.color,
            *cursor_x,
            y,
        );
        *cursor_x += GROUP_HEADER_WIDTH + TAB_GAP;
        for tab in state
            .tabs
            .iter()
            .filter(|tab| !tab.pinned && tab.group_id.as_deref() == Some(group.id.as_str()))
        {
            draw_tab(canvas, text, palette, state, tab, *cursor_x, y + TAB_Y);
            *cursor_x += tab_width(tab) + TAB_GAP;
        }
    }
}

fn has_visible_group_tabs(state: &TabsScreenState, group_id: &str) -> bool {
    state
        .tabs
        .iter()
        .any(|tab| !tab.pinned && tab.group_id.as_deref() == Some(group_id))
}

fn draw_ungrouped_tabs(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    state: &TabsScreenState,
    cursor_x: &mut usize,
    y: usize,
) {
    for tab in state
        .tabs
        .iter()
        .filter(|tab| !tab.pinned && tab.group_id.is_none())
    {
        draw_tab(canvas, text, palette, state, tab, *cursor_x, y + TAB_Y);
        *cursor_x += tab_width(tab) + TAB_GAP;
    }
}

fn draw_tab(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    state: &TabsScreenState,
    tab: &TabsScreenTab,
    x: usize,
    y: usize,
) {
    let width = tab_width(tab);
    let active = state.active_tab_id == tab.id;
    let fill = if active {
        palette.accent
    } else {
        palette.panel
    };
    let text_color = if active {
        palette.background
    } else {
        palette.text
    };
    common::fill(canvas, Rect::new(x, y, width, TAB_HEIGHT), fill);
    common::outline(canvas, palette, Rect::new(x, y, width, TAB_HEIGHT));
    draw_tab_action_icon(canvas, palette, tab, x, y, width);
    draw_tab_label(canvas, text, tab, x, y, width, text_color);
    draw_dirty_dot(canvas, tab, x, y, width);
    draw_group_underline(canvas, tab, x, y, width);
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
    common::cross_icon(
        canvas,
        icon_x + CLOSE_ICON_X_OFFSET,
        y + CLOSE_ICON_Y_OFFSET,
        TAB_CLOSE_SIZE,
        palette.muted,
    );
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
    let label_width = width.saturating_sub(TAB_CLOSE_AREA + TAB_LABEL_X);
    canvas.with_clip(x + TAB_LABEL_X, y, label_width, TAB_HEIGHT, |canvas| {
        text.draw_in_box(
            canvas,
            tab.title.as_str(),
            TextBox::new(x + TAB_LABEL_X, y, label_width, TAB_HEIGHT),
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
    x: usize,
    y: usize,
) {
    let rect = Rect::new(x, y + TAB_Y, GROUP_HEADER_WIDTH, TAB_HEIGHT);
    common::fill(canvas, rect, palette.panel);
    common::outline(canvas, palette, rect);
    common::fill(
        canvas,
        Rect::new(
            x + GROUP_DOT_X,
            y + TAB_Y + GROUP_DOT_Y,
            GROUP_DOT_SIZE,
            GROUP_DOT_SIZE,
        ),
        color,
    );
    text.draw_in_box(
        canvas,
        title,
        TextBox::new(
            x + GROUP_TEXT_X,
            y + TAB_Y,
            GROUP_HEADER_WIDTH - GROUP_TEXT_X,
            TAB_HEIGHT,
        ),
        m::FONT_7,
        palette.text,
    );
}

fn draw_pin_icon(canvas: &mut Canvas, x: usize, y: usize, color: u32) {
    common::fill(
        canvas,
        Rect::new(x + PIN_HEAD_X_OFFSET, y, PIN_HEAD_WIDTH, PIN_ICON_SIZE),
        color,
    );
    common::fill(
        canvas,
        Rect::new(x, y + PIN_CROSS_Y_OFFSET, PIN_ICON_SIZE, PIN_HEAD_WIDTH),
        color,
    );
    common::fill(
        canvas,
        Rect::new(
            x + PIN_STEM_X_OFFSET,
            y + PIN_ICON_SIZE - PIN_STEM_WIDTH,
            PIN_STEM_WIDTH,
            PIN_STEM_HEIGHT,
        ),
        color,
    );
}
