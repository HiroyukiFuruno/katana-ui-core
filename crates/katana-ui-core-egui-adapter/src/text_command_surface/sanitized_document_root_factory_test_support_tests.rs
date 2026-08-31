use super::super::sanitized_tab_projection::adapter::SanitizedTabProjectionClosedEvent;
use super::SanitizedDocumentRootFrame;

impl SanitizedDocumentRootFrame {
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
            .map_or(0, |events| {
                events
                    .iter()
                    .filter(|event| {
                        matches!(
                            event,
                            SanitizedTabProjectionClosedEvent::TabCloseRequested(_)
                        )
                    })
                    .count()
            })
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
}
