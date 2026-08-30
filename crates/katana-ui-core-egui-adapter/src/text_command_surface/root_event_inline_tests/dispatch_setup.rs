pub(super) struct OrderRecorder {
    pub(super) calls: Vec<&'static str>,
    pub(super) context_menu_dispatch_complete: Rc<Cell<bool>>,
}

pub(super) struct RecordingSourcePort {
    pub(super) received: Rc<RefCell<Vec<String>>>,
    pub(super) fail: bool,
}

impl SourceAddressSubmissionPort for RecordingSourcePort {
    fn forward_submission(
        &mut self,
        submission: SourceAddressSubmission,
    ) -> Result<(), SourceAddressSubmissionPortError> {
        if self.fail {
            return Err(SourceAddressSubmissionPortError::Rejected);
        }
        self.received.borrow_mut().push(submission.into_draft());
        Ok(())
    }
}

pub(super) fn source_submission() -> SourceAddressSubmission {
    let mut strip = katana_ui_core::molecule::structured::source_address_strip::SourceAddressStrip::new(
        katana_ui_core::molecule::structured::source_address_strip::SourceAddressPresentation::new(
            "表示", "ツールチップ", "アクセシビリティ",
        ),
    );
    let _ = strip.apply_action(
        katana_ui_core::molecule::structured::source_address_strip::SourceAddressAction::SetDraft(
            "opaque-source-draft".to_owned(),
        ),
    );
    match strip.apply_action(
        katana_ui_core::molecule::structured::source_address_strip::SourceAddressAction::Submit,
    ) {
        Some(
            katana_ui_core::molecule::structured::source_address_strip::SourceAddressEvent::Submitted(
                submission,
            ),
        ) => submission,
        _ => panic!("enabled source address should submit"),
    }
}

impl KucRootEventBatchDispatcher for OrderRecorder {
    type Error = ();

    fn dispatch_text_events(&mut self, events: Vec<TextSurfaceEvent>) -> Result<(), Self::Error> {
        assert_eq!(events.len(), 1);
        self.calls.push("text");
        Ok(())
    }

    fn dispatch_toolbar_events(
        &mut self,
        events: Vec<CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        assert_eq!(events.len(), 1);
        self.calls.push("toolbar");
        Ok(())
    }

    fn dispatch_floating_events(
        &mut self,
        events: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        assert_eq!(events.len(), 1);
        self.calls.push("floating");
        Ok(())
    }

    fn dispatch_search_events(
        &mut self,
        events: Vec<CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        assert_eq!(events.len(), 1);
        self.calls.push("search");
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        assert_eq!(events.len(), 1);
        self.calls.push("context-menu");
        self.context_menu_dispatch_complete.set(true);
        Ok(())
    }

    fn consume_opaque_host_effect_batch(
        &mut self,
        effect_batch: KucOpaqueHostEffectBatch,
    ) -> Result<(), KucOpaqueHostEffectError> {
        effect_batch
            .consume_once()
            .map_err(|_| KucOpaqueHostEffectError)
    }
}

pub(super) fn full_payload() -> RootEventPayload {
    RootEventPayload {
        text: vec![TextSurfaceEvent::FocusChanged(true)],
        toolbar: Some(vec![CommandChromeToolbarEvent::CommandActivated {
            action_id: "toolbar-action".into(),
        }]),
        floating: Some(vec![FloatingCommandToolbarEvent::FocusRetained]),
        search: Some(vec![CommandChromeSearchEvent::CloseRequested]),
        context_menu: Some(vec![
            katana_ui_core::molecule::selection::ContextMenuEvent::Closed {
                reason: katana_ui_core::molecule::selection::ContextMenuCloseReason::Escape,
            },
        ]),
        source_address_submissions: Vec::new(),
        ..RootEventPayload::empty()
    }
}
