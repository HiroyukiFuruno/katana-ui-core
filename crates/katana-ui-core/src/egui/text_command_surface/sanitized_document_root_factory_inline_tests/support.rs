use super::*;

pub(super) const SCREEN_WIDTH: f32 = 640.0;
pub(super) const SCREEN_HEIGHT: f32 = 480.0;

pub(super) struct RecordingForwarder {
    pub(super) calls: usize,
    pub(super) transport_debug: Option<String>,
    pub(super) reject_forwarding: bool,
}

pub(super) struct RetainingForwarder {
    pub(super) calls: usize,
    pub(super) transport_debug: Option<String>,
    pub(super) transport: Option<SanitizedDocumentRootEventTransport>,
}

#[derive(Default)]
pub(super) struct TestRootDispatcher {
    pub(super) reject_text: bool,
}

impl KucRootEventBatchDispatcher for TestRootDispatcher {
    type Error = ();

    fn dispatch_text_events(
        &mut self,
        _events: Vec<crate::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        if self.reject_text { Err(()) } else { Ok(()) }
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
        if self.reject_forwarding {
            return Err(());
        }
        transport
            .dispatch_root_once(&mut TestRootDispatcher::default())
            .map_err(|_| ())?;
        Ok(())
    }
}

impl RetainingForwarder {
    pub(super) fn dispatch_root_once(
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

    pub(super) fn dispatch_root_once_rejecting_text(
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

pub(super) fn input(revision: u64, identity: &[u8], snapshot: &str) -> SanitizedDocumentRootInput {
    SanitizedDocumentRootInput::new(
        revision,
        SanitizedDocumentRootIdentity::from_opaque_bytes(identity.to_vec()),
        snapshot,
        SanitizedDocumentRootStyleKey::Default,
    )
}

pub(super) fn input_with_tabs(revision: u64) -> SanitizedDocumentRootInput {
    input(revision, b"document", "本文 ⭐️").with_tab_projection(SanitizedTabProjection::new([
        SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "ドキュメント",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "最初")
                .with_capabilities(
                    SanitizedTabCapabilities::new()
                        .active_state(true)
                        .close_state(true),
                ),
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([2]), 1, "次の文書")
                .with_capabilities(SanitizedTabCapabilities::new().close_state(true))
                .with_close_presentation(SanitizedTabClosePresentation::new(
                    "×",
                    "閉じる",
                    "次の文書を閉じる",
                )),
        ),
    ]))
}
