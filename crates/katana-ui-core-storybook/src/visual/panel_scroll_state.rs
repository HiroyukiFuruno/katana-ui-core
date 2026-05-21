use super::layout_metrics::{
    INSPECTOR_HEIGHT, INSPECTOR_WIDTH, INSPECTOR_X, INSPECTOR_Y, MAX_SCROLL_Y, NAV_WIDTH,
    PREVIEW_X, SCROLL_STEP,
};

pub(super) const NAV_MAX_SCROLL_Y: usize = 280;
pub(super) const PREVIEW_MAX_SCROLL_Y: usize = 480;
pub(super) const INSPECTOR_MAX_SCROLL_Y: usize = 360;
pub(super) const ROOT_MAX_SCROLL_Y: usize = MAX_SCROLL_Y;
pub(super) const NAV_MAX_SCROLL_X: usize = 0;
pub(super) const PREVIEW_MAX_SCROLL_X: usize = 320;
pub(super) const INSPECTOR_MAX_SCROLL_X: usize = 160;
pub(super) const ROOT_MAX_SCROLL_X: usize = 0;
const PREVIEW_PANEL_RIGHT: usize = INSPECTOR_X - 24;
const CHILD_PANEL_TOP: usize = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PanelScrollRegion {
    Root,
    Navigation,
    Preview,
    Inspector,
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
    pub(super) fn scroll_delta(&mut self, region: PanelScrollRegion, delta_y: f32) -> bool {
        let before = self.offset(region);
        let next = next_offset(before, max_scroll_y(region), delta_y);
        self.set_offset(region, next);
        before != next
    }

    pub(super) fn scroll_delta_x(&mut self, region: PanelScrollRegion, delta_x: f32) -> bool {
        let before = self.offset_x(region);
        let next = next_offset(before, max_scroll_x(region), delta_x);
        self.set_offset_x(region, next);
        before != next
    }

    pub(super) fn set_drag_offset(&mut self, region: PanelScrollRegion, value: usize) -> bool {
        let before = self.offset(region);
        self.set_offset(region, value.min(max_scroll_y(region)));
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
        PanelScrollRegion::Navigation => NAV_MAX_SCROLL_Y,
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
