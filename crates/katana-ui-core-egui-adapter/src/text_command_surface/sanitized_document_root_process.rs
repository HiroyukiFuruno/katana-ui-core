#[path = "sanitized_document_root_surface.rs"]
mod sanitized_document_root_surface;

use super::super::root::{
    EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootError, EguiTextCommandSurfaceRootOutput,
};
use super::super::types::TextCommandSurfaceStyle;
use super::sanitized_document_root_input::SanitizedDocumentRootInput;
use super::sanitized_document_root_style::resolve_style;
use super::sanitized_search_projection::SanitizedSearchProjection;
use super::sanitized_tab_projection::adapter::{
    SanitizedTabProjectionAdapter, SanitizedTabProjectionClosedEvent, SanitizedTabProjectionFrame,
};
use katana_ui_core_text_raster::{PlatformTextRasterConfig, PlatformTextRasterResources};
use std::cell::Cell;
use std::rc::Rc;

/// Private process state for one retained generic document root.
pub(super) struct SanitizedDocumentRootProcess {
    pub(super) input: SanitizedDocumentRootInput,
    root: EguiTextCommandSurfaceRoot,
    style: TextCommandSurfaceStyle,
    tab_adapter: SanitizedTabProjectionAdapter,
    tab_frame: Option<SanitizedTabProjectionFrame>,
    tab_rendered: bool,
    pub(super) generation: Rc<Cell<u64>>,
    search_projection: Option<SanitizedSearchProjection>,
}

impl SanitizedDocumentRootProcess {
    pub(super) fn new(input: SanitizedDocumentRootInput) -> Result<Self, String> {
        let revision = input.revision;
        let mut input = input;
        let (surface, presentation) = sanitized_document_root_surface::from_input(&input);
        let search_projection = input.search_projection.take();
        let identity = input.identity.stable_fingerprint();
        let mut root = EguiTextCommandSurfaceRoot::with_text_raster_resources(
            identity,
            surface,
            PlatformTextRasterResources::new(PlatformTextRasterConfig::default()),
        );
        let _ = root.synchronize_presentation(presentation);
        let style = resolve_style(input.style).map_err(render_style_error)?;
        let tab_adapter =
            SanitizedTabProjectionAdapter::from_projection(input.tab_projection.as_ref());
        Ok(Self {
            input,
            root,
            style,
            tab_adapter,
            tab_frame: None,
            tab_rendered: false,
            generation: Rc::new(Cell::new(revision)),
            search_projection,
        })
    }

    /// Synchronizes one complete host snapshot using the retained identity/revision policy.
    pub(super) fn synchronize(
        &mut self,
        input: SanitizedDocumentRootInput,
    ) -> Result<bool, SanitizedDocumentRootProcessError> {
        if !self.input.identity.same_identity(&input.identity) {
            return Err(SanitizedDocumentRootProcessError::IdentityChanged);
        }
        if input.revision < self.input.revision {
            return Err(SanitizedDocumentRootProcessError::StaleRevision {
                current: self.input.revision,
                received: input.revision,
            });
        }
        if input.revision == self.input.revision {
            if input.snapshot != self.input.snapshot
                || input.readonly != self.input.readonly
                || input.style != self.input.style
                || !input.same_command_projection_as(&self.input)
                || !input.same_search_projection_as(&self.input)
                || !input.same_context_projection_as(&self.input)
                || !input.same_tab_projection_as(&self.input)
            {
                return Err(SanitizedDocumentRootProcessError::RevisionConflict {
                    revision: input.revision,
                });
            }
            return Ok(false);
        }

        let mut input = input;
        let presentation = sanitized_document_root_surface::presentation_from_input(&input);
        let search_projection = input.search_projection.take();
        let changed = self.root.synchronize_presentation(presentation);
        self.style = resolve_style(input.style).map_err(SanitizedDocumentRootProcessError::from)?;
        self.tab_adapter
            .replace_projection(input.tab_projection.as_ref());
        self.tab_frame = None;
        self.tab_rendered = false;
        self.generation.set(input.revision);
        self.input = input;
        self.search_projection = search_projection;
        Ok(changed)
    }

    /// Shows the retained KUC root exactly once for the caller's frame.
    pub(super) fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        let mut tab_frame = None;
        let output = ui
            .vertical(|ui| {
                let frame = self.tab_adapter.show(ui);
                self.tab_rendered = frame.has_render_facts();
                tab_frame = Some(frame);
                self.root.show(ui, &self.style)
            })
            .inner?;
        self.tab_frame = tab_frame;
        Ok(output)
    }

    pub(crate) fn take_tab_closed_events(&mut self) -> Vec<SanitizedTabProjectionClosedEvent> {
        self.tab_frame
            .take()
            .map(SanitizedTabProjectionFrame::into_closed_events)
            .unwrap_or_default()
    }

    pub(super) fn route_search_events(
        &self,
        events: &[katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent],
        revision: u64,
        root_identity_fingerprint: &str,
    ) -> Result<
        Vec<super::sanitized_search_event::SanitizedSearchEventTransport>,
        super::sanitized_search_projection::SanitizedSearchCapabilityRejection,
    > {
        self.search_projection.as_ref().map_or_else(
            || Ok(Vec::new()),
            |projection| {
                super::sanitized_search_event::route_search_events(
                    Some(projection),
                    events,
                    revision,
                    root_identity_fingerprint,
                )
            },
        )
    }

    #[cfg(test)]
    pub(super) fn search_options(
        &self,
    ) -> Option<katana_ui_core::molecule::structured::SearchOptions> {
        self.search_projection.as_ref().map(|projection| {
            super::sanitized_search_projection_adapter::SanitizedSearchPresentation::from(
                projection,
            )
            .value
            .options
        })
    }

    #[cfg(test)]
    pub(super) fn tab_rects(&self) -> &[(String, egui::Rect)] {
        self.tab_frame
            .as_ref()
            .map_or(&[], |frame| frame.boundary_facts().tab_rects)
    }

    #[cfg(test)]
    pub(super) fn tab_close_rects(&self) -> &[(String, egui::Rect)] {
        self.tab_frame
            .as_ref()
            .map_or(&[], |frame| frame.boundary_facts().close_rects)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SanitizedDocumentRootProcessError {
    IdentityChanged,
    StaleRevision { current: u64, received: u64 },
    RevisionConflict { revision: u64 },
    Style(String),
}

impl From<super::super::types::EguiTextCommandSurfaceError> for SanitizedDocumentRootProcessError {
    fn from(value: super::super::types::EguiTextCommandSurfaceError) -> Self {
        Self::Style(value.to_string())
    }
}

pub(super) fn render_style_error(
    error: super::super::types::EguiTextCommandSurfaceError,
) -> String {
    error.to_string()
}

#[cfg(test)]
#[path = "sanitized_document_root_process_test_support_tests.rs"]
mod test_support;
#[cfg(test)]
pub(crate) use test_support::SearchProjectionForIme;

#[cfg(test)]
#[path = "sanitized_document_root_process_inline_tests.rs"]
mod tests;
