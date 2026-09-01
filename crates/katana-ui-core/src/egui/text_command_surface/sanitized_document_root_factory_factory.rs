use super::{SanitizedDocumentRootFactoryError, SanitizedDocumentRootFrame};
use crate::egui::text_command_surface::sanitized_document_root::sanitized_command_event::route_command_events;
use crate::egui::text_command_surface::sanitized_document_root::sanitized_context_event::route_context_menu_events;
use crate::egui::text_command_surface::sanitized_document_root::sanitized_document_root_input::SanitizedDocumentRootInput;
use crate::egui::text_command_surface::sanitized_document_root::sanitized_document_root_process::SanitizedDocumentRootProcess;
use crate::egui::text_command_surface::sanitized_document_root::sanitized_document_root_record::SanitizedDocumentRootRecord;
use std::cell::RefCell;

fn map_search_detach_error(
    _error: crate::egui::text_command_surface::root::EguiTextCommandSurfaceRootEventSearchDetachError,
) -> SanitizedDocumentRootFactoryError {
    SanitizedDocumentRootFactoryError::EventBatchUnavailable
}

fn map_command_detach_error(
    _error: crate::egui::text_command_surface::root::EguiTextCommandSurfaceRootEventCommandDetachError,
) -> SanitizedDocumentRootFactoryError {
    SanitizedDocumentRootFactoryError::EventBatchUnavailable
}

#[cfg(test)]
mod detach_error_tests {
    use super::{map_command_detach_error, map_search_detach_error};
    use crate::egui::text_command_surface::root::{
        EguiTextCommandSurfaceRootEventCommandDetachError,
        EguiTextCommandSurfaceRootEventSearchDetachError,
    };
    use crate::egui::text_command_surface::sanitized_document_root::SanitizedDocumentRootFactoryError;

    #[test]
    fn one_shot_detach_failures_map_to_the_closed_factory_error() {
        assert_eq!(
            map_search_detach_error(
                EguiTextCommandSurfaceRootEventSearchDetachError::AlreadyDetached,
            ),
            SanitizedDocumentRootFactoryError::EventBatchUnavailable
        );
        assert_eq!(
            map_command_detach_error(
                EguiTextCommandSurfaceRootEventCommandDetachError::AlreadyConsumed,
            ),
            SanitizedDocumentRootFactoryError::EventBatchUnavailable
        );
        assert_eq!(
            SanitizedDocumentRootFactoryError::EventBatchUnavailable.to_string(),
            "sanitized document root event batch is unavailable"
        );
    }
}

/// Factory for the retained sanitized document root.
pub struct SanitizedDocumentRootFactory;

impl SanitizedDocumentRootFactory {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Retains one opaque document input and its KUC-owned rendering process.
    pub fn retain(
        &self,
        input: SanitizedDocumentRootInput,
    ) -> Result<SanitizedDocumentRoot, SanitizedDocumentRootFactoryError> {
        Ok(SanitizedDocumentRoot {
            process: SanitizedDocumentRootProcess::new(input),
        })
    }
}

impl Default for SanitizedDocumentRootFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    #[test]
    fn default_factory_uses_the_public_constructor() {
        let _ = SanitizedDocumentRootFactory::default();
    }
}

/// Retained sanitized document root.
pub struct SanitizedDocumentRoot {
    pub(super) process: SanitizedDocumentRootProcess,
}

impl SanitizedDocumentRoot {
    /// Synchronizes the retained process with a complete host snapshot.
    pub fn synchronize(
        &mut self,
        input: SanitizedDocumentRootInput,
    ) -> Result<bool, SanitizedDocumentRootFactoryError> {
        self.process.synchronize(input).map_err(Into::into)
    }

    /// Renders the retained KUC root once for the caller's frame.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<SanitizedDocumentRootFrame, SanitizedDocumentRootFactoryError> {
        let output = self.process.show(ui).map_err(render_error)?;
        let record = SanitizedDocumentRootRecord::from_output(self.process.input.revision, &output);
        #[cfg(test)]
        let command_action_rects =
            output
                .toolbar_record
                .as_ref()
                .map_or_else(Vec::new, |toolbar| {
                    toolbar
                        .actions
                        .iter()
                        .map(|action| (action.bounds, action.secondary_trigger_bounds))
                        .collect()
                });
        #[cfg(test)]
        let floating_action_rects = output
            .floating
            .as_ref()
            .and_then(|floating| floating.record.as_ref())
            .map_or_else(Vec::new, |floating| {
                floating
                    .toolbar
                    .actions
                    .iter()
                    .map(|action| action.bounds)
                    .collect()
            });
        #[cfg(test)]
        let tab_rects = self.process.tab_rects().to_vec();
        #[cfg(test)]
        let tab_close_rects = self.process.tab_close_rects().to_vec();
        let tab_closed_events = self.process.take_tab_closed_events();
        let raw_search_events = output
            .events()
            .detach_search_events()
            .map_err(map_search_detach_error)?;
        let search_events = self
            .process
            .route_search_events(
                &raw_search_events,
                self.process.input.revision,
                &self.process.input.identity.stable_fingerprint(),
            )
            .map_err(SanitizedDocumentRootFactoryError::SearchCapability)?;
        let (raw_command_events, raw_floating_events) = output
            .events()
            .detach_command_events()
            .map_err(map_command_detach_error)?;
        let command_events = route_command_events(
            self.process.input.command_projection.as_ref(),
            self.process.input.floating_command_projection.as_ref(),
            &raw_command_events,
            &raw_floating_events,
            self.process.input.revision,
            &self.process.input.identity.stable_fingerprint(),
        )
        .map_err(SanitizedDocumentRootFactoryError::CommandCapability)?;
        let raw_context_events = output
            .events()
            .detach_context_menu_events()
            .map_err(map_command_detach_error)?;
        let context_menu_events = route_context_menu_events(
            self.process.input.context_projection.as_ref(),
            &raw_context_events,
            self.process.input.revision,
            &self.process.input.identity.stable_fingerprint(),
        )
        .map_err(SanitizedDocumentRootFactoryError::ContextMenuCapability)?;
        Ok(SanitizedDocumentRootFrame {
            output,
            record,
            tab_closed_events: RefCell::new(Some(tab_closed_events)),
            search_events: RefCell::new(Some(search_events)),
            command_events: RefCell::new(Some(command_events)),
            context_menu_events: RefCell::new(Some(context_menu_events)),
            generation: self.process.generation.get(),
            current_generation: self.process.generation.clone(),
            #[cfg(test)]
            tab_rects,
            #[cfg(test)]
            tab_close_rects,
            #[cfg(test)]
            command_action_rects,
            #[cfg(test)]
            floating_action_rects,
        })
    }
}

fn render_error(
    error: crate::egui::text_command_surface::EguiTextCommandSurfaceRootError,
) -> SanitizedDocumentRootFactoryError {
    SanitizedDocumentRootFactoryError::Render(error.to_string())
}

#[cfg(test)]
mod render_error_tests {
    use super::*;

    #[test]
    fn render_errors_remain_closed() {
        let error = render_error(
            crate::egui::text_command_surface::EguiTextCommandSurfaceRootError::Serialization(
                "opaque".into(),
            ),
        );
        assert!(matches!(
            error,
            SanitizedDocumentRootFactoryError::Render(_)
        ));
    }
}
