//! KUC-owned public adapter types for command-surface composition.

use crate::command_chrome::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeAdapter,
    EguiCommandChromeFloatingOutput, EguiCommandChromeOutput, EguiCommandChromeSearchOutput,
    EguiCommandChromeSearchStyle,
};
use crate::context_menu::{
    ContextMenuPaintStyle, ContextMenuPresentation, ContextMenuRasterStyle, EguiContextMenuAdapter,
    EguiContextMenuOutput,
};
use crate::diagnostics_list::EguiDiagnosticsListAdapter;
use crate::status_bar::EguiStatusBarAdapter;
use crate::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceError, EguiTextSurfaceOutput,
    TextSurfaceContextTargetAnchor, TextSurfacePaintStyle, TextSurfaceRasterStyle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeFamilyId, CommandChromeSearchPresentation, CommandChromeSearchStrip,
    CommandChromeToolbar, CommandChromeToolbarPresentation, FloatingCommandToolbar,
    FloatingCommandToolbarVisibility,
};
use katana_ui_core::molecule::structured::source_address_strip::SourceAddressStrip;
use katana_ui_core::render_model::{UiRect, UiStateId};
use katana_ui_core::text_surface::{TextSurface, TextSurfacePresentation};
use katana_ui_core_svg_raster::UiSvgRasterConfig;
use katana_ui_core_text_raster::PlatformFontCatalog;
use katana_ui_core_text_raster::PlatformTextRasterConfig;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

mod error;
pub use error::EguiTextCommandSurfaceError;

/// Consumer-safe retained state for one generic text-command surface.
pub struct EguiTextCommandSurface {
    pub(crate) text: TextSurface,
    pub(crate) toolbar: Option<CommandChromeToolbar>,
    pub(crate) floating: Option<FloatingCommandToolbar>,
    pub(crate) deferred_floating_toolbar: Option<CommandChromeToolbar>,
    pub(crate) floating_visibility: FloatingCommandToolbarVisibility,
    pub(crate) floating_visibility_controlled: bool,
    pub(crate) search: Option<CommandChromeSearchStrip>,
    pub(crate) search_closed_by_interaction: bool,
    pub(crate) context_menu: Option<ContextMenuPresentation>,
    pub(crate) primary_command_family: Option<CommandChromeFamilyId>,
    pub(crate) floating_command_family: Option<CommandChromeFamilyId>,
    pub(crate) source_address: Option<SourceAddressStrip>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiTextCommandSurfaceFloatingPresentation {
    pub toolbar: CommandChromeToolbarPresentation,
    pub visibility: FloatingCommandToolbarVisibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EguiTextCommandSurfaceSearchPresentation {
    pub state_id: UiStateId,
    pub label: String,
    pub value: CommandChromeSearchPresentation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EguiTextCommandSurfacePresentation {
    pub text_state_id: Option<UiStateId>,
    pub text: TextSurfacePresentation,
    pub toolbar: Option<CommandChromeToolbarPresentation>,
    pub floating: Option<EguiTextCommandSurfaceFloatingPresentation>,
    pub search: Option<EguiTextCommandSurfaceSearchPresentation>,
    pub context_menu: Option<ContextMenuPresentation>,
}

pub struct EguiTextCommandSurfaceAdapter {
    pub(crate) catalog: Arc<PlatformFontCatalog>,
    pub(crate) text_raster_config: PlatformTextRasterConfig,
    pub(crate) text: EguiTextSurfaceAdapter,
    pub(crate) chrome: EguiCommandChromeAdapter,
    pub(crate) floating_selection: Option<(usize, usize)>,
    pub(crate) closed_selection: Option<(usize, usize)>,
    pub(crate) context_menu: Option<EguiContextMenuAdapter>,
    pub(crate) context_target: Option<TextSurfaceContextTargetAnchor>,
    pub(crate) metrics: crate::text_surface::SharedTextMetrics,
    pub(crate) source_address: crate::source_address_strip::EguiSourceAddressStripAdapter,
    pub(crate) status_bar: EguiStatusBarAdapter,
    pub(crate) diagnostics_list: EguiDiagnosticsListAdapter,
}

impl EguiTextCommandSurfaceAdapter {
    pub fn with_text_raster_config(
        config: PlatformTextRasterConfig,
    ) -> Result<Self, EguiTextCommandSurfaceError> {
        let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
        let metrics = Rc::new(RefCell::new(
            katana_ui_core_text_raster::PlatformTextMetricsFrame::new(),
        ));
        Ok(Self {
            text: EguiTextSurfaceAdapter::with_catalog_and_metrics(
                Arc::clone(&catalog), config.clone(), Rc::clone(&metrics),
            )
            .map_err(EguiTextSurfaceError::from)?,
            chrome: EguiCommandChromeAdapter::with_catalog_and_metrics(
                Arc::clone(&catalog),
                config.clone(),
                UiSvgRasterConfig::default(),
                Rc::clone(&metrics),
            )?,
            catalog: Arc::clone(&catalog),
            text_raster_config: config.clone(),
            floating_selection: None,
            closed_selection: None,
            context_menu: None,
            context_target: None,
            metrics: Rc::clone(&metrics),
            source_address:
                crate::source_address_strip::EguiSourceAddressStripAdapter::with_catalog_and_metrics(
                    "source-address", Arc::clone(&catalog), config, Rc::clone(&metrics),
                )?,
            status_bar: EguiStatusBarAdapter::new("root-status-bar")?,
            diagnostics_list: EguiDiagnosticsListAdapter::new("root-diagnostics-list")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextCommandSurfaceStyle {
    pub text_raster: TextSurfaceRasterStyle,
    pub text_paint: TextSurfacePaintStyle,
    pub chrome_raster: CommandChromeRasterStyle,
    pub chrome_paint: CommandChromePaintStyle,
    pub search: EguiCommandChromeSearchStyle,
}

impl TextCommandSurfaceStyle {
    pub fn standard() -> Result<Self, EguiTextCommandSurfaceError> {
        Self::from_theme(&katana_ui_core::theme::ThemeSnapshot::dark())
    }

    pub fn from_theme(
        theme: &katana_ui_core::theme::ThemeSnapshot,
    ) -> Result<Self, EguiTextCommandSurfaceError> {
        super::text_command_surface_style_factory::from_theme(theme)
    }

    #[must_use]
    pub fn context_menu_raster_style(&self) -> ContextMenuRasterStyle {
        let icon = self.chrome_raster.icon_color;
        ContextMenuRasterStyle {
            font: self.chrome_raster.font.clone(),
            text_color_rgba: self.chrome_raster.text_color_rgba,
            icon_color_rgba: [icon.red, icon.green, icon.blue, icon.alpha],
            line_height_px: self.chrome_raster.line_height_px,
        }
    }

    #[must_use]
    pub const fn context_menu_paint_style(&self) -> ContextMenuPaintStyle {
        ContextMenuPaintStyle {
            background_rgba: self.chrome_paint.action_rgba,
            highlighted_rgba: self.chrome_paint.hovered_action_rgba,
            disabled_rgba: self.chrome_paint.disabled_action_rgba,
        }
    }
}

#[derive(Debug)]
pub struct EguiTextCommandSurfaceOutput {
    pub root_bounds: UiRect,
    pub text: EguiTextSurfaceOutput,
    pub toolbar: Option<EguiCommandChromeOutput>,
    pub floating: Option<EguiCommandChromeFloatingOutput>,
    pub search: Option<EguiCommandChromeSearchOutput>,
    pub context_menu: Option<EguiContextMenuOutput>,
    pub(crate) source_address: Option<SourceAddressRootOutput>,
    pub(crate) tab_strip: Option<super::tab_strip_retained::TabStripRootOutput>,
    pub(crate) status_bar: Option<crate::status_bar::EguiStatusBarOutput>,
    pub(crate) diagnostics_list: Option<crate::diagnostics_list::EguiDiagnosticsListOutput>,
    pub(crate) accesskit_evidence: Vec<super::accesskit_evidence::AccessKitEvidence>,
    artifact_order: Vec<EguiTextCommandSurfaceChild>,
}

pub(super) struct RootChildOutputs {
    pub toolbar: Option<EguiCommandChromeOutput>,
    pub floating: Option<EguiCommandChromeFloatingOutput>,
    pub search: Option<EguiCommandChromeSearchOutput>,
    pub context_menu: Option<EguiContextMenuOutput>,
    pub source_address: Option<SourceAddressRootOutput>,
    pub accesskit_evidence: Vec<super::accesskit_evidence::AccessKitEvidence>,
    pub artifact_order: Vec<EguiTextCommandSurfaceChild>,
    pub status_bar: Option<crate::status_bar::EguiStatusBarOutput>,
    pub diagnostics_list: Option<crate::diagnostics_list::EguiDiagnosticsListOutput>,
}

impl EguiTextCommandSurfaceOutput {
    pub(super) fn from_root(
        root_bounds: UiRect,
        text: EguiTextSurfaceOutput,
        children: RootChildOutputs,
    ) -> Self {
        Self {
            root_bounds,
            text,
            toolbar: children.toolbar,
            floating: children.floating,
            search: children.search,
            context_menu: children.context_menu,
            source_address: children.source_address,
            tab_strip: None,
            status_bar: children.status_bar,
            diagnostics_list: children.diagnostics_list,
            accesskit_evidence: children.accesskit_evidence,
            artifact_order: children.artifact_order,
        }
    }

    pub(super) fn with_tab_strip(
        mut self,
        tab_strip: Option<super::tab_strip_retained::TabStripRootOutput>,
    ) -> Self {
        self.tab_strip = tab_strip;
        self
    }

    #[must_use]
    pub fn artifact_order(&self) -> &[EguiTextCommandSurfaceChild] {
        &self.artifact_order
    }
}

#[derive(Debug)]
pub(crate) struct SourceAddressRootOutput {
    pub output: crate::source_address_strip::EguiSourceAddressStripOutput,
    pub paint_plan: crate::source_address_strip::SourceAddressPaintPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceChild {
    TabStrip,
    TabStripOverlay,
    SourceAddress,
    Text,
    Toolbar,
    Search,
    Floating,
    ContextMenu,
    StatusBar,
    DiagnosticsList,
}
