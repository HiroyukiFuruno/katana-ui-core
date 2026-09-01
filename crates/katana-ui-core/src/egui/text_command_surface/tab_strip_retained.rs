//! Same-root, proposal-only TabStrip renderer.

use super::tab_strip_projection_lease::{
    TabStripContextMenuPresentation, TabStripGroupDescriptor, TabStripGroupTarget,
    TabStripMenuEntry, TabStripProjection, TabStripProjectionLease, TabStripTabDescriptor,
    TabStripTabTarget, TabStripText,
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

pub(super) const TAB_STRIP_HEIGHT_PX: f32 = 36.0;
const TAB_PADDING_PX: f32 = 10.0;
const TAB_GAP_PX: f32 = 4.0;
const NAVIGATION_CONTROL_WIDTH_PX: f32 = 28.0;
const NAVIGATION_AREA_WIDTH_PX: f32 = (NAVIGATION_CONTROL_WIDTH_PX + TAB_GAP_PX) * 2.0;
const TRAILING_CONTROL_WIDTH_PX: f32 = 24.0;
const ICON_SIZE_PX: u32 = 16;
const OVERLAY_WIDTH_PX: f32 = 232.0;
const OVERLAY_ROW_HEIGHT_PX: f32 = 28.0;
const OVERLAY_PADDING_PX: f32 = 8.0;
const OVERLAY_SEPARATOR_HEIGHT_PX: f32 = 9.0;
const OVERLAY_SWATCH_SIZE_PX: f32 = 18.0;
const CORNER_RADIUS_PX: f32 = 3.0;
const PRIMARY_TEXT_RGBA: [u8; 4] = [218, 218, 218, 255];
const INACTIVE_TAB_RGBA: [u8; 4] = [47, 47, 47, 255];
const DISABLED_TAB_RGBA: [u8; 4] = [40, 40, 40, 255];
const ACTIVE_TAB_RGBA: [u8; 4] = [60, 83, 104, 255];
const OVERLAY_INPUT_RGBA: [u8; 4] = [34, 53, 67, 255];
const SELECTION_RGBA: [u8; 4] = [62, 83, 101, 255];
const PREEDIT_RGBA: [u8; 4] = [121, 192, 255, 255];
const OVERLAY_BACKGROUND_RGBA: [u8; 4] = [43, 43, 43, 255];
const OVERLAY_SEPARATOR_RGBA: [u8; 4] = [82, 82, 82, 255];
const DRAG_GHOST_RGBA: [u8; 4] = [62, 83, 101, 230];
const ICON_ENABLED_RGB: [u8; 3] = [218, 218, 218];
const ICON_DISABLED_RGB: [u8; 3] = [105, 105, 105];
const DROP_LEFT_RATIO: f32 = 0.25;
const DROP_RIGHT_RATIO: f32 = 0.75;
const DRAG_GHOST_OFFSET_PX: f32 = 10.0;
const OVERLAY_SWATCH_INSET_PX: f32 = 4.0;
const OVERLAY_SWATCH_GAP_PX: f32 = 6.0;
const OVERLAY_LABEL_FONT_SIZE_PX: f32 = 14.0;
const OVERLAY_LABEL_FONT_WEIGHT: u16 = 400;
const CHECKMARK_INSET_PX: f32 = 12.0;
const RGBA_ALPHA_INDEX: usize = 3;

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

struct TabStripRenameDraft {
    surface: TextSurface,
    initial: String,
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
        }
    }

    fn value(&self) -> &str {
        &self.surface.state().text_area.value
    }

    fn changed(&self) -> bool {
        self.value() != self.initial
    }
}

struct TabStripLabelInteraction {
    route_path: Option<String>,
}

struct TabStripLabelRenderRequest<'a> {
    text: &'a super::tab_strip_projection_lease::TabStripText,
    path: String,
    x: f32,
    bounds: egui::Rect,
    active: bool,
    active_reveal_pending: &'a mut bool,
    interaction: TabStripLabelInteraction,
    draggable: bool,
}

struct TabStripIconControl<'a> {
    icon: TabStripIcon,
    presentation: &'a super::tab_strip_projection_lease::TabStripControlPresentation,
    enabled: bool,
    path: &'a str,
}

struct TabStripLabelRender {
    advance: f32,
    secondary_clicked: bool,
    drag_started: bool,
    drag_stopped: bool,
    bounds: egui::Rect,
}

struct TabStripDragState {
    source: TabStripTabTarget,
    label: TabStripText,
    pointer: egui::Pos2,
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
    overlay_primary_press: Option<egui::Id>,
    rename_adapter: EguiTextSurfaceAdapter,
    drag: Option<TabStripDragState>,
    drag_release_pending: bool,
    drag_candidates: Vec<TabStripDropCandidate>,
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
            Self::Port(error) => write!(formatter, "tab proposal forwarding failed: {error:?}"),
        }
    }
}

impl std::error::Error for TabStripRetainedError {}

mod interaction;
mod label_paint;
mod overlay;
mod overlay_paint;
mod overlay_panel;
mod paint;
mod retained_state;
mod support;

#[cfg(test)]
#[path = "tab_strip_retained/tab_strip_retained_tests.rs"]
mod tests;
