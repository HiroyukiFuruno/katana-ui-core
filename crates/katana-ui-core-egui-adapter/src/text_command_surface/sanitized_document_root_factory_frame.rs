use crate::text_command_surface::root::EguiTextCommandSurfaceRootOutput;
use crate::text_command_surface::sanitized_document_root::sanitized_command_event::SanitizedCommandActivationTransport;
use crate::text_command_surface::sanitized_document_root::sanitized_context_event::SanitizedContextMenuActivationTransport;
use crate::text_command_surface::sanitized_document_root::sanitized_document_root_record::SanitizedDocumentRootRecord;
use crate::text_command_surface::sanitized_document_root::sanitized_document_root_transport::{
    RootEventForwarding, SanitizedDocumentRootEventForwardError,
    SanitizedDocumentRootEventForwarder, SanitizedDocumentRootEventForwardingReceipt,
};
use crate::text_command_surface::sanitized_document_root::sanitized_search_event::SanitizedSearchEventTransport;
use crate::text_command_surface::sanitized_document_root::sanitized_tab_projection::adapter::SanitizedTabProjectionClosedEvent;
use std::cell::RefCell;

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

impl SanitizedDocumentRootFrame {
    #[must_use]
    pub const fn record(&self) -> &SanitizedDocumentRootRecord {
        &self.record
    }

    #[cfg(test)]
    pub(super) fn tab_closed_event_count(&self) -> usize {
        self.tab_closed_events.borrow().as_ref().map_or(0, Vec::len)
    }

    #[cfg(test)]
    pub(super) fn tab_activation_event_count(&self) -> usize {
        self.tab_closed_events
            .borrow()
            .as_ref()
            .map_or(0, |events| {
                events
                    .iter()
                    .filter(|event| {
                        matches!(event, SanitizedTabProjectionClosedEvent::TabActivated(_))
                    })
                    .count()
            })
    }

    #[cfg(test)]
    pub(super) fn tab_close_request_event_count(&self) -> usize {
        self.tab_closed_events
            .borrow()
            .as_ref()
            .map_or(0, |events| tab_close_request_count(events))
    }

    #[cfg(test)]
    pub(super) fn tab_rects(&self) -> &[(String, egui::Rect)] {
        &self.tab_rects
    }

    #[cfg(test)]
    pub(super) fn tab_close_rects(&self) -> &[(String, egui::Rect)] {
        &self.tab_close_rects
    }

    #[cfg(test)]
    pub(super) fn command_action_rects(
        &self,
    ) -> &[(
        katana_ui_core::render_model::UiRect,
        Option<katana_ui_core::render_model::UiRect>,
    )] {
        &self.command_action_rects
    }

    #[cfg(test)]
    pub(super) fn floating_action_rects(&self) -> &[katana_ui_core::render_model::UiRect] {
        &self.floating_action_rects
    }

    /// Forwards the frame's opaque event transport exactly once.
    pub fn forward_events_once<Forwarder>(
        &self,
        forwarder: &mut Forwarder,
    ) -> Result<
        SanitizedDocumentRootEventForwardingReceipt,
        SanitizedDocumentRootEventForwardError<Forwarder::Error>,
    >
    where
        Forwarder: SanitizedDocumentRootEventForwarder,
    {
        if self.current_generation.get() != self.generation {
            return Err(SanitizedDocumentRootEventForwardError::StaleFrame);
        }
        RootEventForwarding::forward_root_events_once(
            &self.output,
            &self.tab_closed_events,
            &self.search_events,
            &self.command_events,
            &self.context_menu_events,
            forwarder,
        )
    }
}

#[cfg(test)]
fn tab_close_request_count(events: &[SanitizedTabProjectionClosedEvent]) -> usize {
    events
        .iter()
        .map(|event| {
            usize::from(matches!(
                event,
                SanitizedTabProjectionClosedEvent::TabCloseRequested(_)
            ))
        })
        .sum()
}

impl std::fmt::Debug for SanitizedDocumentRootFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedDocumentRootFrame")
            .field("record", &self.record)
            .finish_non_exhaustive()
    }
}
