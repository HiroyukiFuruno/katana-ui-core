#[path = "sanitized_document_root_factory/types.rs"]
mod types;

use super::sanitized_command_event::route_command_events;
use super::sanitized_context_event::route_context_menu_events;
use super::sanitized_document_root_input::SanitizedDocumentRootInput;
use super::sanitized_document_root_process::{
    SanitizedDocumentRootProcess, SanitizedDocumentRootProcessError,
};
use super::sanitized_document_root_record::SanitizedDocumentRootRecord;
use super::sanitized_document_root_transport::{
    SanitizedDocumentRootEventForwardError, SanitizedDocumentRootEventForwarder,
    SanitizedDocumentRootEventForwardingReceipt, forward_root_events_once,
};
use std::cell::RefCell;
pub use types::{
    SanitizedDocumentRoot, SanitizedDocumentRootFactory, SanitizedDocumentRootFactoryError,
    SanitizedDocumentRootFrame,
};

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
            process: SanitizedDocumentRootProcess::new(input)
                .map_err(SanitizedDocumentRootFactoryError::Render)?,
        })
    }
}

impl Default for SanitizedDocumentRootFactory {
    fn default() -> Self {
        Self::new()
    }
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
        let output = self
            .process
            .show(ui)
            .map_err(|error| SanitizedDocumentRootFactoryError::Render(error.to_string()))?;
        self.finish_output(output)
    }

    fn finish_output(
        &mut self,
        output: super::super::root::EguiTextCommandSurfaceRootOutput,
    ) -> Result<SanitizedDocumentRootFrame, SanitizedDocumentRootFactoryError> {
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
        let raw_search_events =
            output
                .events()
                .detach_search_events_exclusively()
                .map_err(|error| {
                    SanitizedDocumentRootFactoryError::Render(format!(
                        "search event detach failed: {error:?}"
                    ))
                })?;
        let search_events = self
            .process
            .route_search_events(
                &raw_search_events,
                self.process.input.revision,
                &self.process.input.identity.stable_fingerprint(),
            )
            .map_err(SanitizedDocumentRootFactoryError::SearchCapability)?;
        let (raw_command_events, raw_floating_events) =
            output.events().detach_command_events().map_err(|error| {
                SanitizedDocumentRootFactoryError::Render(format!(
                    "command event detach failed: {error:?}"
                ))
            })?;
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
            .map_err(|error| {
                SanitizedDocumentRootFactoryError::Render(format!(
                    "context menu event detach failed: {error:?}"
                ))
            })?;
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

impl SanitizedDocumentRootFrame {
    #[must_use]
    pub const fn record(&self) -> &SanitizedDocumentRootRecord {
        &self.record
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
        forward_root_events_once(
            &self.output,
            &self.tab_closed_events,
            &self.search_events,
            &self.command_events,
            &self.context_menu_events,
            forwarder,
        )
    }
}

impl std::fmt::Debug for SanitizedDocumentRootFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedDocumentRootFrame")
            .field("record", &self.record)
            .finish_non_exhaustive()
    }
}

impl From<SanitizedDocumentRootProcessError> for SanitizedDocumentRootFactoryError {
    fn from(value: SanitizedDocumentRootProcessError) -> Self {
        match value {
            SanitizedDocumentRootProcessError::IdentityChanged => Self::IdentityChanged,
            SanitizedDocumentRootProcessError::StaleRevision { current, received } => {
                Self::StaleRevision { current, received }
            }
            SanitizedDocumentRootProcessError::RevisionConflict { revision } => {
                Self::RevisionConflict { revision }
            }
            SanitizedDocumentRootProcessError::Style(error) => Self::Render(error),
        }
    }
}

impl std::fmt::Display for SanitizedDocumentRootFactoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdentityChanged => {
                formatter.write_str("sanitized document root identity cannot change")
            }
            Self::StaleRevision { current, received } => write!(
                formatter,
                "sanitized document root revision {received} is stale; current is {current}"
            ),
            Self::RevisionConflict { revision } => {
                write!(
                    formatter,
                    "sanitized document root revision {revision} conflicts"
                )
            }
            Self::Render(error) => {
                write!(formatter, "sanitized document root render failed: {error}")
            }
            Self::SearchCapability(_) => {
                formatter.write_str("sanitized search capability rejected")
            }
            Self::CommandCapability(_) => {
                formatter.write_str("sanitized command capability rejected")
            }
            Self::ContextMenuCapability(_) => {
                formatter.write_str("sanitized context menu capability rejected")
            }
        }
    }
}

impl std::error::Error for SanitizedDocumentRootFactoryError {}

#[cfg(test)]
#[path = "sanitized_document_root_factory_test_support_tests.rs"]
mod test_support;

#[cfg(test)]
#[path = "sanitized_document_root_factory_inline_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "sanitized_document_root_factory_tests/sanitized_document_root_factory_coverage_tests.rs"]
mod coverage_tests;
