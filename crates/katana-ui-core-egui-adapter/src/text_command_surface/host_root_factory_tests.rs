use super::*;
use crate::text_command_surface::{
    TabStripControlPresentation, TabStripCorrelation, TabStripNavigationPresentation,
    TabStripProjection, TabStripScrollPresentation, TabStripText,
};
use egui::RawInput;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeToolbarPresentation, FloatingCommandToolbarVisibility,
};
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressPresentation, SourceAddressStrip,
};
use katana_ui_core::molecule::{DiagnosticsList, StatusBar, StatusBarSegment};
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

#[path = "host_root_factory_tests/lease_contract.rs"]
mod lease_contract;
#[path = "host_root_factory_tests/synchronization_contract.rs"]
mod synchronization_contract;
#[path = "host_root_factory_tests/token_contract.rs"]
mod token_contract;
