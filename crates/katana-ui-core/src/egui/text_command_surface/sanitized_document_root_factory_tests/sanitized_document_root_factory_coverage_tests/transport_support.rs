const SEARCH_IME_KEYBOARD_TARGET: [u8; 2] = [9, 1];
const SEARCH_IME_COMMAND_TARGET: [u8; 2] = [9, 2];
const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const FLOATING_SURFACE_HORIZONTAL_OFFSET: f32 = 8.0;

use super::super::sanitized_command_projection::SanitizedCommandCapabilityRejection;
use super::super::sanitized_context_projection::SanitizedContextMenuCapabilityRejection;
use super::super::sanitized_document_root_transport::{
    SanitizedDocumentRootEventDispatchError, SanitizedDocumentRootEventForwardError,
};
use super::super::sanitized_search_projection::{
    SanitizedSearchCapabilityRejection, SanitizedSearchControlPresentation,
    SanitizedSearchLocalizedPresentation, SanitizedSearchOperationPresentation,
    SanitizedSearchProjectionBuilder, SanitizedSearchResultSummaryPresentation,
    SanitizedSearchTarget, SanitizedSearchTextOperation, SanitizedSearchTextPresentation,
    SanitizedSearchUnavailablePresentation, SanitizedSearchUnitOperation,
};
use super::super::sanitized_tab_projection::SanitizedTabGroupTarget;
use super::super::{
    SanitizedDocumentRootFactory, SanitizedDocumentRootFactoryError, SanitizedDocumentRootFrame,
};
use crate::egui::text_command_surface::KucRootEventBatchDispatcher;
use crate::egui::text_command_surface::{
    SanitizedContextMenuItem, SanitizedContextMenuProjection,
    SanitizedContextMenuProjectionBuilder, SanitizedContextMenuTarget,
    SanitizedDocumentRootEventForwarder, SanitizedDocumentRootEventTransport,
    SanitizedDocumentRootIdentity, SanitizedDocumentRootInput, SanitizedDocumentRootStyleKey,
    SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation, SanitizedTabGroup,
    SanitizedTabProjection, SanitizedTabTarget,
};
use std::cell::RefCell;
use std::rc::Rc;

struct RecordingForwarder {
    calls: usize,
    transport_debug: Option<String>,
}

struct RetainingForwarder {
    calls: usize,
    transport_debug: Option<String>,
    transport: Option<SanitizedDocumentRootEventTransport>,
}

#[derive(Default)]
struct TestRootDispatcher {
    reject_text: bool,
}

impl KucRootEventBatchDispatcher for TestRootDispatcher {
    type Error = ();

    fn dispatch_text_events(
        &mut self,
        _events: Vec<crate::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        if self.reject_text {
            Err(())
        } else {
            Ok(())
        }
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<crate::molecule::command_chrome::CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<crate::molecule::command_chrome::FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_search_events(
        &mut self,
        _events: Vec<crate::molecule::command_chrome::CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<crate::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl SanitizedDocumentRootEventForwarder for RecordingForwarder {
    type Error = ();

    fn forward_sanitized_document_root_event(
        &mut self,
        mut transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        self.transport_debug = Some(format!("{transport:?}"));
        transport
            .dispatch_root_once(&mut TestRootDispatcher::default())
            .map_err(|_| ())?;
        Ok(())
    }
}

impl RetainingForwarder {
    fn dispatch_root_once(
        &mut self,
    ) -> Result<
        crate::egui::text_command_surface::EguiTextCommandSurfaceRootEventDispatchReceipt,
        SanitizedDocumentRootEventDispatchError<()>,
    > {
        if let Some(transport) = self.transport.as_mut() {
            transport.dispatch_root_once(&mut TestRootDispatcher::default())
        } else {
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
        }
    }

    fn dispatch_root_once_rejecting_text(
        &mut self,
    ) -> Result<
        crate::egui::text_command_surface::EguiTextCommandSurfaceRootEventDispatchReceipt,
        SanitizedDocumentRootEventDispatchError<()>,
    > {
        if let Some(transport) = self.transport.as_mut() {
            transport.dispatch_root_once(&mut TestRootDispatcher { reject_text: true })
        } else {
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
        }
    }
}

impl SanitizedDocumentRootEventForwarder for RetainingForwarder {
    type Error = ();

    fn forward_sanitized_document_root_event(
        &mut self,
        transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        self.transport_debug = Some(format!("{transport:?}"));
        self.transport = Some(transport);
        Ok(())
    }
}

#[test]
fn retained_transport_propagates_a_real_child_dispatch_rejection() {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input(1, b"child-rejection", "本文"))
        .expect("retain succeeds");
    let context = egui::Context::default();
    let frame = run_root_frame_events(&context, &mut root, Vec::new()).1;
    let mut forwarder = RetainingForwarder {
        calls: 0,
        transport_debug: None,
        transport: None,
    };

    frame
        .forward_events_once(&mut forwarder)
        .expect("root forwarding succeeds");
    assert_eq!(
        forwarder.dispatch_root_once_rejecting_text(),
        Err(SanitizedDocumentRootEventDispatchError::Child(()))
    );
    assert_eq!(
        forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
    );
}

#[test]
fn retaining_forwarder_rejects_dispatch_before_transport_is_retained() {
    let mut forwarder = RetainingForwarder {
        calls: 0,
        transport_debug: None,
        transport: None,
    };
    assert_eq!(
        forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
    );
}

struct FailingForwarder {
    calls: usize,
}

impl SanitizedDocumentRootEventForwarder for FailingForwarder {
    type Error = ();

    fn forward_sanitized_document_root_event(
        &mut self,
        _transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        Err(())
    }
}
