const TOOLBAR_DISPATCH_FAILURE: usize = 3;

#[test]
fn transport_dispatches_all_classes_in_fixed_order_and_returns_counts() {
    let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
    let transport = batch
        .transport
        .borrow_mut()
        .take()
        .expect("transport exists");
    let mut dispatcher = OrderRecorder {
        calls: Vec::new(),
        context_menu_dispatch_complete: Rc::new(Cell::new(false)),
    };
    let receipt = transport
        .dispatch_once(&mut dispatcher)
        .expect("dispatch_once succeeds");
    assert_eq!(
        dispatcher.calls,
        vec!["text", "toolbar", "floating", "search", "context-menu"]
    );
    assert_eq!(
        receipt.class_dispatches(),
        &[
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Text,
                event_count: 1,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Toolbar,
                event_count: 1,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Floating,
                event_count: 1,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Search,
                event_count: 1,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::ContextMenu,
                event_count: 1,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::StatusBar,
                event_count: 0,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::DiagnosticsList,
                event_count: 0,
            },
        ]
    );
    assert_eq!(
        receipt
            .class_dispatches()
            .iter()
            .map(|dispatch| dispatch.event_count)
            .collect::<Vec<_>>(),
        vec![
            receipt.text_count(),
            receipt.toolbar_count(),
            receipt.floating_count(),
            receipt.search_count(),
            receipt.context_menu_count(),
            receipt.status_bar_count(),
            receipt.diagnostics_list_count(),
        ]
    );
}

pub(super) struct DispatcherError;

impl KucRootEventBatchDispatcher for DispatcherError {
    type Error = usize;

    fn dispatch_text_events(&mut self, _events: Vec<TextSurfaceEvent>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Err(TOOLBAR_DISPATCH_FAILURE)
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        panic!("must fail closed before floating dispatch")
    }

    fn dispatch_search_events(
        &mut self,
        _events: Vec<CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        panic!("must fail closed before search dispatch")
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        panic!("must fail closed before context-menu dispatch")
    }

    fn consume_opaque_host_effect_batch(
        &mut self,
        _effect_batch: KucOpaqueHostEffectBatch,
    ) -> Result<(), KucOpaqueHostEffectError> {
        panic!("must not consume effect after child dispatch failure")
    }
}

#[test]
fn transport_dispatch_propagates_typed_error_and_does_not_continue() {
    let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
    let transport = batch
        .transport
        .borrow_mut()
        .take()
        .expect("transport exists");
    assert_eq!(
        transport.dispatch_once(&mut DispatcherError),
        Err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher(3))
    );
}

pub(super) struct StageFailingDispatcher {
    pub(super) fail_at: EguiTextCommandSurfaceRootEventChildClass,
    pub(super) calls: Vec<EguiTextCommandSurfaceRootEventChildClass>,
}

impl StageFailingDispatcher {
    fn dispatch(
        &mut self,
        child_class: EguiTextCommandSurfaceRootEventChildClass,
    ) -> Result<(), EguiTextCommandSurfaceRootEventChildClass> {
        self.calls.push(child_class);
        if child_class == self.fail_at {
            Err(child_class)
        } else {
            Ok(())
        }
    }
}

impl KucRootEventBatchDispatcher for StageFailingDispatcher {
    type Error = EguiTextCommandSurfaceRootEventChildClass;

    fn dispatch_text_events(&mut self, _events: Vec<TextSurfaceEvent>) -> Result<(), Self::Error> {
        self.dispatch(EguiTextCommandSurfaceRootEventChildClass::Text)
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        self.dispatch(EguiTextCommandSurfaceRootEventChildClass::Toolbar)
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        self.dispatch(EguiTextCommandSurfaceRootEventChildClass::Floating)
    }

    fn dispatch_search_events(
        &mut self,
        _events: Vec<CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        self.dispatch(EguiTextCommandSurfaceRootEventChildClass::Search)
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        self.dispatch(EguiTextCommandSurfaceRootEventChildClass::ContextMenu)
    }

    fn dispatch_status_bar_events(
        &mut self,
        _events: Vec<StatusBarEvent>,
    ) -> Result<(), Self::Error> {
        self.dispatch(EguiTextCommandSurfaceRootEventChildClass::StatusBar)
    }

    fn dispatch_diagnostics_list_events(
        &mut self,
        _events: Vec<DiagnosticsListEvent>,
    ) -> Result<(), Self::Error> {
        self.dispatch(EguiTextCommandSurfaceRootEventChildClass::DiagnosticsList)
    }
}
