use super::EguiTextCommandSurfaceAdapter;
use super::EguiTextCommandSurfaceError;
use crate::command_chrome::EguiCommandChromeError;
use crate::context_menu::ContextMenuAdapterError;
use crate::diagnostics_list::EguiDiagnosticsListError;
use crate::source_address_strip::EguiSourceAddressStripError;
use crate::status_bar::EguiStatusBarError;
use crate::text_command_surface::TabStripProposalPortError;
use crate::text_command_surface::tab_strip_retained::TabStripRetainedError;
use crate::text_surface::EguiTextSurfaceError;
use katana_ui_core::molecule::command_chrome::CommandChromeFamilyId;
use katana_ui_core_text_raster::PlatformTextRasterConfig;

#[test]
fn with_text_raster_config_accepts_default_settings() {
    assert!(
        EguiTextCommandSurfaceAdapter::with_text_raster_config(PlatformTextRasterConfig::default())
            .is_ok()
    );
}

#[test]
fn error_variants_cover_display_conversion_and_debug() {
    let duplicate = EguiTextCommandSurfaceError::DuplicateCommandFamilyMount {
        family: CommandChromeFamilyId::default(),
    };
    assert_eq!(
        duplicate.to_string(),
        "command family is mounted in both primary and floating slots"
    );
    assert!(format!("{duplicate:?}").contains("DuplicateCommandFamilyMount"));

    let missing_color = EguiTextCommandSurfaceError::MissingThemeColor {
        token: "palette.text",
    };
    assert_eq!(
        missing_color.to_string(),
        "KUC theme is missing required color token: palette.text"
    );
    assert!(format!("{missing_color:?}").contains("MissingThemeColor"));

    let missing_font = EguiTextCommandSurfaceError::MissingThemeFont { token: "font.body" };
    assert_eq!(
        missing_font.to_string(),
        "KUC theme is missing required font token: font.body"
    );
    assert!(format!("{missing_font:?}").contains("MissingThemeFont"));

    let missing_spacing = EguiTextCommandSurfaceError::MissingThemeSpacing {
        token: "spacing.xs",
    };
    assert_eq!(
        missing_spacing.to_string(),
        "KUC theme is missing required spacing token: spacing.xs"
    );
    assert!(format!("{missing_spacing:?}").contains("MissingThemeSpacing"));

    let invalid_font = EguiTextCommandSurfaceError::InvalidThemeFont {
        token: "font.body",
        reason: "invalid weight",
    };
    assert_eq!(
        invalid_font.to_string(),
        "KUC theme has invalid font token font.body: invalid weight"
    );
    assert!(format!("{invalid_font:?}").contains("InvalidThemeFont"));

    let invalid_spacing = EguiTextCommandSurfaceError::InvalidThemeSpacing {
        token: "spacing.md",
        reason: "zero value",
    };
    assert_eq!(
        invalid_spacing.to_string(),
        "KUC theme has invalid spacing token spacing.md: zero value"
    );
    assert!(format!("{invalid_spacing:?}").contains("InvalidThemeSpacing"));

    let source = EguiTextCommandSurfaceError::SourceAddress(
        EguiSourceAddressStripError::PaintPlanNotProduced,
    );
    assert_eq!(
        source.to_string(),
        "text-command source address failed: source-address did not produce a paint plan"
    );
    assert!(format!("{source:?}").contains("SourceAddress"));

    let diagnostics = EguiTextCommandSurfaceError::Diagnostics(EguiDiagnosticsListError::Raster(
        katana_ui_core_text_raster::PlatformTextRasterError::EmptyText,
    ));
    assert!(
        diagnostics
            .to_string()
            .starts_with("text-command diagnostics failed: diagnostics raster failed:")
    );
    assert!(format!("{diagnostics:?}").contains("Diagnostics"));

    let status_bar =
        EguiTextCommandSurfaceError::StatusBar(EguiStatusBarError::PaintPlanNotProduced);
    assert_eq!(
        status_bar.to_string(),
        "text-command status bar failed: status-bar did not produce a paint plan"
    );
    assert!(format!("{status_bar:?}").contains("StatusBar"));

    let text = EguiTextCommandSurfaceError::from(EguiTextSurfaceError::FrameNotProduced);
    assert_eq!(
        text.to_string(),
        "text-command text surface failed: egui did not produce a text surface frame"
    );
    assert!(format!("{text:?}").contains("Text"));

    let chrome = EguiTextCommandSurfaceError::from(EguiCommandChromeError::ArtifactSerialization(
        "x".to_string(),
    ));
    assert_eq!(
        chrome.to_string(),
        "text-command command chrome failed: command chrome artifact serialization failed: x"
    );
    assert!(format!("{chrome:?}").contains("Chrome"));

    let context_menu = EguiTextCommandSurfaceError::from(ContextMenuAdapterError::Raster(
        katana_ui_core_text_raster::PlatformTextRasterError::EmptyText,
    ));
    assert!(
        context_menu
            .to_string()
            .starts_with("text-command context menu failed: context menu raster failed:")
    );
    assert!(format!("{context_menu:?}").contains("ContextMenu"));

    let tab_strip = EguiTextCommandSurfaceError::from(TabStripRetainedError::Port(
        TabStripProposalPortError::Rejected,
    ));
    assert!(
        tab_strip
            .to_string()
            .starts_with("text-command tab strip failed: tab proposal forwarding failed:")
    );
    assert!(format!("{tab_strip:?}").contains("TabStrip"));

    let converted_from_source =
        EguiTextCommandSurfaceError::from(EguiTextSurfaceError::FrameNotProduced);
    let converted_from_status =
        EguiTextCommandSurfaceError::from(EguiStatusBarError::PaintPlanNotProduced);
    assert!(matches!(
        converted_from_source,
        EguiTextCommandSurfaceError::Text(_)
    ));
    assert!(matches!(
        converted_from_status,
        EguiTextCommandSurfaceError::StatusBar(_)
    ));

    let converted_from_diagnostics =
        EguiTextCommandSurfaceError::from(EguiDiagnosticsListError::PaintPlanNotProduced);
    let converted_from_source_strip = EguiTextCommandSurfaceError::from(
        EguiSourceAddressStripError::TextSurface(EguiTextSurfaceError::FrameNotProduced),
    );
    assert!(matches!(
        converted_from_diagnostics,
        EguiTextCommandSurfaceError::Diagnostics(_)
    ));
    assert!(matches!(
        converted_from_source_strip,
        EguiTextCommandSurfaceError::SourceAddress(_)
    ));

    let from_tab_strip = EguiTextCommandSurfaceError::from(TabStripRetainedError::MissingRoute);
    assert!(matches!(
        from_tab_strip,
        EguiTextCommandSurfaceError::TabStrip { .. }
    ));
}
