use super::super::super::root::EguiTextCommandSurfaceRootOutput;
use super::super::sanitized_command_event::SanitizedCommandActivationTransport;
use super::super::sanitized_context_event::SanitizedContextMenuActivationTransport;
use super::super::sanitized_document_root_process::SanitizedDocumentRootProcess;
use super::super::sanitized_document_root_record::SanitizedDocumentRootRecord;
use super::super::sanitized_search_event::SanitizedSearchEventTransport;
use super::super::sanitized_tab_projection::adapter::SanitizedTabProjectionClosedEvent;
use std::cell::RefCell;

/// Factory for the retained sanitized document root.
pub struct SanitizedDocumentRootFactory;

/// Retained sanitized document root.
pub struct SanitizedDocumentRoot {
    pub(super) process: SanitizedDocumentRootProcess,
}

/// Closed frame returned by the retained sanitized document root.
pub struct SanitizedDocumentRootFrame {
    pub(super) output: EguiTextCommandSurfaceRootOutput,
    pub(super) record: SanitizedDocumentRootRecord,
    pub(super) tab_closed_events: RefCell<Option<Vec<SanitizedTabProjectionClosedEvent>>>,
    pub(super) search_events: RefCell<Option<Vec<SanitizedSearchEventTransport>>>,
    pub(super) command_events: RefCell<Option<Vec<SanitizedCommandActivationTransport>>>,
    pub(super) context_menu_events: RefCell<Option<Vec<SanitizedContextMenuActivationTransport>>>,
    pub(super) generation: u64,
    pub(super) current_generation: std::rc::Rc<std::cell::Cell<u64>>,
    #[cfg(test)]
    pub(super) tab_rects: Vec<(String, egui::Rect)>,
    #[cfg(test)]
    pub(super) tab_close_rects: Vec<(String, egui::Rect)>,
    #[cfg(test)]
    pub(super) command_action_rects: Vec<(
        katana_ui_core::render_model::UiRect,
        Option<katana_ui_core::render_model::UiRect>,
    )>,
    #[cfg(test)]
    pub(super) floating_action_rects: Vec<katana_ui_core::render_model::UiRect>,
}

/// Errors reserved for the retained sanitized document root contract.
#[derive(Debug, PartialEq, Eq)]
pub enum SanitizedDocumentRootFactoryError {
    IdentityChanged,
    StaleRevision {
        current: u64,
        received: u64,
    },
    RevisionConflict {
        revision: u64,
    },
    Render(String),
    SearchCapability(super::super::sanitized_search_projection::SanitizedSearchCapabilityRejection),
    CommandCapability(
        super::super::sanitized_command_projection::SanitizedCommandCapabilityRejection,
    ),
    ContextMenuCapability(
        super::super::sanitized_context_projection::SanitizedContextMenuCapabilityRejection,
    ),
}
