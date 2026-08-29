//! Closed retained root contract for KUC text-command composition.

mod interaction_locator;
mod root_event;
mod root_frame;
#[path = "root/root_types.rs"]
mod root_types;

use super::source_address_projection_lease::SourceAddressProjectionLease;
use super::status_diagnostics_projection_lease::StatusDiagnosticsProjectionLease;
use super::tab_strip_projection_lease::TabStripProjectionLease;
use super::tab_strip_retained::TabStripRetainedState;
use super::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
    TextCommandSurfaceStyle,
};
use crate::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor,
};
use root_event::build_event_batch;
use root_frame::build_frame;

pub use interaction_locator::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueClickContinuation,
    KucOpaqueClickContinuationError, KucOpaqueInteractionRequest, KucOpaqueSearchTraceContinuation,
    KucOpaqueTextSelectionContinuation, KucSearchTraceContinuationError,
    KucTextSelectionContinuationError,
};
pub use root_event::{
    EguiTextCommandSurfaceRootEventBatch, EguiTextCommandSurfaceRootEventBatchDispatchError,
    EguiTextCommandSurfaceRootEventBatchForwardError, EguiTextCommandSurfaceRootEventChildClass,
    EguiTextCommandSurfaceRootEventClassDispatch, EguiTextCommandSurfaceRootEventDispatchReceipt,
    EguiTextCommandSurfaceRootEventForwardingReceipt, EguiTextCommandSurfaceRootEventTransport,
    KucOpaqueHostEffectAttachError, KucOpaqueHostEffectBatch, KucOpaqueHostEffectError,
    KucRootEffectRouter, KucRootEventBatchContext, KucRootEventBatchDispatcher,
    KucRootEventBatchForwarder,
};
pub use root_frame::{
    EguiTextCommandSurfaceRootAccessKitReference, EguiTextCommandSurfaceRootDimensions,
    EguiTextCommandSurfaceRootFrame,
};
pub use root_types::{
    EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootError, EguiTextCommandSurfaceRootOutput,
};

impl EguiTextCommandSurfaceRoot {
    pub(crate) fn evidence_catalog(&self) -> &katana_ui_core_text_raster::PlatformFontCatalog {
        &self.adapter.catalog
    }

    /// Creates a root with an identity derived from the retained text state id.
    pub fn new(surface: EguiTextCommandSurface) -> Result<Self, EguiTextCommandSurfaceError> {
        let identity = format!(
            "kuc.text-command-root/{}",
            surface.text().state().text_area.state_id.as_str()
        );
        Self::with_identity(identity, surface)
    }

    /// Creates a root with a caller-provided opaque, stable identity.
    pub fn with_identity(
        identity: impl Into<String>,
        surface: EguiTextCommandSurface,
    ) -> Result<Self, EguiTextCommandSurfaceError> {
        Ok(Self {
            surface,
            adapter: EguiTextCommandSurfaceAdapter::with_text_raster_config(
                katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
            )?,
            identity: identity.into(),
            state_revision: 0,
            frame_serial: 0,
            source_address_submission_port: None,
            tab_strip: None,
            status_bar: None,
            diagnostics_list: None,
        })
    }

    /// Creates a root whose retained text children use one catalog policy.
    pub fn with_text_raster_config(
        identity: impl Into<String>,
        surface: EguiTextCommandSurface,
        config: katana_ui_core_text_raster::PlatformTextRasterConfig,
    ) -> Result<Self, EguiTextCommandSurfaceError> {
        Ok(Self {
            surface,
            adapter: EguiTextCommandSurfaceAdapter::with_text_raster_config(config)?,
            identity: identity.into(),
            state_revision: 0,
            frame_serial: 0,
            source_address_submission_port: None,
            tab_strip: None,
            status_bar: None,
            diagnostics_list: None,
        })
    }

    /// Synchronizes generic controlled presentation without exposing child models.
    pub fn synchronize_presentation(
        &mut self,
        presentation: super::types::EguiTextCommandSurfacePresentation,
    ) -> bool {
        let changed = self.surface.synchronize_presentation(presentation);
        if changed {
            self.state_revision = self.state_revision.saturating_add(1);
        }
        changed
    }

    pub fn attach_source_address(&mut self, lease: SourceAddressProjectionLease) {
        let (strip, port) = lease.into_parts();
        self.surface.set_source_address(strip);
        self.source_address_submission_port = port;
    }

    /// Mounts the generic KUC status child into this retained root.
    pub(crate) fn attach_status_bar(&mut self, status_bar: katana_ui_core::molecule::StatusBar) {
        self.status_bar = Some(status_bar);
    }

    /// Mounts the generic KUC diagnostics child into this retained root.
    pub(crate) fn attach_diagnostics_list(
        &mut self,
        diagnostics_list: katana_ui_core::molecule::DiagnosticsList,
    ) {
        self.diagnostics_list = Some(diagnostics_list);
    }

    /// Consumes a generic child projection without exposing child models through the root API.
    pub fn attach_status_diagnostics(&mut self, lease: StatusDiagnosticsProjectionLease) {
        let (status_bar, diagnostics_list) = lease.into_parts();
        if let Some(status_bar) = status_bar {
            self.attach_status_bar(status_bar);
        }
        if let Some(diagnostics_list) = diagnostics_list {
            self.attach_diagnostics_list(diagnostics_list);
        }
    }

    pub(crate) fn attach_tab_strip(
        &mut self,
        lease: TabStripProjectionLease,
    ) -> Result<bool, EguiTextCommandSurfaceError> {
        self.tab_strip = Some(TabStripRetainedState::from_lease(
            lease,
            std::sync::Arc::clone(&self.adapter.catalog),
            self.adapter.text_raster_config.clone(),
        )?);
        Ok(true)
    }

    pub(crate) fn clear_tab_strip(&mut self) -> bool {
        self.tab_strip.take().is_some()
    }

    pub(crate) fn clear_status_diagnostics(&mut self) -> bool {
        let changed = self.status_bar.is_some() || self.diagnostics_list.is_some();
        self.status_bar = None;
        self.diagnostics_list = None;
        changed
    }

    pub(crate) fn synchronize_command_families(
        &mut self,
        primary: Option<katana_ui_core::molecule::command_chrome::CommandChromeFamilyId>,
        floating: Option<katana_ui_core::molecule::command_chrome::CommandChromeFamilyId>,
    ) -> bool {
        self.surface.synchronize_command_families(primary, floating)
    }

    /// Renders one actual root frame and returns only its closed frame/event contracts.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        style: &TextCommandSurfaceStyle,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        ui.ctx().enable_accesskit();
        self.frame_serial = self.frame_serial.saturating_add(1);
        let mut output = self.adapter.show_with_tab_strip(
            ui,
            &mut self.surface,
            style,
            self.tab_strip.as_mut(),
            self.status_bar.as_mut(),
            self.diagnostics_list.as_mut(),
        )?;
        let mut events =
            build_event_batch(&mut output, self.source_address_submission_port.clone())
                .map_err(EguiTextCommandSurfaceRootError::Serialization)?;
        if events.has_events() {
            self.state_revision = self.state_revision.saturating_add(1);
        }
        events.set_root_metadata(&self.identity, self.state_revision);
        let context = events.current_context();
        let bound_evidence = super::accesskit_evidence::bind_frame(
            output.accesskit_evidence.clone(),
            &self.identity,
            &context,
        );
        let plans = output.artifact_paint_plans()?;
        let composite = ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(output.root_bounds),
            plans: &plans,
        })?;
        let frame = build_frame(&self.identity, self.state_revision, &output, &composite)
            .map_err(EguiTextCommandSurfaceRootError::Serialization)?;
        let locator = interaction_locator::KucInteractionLocator::from_output(
            &self.identity,
            &context,
            self.frame_serial,
            &output,
            &bound_evidence,
        );
        let artifact_order = output.artifact_order().to_vec();
        Ok(EguiTextCommandSurfaceRootOutput {
            evidence_text: output.text,
            evidence_composite: composite,
            locator,
            artifact_order,
            frame,
            events,
            #[cfg(test)]
            toolbar_record: output.toolbar.map(|value| value.record),
            #[cfg(test)]
            floating: output.floating,
            #[cfg(test)]
            context_menu_record: output.context_menu.and_then(|value| value.record),
            #[cfg(test)]
            search_record: output.search.map(|value| value.record),
        })
    }
}

impl EguiTextCommandSurfaceRootOutput {
    #[must_use]
    pub const fn frame(&self) -> &EguiTextCommandSurfaceRootFrame {
        &self.frame
    }

    #[must_use]
    pub const fn events(&self) -> &EguiTextCommandSurfaceRootEventBatch {
        &self.events
    }

    /// Returns the final root-owned RGBA pixels for visual artifact encoding.
    ///
    /// The returned buffer is the already-composited root frame. Child paint
    /// plans, texture handles, and child geometry remain private to KUC.
    #[must_use]
    pub fn rgba_pixels(&self) -> &[u8] {
        &self.evidence_composite.rgba_pixels
    }

    #[must_use]
    pub const fn interaction_locator(&self) -> &interaction_locator::KucInteractionLocator {
        &self.locator
    }

    #[must_use]
    pub fn artifact_order(&self) -> &[super::types::EguiTextCommandSurfaceChild] {
        &self.artifact_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_menu::{ContextMenuPresentation, ContextMenuPresentationItem};
    use katana_ui_core::atom::{
        TextArea, TextAreaAction, TextAreaCompositionPhase, TextAreaSelection,
    };
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeAction, CommandChromeFamilyId, CommandChromeToolbar,
        FloatingCommandToolbarVisibility,
    };
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeSearchStrip, CommandChromeText, SearchControlStrings,
        SearchResultSummaryTemplate,
    };
    use katana_ui_core::molecule::structured::SearchControlStrip;
    use katana_ui_core::molecule::structured::source_address_strip::{
        SourceAddressPresentation, SourceAddressStrip,
    };
    use katana_ui_core::text_surface::TextSurfaceAction;
    use katana_ui_core::text_surface::{
        TextSurface, TextSurfaceAutomaticGutterPresentation, TextSurfaceProps, TextSurfaceViewport,
    };
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct CountingTabStripPort(Rc<Cell<usize>>);

    impl super::super::tab_strip_proposal_port::TabStripProposalPort for CountingTabStripPort {
        fn forward_proposal(
            &mut self,
            proposal: super::super::tab_strip_proposal_port::TabStripProposal,
        ) -> Result<(), super::super::tab_strip_proposal_port::TabStripProposalPortError> {
            proposal.consume_for_port();
            self.0.set(self.0.get().saturating_add(1));
            Ok(())
        }
    }

    struct NavigationTabStripPort(Rc<RefCell<Vec<bool>>>);

    impl super::super::tab_strip_proposal_port::TabStripProposalPort for NavigationTabStripPort {
        fn forward_proposal(
            &mut self,
            proposal: super::super::tab_strip_proposal_port::TabStripProposal,
        ) -> Result<(), super::super::tab_strip_proposal_port::TabStripProposalPortError> {
            let direction = proposal
                .navigation_direction_for_test()
                .expect("navigation renderer must forward only a navigation proposal");
            proposal.consume_for_port();
            self.0.borrow_mut().push(direction);
            Ok(())
        }
    }

    struct GroupCollapseTabStripPort(Rc<RefCell<Vec<bool>>>);

    impl super::super::tab_strip_proposal_port::TabStripProposalPort for GroupCollapseTabStripPort {
        fn forward_proposal(
            &mut self,
            proposal: super::super::tab_strip_proposal_port::TabStripProposal,
        ) -> Result<(), super::super::tab_strip_proposal_port::TabStripProposalPortError> {
            let collapsed = proposal
                .group_collapsed_for_test()
                .expect("group header must forward only a collapse proposal");
            proposal.consume_for_port();
            self.0.borrow_mut().push(collapsed);
            Ok(())
        }
    }

    struct TrailingTabStripPort(
        Rc<RefCell<Vec<super::super::tab_strip_proposal_port::TabStripProposalOperationClass>>>,
    );

    impl super::super::tab_strip_proposal_port::TabStripProposalPort for TrailingTabStripPort {
        fn forward_proposal(
            &mut self,
            proposal: super::super::tab_strip_proposal_port::TabStripProposal,
        ) -> Result<(), super::super::tab_strip_proposal_port::TabStripProposalPortError> {
            let class = proposal.operation_class_for_test();
            proposal.consume_for_port();
            self.0.borrow_mut().push(class);
            Ok(())
        }
    }

    fn collision_root() -> Result<EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceError> {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()).with_toolbar(
            CommandChromeToolbar::new().action(CommandChromeAction::new("base", "基準")),
        );
        EguiTextCommandSurfaceRoot::with_identity("collision-root", surface)
    }

    #[test]
    fn duplicate_command_family_is_rejected_before_render() -> Result<(), Box<dyn std::error::Error>>
    {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface())
            .with_toolbar(CommandChromeToolbar::new().action(CommandChromeAction::new("p", "P")))
            .with_floating_toolbar(
                CommandChromeToolbar::new().action(CommandChromeAction::new("f", "F")),
                FloatingCommandToolbarVisibility::Visible,
            );
        let mut root = EguiTextCommandSurfaceRoot::with_identity("duplicate-family", surface)?;
        let context = egui::Context::default();
        let style = TextCommandSurfaceStyle::standard()?;
        let mut result = None;
        let _ = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 360.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| result = Some(root.show(ui, &style)),
        );
        let error = result
            .ok_or_else(|| "root invocation missing".to_owned())?
            .err()
            .ok_or_else(|| "duplicate family unexpectedly rendered".to_owned())?;
        assert!(matches!(
            error,
            EguiTextCommandSurfaceRootError::Surface(
                EguiTextCommandSurfaceError::DuplicateCommandFamilyMount { .. }
            )
        ));

        Ok(())
    }

    #[test]
    fn source_address_lease_mounts_before_legacy_children_and_keeps_state_local()
    -> Result<(), Box<dyn std::error::Error>> {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface())
            .with_toolbar(CommandChromeToolbar::new().action(CommandChromeAction::new("p", "P")));
        let mut root = EguiTextCommandSurfaceRoot::with_identity("source-address-root", surface)?;
        root.attach_source_address(SourceAddressProjectionLease::new(SourceAddressStrip::new(
            SourceAddressPresentation::new("ソース", "ソースの説明", "ソース入力"),
        )));

        let output = render(&context_for_test(), &mut root)?;

        assert_eq!(
            output
                .events
                .current_context()
                .source_address_submission_count(),
            0
        );
        assert!(
            output
                .evidence_composite
                .rgba_pixels
                .iter()
                .any(|pixel| *pixel != 0)
        );

        Ok(())
    }

    #[test]
    fn tab_strip_click_forwards_one_proposal_without_locally_accepting_presentation()
    -> Result<(), Box<dyn std::error::Error>> {
        use crate::text_command_surface::{
            TabStripCorrelation, TabStripProjection, TabStripProjectionLease,
            TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
        };

        let count = Rc::new(Cell::new(0));
        let projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"root-test-correlation"),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"root-test-tab"),
                TabStripText::new("日本語 ⭐️"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
        );
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(CountingTabStripPort(Rc::clone(&count))),
        )?;

        let context = context_for_test();
        let initial = render(&context, &mut root)?;
        let tab_center = egui::pos2(40.0, 18.0);
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(tab_center, true)],
                ..egui::RawInput::default()
            },
        )?;
        let released = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(tab_center, false)],
                ..egui::RawInput::default()
            },
        )?;

        assert_eq!(1, count.get());
        assert_eq!(
            initial.evidence_composite.pixel_hash, released.evidence_composite.pixel_hash,
            "a proposal cannot mutate retained tab presentation before a newer host lease"
        );
        assert!(released.evidence_composite.non_transparent_pixel_count > 0);

        Ok(())
    }

    #[test]
    fn tab_strip_drag_uses_physical_pointer_and_forwards_one_start_then_one_destination()
    -> Result<(), Box<dyn std::error::Error>> {
        use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
        use crate::text_command_surface::{
            TabStripCorrelation, TabStripProjection, TabStripProjectionLease,
            TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
        };

        let forwarded = Rc::new(RefCell::new(Vec::new()));
        let projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"tab-drag-correlation"),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab-drag-source"),
                TabStripText::new("source ⭐️"),
            )
            .capabilities(
                TabStripTabCapabilities::new()
                    .selectable(true)
                    .draggable(true),
            ),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab-drag-destination"),
                TabStripText::new("destination ⭐️"),
            )
            .capabilities(
                TabStripTabCapabilities::new()
                    .selectable(true)
                    .accepts_tab_drop(true),
            ),
        );
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-drag-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
        )?;
        let context = context_for_test();
        context.enable_accesskit();
        let (initial_platform, initial) =
            render_with_platform_output(&context, &mut root, egui::RawInput::default())?;
        let (_, source, _) = accesskit_button(&initial_platform, "source ⭐️")?;
        let (_, destination, _) = accesskit_button(&initial_platform, "destination ⭐️")?;
        let source_pointer = source.center();
        let destination_pointer = egui::pos2(destination.max.x - 2.0, destination.center().y);

        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(source_pointer, true)],
                ..egui::RawInput::default()
            },
        )?;
        let dragging = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![egui::Event::PointerMoved(destination_pointer)],
                ..egui::RawInput::default()
            },
        )?;
        assert_ne!(
            initial.evidence_composite.pixel_hash, dragging.evidence_composite.pixel_hash,
            "an active drag must be visible in the root-owned artifact"
        );
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(destination_pointer),
                    pointer_button(destination_pointer, false),
                ],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(
            [
                TabStripProposalOperationClass::StartDrag,
                TabStripProposalOperationClass::FinishDragAfter,
            ],
            forwarded.borrow().as_slice(),
            "a drag must be a single start proposal followed by one opaque destination proposal"
        );

        Ok(())
    }

    #[test]
    fn tab_strip_drag_cancels_for_escape_or_a_host_rejected_destination()
    -> Result<(), Box<dyn std::error::Error>> {
        use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
        use crate::text_command_surface::{
            TabStripCorrelation, TabStripProjection, TabStripProjectionLease,
            TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
        };

        let projection = || {
            TabStripProjection::new(
                1,
                TabStripCorrelation::from_opaque_bytes(b"tab-drag-cancel-correlation"),
            )
            .tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"tab-drag-cancel-source"),
                    TabStripText::new("source ⭐️"),
                )
                .capabilities(
                    TabStripTabCapabilities::new()
                        .selectable(true)
                        .draggable(true),
                ),
            )
            .tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"tab-drag-rejected-destination"),
                    TabStripText::new("rejected ⭐️"),
                )
                .capabilities(TabStripTabCapabilities::new().selectable(true)),
            )
        };

        for escape in [false, true] {
            let forwarded = Rc::new(RefCell::new(Vec::new()));
            let mut root = EguiTextCommandSurfaceRoot::with_identity(
                format!("tab-strip-drag-cancel-root-{escape}"),
                EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
            )?;
            root.attach_tab_strip(
                TabStripProjectionLease::new(projection())
                    .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
            )?;
            let context = context_for_test();
            context.enable_accesskit();
            let (platform, _) =
                render_with_platform_output(&context, &mut root, egui::RawInput::default())?;
            let (_, source, _) = accesskit_button(&platform, "source ⭐️")?;
            let (_, rejected, _) = accesskit_button(&platform, "rejected ⭐️")?;
            let source_pointer = source.center();
            let rejected_pointer = rejected.center();
            let _ = render_with_input(
                &context,
                &mut root,
                egui::RawInput {
                    events: vec![pointer_button(source_pointer, true)],
                    ..egui::RawInput::default()
                },
            )?;
            let _ = render_with_input(
                &context,
                &mut root,
                egui::RawInput {
                    events: vec![egui::Event::PointerMoved(rejected_pointer)],
                    ..egui::RawInput::default()
                },
            )?;
            let end_event = if escape {
                key_press(egui::Key::Escape)
            } else {
                pointer_button(rejected_pointer, false)
            };
            let _ = render_with_input(
                &context,
                &mut root,
                egui::RawInput {
                    events: vec![egui::Event::PointerMoved(rejected_pointer), end_event],
                    ..egui::RawInput::default()
                },
            )?;
            assert_eq!(
                [
                    TabStripProposalOperationClass::StartDrag,
                    TabStripProposalOperationClass::CancelDrag,
                ],
                forwarded.borrow().as_slice(),
                "escape and a non-accepting destination must both cancel without a fallback reorder"
            );
        }

        Ok(())
    }

    #[test]
    fn tab_strip_drag_uses_only_host_projected_group_or_end_destinations()
    -> Result<(), Box<dyn std::error::Error>> {
        use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
        use crate::text_command_surface::{
            TabStripCorrelation, TabStripGroupCapabilities, TabStripGroupDescriptor,
            TabStripGroupTarget, TabStripProjection, TabStripProjectionLease,
            TabStripSurfaceCapabilities, TabStripTabCapabilities, TabStripTabDescriptor,
            TabStripTabTarget, TabStripText,
        };

        let cases = [
            (
                "group",
                TabStripProposalOperationClass::FinishDragInGroup,
                true,
            ),
            (
                "end",
                TabStripProposalOperationClass::FinishDragAtEnd,
                false,
            ),
        ];
        for (case, expected, group_destination) in cases {
            let forwarded = Rc::new(RefCell::new(Vec::new()));
            let mut projection = TabStripProjection::new(
                1,
                TabStripCorrelation::from_opaque_bytes(format!("tab-drag-{case}-correlation")),
            )
            .tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(format!("tab-drag-{case}-source")),
                    TabStripText::new("source ⭐️"),
                )
                .capabilities(
                    TabStripTabCapabilities::new()
                        .selectable(true)
                        .draggable(true),
                ),
            );
            if group_destination {
                projection = projection.group(
                    TabStripGroupDescriptor::new(
                        TabStripGroupTarget::from_opaque_bytes(b"tab-drag-group-destination"),
                        TabStripText::new("group ⭐️"),
                    )
                    .capabilities(
                        TabStripGroupCapabilities::new()
                            .collapsible(true)
                            .accepts_tab_drop(true),
                    ),
                );
            } else {
                projection = projection.capabilities(
                    TabStripSurfaceCapabilities::new().tab_drop_at_end_available(true),
                );
            }
            let mut root = EguiTextCommandSurfaceRoot::with_identity(
                format!("tab-strip-drag-{case}-root"),
                EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
            )?;
            root.attach_tab_strip(
                TabStripProjectionLease::new(projection)
                    .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
            )?;
            let context = context_for_test();
            context.enable_accesskit();
            let (platform, _) =
                render_with_platform_output(&context, &mut root, egui::RawInput::default())?;
            let (_, source, _) = accesskit_button(&platform, "source ⭐️")?;
            let destination = if group_destination {
                let (_, group, _) = accesskit_button(&platform, "group ⭐️")?;
                group.center()
            } else {
                egui::pos2(600.0, source.center().y)
            };
            let _ = render_with_input(
                &context,
                &mut root,
                egui::RawInput {
                    events: vec![pointer_button(source.center(), true)],
                    ..egui::RawInput::default()
                },
            )?;
            let _ = render_with_input(
                &context,
                &mut root,
                egui::RawInput {
                    events: vec![egui::Event::PointerMoved(destination)],
                    ..egui::RawInput::default()
                },
            )?;
            let _ = render_with_input(
                &context,
                &mut root,
                egui::RawInput {
                    events: vec![
                        egui::Event::PointerMoved(destination),
                        pointer_button(destination, false),
                    ],
                    ..egui::RawInput::default()
                },
            )?;
            assert_eq!(
                [TabStripProposalOperationClass::StartDrag, expected],
                forwarded.borrow().as_slice(),
                "the renderer must only use the destination capability projected by the host"
            );
        }

        Ok(())
    }

    #[test]
    fn tab_strip_context_menu_uses_secondary_input_accesskit_and_one_opaque_route()
    -> Result<(), Box<dyn std::error::Error>> {
        use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
        use crate::text_command_surface::{
            TabStripContextMenuPresentation, TabStripCorrelation, TabStripMenuEntry,
            TabStripMenuOperation, TabStripProjection, TabStripProjectionLease,
            TabStripTabDescriptor, TabStripTabTarget, TabStripText,
        };

        let forwarded = Rc::new(RefCell::new(Vec::new()));
        let projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"tab-context-menu-correlation"),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab-context-menu-target"),
                TabStripText::new("日本語 ⭐️"),
            )
            .context_menu(TabStripContextMenuPresentation::new().entry(
                TabStripMenuEntry::action(
                    TabStripText::new("閉じる ⭐️"),
                    TabStripText::new("閉じる ⭐️"),
                    TabStripMenuOperation::RequestClose,
                ),
            )),
        );
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-context-menu-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
        )?;
        let context = context_for_test();
        context.enable_accesskit();
        let initial = render(&context, &mut root)?;
        let tab = egui::pos2(40.0, 18.0);
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![secondary_pointer_button(tab, true)],
                ..egui::RawInput::default()
            },
        )?;
        let (opened_platform, opened) = render_with_platform_output(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![secondary_pointer_button(tab, false)],
                ..egui::RawInput::default()
            },
        )?;
        let (_, bounds, _disabled) = accesskit_button(&opened_platform, "閉じる ⭐️")?;
        assert_ne!(
            initial.evidence_composite.pixel_hash, opened.evidence_composite.pixel_hash,
            "the root artifact must include the actual foreground menu"
        );
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(bounds.center()),
                    pointer_button(bounds.center(), true),
                ],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(bounds.center()),
                    pointer_button(bounds.center(), false),
                ],
                ..egui::RawInput::default()
            },
        )?;
        assert!(bounds.width() > 0.0 && bounds.height() > 0.0);
        assert_eq!(
            &[TabStripProposalOperationClass::RequestClose],
            forwarded.borrow().as_slice(),
            "the menu must forward only the route table's opaque close proposal"
        );

        Ok(())
    }

    #[test]
    fn group_popup_rasters_rename_and_projects_host_swatch_without_local_confirmation()
    -> Result<(), Box<dyn std::error::Error>> {
        use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
        use crate::text_command_surface::{
            TabStripCorrelation, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
            TabStripGroupTarget, TabStripProjection, TabStripProjectionLease,
            TabStripSwatchDescriptor, TabStripSwatchTarget, TabStripText,
        };
        use katana_ui_core::molecule::RgbaColor;

        let forwarded = Rc::new(RefCell::new(Vec::new()));
        let projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"group-popup-correlation"),
        )
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group-popup-target"),
                TabStripText::new("グループ ⭐️"),
            )
            .swatch(
                TabStripSwatchDescriptor::new(
                    TabStripSwatchTarget::from_opaque_bytes(b"group-popup-swatch"),
                    RgbaColor::new(253, 211, 98, 255),
                )
                .accessibility_label(TabStripText::new("黄色 ⭐️")),
            )
            .popup(
                TabStripGroupPopupPresentation::new()
                    .rename_placeholder(TabStripText::new("グループ名 ⭐️")),
            ),
        );
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-group-popup-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
        )?;
        let context = context_for_test();
        context.enable_accesskit();
        let _ = render(&context, &mut root)?;
        let header = egui::pos2(40.0, 18.0);
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![secondary_pointer_button(header, true)],
                ..egui::RawInput::default()
            },
        )?;
        let (opened_platform, opened) = render_with_platform_output(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![secondary_pointer_button(header, false)],
                ..egui::RawInput::default()
            },
        )?;
        let input = accesskit_text_input(&opened_platform, "グループ名 ⭐️")?;
        assert!(input.width() > 0.0 && input.height() > 0.0);
        assert!(opened.evidence_composite.non_transparent_pixel_count > 0);
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(input.center(), true)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(input.center(), false)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![key_press(egui::Key::Enter)],
                ..egui::RawInput::default()
            },
        )?;
        assert!(
            forwarded.borrow().is_empty(),
            "an unchanged group name must not create a rename proposal"
        );
        let preedit = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![egui::Event::Ime(egui::ImeEvent::Preedit {
                    text: "変更 ⭐️".to_owned(),
                    active_range_chars: None,
                })],
                ..egui::RawInput::default()
            },
        )?;
        assert_ne!(
            opened.evidence_composite.pixel_hash, preedit.evidence_composite.pixel_hash,
            "IME preedit must be platform-rastered in the foreground popup"
        );
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![egui::Event::Ime(egui::ImeEvent::Commit(
                    "変更 ⭐️".to_owned(),
                ))],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![key_press(egui::Key::Enter)],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(
            &[TabStripProposalOperationClass::RenameGroup],
            forwarded.borrow().as_slice(),
            "rename must leave KUC only as its one-shot opaque group proposal"
        );

        Ok(())
    }

    #[test]
    fn group_popup_swatch_uses_its_host_projected_color_and_forwards_recolor_once()
    -> Result<(), Box<dyn std::error::Error>> {
        use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
        use crate::text_command_surface::{
            TabStripCorrelation, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
            TabStripGroupTarget, TabStripProjection, TabStripProjectionLease,
            TabStripSwatchDescriptor, TabStripSwatchTarget, TabStripText,
        };
        use katana_ui_core::molecule::RgbaColor;

        let forwarded = Rc::new(RefCell::new(Vec::new()));
        let projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"group-swatch-correlation"),
        )
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group-swatch-target"),
                TabStripText::new("色 ⭐️"),
            )
            .swatch(
                TabStripSwatchDescriptor::new(
                    TabStripSwatchTarget::from_opaque_bytes(b"group-swatch-target"),
                    RgbaColor::new(17, 177, 127, 255),
                )
                .accessibility_label(TabStripText::new("緑 ⭐️")),
            )
            .popup(
                TabStripGroupPopupPresentation::new()
                    .rename_placeholder(TabStripText::new("色のグループ ⭐️")),
            ),
        );
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-group-swatch-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
        )?;
        let context = context_for_test();
        context.enable_accesskit();
        let _ = render(&context, &mut root)?;
        let header = egui::pos2(40.0, 18.0);
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![secondary_pointer_button(header, true)],
                ..egui::RawInput::default()
            },
        )?;
        let (opened_platform, _) = render_with_platform_output(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![secondary_pointer_button(header, false)],
                ..egui::RawInput::default()
            },
        )?;
        let (node, swatch, _) = accesskit_button(&opened_platform, "緑 ⭐️")?;
        let input = accesskit_text_input(&opened_platform, "色のグループ ⭐️")?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(input.center()),
                    pointer_button(input.center(), true),
                ],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(input.center()),
                    pointer_button(input.center(), false),
                ],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![egui::Event::Ime(egui::ImeEvent::Commit(
                    "変更 ⭐️".to_owned(),
                ))],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![key_press(egui::Key::Enter), accesskit_click(node)],
                ..egui::RawInput::default()
            },
        )?;
        assert!(swatch.width() > 0.0 && swatch.height() > 0.0);
        assert_eq!(
            &[TabStripProposalOperationClass::RecolorGroup],
            forwarded.borrow().as_slice(),
            "same-frame recolor must take precedence over a pending rename proposal"
        );

        Ok(())
    }

    #[test]
    fn tab_strip_group_header_forwards_collapse_without_locally_changing_its_presentation()
    -> Result<(), Box<dyn std::error::Error>> {
        use crate::text_command_surface::{
            TabStripCorrelation, TabStripGroupCapabilities, TabStripGroupDescriptor,
            TabStripGroupTarget, TabStripProjection, TabStripProjectionLease, TabStripText,
        };

        let forwarded = Rc::new(RefCell::new(Vec::new()));
        let projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"group-collapse-correlation"),
        )
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group"),
                TabStripText::new("グループ ⭐️"),
            )
            .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
        );
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-group-collapse-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(GroupCollapseTabStripPort(Rc::clone(&forwarded))),
        )?;

        let context = context_for_test();
        let initial = render(&context, &mut root)?;
        let group_header = egui::pos2(40.0, 18.0);
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(group_header, true)],
                ..egui::RawInput::default()
            },
        )?;
        let released = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(group_header, false)],
                ..egui::RawInput::default()
            },
        )?;

        assert_eq!(vec![true], *forwarded.borrow());
        assert_eq!(
            initial.evidence_composite.pixel_hash, released.evidence_composite.pixel_hash,
            "a group-collapse proposal cannot mutate retained presentation before host republish"
        );

        Ok(())
    }

    #[test]
    fn tab_strip_active_reveal_changes_only_retained_scroll_artifact()
    -> Result<(), Box<dyn std::error::Error>> {
        use crate::text_command_surface::{
            TabStripCorrelation, TabStripProjection, TabStripProjectionLease,
            TabStripScrollPresentation, TabStripTabCapabilities, TabStripTabDescriptor,
            TabStripTabTarget, TabStripText,
        };

        let projection = |reveal_active| {
            let mut projection = TabStripProjection::new(
                1,
                TabStripCorrelation::from_opaque_bytes(b"active-reveal-correlation"),
            )
            .scroll_presentation(
                TabStripScrollPresentation::new().request_active_reveal(reveal_active),
            );
            for index in 0..6 {
                projection = projection.tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(format!("tab-{index}").into_bytes()),
                        TabStripText::new(format!("長いタブ {index} ⭐️")),
                    )
                    .capabilities(TabStripTabCapabilities::new().active(index == 5)),
                );
            }
            projection
        };
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-active-reveal-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(TabStripProjectionLease::new(projection(false)))?;
        let context = context_for_test();
        let before = render_with_input_at_size(
            &context,
            &mut root,
            egui::RawInput {
                time: Some(1.0),
                ..egui::RawInput::default()
            },
            egui::vec2(180.0, 360.0),
        )?;

        root.attach_tab_strip(TabStripProjectionLease::new(projection(true)))?;
        let revealed = render_with_input_at_size(
            &context,
            &mut root,
            egui::RawInput {
                time: Some(2.0),
                ..egui::RawInput::default()
            },
            egui::vec2(180.0, 360.0),
        )?;
        let settled = render_with_input_at_size(
            &context,
            &mut root,
            egui::RawInput {
                time: Some(3.0),
                ..egui::RawInput::default()
            },
            egui::vec2(180.0, 360.0),
        )?;
        let visible = render_with_input_at_size(
            &context,
            &mut root,
            egui::RawInput {
                time: Some(4.0),
                ..egui::RawInput::default()
            },
            egui::vec2(180.0, 360.0),
        )?;

        assert_ne!(
            before.evidence_composite.pixel_hash, visible.evidence_composite.pixel_hash,
            "active reveal must move the clipped retained tab artifact under constrained width"
        );
        assert!(
            revealed.evidence_composite.non_transparent_pixel_count > 0
                && settled.evidence_composite.non_transparent_pixel_count > 0
                && visible.evidence_composite.non_transparent_pixel_count > 0
        );

        Ok(())
    }

    #[test]
    fn tab_strip_manual_horizontal_scroll_changes_artifact_without_a_proposal()
    -> Result<(), Box<dyn std::error::Error>> {
        use crate::text_command_surface::{
            TabStripCorrelation, TabStripProjection, TabStripProjectionLease,
            TabStripTabDescriptor, TabStripTabTarget, TabStripText,
        };

        let mut projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"manual-scroll-correlation"),
        );
        for index in 0..6 {
            projection = projection.tab(TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(format!("tab-{index}").into_bytes()),
                TabStripText::new(format!("手動スクロール {index} ⭐️")),
            ));
        }
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-manual-scroll-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(TabStripProjectionLease::new(projection))?;
        let context = context_for_test();
        let before = render_with_input_at_size(
            &context,
            &mut root,
            egui::RawInput {
                time: Some(1.0),
                ..egui::RawInput::default()
            },
            egui::vec2(180.0, 360.0),
        )?;
        let _ = render_with_input_at_size(
            &context,
            &mut root,
            egui::RawInput {
                time: Some(2.0),
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(80.0, 18.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(-160.0, 0.0),
                        phase: egui::TouchPhase::Move,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                ..egui::RawInput::default()
            },
            egui::vec2(180.0, 360.0),
        )?;
        let visible = render_with_input_at_size(
            &context,
            &mut root,
            egui::RawInput {
                time: Some(3.0),
                ..egui::RawInput::default()
            },
            egui::vec2(180.0, 360.0),
        )?;

        assert_ne!(
            before.evidence_composite.pixel_hash, visible.evidence_composite.pixel_hash,
            "horizontal wheel input must change only the clipped retained tab artifact"
        );

        Ok(())
    }

    #[test]
    fn tab_strip_navigation_forwards_enabled_previous_and_rejects_disabled_next()
    -> Result<(), Box<dyn std::error::Error>> {
        use crate::text_command_surface::{
            TabStripControlPresentation, TabStripCorrelation, TabStripNavigationPresentation,
            TabStripProjection, TabStripProjectionLease, TabStripSurfaceCapabilities, TabStripText,
        };

        let directions = Rc::new(RefCell::new(Vec::new()));
        let projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"navigation-correlation"),
        )
        .capabilities(TabStripSurfaceCapabilities::new().previous_available(true))
        .navigation(TabStripNavigationPresentation::new(
            TabStripControlPresentation::new(
                TabStripText::new("Previous"),
                TabStripText::new("Previous tab"),
            ),
            TabStripControlPresentation::new(
                TabStripText::new("Next"),
                TabStripText::new("Next tab"),
            ),
        ));
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-navigation-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(NavigationTabStripPort(Rc::clone(&directions))),
        )?;

        let context = context_for_test();
        let _ = render(&context, &mut root)?;
        /* WHY: Navigation is fixed to the trailing 64 px of the 640 px root frame. */
        let previous = egui::pos2(594.0, 18.0);
        let next = egui::pos2(626.0, 18.0);
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(previous, true)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(previous, false)],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(&[true], directions.borrow().as_slice());

        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(next, true)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(next, false)],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(&[true], directions.borrow().as_slice());

        Ok(())
    }

    #[test]
    fn tab_strip_accesskit_keyboard_and_same_frame_activation_forward_only_current_routes()
    -> Result<(), Box<dyn std::error::Error>> {
        use crate::text_command_surface::{
            TabStripControlPresentation, TabStripCorrelation, TabStripNavigationPresentation,
            TabStripProjection, TabStripProjectionLease, TabStripSurfaceCapabilities, TabStripText,
        };

        let directions = Rc::new(RefCell::new(Vec::new()));
        let projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"accesskit-navigation-correlation"),
        )
        .capabilities(TabStripSurfaceCapabilities::new().previous_available(true))
        .navigation(TabStripNavigationPresentation::new(
            TabStripControlPresentation::new(
                TabStripText::new("Previous"),
                TabStripText::new("Previous tab"),
            ),
            TabStripControlPresentation::new(
                TabStripText::new("Next"),
                TabStripText::new("Next tab"),
            ),
        ));
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-accesskit-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(NavigationTabStripPort(Rc::clone(&directions))),
        )?;

        let context = context_for_test();
        context.enable_accesskit();
        let (initial_platform, initial) =
            render_with_platform_output(&context, &mut root, egui::RawInput::default())?;
        let (previous_node, previous_bounds, previous_disabled) =
            accesskit_button(&initial_platform, "Previous tab")?;
        let (next_node, _, next_disabled) = accesskit_button(&initial_platform, "Next tab")?;
        assert!(!previous_disabled);
        assert!(next_disabled);
        assert!(previous_bounds.width() > 0.0 && previous_bounds.height() > 0.0);
        assert_eq!(64, initial.frame().accessibility().snapshot_hash().len());
        let public_frame = format!("{:?}", initial.frame());
        for forbidden in ["Previous tab", "accesskit-navigation-correlation"] {
            assert!(!public_frame.contains(forbidden));
        }

        let (accesskit_platform, _) = render_with_platform_output(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![accesskit_click(previous_node)],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(&[true], directions.borrow().as_slice());
        assert!(
            accesskit_platform
                .platform_output
                .accesskit_update
                .is_some()
        );

        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(previous_bounds.center(), true)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(previous_bounds.center(), false)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![key_press(egui::Key::Enter)],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(&[true, true, true], directions.borrow().as_slice());

        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(previous_bounds.center(), true)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![
                    pointer_button(previous_bounds.center(), false),
                    key_press(egui::Key::Enter),
                ],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(
            &[true, true, true, true],
            directions.borrow().as_slice(),
            "pointer and keyboard activation in one frame must forward one proposal"
        );

        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![accesskit_click(next_node)],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(
            &[true, true, true, true],
            directions.borrow().as_slice(),
            "a disabled current-frame route must reject an AccessKit click"
        );

        Ok(())
    }

    #[test]
    fn tab_strip_trailing_control_maps_close_and_pinned_to_distinct_proposals()
    -> Result<(), Box<dyn std::error::Error>> {
        use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
        use crate::text_command_surface::{
            TabStripControlPresentation, TabStripCorrelation, TabStripProjection,
            TabStripProjectionLease, TabStripTabCapabilities, TabStripTabDescriptor,
            TabStripTabTarget, TabStripText,
        };

        let normal = TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"closable"),
            TabStripText::new("A"),
        )
        .capabilities(TabStripTabCapabilities::new().closeable(true))
        .trailing_control(TabStripControlPresentation::new(
            TabStripText::new("Close"),
            TabStripText::new("Close tab"),
        ));
        let pinned = TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"pinned"),
            TabStripText::new("B"),
        )
        .capabilities(TabStripTabCapabilities::new().pinned(true))
        .trailing_control(TabStripControlPresentation::new(
            TabStripText::new("Unpin"),
            TabStripText::new("Unpin tab"),
        ));
        let forwarded = Rc::new(RefCell::new(Vec::new()));
        let projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"trailing-correlation"),
        )
        .tab(normal)
        .tab(pinned);
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "tab-strip-trailing-root",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
        )?;

        let context = context_for_test();
        let _ = render(&context, &mut root)?;
        let close = egui::pos2(48.0, 18.0);
        let unpin = egui::pos2(108.0, 18.0);
        for position in [close, unpin] {
            let _ = render_with_input(
                &context,
                &mut root,
                egui::RawInput {
                    events: vec![pointer_button(position, true)],
                    ..egui::RawInput::default()
                },
            )?;
            let _ = render_with_input(
                &context,
                &mut root,
                egui::RawInput {
                    events: vec![pointer_button(position, false)],
                    ..egui::RawInput::default()
                },
            )?;
        }
        assert_eq!(
            [
                TabStripProposalOperationClass::RequestClose,
                TabStripProposalOperationClass::Unpin,
            ],
            forwarded.borrow().as_slice(),
        );

        Ok(())
    }

    #[test]
    fn distinct_command_families_render_once_in_their_slots()
    -> Result<(), Box<dyn std::error::Error>> {
        let surface = EguiTextCommandSurface::new(selected_surface())
            .with_toolbar(
                CommandChromeToolbar::new()
                    .command_family(CommandChromeFamilyId::new("primary"))
                    .action(CommandChromeAction::new("p", "P")),
            )
            .with_floating_toolbar(
                CommandChromeToolbar::new()
                    .command_family(CommandChromeFamilyId::new("floating"))
                    .action(CommandChromeAction::new("f", "F")),
                FloatingCommandToolbarVisibility::Visible,
            );
        let mut root = EguiTextCommandSurfaceRoot::with_identity("distinct-families", surface)?;
        let output = render(&context_for_test(), &mut root)?;
        assert_eq!(
            output.toolbar_record.as_ref().map(|record| {
                record
                    .actions
                    .iter()
                    .filter(|action| action.action_id == "p")
                    .count()
            }),
            Some(1)
        );
        assert_eq!(
            output
                .floating
                .as_ref()
                .and_then(|value| value.record.as_ref())
                .map(|record| {
                    record
                        .toolbar
                        .actions
                        .iter()
                        .filter(|action| action.action_id == "f")
                        .count()
                }),
            Some(1)
        );

        Ok(())
    }

    #[test]
    fn retained_family_update_preserves_text_selection_scroll_focus_and_composition()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            "retained-family-update",
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        let _ = root.surface.text.apply_action(TextSurfaceAction::ScrollBy {
            delta_x: 0,
            delta_y: 24,
        });
        let _ = root
            .surface
            .text
            .apply_action(TextSurfaceAction::SetFocus(true));
        let _ =
            root.surface
                .text
                .apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
                    TextAreaSelection { start: 1, end: 3 },
                )));
        let _ = root.surface.text.apply_action(TextSurfaceAction::TextArea(
            TextAreaAction::composition(TextAreaCompositionPhase::Update, "入力中⭐️", 3),
        ));
        let before = root.surface.text.state().clone();

        assert!(root.synchronize_command_families(
            Some(CommandChromeFamilyId::new("primary-next")),
            Some(CommandChromeFamilyId::new("floating-next")),
        ));
        assert_eq!(root.surface.text.state(), &before);

        Ok(())
    }

    #[test]
    fn floating_only_surface_remains_supported() -> Result<(), Box<dyn std::error::Error>> {
        let surface = EguiTextCommandSurface::new(selected_surface()).with_floating_toolbar(
            CommandChromeToolbar::new().action(CommandChromeAction::new("f", "F")),
            FloatingCommandToolbarVisibility::Visible,
        );
        let mut root = EguiTextCommandSurfaceRoot::with_identity("floating-only", surface)?;
        let output = render(&context_for_test(), &mut root)?;
        assert!(output.toolbar_record.is_none());
        assert!(output.floating.and_then(|value| value.record).is_some());

        Ok(())
    }

    struct EguiTextSurfaceForTest;

    impl EguiTextSurfaceForTest {
        fn surface() -> katana_ui_core::text_surface::TextSurface {
            let mut props = TextSurfaceProps::new(
                TextArea::new("collision-text").value("本文 ⭐️"),
                Vec::new(),
                TextSurfaceViewport::new(0, 0, 640, 360),
            );
            props.accessibility_label = "collision text".to_owned();
            TextSurface::new(props)
        }
    }

    fn render(
        context: &egui::Context,
        root: &mut EguiTextCommandSurfaceRoot,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        render_with_input(context, root, egui::RawInput::default())
    }

    fn render_with_input(
        context: &egui::Context,
        root: &mut EguiTextCommandSurfaceRoot,
        input: egui::RawInput,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        render_with_input_at_size(context, root, input, egui::vec2(640.0, 360.0))
    }

    fn render_with_input_at_size(
        context: &egui::Context,
        root: &mut EguiTextCommandSurfaceRoot,
        input: egui::RawInput,
        size: egui::Vec2,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        let style = TextCommandSurfaceStyle::standard()?;
        let mut output = None;
        let _ = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..input
            },
            |ui| output = Some(root.show(ui, &style)),
        );
        output.ok_or_else(|| {
            EguiTextCommandSurfaceRootError::Serialization("root frame missing".to_owned())
        })?
    }

    fn render_with_platform_output(
        context: &egui::Context,
        root: &mut EguiTextCommandSurfaceRoot,
        input: egui::RawInput,
    ) -> Result<(egui::FullOutput, EguiTextCommandSurfaceRootOutput), EguiTextCommandSurfaceRootError>
    {
        let style = TextCommandSurfaceStyle::standard()?;
        let mut root_output = None;
        let platform_output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 360.0),
                )),
                ..input
            },
            |ui| root_output = Some(root.show(ui, &style)),
        );
        Ok((
            platform_output,
            root_output.ok_or_else(|| {
                EguiTextCommandSurfaceRootError::Serialization("root frame missing".to_owned())
            })??,
        ))
    }

    fn selected_surface() -> katana_ui_core::text_surface::TextSurface {
        let value = "選択範囲 ⭐️";
        let mut props = TextSurfaceProps::new(
            TextArea::new("selected-text").value(value),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 640, 360),
        );
        props.accessibility_label = "selected text".to_owned();
        let mut presentation =
            katana_ui_core::text_surface::TextSurfacePresentation::from_props(&props);
        presentation.selection_start = 0;
        presentation.selection_end = value.len();
        let mut surface = TextSurface::new(props);
        assert!(surface.synchronize_presentation(presentation));
        surface
    }

    fn context_for_test() -> egui::Context {
        egui::Context::default()
    }

    fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn secondary_pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Secondary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn key_press(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn accesskit_click(node: egui::accesskit::NodeId) -> egui::Event {
        egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
            action: egui::accesskit::Action::Click,
            target_tree: egui::accesskit::TreeId::ROOT,
            target_node: node,
            data: None,
        })
    }

    fn accesskit_button(
        output: &egui::FullOutput,
        label: &str,
    ) -> Result<(egui::accesskit::NodeId, egui::Rect, bool), String> {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .and_then(|update| {
                update.nodes.iter().find_map(|(node_id, node)| {
                    (node.role() == egui::accesskit::Role::Button && node.label() == Some(label))
                        .then(|| {
                            node.bounds().map(|bounds| {
                                (
                                    *node_id,
                                    egui::Rect::from_min_max(
                                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                                    ),
                                    node.is_disabled(),
                                )
                            })
                        })
                        .flatten()
                })
            })
            .ok_or_else(|| format!("current frame lacks AccessKit button: {label}"))
    }

    fn accesskit_text_input(output: &egui::FullOutput, label: &str) -> Result<egui::Rect, String> {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .and_then(|update| {
                update.nodes.iter().find_map(|(_, node)| {
                    (node.role() == egui::accesskit::Role::TextInput && node.label() == Some(label))
                        .then(|| {
                            node.bounds().map(|bounds| {
                                egui::Rect::from_min_max(
                                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                                )
                            })
                        })
                        .flatten()
                })
            })
            .ok_or_else(|| format!("current frame lacks AccessKit text input: {label}"))
    }

    fn search_strip() -> CommandChromeSearchStrip {
        let text = |label: &str| CommandChromeText::new(label, label, label);
        CommandChromeSearchStrip::new(
            SearchControlStrip::new("検索")
                .query("検索語")
                .replace_mode(katana_ui_core::molecule::structured::ReplaceMode::Visible)
                .replace_value("置換語")
                .result_position(2, Some(0)),
            SearchControlStrings {
                strip: text("検索"),
                query: text("検索語"),
                replace: text("置換"),
                match_case: text("大文字小文字"),
                whole_word: text("単語"),
                use_regex: text("正規表現"),
                previous: text("前へ"),
                next: text("次へ"),
                replace_one: text("置換"),
                replace_all: text("すべて置換"),
                close: text("閉じる"),
                result_summary: SearchResultSummaryTemplate {
                    empty: "検索結果なし".into(),
                    zero_results: "0".into(),
                    single_result: "1".into(),
                    indexed_result: "{active} / {count}".into(),
                    count_results: "{count}".into(),
                },
            },
        )
    }

    #[test]
    fn retained_root_shares_one_catalog_across_all_text_children()
    -> Result<(), Box<dyn std::error::Error>> {
        let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface())
            .with_toolbar(
                CommandChromeToolbar::new().action(CommandChromeAction::new("base", "基準 ⭐️")),
            )
            .with_search_strip(search_strip())
            .with_context_menu(ContextMenuPresentation {
                visible: false,
                items: vec![ContextMenuPresentationItem::action("copy", "コピー")],
            });
        let mut root = EguiTextCommandSurfaceRoot::with_text_raster_config(
            "shared-catalog-root",
            surface,
            katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
        )?;
        let context = egui::Context::default();
        let _ = render(&context, &mut root)?;

        let root_catalog = root.evidence_catalog();
        assert_eq!(root_catalog.stats().font_database_discoveries, 1);
        let text_catalog = root.adapter.text.catalog();
        let chrome_catalog = root.adapter.chrome.catalog();
        let menu_catalog = root
            .adapter
            .context_menu
            .as_ref()
            .expect("context-menu child is instantiated by the real frame")
            .catalog();
        assert!(std::sync::Arc::ptr_eq(&text_catalog, &chrome_catalog));
        assert!(std::sync::Arc::ptr_eq(&text_catalog, &menu_catalog));
        assert!(std::sync::Arc::ptr_eq(&text_catalog, &root.adapter.catalog));
        assert_eq!(text_catalog.stats().font_database_discoveries, 1);
        assert_eq!(chrome_catalog.stats().font_database_discoveries, 1);
        assert_eq!(menu_catalog.stats().font_database_discoveries, 1);
        assert_eq!(text_catalog.fingerprint(), root_catalog.fingerprint());
        assert_eq!(chrome_catalog.fingerprint(), root_catalog.fingerprint());
        assert_eq!(menu_catalog.fingerprint(), root_catalog.fingerprint());

        Ok(())
    }

    #[test]
    fn actual_root_shares_one_metrics_frame_across_text_slots_and_scales()
    -> Result<(), Box<dyn std::error::Error>> {
        let value = "本文 日本語 ⭐️\n二行目";
        let mut surface = selected_surface();
        let mut presentation =
            katana_ui_core::text_surface::TextSurfacePresentation::from_props(surface.props());
        presentation.value = value.to_owned();
        presentation.selection_start = 0;
        presentation.selection_end = value.len();
        presentation.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
        assert!(surface.synchronize_presentation(presentation));
        let surface = EguiTextCommandSurface::new(surface)
            .with_toolbar(
                CommandChromeToolbar::new()
                    .command_family(CommandChromeFamilyId::new("metrics-primary"))
                    .action(CommandChromeAction::new("base", "本文 ⭐️")),
            )
            .with_floating_toolbar(
                CommandChromeToolbar::new()
                    .command_family(CommandChromeFamilyId::new("metrics-floating"))
                    .action(CommandChromeAction::new("float", "選択")),
                FloatingCommandToolbarVisibility::Visible,
            )
            .with_search_strip(search_strip())
            .with_context_menu(ContextMenuPresentation {
                visible: true,
                items: vec![ContextMenuPresentationItem::action(
                    "context-format",
                    "整形 ⭐️",
                )],
            });
        let mut root = EguiTextCommandSurfaceRoot::with_identity("metrics-frame", surface)?;
        let context = context_for_test();

        let initial = render(&context, &mut root)?;
        let mut context_input = egui::RawInput::default();
        initial
            .interaction_locator()
            .request_context_open()
            .expect("context menu opener is exposed by the actual root frame")
            .apply_to_raw_input_once(&mut context_input)
            .expect("context menu opener request is one-shot");
        let opened = render_with_input(&context, &mut root, context_input)?;
        assert!(opened.context_menu_record.is_some());
        let first = root.adapter.metrics.borrow().clone();
        assert!(first.records().iter().any(|metric| metric.text == value));
        assert!(first.records().iter().any(|metric| metric.text == "1"));
        assert!(first.records().iter().any(|metric| metric.text == "検索語"));
        assert!(
            first
                .records()
                .iter()
                .any(|metric| metric.text == "本文 ⭐️")
        );
        assert!(first.records().iter().any(|metric| metric.text == "選択"));
        assert!(
            first
                .records()
                .iter()
                .any(|metric| metric.text == "整形 ⭐️")
        );
        assert!(
            first
                .records()
                .iter()
                .any(|metric| metric.text.contains("⭐️") && !metric.text.contains('☆'))
        );

        let _ = render(&context, &mut root)?;
        assert!(
            root.adapter
                .metrics
                .borrow()
                .records()
                .iter()
                .all(|metric| metric.scale_factor == 1.0)
        );

        context.set_pixels_per_point(2.0);
        let _ = render(&context, &mut root)?;
        let scaled = root.adapter.metrics.borrow().clone();
        assert!(
            scaled
                .records()
                .iter()
                .all(|metric| metric.scale_factor == 2.0)
        );
        assert_ne!(scaled, first);

        Ok(())
    }

    #[test]
    fn actual_root_same_bounds_is_fail_closed_without_input_or_effect_dispatch()
    -> Result<(), Box<dyn std::error::Error>> {
        let context = egui::Context::default();
        context.enable_accesskit();
        let mut root = collision_root()?;
        let output = render(&context, &mut root)?;
        let before_context = output.events().current_context();
        let input = egui::RawInput::default();
        for identity in ["collision-left", "collision-right"] {
            assert!(matches!(
                output
                    .interaction_locator()
                    .request(KucInteractionSelector::new(
                        identity,
                        KucInteractionActionClass::Toolbar,
                    )),
                Err(KucInteractionLocatorError::Ambiguous)
            ));
            assert!(input.events.is_empty());
        }
        let unknown = output
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "collision-unknown",
                KucInteractionActionClass::Toolbar,
            ));
        assert!(matches!(unknown, Err(KucInteractionLocatorError::Missing)));
        assert!(input.events.is_empty());
        assert_eq!(output.events().current_context(), before_context);

        let effect_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let effect_count_for_handler = effect_count.clone();
        let effect = KucOpaqueHostEffectBatch::from_handler(move || {
            effect_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        });
        output
            .events()
            .attach_opaque_host_effect_batch(effect)
            .expect("effect attached to untouched batch");
        assert_eq!(effect_count.load(std::sync::atomic::Ordering::SeqCst), 0);

        Ok(())
    }
}
