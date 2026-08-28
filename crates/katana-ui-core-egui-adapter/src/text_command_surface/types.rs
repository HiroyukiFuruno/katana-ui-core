//! KUC-owned public adapter types for command-surface composition.

use crate::command_chrome::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeAdapter,
    EguiCommandChromeError, EguiCommandChromeFloatingOutput, EguiCommandChromeOutput,
    EguiCommandChromeSearchOutput, EguiCommandChromeSearchStyle,
};
use crate::context_menu::{
    ContextMenuAdapterError, ContextMenuPaintStyle, ContextMenuPresentation,
    ContextMenuRasterStyle, EguiContextMenuAdapter, EguiContextMenuOutput,
};
use crate::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceError, EguiTextSurfaceOutput,
    TextSurfaceContextTargetAnchor, TextSurfacePaintStyle, TextSurfaceRasterStyle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeFamilyId, CommandChromeSearchPresentation, CommandChromeSearchStrip,
    CommandChromeToolbar, CommandChromeToolbarPresentation, FloatingCommandToolbar,
    FloatingCommandToolbarVisibility,
};
use katana_ui_core::render_model::{UiRect, UiStateId};
use katana_ui_core::text_surface::{TextSurface, TextSurfacePresentation};
use katana_ui_core_svg_raster::UiSvgRasterConfig;
use katana_ui_core_text_raster::PlatformFontCatalog;
use katana_ui_core_text_raster::PlatformTextRasterConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Consumer-safe retained state for one generic text-command surface.
///
/// The consumer owns this one model but cannot supply geometry, a previous frame, or
/// independently borrowed child models to the root adapter.
pub struct EguiTextCommandSurface {
    pub(crate) text: TextSurface,
    pub(crate) toolbar: Option<CommandChromeToolbar>,
    pub(crate) floating: Option<FloatingCommandToolbar>,
    pub(crate) deferred_floating_toolbar: Option<CommandChromeToolbar>,
    pub(crate) floating_visibility: FloatingCommandToolbarVisibility,
    pub(crate) floating_visibility_controlled: bool,
    pub(crate) search: Option<CommandChromeSearchStrip>,
    pub(crate) context_menu: Option<ContextMenuPresentation>,
}

/// Generic controlled presentation for an optional floating command surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiTextCommandSurfaceFloatingPresentation {
    pub toolbar: CommandChromeToolbarPresentation,
    pub visibility: FloatingCommandToolbarVisibility,
}

/// Generic controlled presentation that creates or retains a search-strip identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EguiTextCommandSurfaceSearchPresentation {
    pub state_id: UiStateId,
    pub label: String,
    pub value: CommandChromeSearchPresentation,
}

/// Consumer-owned values synchronized into one retained KUC text-command surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EguiTextCommandSurfacePresentation {
    pub text_state_id: Option<UiStateId>,
    pub text: TextSurfacePresentation,
    pub toolbar: Option<CommandChromeToolbarPresentation>,
    pub floating: Option<EguiTextCommandSurfaceFloatingPresentation>,
    pub search: Option<EguiTextCommandSurfaceSearchPresentation>,
    pub context_menu: Option<ContextMenuPresentation>,
}

/// Retained owner of generic root-space allocation and child adapter state.
pub struct EguiTextCommandSurfaceAdapter {
    pub(crate) catalog: Arc<PlatformFontCatalog>,
    pub(crate) text_raster_config: PlatformTextRasterConfig,
    pub(crate) text: EguiTextSurfaceAdapter,
    pub(crate) chrome: EguiCommandChromeAdapter,
    pub(crate) floating_selection: Option<(usize, usize)>,
    pub(crate) closed_selection: Option<(usize, usize)>,
    pub(crate) context_menu: Option<EguiContextMenuAdapter>,
    pub(crate) context_target: Option<TextSurfaceContextTargetAnchor>,
}

impl EguiTextCommandSurfaceAdapter {
    #[must_use]
    pub(crate) fn with_text_raster_config(config: PlatformTextRasterConfig) -> Self {
        let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
        Self {
            text: EguiTextSurfaceAdapter::with_catalog(Arc::clone(&catalog), config.clone()),
            chrome: EguiCommandChromeAdapter::with_catalog(
                Arc::clone(&catalog),
                config.clone(),
                UiSvgRasterConfig::default(),
            ),
            catalog,
            text_raster_config: config,
            floating_selection: None,
            closed_selection: None,
            context_menu: None,
            context_target: None,
        }
    }
}

impl Default for EguiTextCommandSurfaceAdapter {
    fn default() -> Self {
        Self::with_text_raster_config(PlatformTextRasterConfig::default())
    }
}

/// Rendering styles supplied as KUC presentation data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextCommandSurfaceStyle {
    pub text_raster: TextSurfaceRasterStyle,
    pub text_paint: TextSurfacePaintStyle,
    pub chrome_raster: CommandChromeRasterStyle,
    pub chrome_paint: CommandChromePaintStyle,
    pub search: EguiCommandChromeSearchStyle,
}

impl TextCommandSurfaceStyle {
    /// Builds the standard KUC text-command surface style from the standard theme.
    #[must_use]
    pub fn standard() -> Self {
        Self::from_theme(&katana_ui_core::theme::ThemeSnapshot::dark())
    }

    /// Builds a text-command surface style from generic KUC theme tokens.
    #[must_use]
    pub fn from_theme(theme: &katana_ui_core::theme::ThemeSnapshot) -> Self {
        super::text_command_surface_style_factory::from_theme(theme)
    }

    /// Reuses the generic command-chrome theme for the contextual overlay.
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

    /// Reuses generic command-chrome panel tokens for ContextMenu painting.
    #[must_use]
    pub const fn context_menu_paint_style(&self) -> ContextMenuPaintStyle {
        ContextMenuPaintStyle {
            background_rgba: self.chrome_paint.action_rgba,
            highlighted_rgba: self.chrome_paint.hovered_action_rgba,
            disabled_rgba: self.chrome_paint.disabled_action_rgba,
        }
    }
}

/// Immutable records from a complete KUC text-command frame.
#[derive(Debug)]
pub struct EguiTextCommandSurfaceOutput {
    pub root_bounds: UiRect,
    pub text: EguiTextSurfaceOutput,
    pub toolbar: Option<EguiCommandChromeOutput>,
    pub floating: Option<EguiCommandChromeFloatingOutput>,
    pub search: Option<EguiCommandChromeSearchOutput>,
    pub context_menu: Option<EguiContextMenuOutput>,
    pub(crate) accesskit_evidence: Vec<super::accesskit_evidence::AccessKitEvidence>,
    artifact_order: Vec<EguiTextCommandSurfaceChild>,
}

impl EguiTextCommandSurfaceOutput {
    pub(super) fn from_root(
        root_bounds: UiRect,
        text: EguiTextSurfaceOutput,
        toolbar: Option<EguiCommandChromeOutput>,
        floating: Option<EguiCommandChromeFloatingOutput>,
        search: Option<EguiCommandChromeSearchOutput>,
        context_menu: Option<EguiContextMenuOutput>,
        accesskit_evidence: Vec<super::accesskit_evidence::AccessKitEvidence>,
        artifact_order: Vec<EguiTextCommandSurfaceChild>,
    ) -> Self {
        Self {
            root_bounds,
            text,
            toolbar,
            floating,
            search,
            context_menu,
            accesskit_evidence,
            artifact_order,
        }
    }

    /// Returns the KUC-owned artifact layer order for this root frame.
    #[must_use]
    pub fn artifact_order(&self) -> &[EguiTextCommandSurfaceChild] {
        &self.artifact_order
    }
}

/// KUC-owned artifact layer kinds in root paint order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceChild {
    Text,
    Toolbar,
    Search,
    Floating,
    ContextMenu,
}

/// Errors produced while composing retained KUC child adapters.
#[derive(Debug)]
pub enum EguiTextCommandSurfaceError {
    DuplicateCommandFamilyMount { family: CommandChromeFamilyId },
    Text(EguiTextSurfaceError),
    Chrome(EguiCommandChromeError),
    ContextMenu(ContextMenuAdapterError),
}

impl From<EguiTextSurfaceError> for EguiTextCommandSurfaceError {
    fn from(value: EguiTextSurfaceError) -> Self {
        Self::Text(value)
    }
}

impl From<EguiCommandChromeError> for EguiTextCommandSurfaceError {
    fn from(value: EguiCommandChromeError) -> Self {
        Self::Chrome(value)
    }
}

impl From<ContextMenuAdapterError> for EguiTextCommandSurfaceError {
    fn from(value: ContextMenuAdapterError) -> Self {
        Self::ContextMenu(value)
    }
}

impl std::fmt::Display for EguiTextCommandSurfaceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateCommandFamilyMount { family } => write!(
                formatter,
                "command family is mounted in both primary and floating slots: {}",
                family.as_str()
            ),
            Self::Text(error) => write!(formatter, "text-command text surface failed: {error}"),
            Self::Chrome(error) => write!(formatter, "text-command command chrome failed: {error}"),
            Self::ContextMenu(error) => {
                write!(formatter, "text-command context menu failed: {error}")
            }
        }
    }
}

impl std::error::Error for EguiTextCommandSurfaceError {}

#[cfg(test)]
mod error_tests {
    use super::*;
    use katana_ui_core_svg_raster::UiSvgRasterError;
    use katana_ui_core_text_raster::PlatformTextRasterError;

    #[test]
    fn text_command_surface_error_conversions_and_display_cover_every_variant() {
        let duplicate = EguiTextCommandSurfaceError::DuplicateCommandFamilyMount {
            family: CommandChromeFamilyId::new("opaque-family"),
        };
        assert_eq!(
            duplicate.to_string(),
            "command family is mounted in both primary and floating slots: opaque-family"
        );

        let text = EguiTextCommandSurfaceError::from(EguiTextSurfaceError::FrameNotProduced);
        assert!(
            text.to_string()
                .starts_with("text-command text surface failed:")
        );

        let chrome = EguiTextCommandSurfaceError::from(EguiCommandChromeError::from(
            PlatformTextRasterError::EmptyText,
        ));
        assert!(
            chrome
                .to_string()
                .starts_with("text-command command chrome failed:")
        );

        let context = EguiTextCommandSurfaceError::from(ContextMenuAdapterError::from(
            UiSvgRasterError::EmptySource,
        ));
        assert!(
            context
                .to_string()
                .starts_with("text-command context menu failed:")
        );
    }

    #[test]
    fn text_command_surface_error_implements_error() {
        let error: &dyn std::error::Error =
            &EguiTextCommandSurfaceError::DuplicateCommandFamilyMount {
                family: CommandChromeFamilyId::new("opaque-family"),
            };
        assert!(error.to_string().contains("opaque-family"));
    }
}
