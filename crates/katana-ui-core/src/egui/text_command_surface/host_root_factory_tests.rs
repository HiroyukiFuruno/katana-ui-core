use super::*;
use crate::egui::text_command_surface::{
    TabStripControlPresentation, TabStripCorrelation, TabStripNavigationPresentation,
    TabStripProjection, TabStripScrollPresentation, TabStripText,
};
use crate::molecule::command_chrome::{
    CommandChromeAction, CommandChromeToolbarPresentation, FloatingCommandToolbarVisibility,
};
use crate::molecule::structured::source_address_strip::{
    SourceAddressPresentation, SourceAddressStrip,
};
use crate::molecule::{DiagnosticsList, StatusBar, StatusBarSegment};
use crate::render_model::UiTextSpan;
use crate::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use egui::RawInput;

#[path = "host_root_factory_tests/lease_contract.rs"]
mod lease_contract;
#[path = "host_root_factory_tests/lease_identity.rs"]
mod lease_identity;
#[path = "host_root_factory_tests/preserving_state_drag.rs"]
mod preserving_state_drag;
#[path = "host_root_factory_tests/synchronization_contract.rs"]
mod synchronization_contract;
#[path = "host_root_factory_tests/token_contract.rs"]
mod token_contract;
