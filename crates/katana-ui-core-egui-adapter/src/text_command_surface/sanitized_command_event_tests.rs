use super::{SanitizedCommandCapabilityRejection, route_command_events};
use crate::text_command_surface::{
    SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
    SanitizedCommandProjection, SanitizedCommandTarget,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::toolbar::KeyCombo;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::rc::Rc;

include!("sanitized_command_event_tests/routing.rs");
include!("sanitized_command_event_tests/edge_cases.rs");
