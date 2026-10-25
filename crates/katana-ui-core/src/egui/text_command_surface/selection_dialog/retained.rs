mod files;
mod focus;
mod input;
mod search;
mod search_actions;

#[cfg(test)]
#[path = "retained/coverage_tests.rs"]
mod coverage_tests;

use super::super::TextCommandSurfaceStyle;
use super::model::{
    FileSelectionProjection, KucDialogTarget, SearchResultMode, SearchResultProjection,
    SelectionDialogEvent, SelectionDialogOperation, SelectionDialogProjection,
    SelectionDialogProjectionError, SelectionDialogProjectionLease,
};
use super::paint::{DialogPainter, activated, fill, rect};
use crate::atom::{TextArea, TextAreaEvent, TextAreaWrapPolicy};
use crate::egui::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy, EguiTextSurfaceKey,
    TextSurfacePaintOperation, TextSurfacePaintPlan,
};
use crate::text_raster::PlatformTextRasterResources;
use crate::text_surface::{TextSurface, TextSurfaceEvent, TextSurfaceProps, TextSurfaceViewport};
use std::collections::HashMap;

const SCREEN_MARGIN: f32 = 40.0;
const MAXIMUM_PANEL_WIDTH: f32 = 620.0;
const RESERVED_CONTROL_HEIGHT: f32 = 160.0;

#[derive(Debug)]
pub(crate) struct SelectionDialogOutput {
    pub(crate) paint_plan: TextSurfacePaintPlan,
    pub(crate) events: Vec<SelectionDialogEvent>,
}

struct DialogInput {
    surface: TextSurface,
    adapter: EguiTextSurfaceAdapter,
}

pub(crate) struct SelectionDialogRetainedState {
    identity: crate::render_model::UiStateId,
    lease: SelectionDialogProjectionLease,
    resources: PlatformTextRasterResources,
    painter: DialogPainter,
    occurrence: u64,
    selected: Option<KucDialogTarget>,
    inputs: HashMap<String, DialogInput>,
    restore_focus: Option<egui::Id>,
    modal_layer: Option<egui::LayerId>,
    deferred_input: Vec<egui::Event>,
    mounted: bool,
    failure_reported: bool,
}

impl SelectionDialogRetainedState {
    pub(crate) fn new(
        lease: SelectionDialogProjectionLease,
        resources: &PlatformTextRasterResources,
    ) -> Self {
        Self {
            identity: crate::render_model::UiStateId::next_for(
                crate::render_model::UiNodeKind::Modal,
            ),
            lease,
            resources: resources.clone(),
            painter: DialogPainter::new(resources),
            occurrence: 0,
            selected: None,
            inputs: HashMap::new(),
            restore_focus: None,
            modal_layer: None,
            deferred_input: Vec::new(),
            mounted: false,
            failure_reported: false,
        }
    }

    pub(crate) fn synchronize(
        &mut self,
        lease: SelectionDialogProjectionLease,
    ) -> Result<bool, SelectionDialogProjectionError> {
        if lease.revision <= self.lease.revision {
            return Err(SelectionDialogProjectionError::StaleRevision {
                current: self.lease.revision,
                received: lease.revision,
            });
        }
        self.selected = None;
        self.failure_reported = false;
        if !self.lease.open && lease.open {
            self.mounted = false;
        }
        self.lease = lease;
        Ok(true)
    }

    pub(crate) fn is_open(&self) -> bool {
        self.lease.open
    }

    pub(crate) fn capture_restore_focus(&mut self, context: &egui::Context) {
        if self.lease.open && !self.mounted && self.restore_focus.is_none() {
            self.restore_focus = context.memory(|memory| memory.focused());
        }
    }

    fn event(
        &mut self,
        events: &mut Vec<SelectionDialogEvent>,
        operation: SelectionDialogOperation,
    ) {
        self.occurrence = self.occurrence.saturating_add(1);
        events.push(SelectionDialogEvent {
            revision: self.lease.revision,
            occurrence: self.occurrence,
            operation,
        });
    }

    pub(crate) fn show(
        &mut self,
        context: &egui::Context,
        style: &TextCommandSurfaceStyle,
    ) -> Result<Option<SelectionDialogOutput>, String> {
        if !self.lease.open {
            return Ok(None);
        }
        let first = !self.mounted;
        if first {
            self.capture_restore_focus(context);
            self.mounted = true;
        }
        let mut operations = Vec::new();
        let mut events = Vec::new();
        let screen = context.content_rect();
        let title = self.lease.title.clone();
        let id = egui::Id::new(("kuc.selection-dialog", self.identity.as_str()));
        let close_operation = match &self.lease.projection {
            SelectionDialogProjection::File(_) => SelectionDialogOperation::Cancelled,
            SelectionDialogProjection::Search(_) => SelectionDialogOperation::Closed,
        };
        let mut projection = std::mem::replace(
            &mut self.lease.projection,
            SelectionDialogProjection::File(FileSelectionProjection {
                items: Vec::new(),
                failure: None,
            }),
        );
        let response = egui::Modal::new(id).show(context, |ui| -> Result<(), String> {
            ui.set_width((screen.width() - SCREEN_MARGIN).clamp(1.0, MAXIMUM_PANEL_WIDTH));
            self.painter.label(
                ui,
                &title,
                "kuc.dialog.title",
                true,
                false,
                false,
                style,
                &mut operations,
            )?;
            egui::ScrollArea::vertical()
                .max_height((screen.height() - RESERVED_CONTROL_HEIGHT).max(SCREEN_MARGIN))
                .show(ui, |ui| match &mut projection {
                    SelectionDialogProjection::File(files) => {
                        self.files(ui, files, style, &mut operations, &mut events)
                    }
                    SelectionDialogProjection::Search(search) => {
                        self.search(ui, search, style, first, &mut operations, &mut events)
                    }
                })
                .inner?;
            let cancel = self.lease.controls.cancel.clone();
            let response = self.painter.label(
                ui,
                &cancel,
                "kuc.dialog.cancel",
                true,
                true,
                false,
                style,
                &mut operations,
            )?;
            if activated(ui, &response) {
                self.event(&mut events, close_operation.clone());
                self.lease.open = false;
            }
            Ok(())
        });
        self.lease.projection = projection;
        self.modal_layer = Some(response.response.layer_id);
        let should_close = response.should_close();
        response.inner?;
        if should_close && self.lease.open {
            self.event(&mut events, close_operation);
            self.lease.open = false;
        }
        if !self.lease.open
            && let Some(id) = self.restore_focus
        {
            context.memory_mut(|memory| memory.request_focus(id));
            context.request_repaint();
        }
        operations.insert(
            0,
            fill(
                response.response.rect,
                screen,
                style.text_paint.background_rgba,
            ),
        );
        operations.insert(0, fill(screen, screen, [0, 0, 0, 100]));
        Ok(Some(SelectionDialogOutput {
            paint_plan: TextSurfacePaintPlan {
                surface_bounds: rect(screen),
                viewport_bounds: rect(screen),
                operations,
            },
            events,
        }))
    }
}
