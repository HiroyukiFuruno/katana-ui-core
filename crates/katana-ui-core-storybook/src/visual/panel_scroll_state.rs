use super::layout_metrics::{
    INSPECTOR_HEIGHT, INSPECTOR_WIDTH, INSPECTOR_X, INSPECTOR_Y, MAX_SCROLL_Y, NAV_WIDTH,
    PREVIEW_X, SCROLL_STEP,
};
use super::navigation_tree::TreeExpansionState;

pub(super) const PREVIEW_MAX_SCROLL_Y: usize = 480;
pub(super) const INSPECTOR_MAX_SCROLL_Y: usize = 360;
pub(super) const ROOT_MAX_SCROLL_Y: usize = MAX_SCROLL_Y;
pub(super) const NAV_MAX_SCROLL_X: usize = 0;
pub(super) const PREVIEW_MAX_SCROLL_X: usize = 320;
pub(super) const INSPECTOR_MAX_SCROLL_X: usize = 160;
pub(super) const ROOT_MAX_SCROLL_X: usize = 0;
const PREVIEW_PANEL_RIGHT: usize = INSPECTOR_X - 24;
const CHILD_PANEL_TOP: usize = 120;
const INSPECTOR_VIEWPORT_WIDTH: usize = 352;
const INSPECTOR_VIEWPORT_HEIGHT: usize = 776;
const INSPECTOR_CONTENT_WIDTH: usize = 512;
const INSPECTOR_CONTENT_HEIGHT: usize = 1136;
const PREVIEW_VIEWPORT_WIDTH: usize = 720;
const PREVIEW_VIEWPORT_HEIGHT: usize = 714;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PanelScrollRegion {
    Root,
    Navigation,
    Preview,
    Inspector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PanelOverflow {
    pub(super) viewport_width: usize,
    pub(super) viewport_height: usize,
    pub(super) content_width: usize,
    pub(super) content_height: usize,
}

impl PanelOverflow {
    pub(super) const fn new(
        viewport_width: usize,
        viewport_height: usize,
        content_width: usize,
        content_height: usize,
    ) -> Self {
        Self {
            viewport_width,
            viewport_height,
            content_width,
            content_height,
        }
    }

    pub(super) fn max_x(self) -> usize {
        self.content_width.saturating_sub(self.viewport_width)
    }

    pub(super) fn max_y(self) -> usize {
        self.content_height.saturating_sub(self.viewport_height)
    }

    pub(super) fn overflows_x(self) -> bool {
        self.max_x() > 0
    }

    pub(super) fn overflows_y(self) -> bool {
        self.max_y() > 0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct PanelScrollOffsets {
    pub(super) root_x: usize,
    pub(super) root_y: usize,
    pub(super) navigation_x: usize,
    pub(super) navigation_y: usize,
    pub(super) preview_x: usize,
    pub(super) preview_y: usize,
    pub(super) inspector_x: usize,
    pub(super) inspector_y: usize,
}

impl PanelScrollOffsets {
    #[cfg(test)]
    pub(super) fn scroll_delta(&mut self, region: PanelScrollRegion, delta_y: f32) -> bool {
        self.scroll_delta_with_max(region, max_scroll_y(region), delta_y)
    }

    pub(super) fn scroll_delta_with_max(
        &mut self,
        region: PanelScrollRegion,
        max_offset: usize,
        delta_y: f32,
    ) -> bool {
        let before = self.offset(region);
        let next = next_offset(before, max_offset, delta_y);
        self.set_offset(region, next);
        before != next
    }

    pub(super) fn scroll_delta_x(&mut self, region: PanelScrollRegion, delta_x: f32) -> bool {
        let before = self.offset_x(region);
        let next = next_offset(before, max_scroll_x(region), delta_x);
        self.set_offset_x(region, next);
        before != next
    }

    #[cfg(test)]
    pub(super) fn set_drag_offset(&mut self, region: PanelScrollRegion, value: usize) -> bool {
        self.set_drag_offset_with_max(region, value, max_scroll_y(region))
    }

    pub(super) fn set_drag_offset_with_max(
        &mut self,
        region: PanelScrollRegion,
        value: usize,
        max_offset: usize,
    ) -> bool {
        let before = self.offset(region);
        self.set_offset(region, value.min(max_offset));
        before != self.offset(region)
    }

    pub(super) fn set_drag_offset_x(&mut self, region: PanelScrollRegion, value: usize) -> bool {
        let before = self.offset_x(region);
        self.set_offset_x(region, value.min(max_scroll_x(region)));
        before != self.offset_x(region)
    }

    pub(super) fn offset(self, region: PanelScrollRegion) -> usize {
        match region {
            PanelScrollRegion::Root => self.root_y,
            PanelScrollRegion::Navigation => self.navigation_y,
            PanelScrollRegion::Preview => self.preview_y,
            PanelScrollRegion::Inspector => self.inspector_y,
        }
    }

    pub(super) fn offset_x(self, region: PanelScrollRegion) -> usize {
        match region {
            PanelScrollRegion::Root => self.root_x,
            PanelScrollRegion::Navigation => self.navigation_x,
            PanelScrollRegion::Preview => self.preview_x,
            PanelScrollRegion::Inspector => self.inspector_x,
        }
    }

    fn set_offset(&mut self, region: PanelScrollRegion, value: usize) {
        match region {
            PanelScrollRegion::Root => self.root_y = value,
            PanelScrollRegion::Navigation => self.navigation_y = value,
            PanelScrollRegion::Preview => self.preview_y = value,
            PanelScrollRegion::Inspector => self.inspector_y = value,
        }
    }

    fn set_offset_x(&mut self, region: PanelScrollRegion, value: usize) {
        match region {
            PanelScrollRegion::Root => self.root_x = value,
            PanelScrollRegion::Navigation => self.navigation_x = value,
            PanelScrollRegion::Preview => self.preview_x = value,
            PanelScrollRegion::Inspector => self.inspector_x = value,
        }
    }
}

pub(super) fn region_at(x: usize, y: usize) -> PanelScrollRegion {
    if x < NAV_WIDTH && y >= CHILD_PANEL_TOP {
        return PanelScrollRegion::Navigation;
    }
    if (INSPECTOR_X..INSPECTOR_X + INSPECTOR_WIDTH).contains(&x)
        && (INSPECTOR_Y..INSPECTOR_Y + INSPECTOR_HEIGHT).contains(&y)
    {
        return PanelScrollRegion::Inspector;
    }
    if (PREVIEW_X..PREVIEW_PANEL_RIGHT).contains(&x) && y >= CHILD_PANEL_TOP {
        return PanelScrollRegion::Preview;
    }
    PanelScrollRegion::Root
}

fn next_offset(current: usize, max_scroll_y: usize, delta_y: f32) -> usize {
    if delta_y < 0.0 {
        return (current + SCROLL_STEP).min(max_scroll_y);
    }
    current.saturating_sub(SCROLL_STEP)
}

pub(super) fn max_scroll_y(region: PanelScrollRegion) -> usize {
    match region {
        PanelScrollRegion::Root => ROOT_MAX_SCROLL_Y,
        PanelScrollRegion::Navigation => {
            super::navigation_tree::max_scroll_y(TreeExpansionState::default())
        }
        PanelScrollRegion::Preview => PREVIEW_MAX_SCROLL_Y,
        PanelScrollRegion::Inspector => INSPECTOR_MAX_SCROLL_Y,
    }
}

pub(super) fn max_scroll_x(region: PanelScrollRegion) -> usize {
    match region {
        PanelScrollRegion::Root => ROOT_MAX_SCROLL_X,
        PanelScrollRegion::Navigation => NAV_MAX_SCROLL_X,
        PanelScrollRegion::Preview => PREVIEW_MAX_SCROLL_X,
        PanelScrollRegion::Inspector => INSPECTOR_MAX_SCROLL_X,
    }
}

pub(super) fn overflow_for(
    region: PanelScrollRegion,
    selected_page: &str,
    expansion: TreeExpansionState,
) -> PanelOverflow {
    match region {
        PanelScrollRegion::Root => PanelOverflow::new(
            super::render::WIDTH,
            super::render::VIEWPORT_HEIGHT,
            super::render::WIDTH,
            super::render::CANVAS_HEIGHT,
        ),
        PanelScrollRegion::Navigation => navigation_overflow(expansion),
        PanelScrollRegion::Preview => preview_overflow(selected_page),
        PanelScrollRegion::Inspector => PanelOverflow::new(
            INSPECTOR_VIEWPORT_WIDTH,
            INSPECTOR_VIEWPORT_HEIGHT,
            INSPECTOR_CONTENT_WIDTH,
            INSPECTOR_CONTENT_HEIGHT,
        ),
    }
}

fn navigation_overflow(expansion: TreeExpansionState) -> PanelOverflow {
    let viewport_height = super::layout_metrics::navigation_menu_panel_rect()
        .bottom()
        .saturating_sub(super::layout_metrics::NAV_FIRST_ROW_Y);
    PanelOverflow::new(
        super::layout_metrics::NAV_ROW_WIDTH,
        viewport_height,
        super::layout_metrics::NAV_ROW_WIDTH,
        viewport_height + super::navigation_tree::max_scroll_y(expansion),
    )
}

fn preview_overflow(selected_page: &str) -> PanelOverflow {
    let _ = selected_page;
    PanelOverflow::new(
        PREVIEW_VIEWPORT_WIDTH,
        PREVIEW_VIEWPORT_HEIGHT,
        PREVIEW_VIEWPORT_WIDTH,
        PREVIEW_VIEWPORT_HEIGHT,
    )
}
