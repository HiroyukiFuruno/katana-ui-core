use crate::visual::panel_scroll_state::PanelScrollRegion;
use crate::visual::{layout_metrics::SCROLL_STEP, panel_layout};

pub(crate) fn region_at(x: usize, y: usize) -> PanelScrollRegion {
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

pub(crate) fn next_offset(current: usize, max_scroll_y: usize, delta_y: f32) -> usize {
    if delta_y < 0.0 {
        return (current + SCROLL_STEP).min(max_scroll_y);
    }
    current.saturating_sub(SCROLL_STEP)
}
