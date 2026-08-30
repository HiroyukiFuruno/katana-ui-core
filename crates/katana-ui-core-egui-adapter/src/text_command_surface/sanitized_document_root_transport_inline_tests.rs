use super::{
    SanitizedCommandActivationTransport, SanitizedContextMenuActivationTransport,
    SanitizedDocumentRootEventDispatchError, SanitizedDocumentRootEventForwarder,
    SanitizedDocumentRootEventTransport, SanitizedSearchEventTransport,
    SanitizedTabProjectionClosedEvent, forward_root_events_once,
};
use crate::text_command_surface::{
    EguiTextCommandSurfaceRootFactory, FullTextCommandSurfaceScenarioFactory,
    FullTextCommandSurfaceScenarioId, KucRootEventBatchDispatcher, SanitizedDocumentRootFactory,
    SanitizedDocumentRootIdentity, SanitizedDocumentRootInput, SanitizedDocumentRootStyleKey,
};
use std::cell::RefCell;

const ROOT_VIEWPORT_SIZE: egui::Vec2 = egui::vec2(640.0, 480.0);

fn required<'a>(source: &'a str, marker: &str) -> &'a str {
    match source.split_once(marker) {
        Some((_, remainder)) => remainder,
        None => panic!("source marker is missing: {marker}"),
    }
}

fn before<'a>(source: &'a str, marker: &str) -> &'a str {
    match source.split_once(marker) {
        Some((prefix, _)) => prefix,
        None => panic!("source marker is missing: {marker}"),
    }
}

struct RetainingForwarder {
    transport: Option<SanitizedDocumentRootEventTransport>,
}

impl SanitizedDocumentRootEventForwarder for RetainingForwarder {
    type Error = ();

    fn forward_sanitized_document_root_event(
        &mut self,
        transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.transport = Some(transport);
        Ok(())
    }
}

struct TestDispatcher {
    fail_text: bool,
}

impl KucRootEventBatchDispatcher for TestDispatcher {
    type Error = &'static str;

    fn dispatch_text_events(
        &mut self,
        _events: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        if self.fail_text {
            Err("text dispatch rejected")
        } else {
            Ok(())
        }
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_search_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn real_sanitized_transport() -> SanitizedDocumentRootEventTransport {
    let input = SanitizedDocumentRootInput::new(
        1,
        SanitizedDocumentRootIdentity::from_opaque_bytes(b"transport-root"),
        "本文 ⭐️",
        SanitizedDocumentRootStyleKey::Default,
    );
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input)
        .expect("sanitized root retains");
    let context = egui::Context::default();
    let mut frame = None;
    crate::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                ROOT_VIEWPORT_SIZE,
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("sanitized root renders"));
            });
        },
    );
    let frame = frame.expect("sanitized frame exists");
    let mut forwarder = RetainingForwarder { transport: None };
    frame
        .forward_events_once(&mut forwarder)
        .expect("real frame forwards its opaque transport");
    forwarder
        .transport
        .expect("forwarder retained the real transport")
}

include!("sanitized_document_root_transport_inline_tests/introspection.rs");
include!("sanitized_document_root_transport_inline_tests/dispatch.rs");
