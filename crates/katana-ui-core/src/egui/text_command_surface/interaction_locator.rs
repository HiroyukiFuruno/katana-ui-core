//! Generic current-frame interaction requests for the retained command surface.

use super::super::accesskit_evidence::{
    AccessKitEvidence, AccessKitTargetClass, BoundAccessKitEvidence,
};
use super::super::types::EguiTextCommandSurfaceOutput;
use super::root_event::KucRootEventBatchContext;
use crate::egui::command_chrome::{
    EguiCommandChromeActionFrame, EguiCommandChromeFrameRecord, EguiCommandChromeSearchFrameRecord,
};
use crate::egui::context_menu::EguiContextMenuFrameRecord;
use std::cell::RefCell;
use std::collections::HashSet;

mod click;
mod continuations;
mod locator;
mod targets;
#[cfg(test)]
mod tests;
mod types;

pub use types::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueClickContinuation,
    KucOpaqueClickContinuationError, KucOpaqueInteractionRequest, KucOpaqueSearchTraceContinuation,
    KucOpaqueTextSelectionContinuation, KucSearchTraceContinuationError,
    KucTextSelectionContinuationError,
};
