use super::dedicated_tabs_metrics::{
    GROUP_HEADER_WIDTH, STRIP_LEADING_INSET, STRIP_X, TAB_CLOSE_AREA, TAB_GAP, TAB_HEIGHT, TAB_Y,
    tab_width,
};
use super::dedicated_tabs_scroll;
use super::layout_metrics::LayoutRect;
use super::screen_state_tabs::{TabsScreenGroup, TabsScreenState, TabsScreenTab};

pub(super) enum TabsLayoutItem<'a> {
    GroupHeader {
        group: &'a TabsScreenGroup,
        title: &'a str,
        color: u32,
        rect: LayoutRect,
    },
    Tab {
        tab: &'a TabsScreenTab,
        rect: LayoutRect,
    },
}

pub(super) fn layout_items<'a>(
    origin_x: usize,
    origin_y: usize,
    state: &'a TabsScreenState,
) -> Vec<TabsLayoutItem<'a>> {
    let mut items = Vec::new();
    let scroll_x = dedicated_tabs_scroll::scroll_x(state) as isize;
    let mut cursor_x = (origin_x + STRIP_X + STRIP_LEADING_INSET) as isize - scroll_x;
    push_pinned_tabs(&mut items, state, &mut cursor_x, origin_x, origin_y);
    push_grouped_tabs(&mut items, state, &mut cursor_x, origin_x, origin_y);
    push_unknown_group_tabs(&mut items, state, &mut cursor_x, origin_x, origin_y);
    push_ungrouped_tabs(&mut items, state, &mut cursor_x, origin_x, origin_y);
    items
}

pub(super) fn tab_hit_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
    state: &TabsScreenState,
) -> Option<(String, LayoutRect)> {
    for item in layout_items(origin_x, origin_y, state) {
        if let TabsLayoutItem::Tab { tab, rect } = item
            && rect.contains(x, y)
        {
            return Some((tab.id.clone(), rect));
        }
    }
    None
}

pub(super) fn group_hit_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
    state: &TabsScreenState,
) -> Option<(String, LayoutRect)> {
    for item in layout_items(origin_x, origin_y, state) {
        if let TabsLayoutItem::GroupHeader { group, rect, .. } = item
            && rect.contains(x, y)
        {
            return Some((group.id.clone(), rect));
        }
    }
    None
}

pub(super) fn pin_icon_hit_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
    state: &TabsScreenState,
) -> Option<String> {
    for item in layout_items(origin_x, origin_y, state) {
        if let TabsLayoutItem::Tab { tab, rect } = item
            && tab.pinned
            && pin_icon_rect(rect).contains(x, y)
        {
            return Some(tab.id.clone());
        }
    }
    None
}

#[cfg(test)]
pub(super) fn tab_rect_for_id(
    origin_x: usize,
    origin_y: usize,
    state: &TabsScreenState,
    tab_id: &str,
) -> Option<LayoutRect> {
    for item in layout_items(origin_x, origin_y, state) {
        if let TabsLayoutItem::Tab { tab, rect } = item
            && tab.id == tab_id
        {
            return Some(rect);
        }
    }
    None
}

#[cfg(test)]
pub(super) fn group_rect_for_id(
    origin_x: usize,
    origin_y: usize,
    state: &TabsScreenState,
    group_id: &str,
) -> Option<LayoutRect> {
    for item in layout_items(origin_x, origin_y, state) {
        if let TabsLayoutItem::GroupHeader { group, rect, .. } = item
            && group.id == group_id
        {
            return Some(rect);
        }
    }
    None
}

#[cfg(test)]
pub(super) fn pin_icon_rect_for_id(
    origin_x: usize,
    origin_y: usize,
    state: &TabsScreenState,
    tab_id: &str,
) -> Option<LayoutRect> {
    for item in layout_items(origin_x, origin_y, state) {
        if let TabsLayoutItem::Tab { tab, rect } = item
            && tab.id == tab_id
            && tab.pinned
        {
            return Some(pin_icon_rect(rect));
        }
    }
    None
}

fn push_pinned_tabs<'a>(
    items: &mut Vec<TabsLayoutItem<'a>>,
    state: &'a TabsScreenState,
    cursor_x: &mut isize,
    origin_x: usize,
    origin_y: usize,
) {
    let start_x = *cursor_x;
    for tab in state.tabs.iter().filter(|tab| tab.pinned) {
        push_tab(items, tab, cursor_x, origin_x, origin_y);
    }
    if *cursor_x > start_x {
        *cursor_x += TAB_GAP as isize;
    }
}

fn push_grouped_tabs<'a>(
    items: &mut Vec<TabsLayoutItem<'a>>,
    state: &'a TabsScreenState,
    cursor_x: &mut isize,
    origin_x: usize,
    origin_y: usize,
) {
    for group in &state.groups {
        if !has_visible_group_tabs(state, group.id.as_str()) {
            continue;
        }
        push_group_header(items, group, cursor_x, origin_x, origin_y);
        *cursor_x += (GROUP_HEADER_WIDTH + TAB_GAP) as isize;
        if group.collapsed {
            continue;
        }
        for tab in state
            .tabs
            .iter()
            .filter(|tab| !tab.pinned && tab.group_id.as_deref() == Some(group.id.as_str()))
        {
            push_tab(items, tab, cursor_x, origin_x, origin_y);
        }
    }
}

fn push_ungrouped_tabs<'a>(
    items: &mut Vec<TabsLayoutItem<'a>>,
    state: &'a TabsScreenState,
    cursor_x: &mut isize,
    origin_x: usize,
    origin_y: usize,
) {
    for tab in state
        .tabs
        .iter()
        .filter(|tab| !tab.pinned && tab.group_id.is_none())
    {
        push_tab(items, tab, cursor_x, origin_x, origin_y);
    }
}

fn push_unknown_group_tabs<'a>(
    items: &mut Vec<TabsLayoutItem<'a>>,
    state: &'a TabsScreenState,
    cursor_x: &mut isize,
    origin_x: usize,
    origin_y: usize,
) {
    for tab in state.tabs.iter().filter(|tab| {
        !tab.pinned
            && tab.group_id.as_ref().is_some_and(|group_id| {
                state
                    .groups
                    .iter()
                    .all(|group| group.id.as_str() != group_id.as_str())
            })
    }) {
        push_tab(items, tab, cursor_x, origin_x, origin_y);
    }
}

fn push_tab<'a>(
    items: &mut Vec<TabsLayoutItem<'a>>,
    tab: &'a TabsScreenTab,
    cursor_x: &mut isize,
    origin_x: usize,
    origin_y: usize,
) {
    let width = tab_width(tab);
    if let Some(rect) = clipped_rect(*cursor_x, origin_x, origin_y, width) {
        items.push(TabsLayoutItem::Tab { tab, rect });
    }
    *cursor_x += (width + TAB_GAP) as isize;
}

fn push_group_header<'a>(
    items: &mut Vec<TabsLayoutItem<'a>>,
    group: &'a TabsScreenGroup,
    cursor_x: &isize,
    origin_x: usize,
    origin_y: usize,
) {
    if let Some(rect) = clipped_rect(*cursor_x, origin_x, origin_y, GROUP_HEADER_WIDTH) {
        items.push(TabsLayoutItem::GroupHeader {
            group,
            title: group.title.as_str(),
            color: group.color,
            rect,
        });
    }
}

fn clipped_rect(x: isize, origin_x: usize, origin_y: usize, width: usize) -> Option<LayoutRect> {
    let clip_left = (origin_x + STRIP_X) as isize;
    let clip_right = (origin_x + STRIP_X + super::dedicated_tabs_metrics::STRIP_WIDTH) as isize;
    let left = x.max(clip_left);
    let right = (x + width as isize).min(clip_right);
    if right <= left {
        return None;
    }
    Some(LayoutRect::new(
        left as usize,
        origin_y + TAB_Y,
        (right - left) as usize,
        TAB_HEIGHT,
    ))
}

fn pin_icon_rect(tab_rect: LayoutRect) -> LayoutRect {
    LayoutRect::new(
        tab_rect.x + tab_rect.width.saturating_sub(TAB_CLOSE_AREA),
        tab_rect.y,
        TAB_CLOSE_AREA,
        TAB_HEIGHT,
    )
}

fn has_visible_group_tabs(state: &TabsScreenState, group_id: &str) -> bool {
    state
        .tabs
        .iter()
        .any(|tab| !tab.pinned && tab.group_id.as_deref() == Some(group_id))
}
