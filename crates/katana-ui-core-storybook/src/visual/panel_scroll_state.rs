use super::layout_metrics::{MAX_SCROLL_Y, SCROLL_STEP};
use super::navigation_tree::TreeExpansionState;
use super::panel_layout;

const INSPECTOR_EXTRA_SCROLL_X: usize = 160;
const INSPECTOR_EXTRA_SCROLL_Y: usize = 360;

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
    if panel_layout::region_frame(PanelScrollRegion::Navigation).contains(x, y) {
        return PanelScrollRegion::Navigation;
    }
    if panel_layout::region_frame(PanelScrollRegion::Inspector).contains(x, y) {
        return PanelScrollRegion::Inspector;
    }
    if panel_layout::region_frame(PanelScrollRegion::Preview).contains(x, y) {
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
        PanelScrollRegion::Root => MAX_SCROLL_Y,
        PanelScrollRegion::Navigation => {
            super::navigation_tree::max_scroll_y(TreeExpansionState::default())
        }
        PanelScrollRegion::Preview => 0,
        PanelScrollRegion::Inspector => INSPECTOR_EXTRA_SCROLL_Y,
    }
}

pub(super) fn max_scroll_x(region: PanelScrollRegion) -> usize {
    match region {
        PanelScrollRegion::Root | PanelScrollRegion::Navigation | PanelScrollRegion::Preview => 0,
        PanelScrollRegion::Inspector => INSPECTOR_EXTRA_SCROLL_X,
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
        PanelScrollRegion::Inspector => inspector_overflow(),
    }
}

fn navigation_overflow(expansion: TreeExpansionState) -> PanelOverflow {
    let viewport = panel_layout::region_layout(PanelScrollRegion::Navigation).content_viewport;
    PanelOverflow::new(
        viewport.width,
        viewport.height,
        viewport.width,
        viewport.height + super::navigation_tree::max_scroll_y(expansion),
    )
}

fn preview_overflow(selected_page: &str) -> PanelOverflow {
    let _ = selected_page;
    let viewport = panel_layout::region_layout(PanelScrollRegion::Preview).content_viewport;
    PanelOverflow::new(
        viewport.width,
        viewport.height,
        viewport.width,
        viewport.height,
    )
}

fn inspector_overflow() -> PanelOverflow {
    let viewport = panel_layout::region_layout(PanelScrollRegion::Inspector).content_viewport;
    PanelOverflow::new(
        viewport.width,
        viewport.height,
        viewport.width + INSPECTOR_EXTRA_SCROLL_X,
        viewport.height + INSPECTOR_EXTRA_SCROLL_Y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_scrollbar_offset_step_stays_moderate_for_viewport_size() {
        let mut offsets = PanelScrollOffsets::default();
        let max_reasonable_step = super::super::render::VIEWPORT_HEIGHT / 16;

        assert!(SCROLL_STEP <= max_reasonable_step);
        assert!(offsets.scroll_delta(PanelScrollRegion::Root, -1.0));
        assert_eq!(SCROLL_STEP, offsets.root_y);
    }

    #[test]
    fn visible_scrollbar_small_overflow_reaches_max_with_existing_step_limit() {
        let mut offsets = PanelScrollOffsets::default();
        let small_max_offset = SCROLL_STEP + SCROLL_STEP / 2;

        assert!(offsets.scroll_delta_with_max(PanelScrollRegion::Root, small_max_offset, -1.0));
        assert_eq!(SCROLL_STEP, offsets.root_y);
        assert!(offsets.scroll_delta_with_max(PanelScrollRegion::Root, small_max_offset, -1.0));
        assert_eq!(small_max_offset, offsets.root_y);
        assert!(!offsets.scroll_delta_with_max(PanelScrollRegion::Root, small_max_offset, -1.0));
        assert_eq!(small_max_offset, offsets.root_y);
    }
}
