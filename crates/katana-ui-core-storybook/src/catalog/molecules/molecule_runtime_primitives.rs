use super::super::{StoryCatalog, StoryExample};
mod collapsible_panel_story;
mod motion_story;
mod skeleton_cluster_story;
mod virtualization_story;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{
    MotionContext, MotionDisableContext, MotionDistanceToken, MotionDurationToken,
    MotionEasingToken, MotionResolver, ScaleOrigin, ShimmerDirection, ShimmerSpeed, SlideDirection,
    UiAction, UiCallbackLog,
};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core::{atom, layout, molecule};
use molecule::{
    CollapsiblePanelAction, CollapsiblePanelWidth, MotionDefaults, MotionSpec, MotionTarget,
    PanelMode, PanelSide, ReducedMotionPolicy, RowHeightProvider, SkeletonClusterPreset,
    VirtualizationConfig,
};

const PANEL_MIN_WIDTH: u16 = 180;
const PANEL_MAX_WIDTH: u16 = 360;
const PANEL_DEFAULT_WIDTH: u16 = 240;
const PANEL_RESIZED_WIDTH: u16 = 320;
const VIRTUAL_TOTAL_ROWS: usize = 10_000;
const VIRTUAL_ROW_HEIGHT: u32 = 28;
const VIRTUAL_VIEWPORT_HEIGHT: u32 = 168;
const VIRTUAL_SCROLL_OFFSET: u32 = 1_260;
const VIRTUAL_FOCUSED_INDEX: usize = 120;
const MOTION_PHASE: u16 = 2;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        collapsible_panel_story::collapsible_panel_story(),
        virtualization_story::virtualization_story(),
        skeleton_cluster_story::skeleton_cluster_story(),
        motion_story::motion_story(),
    ]
}
