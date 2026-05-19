use super::{StorybookInteractionSpec, spec};

pub(super) fn for_page(page: &str) -> Option<StorybookInteractionSpec> {
    match page {
        "collapsible-panel" => Some(spec(
            "collapsible_panel_resize",
            "collapsible_panel_width_changed",
            "panel.width",
            "320",
            "mode=floating_overlay",
        )),
        "virtualization" => Some(spec(
            "virtualized_scroll",
            "virtual_range_changed",
            "viewport.offset",
            "1260",
            "rows=visible",
        )),
        "skeleton-cluster" => Some(spec(
            "skeleton_cluster_preset_apply",
            "skeleton_cluster_changed",
            "skeleton_cluster.preset",
            "Card",
            "items=3",
        )),
        "motion" => Some(spec(
            "motion_reduce",
            "motion_snapshot_changed",
            "motion.reduced_motion",
            "true",
            "instant=true",
        )),
        "window-control-button-group" => Some(spec(
            "window_control_press",
            "window_control_pressed",
            "window_controls.position",
            "Leading",
            "pressed=Close",
        )),
        "startup-state-panel" => Some(spec(
            "startup_state_error",
            "startup_state_changed",
            "startup.state",
            "Error",
            "retry=true",
        )),
        _ => None,
    }
}
