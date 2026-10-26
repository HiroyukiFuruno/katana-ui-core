use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceRootEventTransport, KucRootEventBatchDispatcher,
    KucRootEventBatchForwarder,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::selection::ContextMenuEvent;
use katana_ui_core::text_surface::TextSurfaceEvent;

#[derive(Default)]
pub struct LiveForwarder {
    pub text_events: Vec<TextSurfaceEvent>,
    pub dispatches: usize,
}

impl KucRootEventBatchDispatcher for LiveForwarder {
    type Error = String;
    fn dispatch_text_events(&mut self, events: Vec<TextSurfaceEvent>) -> Result<(), String> {
        self.text_events.extend(events);
        Ok(())
    }
    fn dispatch_toolbar_events(&mut self, _: Vec<CommandChromeToolbarEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_floating_events(
        &mut self,
        _: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_search_events(&mut self, _: Vec<CommandChromeSearchEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_context_menu_events(&mut self, _: Vec<ContextMenuEvent>) -> Result<(), String> {
        Ok(())
    }
}

impl KucRootEventBatchForwarder for LiveForwarder {
    type Error = String;
    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), String> {
        transport
            .dispatch_once(self)
            .map_err(|error| format!("{error:?}"))?;
        self.dispatches += 1;
        Ok(())
    }
}
