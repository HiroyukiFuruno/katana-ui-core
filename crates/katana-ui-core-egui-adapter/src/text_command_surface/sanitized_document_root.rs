//! Minimal consumer-safe document root for the generic text-command surface.

#[path = "sanitized_command_event.rs"]
mod sanitized_command_event;
#[path = "sanitized_command_projection.rs"]
mod sanitized_command_projection;
#[path = "sanitized_command_projection_adapter.rs"]
mod sanitized_command_projection_adapter;
#[path = "sanitized_context_event.rs"]
mod sanitized_context_event;
#[path = "sanitized_context_projection.rs"]
mod sanitized_context_projection;
#[path = "sanitized_context_projection_adapter.rs"]
mod sanitized_context_projection_adapter;
#[path = "sanitized_document_root_factory.rs"]
mod sanitized_document_root_factory;
#[path = "sanitized_document_root_input.rs"]
mod sanitized_document_root_input;
#[path = "sanitized_document_root_process.rs"]
mod sanitized_document_root_process;
#[path = "sanitized_document_root_record.rs"]
mod sanitized_document_root_record;
#[path = "sanitized_document_root_style.rs"]
mod sanitized_document_root_style;
#[path = "sanitized_document_root_transport.rs"]
mod sanitized_document_root_transport;
#[path = "sanitized_search_event.rs"]
pub(super) mod sanitized_search_event;
#[path = "sanitized_search_projection.rs"]
pub(super) mod sanitized_search_projection;
#[path = "sanitized_search_projection_adapter.rs"]
mod sanitized_search_projection_adapter;
mod sanitized_tab_projection {
    include!("sanitized_tab_projection.rs");
    pub(super) mod adapter {
        include!("sanitized_tab_projection_adapter.rs");
    }
}

pub use sanitized_command_projection::{
    SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
    SanitizedCommandProjection, SanitizedCommandTarget,
};
pub use sanitized_context_projection::{
    SanitizedContextMenuCapabilityRejection, SanitizedContextMenuItem,
    SanitizedContextMenuProjection, SanitizedContextMenuProjectionBuilder,
    SanitizedContextMenuTarget,
};

pub use sanitized_document_root_factory::{
    SanitizedDocumentRoot, SanitizedDocumentRootFactory, SanitizedDocumentRootFactoryError,
    SanitizedDocumentRootFrame,
};
pub use sanitized_document_root_input::{
    SanitizedDocumentRootIdentity, SanitizedDocumentRootInput,
};
pub use sanitized_document_root_record::{
    SanitizedDocumentRootRecord, SanitizedDocumentRootRecordDimensions,
};
pub use sanitized_document_root_style::SanitizedDocumentRootStyleKey;
pub use sanitized_document_root_transport::{
    SanitizedDocumentRootEventDispatchError, SanitizedDocumentRootEventForwardError,
    SanitizedDocumentRootEventForwarder, SanitizedDocumentRootEventForwardingReceipt,
    SanitizedDocumentRootEventTransport,
};
pub use sanitized_search_projection::{
    SanitizedSearchCapabilityRejection, SanitizedSearchControlPresentation,
    SanitizedSearchLocalizedPresentation, SanitizedSearchOperationPresentation,
    SanitizedSearchOperationSlot, SanitizedSearchProjection, SanitizedSearchProjectionBuildError,
    SanitizedSearchProjectionBuilder, SanitizedSearchResultSummaryPresentation,
    SanitizedSearchTarget, SanitizedSearchTextOperation, SanitizedSearchTextPresentation,
    SanitizedSearchUnavailablePresentation, SanitizedSearchUnitOperation,
};

pub use sanitized_tab_projection::{
    SanitizedTab, SanitizedTabCapabilities, SanitizedTabGroup, SanitizedTabGroupCapabilities,
    SanitizedTabGroupTarget, SanitizedTabProjection, SanitizedTabTarget,
};
pub type SanitizedTabClosePresentation = sanitized_tab_projection::SanitizedTabClosePresentation;

#[cfg(test)]
mod source_guards {
    #[test]
    fn public_command_projection_stays_generic_and_output_free() {
        let source = include_str!("sanitized_command_projection.rs");
        let forbidden = [
            "markdown",
            "katana_language",
            "katana::",
            "KLE",
            "coordinate",
            "geometry",
            "range",
            "EguiTextCommandSurfaceRootOutput",
            "texture",
            "paint",
        ];
        let source = source.to_ascii_lowercase();
        for term in forbidden {
            assert!(
                !source.contains(&term.to_ascii_lowercase()),
                "sanitized command projection leaked forbidden term: {term}"
            );
        }
        assert!(!source.contains("KatanA"));
        assert!(!source.contains("pub fn bytes"));
        assert!(!source.contains("pub fn as_bytes"));
        assert!(!source.contains("serialize"));
    }

    #[test]
    fn tab_projection_is_reexported_by_the_sanitized_facade() {
        let _ = super::SanitizedTabProjection::new([super::SanitizedTabGroup::new(
            super::sanitized_tab_projection::SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "tabs",
        )]);
        let _ = super::SanitizedTabCapabilities::new()
            .active_state(true)
            .dirty_state(false)
            .pinned_state(false)
            .close_state(true);
        let _ = super::sanitized_tab_projection::SanitizedTabGroupTarget::from_opaque_bytes([1]);
        let _ = super::sanitized_tab_projection::SanitizedTabGroupCapabilities::new()
            .collapse_state(true)
            .menu_state(true)
            .rename_state(true)
            .recolor_state(true)
            .close_state(true)
            .ungroup_state(true)
            .drag_state(true);
    }

    #[test]
    fn tab_projection_source_stays_generic_and_output_free() {
        let source = include_str!("sanitized_tab_projection.rs");
        let source = source
            .split("#[cfg(test)]")
            .next()
            .unwrap_or(source)
            .to_ascii_lowercase();
        for forbidden in [
            "document",
            "path",
            "markdown",
            "katana_language",
            "katana::",
            "kle",
            "geometry",
            "coordinate",
            "egui output",
            "serialize",
            "clone",
            "payload",
            "hash",
        ] {
            assert!(
                !source.contains(forbidden),
                "tab projection leaked forbidden term: {forbidden}"
            );
        }
    }
}
