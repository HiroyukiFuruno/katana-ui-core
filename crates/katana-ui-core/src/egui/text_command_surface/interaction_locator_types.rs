//! Public interaction-locator API surface for text-command surfaces.

use std::cell::RefCell;
use std::collections::HashSet;

use crate::egui::text_command_surface::accesskit_evidence::AccessKitEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KucInteractionActionClass {
    TextSurfaceContextTarget,
    Toolbar,
    FloatingToolbar,
    DropdownTrigger,
    DropdownItem,
    SearchControl,
    ContextMenuItem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucInteractionSelector {
    action_identity: String,
    action_class: KucInteractionActionClass,
}

impl KucInteractionSelector {
    #[must_use]
    pub fn new(
        action_identity: impl Into<String>,
        action_class: KucInteractionActionClass,
    ) -> Self {
        Self {
            action_identity: action_identity.into(),
            action_class,
        }
    }

    #[must_use]
    pub(crate) fn action_identity(&self) -> &str {
        &self.action_identity
    }

    #[must_use]
    pub(crate) fn action_class(&self) -> KucInteractionActionClass {
        self.action_class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucInteractionLocatorError {
    Missing,
    Disabled,
    Hidden,
    Ambiguous,
    Duplicate,
}

impl std::fmt::Display for KucInteractionLocatorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Missing => "interaction action is missing",
            Self::Disabled => "interaction action is disabled",
            Self::Hidden => "interaction action is hidden",
            Self::Ambiguous => "interaction action is ambiguous",
            Self::Duplicate => "interaction action is duplicated",
        })
    }
}

impl std::error::Error for KucInteractionLocatorError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucInteractionRequestError {
    RootMismatch,
    Stale,
    Duplicate,
    AlreadyQueued,
}

impl std::fmt::Display for KucInteractionRequestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::RootMismatch => "interaction request belongs to another root",
            Self::Stale => "interaction request is stale",
            Self::Duplicate => "interaction request is duplicated",
            Self::AlreadyQueued => "interaction request is already queued",
        })
    }
}

impl std::error::Error for KucInteractionRequestError {}

/// Opaque one-shot input generated from one current-frame action.
pub struct KucOpaqueInteractionRequest {
    pub(crate) root_identity: String,
    pub(crate) state_revision: u64,
    pub(crate) correlation_fingerprint: String,
    pub(crate) events: Vec<egui::Event>,
    pub(crate) queued: bool,
}

impl std::fmt::Debug for KucOpaqueInteractionRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucOpaqueInteractionRequest(..)")
    }
}

/// Current-frame locator. Geometry and egui identity resolution stay inside KUC.
pub struct KucInteractionLocator {
    pub(crate) root_identity: String,
    pub(crate) state_revision: u64,
    pub(crate) correlation_fingerprint: String,
    pub(crate) targets: Vec<LocatorTarget>,
    pub(crate) ambiguous_bounds: Vec<crate::render_model::UiRect>,
    pub(crate) hidden: HashSet<(String, KucInteractionActionClass)>,
    pub(crate) requested: RefCell<HashSet<(String, KucInteractionActionClass)>>,
}

#[derive(Debug)]
pub(crate) struct LocatorTarget {
    pub(crate) action_identity: String,
    pub(crate) action_class: KucInteractionActionClass,
    pub(crate) disabled: bool,
    pub(crate) evidence: AccessKitEvidence,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_accessors_preserve_identity_and_class() {
        let selector = KucInteractionSelector::new(
            "opaque-action",
            KucInteractionActionClass::FloatingToolbar,
        );
        assert_eq!(selector.action_identity(), "opaque-action");
        assert_eq!(
            selector.action_class(),
            KucInteractionActionClass::FloatingToolbar
        );
    }

    #[test]
    fn locator_error_display_covers_every_variant() {
        let cases = [
            (
                KucInteractionLocatorError::Missing,
                "interaction action is missing",
            ),
            (
                KucInteractionLocatorError::Disabled,
                "interaction action is disabled",
            ),
            (
                KucInteractionLocatorError::Hidden,
                "interaction action is hidden",
            ),
            (
                KucInteractionLocatorError::Ambiguous,
                "interaction action is ambiguous",
            ),
            (
                KucInteractionLocatorError::Duplicate,
                "interaction action is duplicated",
            ),
        ];
        for (error, expected) in cases {
            let source: &dyn std::error::Error = &error;
            assert_eq!(source.to_string(), expected);
        }
    }

    #[test]
    fn request_error_display_covers_every_variant() {
        let cases = [
            (
                KucInteractionRequestError::RootMismatch,
                "interaction request belongs to another root",
            ),
            (
                KucInteractionRequestError::Stale,
                "interaction request is stale",
            ),
            (
                KucInteractionRequestError::Duplicate,
                "interaction request is duplicated",
            ),
            (
                KucInteractionRequestError::AlreadyQueued,
                "interaction request is already queued",
            ),
        ];
        for (error, expected) in cases {
            let source: &dyn std::error::Error = &error;
            assert_eq!(source.to_string(), expected);
        }
    }

    #[test]
    fn opaque_request_debug_hides_payload() {
        let request = KucOpaqueInteractionRequest {
            root_identity: "secret-root".into(),
            state_revision: 1,
            correlation_fingerprint: "secret-correlation".into(),
            events: vec![egui::Event::Copy],
            queued: false,
        };
        assert_eq!(format!("{request:?}"), "KucOpaqueInteractionRequest(..)");
    }
}
