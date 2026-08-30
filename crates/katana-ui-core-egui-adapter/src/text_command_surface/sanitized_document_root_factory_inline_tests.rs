use super::super::sanitized_command_projection::SanitizedCommandCapabilityRejection;
use super::super::sanitized_context_projection::SanitizedContextMenuCapabilityRejection;
use super::super::sanitized_document_root_transport::{
    SanitizedDocumentRootEventDispatchError, SanitizedDocumentRootEventForwardError,
};
use super::super::sanitized_search_projection::{
    SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
    SanitizedSearchOperationPresentation, SanitizedSearchProjectionBuilder,
    SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget, SanitizedSearchTextOperation,
    SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
    SanitizedSearchUnitOperation,
};
use super::super::sanitized_tab_projection::SanitizedTabGroupTarget;
use super::{
    SanitizedDocumentRoot, SanitizedDocumentRootFactory, SanitizedDocumentRootFactoryError,
    SanitizedDocumentRootFrame,
};
use crate::text_command_surface::KucRootEventBatchDispatcher;
use crate::text_command_surface::{
    SanitizedContextMenuItem, SanitizedContextMenuProjection,
    SanitizedContextMenuProjectionBuilder, SanitizedContextMenuTarget,
    SanitizedDocumentRootEventForwarder, SanitizedDocumentRootEventTransport,
    SanitizedDocumentRootIdentity, SanitizedDocumentRootInput, SanitizedDocumentRootStyleKey,
    SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation, SanitizedTabGroup,
    SanitizedTabProjection, SanitizedTabTarget,
};
use std::cell::RefCell;
use std::rc::Rc;

#[path = "sanitized_document_root_factory_inline_tests/command_activation.rs"]
mod command_activation;
#[path = "sanitized_document_root_factory_inline_tests/command_fail_closed.rs"]
mod command_fail_closed;
#[path = "sanitized_document_root_factory_inline_tests/command_support.rs"]
mod command_support;
#[path = "sanitized_document_root_factory_inline_tests/context_menu.rs"]
mod context_menu;
#[path = "sanitized_document_root_factory_inline_tests/context_menu_debug.rs"]
mod context_menu_debug;
#[path = "sanitized_document_root_factory_inline_tests/failure_contracts.rs"]
mod failure_contracts;
#[path = "sanitized_document_root_factory_inline_tests/floating_activation.rs"]
mod floating_activation;
#[path = "sanitized_document_root_factory_inline_tests/floating_lifecycle.rs"]
mod floating_lifecycle;
#[path = "sanitized_document_root_factory_inline_tests/readonly.rs"]
mod readonly;
#[path = "sanitized_document_root_factory_inline_tests/retained_transport.rs"]
mod retained_transport;
#[path = "sanitized_document_root_factory_inline_tests/root_event_channels.rs"]
mod root_event_channels;
#[path = "sanitized_document_root_factory_inline_tests/root_event_detach.rs"]
mod root_event_detach;
#[path = "sanitized_document_root_factory_inline_tests/root_forwarding.rs"]
mod root_forwarding;
#[path = "sanitized_document_root_factory_inline_tests/root_frame_support.rs"]
mod root_frame_support;
#[path = "sanitized_document_root_factory_inline_tests/search_accesskit.rs"]
mod search_accesskit;
#[path = "sanitized_document_root_factory_inline_tests/search_raw_input.rs"]
mod search_raw_input;
#[path = "sanitized_document_root_factory_inline_tests/search_state.rs"]
mod search_state;
#[path = "sanitized_document_root_factory_inline_tests/search_support.rs"]
mod search_support;
#[path = "sanitized_document_root_factory_inline_tests/search_support_tail.rs"]
mod search_support_tail;
#[path = "sanitized_document_root_factory_inline_tests/support.rs"]
mod support;
#[path = "sanitized_document_root_factory_inline_tests/tab_close_keyboard.rs"]
mod tab_close_keyboard;
#[path = "sanitized_document_root_factory_inline_tests/tab_close_pointer.rs"]
mod tab_close_pointer;
