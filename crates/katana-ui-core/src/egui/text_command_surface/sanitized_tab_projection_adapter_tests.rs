use super::{
    SanitizedTabGroup, SanitizedTabProjectionAdapter, SanitizedTabProjectionClosedEvent,
    projection_to_state, projection_to_strip, structural_id,
};
use crate::egui::text_command_surface::sanitized_document_root::sanitized_tab_projection::{
    SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation, SanitizedTabGroupTarget,
    SanitizedTabProjection, SanitizedTabTarget,
};
use crate::molecule::structured::{
    CloseableTabGroupId, CloseableTabId, CloseableTabStripEvent,
};
use crate::render_model::UiIconProps;

const SCREEN_SIZE: egui::Vec2 = egui::vec2(600.0, 160.0);
const FINGERPRINT_HEX_LENGTH: usize = 64;

include!("sanitized_tab_projection_adapter_tests/behavior.rs");
include!("sanitized_tab_projection_adapter_tests/events.rs");
include!("sanitized_tab_projection_adapter_tests/helpers.rs");
