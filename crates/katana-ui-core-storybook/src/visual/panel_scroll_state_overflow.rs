use super::{PanelOverflow, PanelScrollRegion};
use crate::visual::navigation_tree::TreeExpansionState;
use crate::visual::{panel_layout, render};

const INSPECTOR_EXTRA_SCROLL_Y: usize = 360;

pub(crate) struct PanelScrollOverflowModel;

impl PanelScrollOverflowModel {
    pub(crate) fn max_scroll_y_for(
        region: PanelScrollRegion,
        selected_page: &str,
        tree_expansion: TreeExpansionState,
    ) -> usize {
        Self::overflow_for(region, selected_page, tree_expansion).max_y()
    }

    pub(crate) fn max_scroll_x_for(
        region: PanelScrollRegion,
        selected_page: &str,
        tree_expansion: TreeExpansionState,
    ) -> usize {
        Self::overflow_for(region, selected_page, tree_expansion).max_x()
    }

    pub(crate) fn overflow_for(
        region: PanelScrollRegion,
        selected_page: &str,
        expansion: TreeExpansionState,
    ) -> PanelOverflow {
        match region {
            PanelScrollRegion::Root => PanelOverflow::new(
                render::WIDTH,
                render::VIEWPORT_HEIGHT,
                render::WIDTH,
                render::CANVAS_HEIGHT,
            ),
            PanelScrollRegion::Navigation => navigation_overflow(expansion),
            PanelScrollRegion::Preview => preview_overflow(selected_page),
            PanelScrollRegion::Inspector => inspector_overflow(),
        }
    }
}

fn navigation_overflow(expansion: TreeExpansionState) -> PanelOverflow {
    let viewport = panel_layout::region_layout(PanelScrollRegion::Navigation).content_viewport;
    PanelOverflow::new(
        viewport.width,
        viewport.height,
        viewport.width,
        viewport.height + crate::visual::navigation_tree::max_scroll_y(expansion),
    )
}

fn preview_overflow(selected_page: &str) -> PanelOverflow {
    let viewport = panel_layout::region_layout(PanelScrollRegion::Preview).content_viewport;
    if selected_page == "panel" {
        return preview_content_overflow(
            viewport.width,
            viewport.height,
            viewport.width,
            viewport.height,
        );
    }
    preview_content_overflow(
        viewport.width,
        viewport.height,
        viewport.width,
        viewport.height,
    )
}

fn preview_content_overflow(
    viewport_width: usize,
    viewport_height: usize,
    content_width: usize,
    content_height: usize,
) -> PanelOverflow {
    PanelOverflow::new(
        viewport_width,
        viewport_height,
        content_width,
        content_height,
    )
}

fn inspector_overflow() -> PanelOverflow {
    let viewport = panel_layout::region_layout(PanelScrollRegion::Inspector).content_viewport;
    PanelOverflow::new(
        viewport.width,
        viewport.height,
        viewport.width,
        viewport.height + INSPECTOR_EXTRA_SCROLL_Y,
    )
}
