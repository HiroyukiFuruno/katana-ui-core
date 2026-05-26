#[path = "panel_scroll_state_overflow.rs"]
mod panel_scroll_state_overflow;
pub(crate) use self::panel_scroll_state_overflow::PanelScrollOverflowModel;
#[path = "panel_scroll_state_region.rs"]
mod panel_scroll_state_region;
pub(crate) use self::panel_scroll_state_region::PanelScrollRegionModel;

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
        self.scroll_delta_with_max(
            region,
            PanelScrollOverflowModel::max_scroll_y_for(region, "", Default::default()),
            delta_y,
        )
    }

    pub(super) fn scroll_delta_with_max(
        &mut self,
        region: PanelScrollRegion,
        max_offset: usize,
        delta_y: f32,
    ) -> bool {
        let before = self.offset(region);
        let next = PanelScrollRegionModel::next_offset(before, max_offset, delta_y);
        self.set_offset(region, next);
        before != next
    }

    #[cfg(test)]
    pub(super) fn scroll_delta_x(&mut self, region: PanelScrollRegion, delta_x: f32) -> bool {
        let before = self.offset_x(region);
        let next = PanelScrollRegionModel::next_offset(
            before,
            PanelScrollOverflowModel::max_scroll_x_for(region, "", Default::default()),
            delta_x,
        );
        self.set_offset_x(region, next);
        before != next
    }

    pub(super) fn scroll_delta_x_with_max(
        &mut self,
        region: PanelScrollRegion,
        max_offset: usize,
        delta_x: f32,
    ) -> bool {
        let before = self.offset_x(region);
        let next = PanelScrollRegionModel::next_offset(before, max_offset, delta_x);
        self.set_offset_x(region, next);
        before != next
    }

    #[cfg(test)]
    pub(super) fn set_drag_offset(&mut self, region: PanelScrollRegion, value: usize) -> bool {
        self.set_drag_offset_with_max(
            region,
            value,
            PanelScrollOverflowModel::max_scroll_y_for(region, "", Default::default()),
        )
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

    #[cfg(test)]
    pub(super) fn set_drag_offset_x(&mut self, region: PanelScrollRegion, value: usize) -> bool {
        let before = self.offset_x(region);
        self.set_offset_x(
            region,
            value.min(PanelScrollOverflowModel::max_scroll_x_for(
                region,
                "",
                Default::default(),
            )),
        );
        before != self.offset_x(region)
    }

    pub(super) fn set_drag_offset_x_with_max(
        &mut self,
        region: PanelScrollRegion,
        value: usize,
        max_offset: usize,
    ) -> bool {
        let before = self.offset_x(region);
        self.set_offset_x(region, value.min(max_offset));
        before != self.offset_x(region)
    }

    pub(super) fn offset_with_max(self, region: PanelScrollRegion, max_offset: usize) -> usize {
        self.offset(region).min(max_offset)
    }

    pub(super) fn offset_x_with_max(self, region: PanelScrollRegion, max_offset: usize) -> usize {
        self.offset_x(region).min(max_offset)
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
