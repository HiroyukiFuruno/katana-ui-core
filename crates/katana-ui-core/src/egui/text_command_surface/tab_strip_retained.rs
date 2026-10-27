//! Same-root, proposal-only TabStrip renderer.

use super::tab_strip_projection_lease::{
    TabStripContextMenuPresentation, TabStripGroupDescriptor, TabStripGroupTarget,
    TabStripMenuEntry, TabStripProjection, TabStripProjectionLease, TabStripRootItem,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use super::tab_strip_proposal_port::{
    TabStripProposal, TabStripProposalOperation, TabStripProposalPortError,
    TabStripProposalPortHandle, TabStripTabPlacement,
};
use super::tab_strip_route_table::TabStripRouteTable;
use super::tab_strip_text_raster::TabStripTextRasterizer;
use super::{tab_strip_projection_lease, tab_strip_proposal_port};
use crate::atom::{
    TextArea, TextAreaEvent, TextAreaNewlineKey, TextAreaSubmitKey, TextAreaWrapPolicy,
};
use crate::egui::tab_strip_paint::{
    TabStripPaintOperation, TabStripPaintOperationKind, TabStripPaintPlan, TabStripPaintTexture,
};
use crate::egui::text_command_surface::accesskit_evidence::publish_labeled_button_accesskit;
use crate::egui::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceError, EguiTextSurfaceInputPolicy,
    TextSurfacePaintOperationKind, TextSurfacePaintStyle, TextSurfaceRasterStyle,
};
use crate::egui::texture_cache::{DEFAULT_TEXTURE_CACHE_CAPACITY, RgbaTextureCache};
use crate::molecule::RgbaColor;
use crate::molecule::tab_strip_icon_catalog::TabStripIcon;
use crate::render_model::UiRect;
use crate::svg_raster::{UiSvgRasterConfig, UiSvgRasterError, UiSvgRasterRequest, UiSvgRasterizer};
use crate::text_raster::{
    PlatformFontCatalog, PlatformTextMetricsFrame, PlatformTextRasterConfig,
    PlatformTextRasterError,
};
use crate::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceEvent, TextSurfaceFocusRequest,
    TextSurfaceFocusRequestToken, TextSurfaceProps, TextSurfaceViewport,
};
use crate::theme::{FontFamily, FontToken};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use label_layout::{TabStripLabelInteraction, TabStripLabelRender, TabStripLabelRenderRequest};

pub(super) use constants::TAB_STRIP_HEIGHT_PX;
use constants::{
    ACTIVE_TAB_RGBA, CHECKMARK_INSET_PX, CORNER_RADIUS_PX, DISABLED_TAB_RGBA, DRAG_GHOST_OFFSET_PX,
    DRAG_GHOST_RGBA, DROP_LEFT_RATIO, DROP_RIGHT_RATIO, ICON_DISABLED_RGB, ICON_ENABLED_RGB,
    ICON_SIZE_PX, INACTIVE_TAB_RGBA, NAVIGATION_AREA_WIDTH_PX, NAVIGATION_CONTROL_WIDTH_PX,
    OVERLAY_BACKGROUND_RGBA, OVERLAY_INPUT_RGBA, OVERLAY_LABEL_FONT_SIZE_PX,
    OVERLAY_LABEL_FONT_WEIGHT, OVERLAY_PADDING_PX, OVERLAY_ROW_HEIGHT_PX,
    OVERLAY_SEPARATOR_HEIGHT_PX, OVERLAY_SEPARATOR_RGBA, OVERLAY_SWATCH_GAP_PX,
    OVERLAY_SWATCH_INSET_PX, OVERLAY_SWATCH_SIZE_PX, OVERLAY_WIDTH_PX, PREEDIT_RGBA,
    PRIMARY_TEXT_RGBA, RGBA_ALPHA_INDEX, SELECTION_RGBA, TAB_GAP_PX, TAB_PADDING_PX,
    TRAILING_CONTROL_WIDTH_PX,
};

#[derive(Clone)]
enum TabStripOverlayState {
    Closed,
    TabMenu {
        path: String,
        anchor: egui::Pos2,
        submenu_path: Vec<usize>,
    },
    GroupPopup {
        path: String,
        anchor: egui::Pos2,
        submenu_path: Vec<usize>,
        rename: Option<Box<TabStripRenameDraft>>,
    },
}

#[derive(Clone)]
struct TabStripRenameDraft {
    surface: TextSurface,
    initial: String,
    requested_start: bool,
}

impl TabStripRenameDraft {
    fn new(initial: &str, placeholder: &str) -> Self {
        let text_area = TextArea::new("kuc.overlay.single-line-input")
            .stable_state_id("kuc.overlay.single-line-input")
            .value(initial)
            .placeholder(placeholder)
            .min_rows(1)
            .max_rows(1)
            .auto_grow(false)
            .wrap_policy(TextAreaWrapPolicy::None)
            .submit_key(TextAreaSubmitKey::Enter)
            .newline_key(TextAreaNewlineKey::Disabled)
            .ime_enabled(true);
        let mut props = TextSurfaceProps::new(
            text_area,
            Vec::new(),
            TextSurfaceViewport::new(
                0,
                0,
                (OVERLAY_WIDTH_PX - OVERLAY_PADDING_PX * 2.0) as u32,
                OVERLAY_ROW_HEIGHT_PX as u32,
            ),
        );
        props.accessibility_label = placeholder.to_owned();
        props.focus_request = Some(TextSurfaceFocusRequest::new(
            TextSurfaceFocusRequestToken::new("kuc.overlay.single-line-input.focus"),
            true,
        ));
        let mut surface = TextSurface::new(props);
        let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
        Self {
            surface,
            initial: initial.to_owned(),
            requested_start: false,
        }
    }

    fn value(&self) -> &str {
        &self.surface.state().text_area.value
    }

    fn changed(&self) -> bool {
        self.value() != self.initial
    }
}

struct TabStripPendingRenameStart {
    target: TabStripGroupTarget,
    correlation: tab_strip_projection_lease::TabStripCorrelation,
    port: TabStripProposalPortHandle,
    commit: pass_commit::TabStripPassCommit,
}

struct TabStripPendingRenameChanges {
    target: TabStripGroupTarget,
    correlation: tab_strip_projection_lease::TabStripCorrelation,
    port: TabStripProposalPortHandle,
    commit: pass_commit::TabStripPassCommit,
    names: Vec<TabStripText>,
    before: Option<TabStripOverlayState>,
}

struct TabStripIconControl<'a> {
    icon: TabStripIcon,
    presentation: &'a super::tab_strip_projection_lease::TabStripControlPresentation,
    enabled: bool,
    path: &'a str,
}

struct TabStripDragState {
    source: TabStripTabTarget,
    origin: Option<drag_origin::TabStripDragOrigin>,
    label: TabStripText,
    pointer: egui::Pos2,
    center_offset_x: f32,
    ghost_width: f32,
}

struct TabStripGroupDragState {
    source: TabStripGroupTarget,
    label: TabStripText,
    pointer: egui::Pos2,
}

struct TabStripResolvedGroupDrop {
    destination: super::tab_strip_proposal_port::TabStripGroupPlacement,
    indicator: egui::Rect,
}

enum TabStripDropCandidateKind {
    Tab(TabStripTabTarget),
    Group(TabStripGroupTarget),
}

struct TabStripDropCandidate {
    bounds: egui::Rect,
    kind: TabStripDropCandidateKind,
}

struct TabStripResolvedDrop {
    destination: TabStripTabPlacement,
    indicator: egui::Rect,
}

struct TabStripTrailingControl<'a> {
    tab: &'a TabStripTabDescriptor,
    presentation: &'a super::tab_strip_projection_lease::TabStripControlPresentation,
    path: String,
}

pub(crate) struct TabStripRetainedState {
    projection: TabStripProjection,
    port: Option<TabStripProposalPortHandle>,
    rasterizer: TabStripTextRasterizer,
    svg_rasterizer: UiSvgRasterizer,
    textures: RgbaTextureCache,
    routes: TabStripRouteTable,
    active_reveal_pending: bool,
    horizontal_scroll_offset: f32,
    next_nonce: u64,
    overlay: TabStripOverlayState,
    overlay_input: overlay_input::TabStripOverlayInput,
    rename_adapter: EguiTextSurfaceAdapter,
    drag: Option<TabStripDragState>,
    group_drag: Option<TabStripGroupDragState>,
    drag_release_pending: bool,
    group_drag_release_pending: bool,
    drag_candidates: Vec<TabStripDropCandidate>,
    tab_viewport: Option<egui::Rect>,
    group_drag_candidates: Vec<TabStripDropCandidate>,
    inline_rename_consumed: Vec<TabStripGroupTarget>,
    member_color: Option<RgbaColor>,
    pending_rename_start: Option<TabStripPendingRenameStart>,
    pending_rename_changes: Option<TabStripPendingRenameChanges>,
}

#[derive(Debug)]
pub(crate) struct TabStripRootOutput {
    pub(crate) paint_plan: TabStripPaintPlan,
    pub(crate) overlay_paint_plan: Option<TabStripPaintPlan>,
    horizontal_scroll_offset: f32,
}

#[derive(Debug)]
pub(crate) enum TabStripRetainedError {
    Raster(PlatformTextRasterError),
    Svg(UiSvgRasterError),
    TextSurface(EguiTextSurfaceError),
    MissingPort,
    MissingRoute,
    MissingOverlayBounds,
    InvalidInlineRenameRequest,
    Port(TabStripProposalPortError),
}

impl std::fmt::Display for TabStripRetainedError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raster(error) => write!(formatter, "tab label rasterization failed: {error}"),
            Self::Svg(error) => write!(formatter, "tab icon rasterization failed: {error:?}"),
            Self::TextSurface(error) => write!(formatter, "tab rename input failed: {error}"),
            Self::MissingPort => formatter.write_str("tab operation requires a proposal port"),
            Self::MissingRoute => formatter.write_str("tab interaction route is unavailable"),
            Self::MissingOverlayBounds => {
                formatter.write_str("tab overlay did not produce combined bounds")
            }
            Self::InvalidInlineRenameRequest => {
                formatter.write_str("inline rename requires a renamable group and placeholder")
            }
            Self::Port(error) => write!(formatter, "tab proposal forwarding failed: {error:?}"),
        }
    }
}

impl std::error::Error for TabStripRetainedError {}

mod constants;
mod drag_feedback;
mod drag_origin;
mod group_drag;
mod group_popup;
mod interaction;
mod label_layout;
mod label_paint;
mod lease_state;
mod nearest_drop;
mod overlay;
mod overlay_input;
mod overlay_paint;
mod overlay_panel;
mod overlay_tree;
mod paint;
mod pass_commit;
mod rename_delivery;
mod retained_state;
mod support;
mod tab_paint;

#[cfg(test)]
#[path = "tab_strip_retained/tab_strip_retained_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "tab_strip_retained/coverage_gap_tests.rs"]
mod coverage_gap_tests;
