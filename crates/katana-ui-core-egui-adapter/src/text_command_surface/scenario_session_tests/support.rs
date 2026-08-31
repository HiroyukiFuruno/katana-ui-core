use super::{
    valid_selection, FullTextCommandSurfaceScenarioError, FullTextCommandSurfaceScenarioSession,
    ScenarioSessionState, ScenarioSessionUpdate,
};
use crate::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceRootEventTransport,
    EguiTextCommandSurfaceRootFactory, FullTextCommandSurfaceScenarioFactory,
    FullTextCommandSurfaceScenarioId, KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
    TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeCapability, CommandChromeSearchEvent, CommandChromeToolbarEvent,
    FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::selection::ContextMenuEvent;
use katana_ui_core::molecule::structured::ReplaceMode;
use katana_ui_core::text_surface::TextSurfaceEvent;
use std::convert::Infallible;

struct SessionDispatcher;

impl KucRootEventBatchDispatcher for SessionDispatcher {
    type Error = Infallible;

    fn dispatch_text_events(&mut self, _events: Vec<TextSurfaceEvent>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_search_events(
        &mut self,
        _events: Vec<CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct SessionForwarder;

impl KucRootEventBatchForwarder for SessionForwarder {
    type Error = String;

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        transport
            .dispatch_once(&mut SessionDispatcher)
            .map(|_| ())
            .map_err(|error| format!("session event dispatch failed: {error:?}"))
    }
}

fn render_and_forward(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    lease: crate::text_command_surface::EguiTextCommandSurfaceHostProjectionLease,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
    root.synchronize_with_lease(lease)
        .expect("current scenario lease synchronizes");
    render_current_and_forward(context, root, input)
}

fn render_current_and_forward(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
    let mut output = None;
    let mut platform_output = context.run_ui(input, |ui| {
        output = Some(root.show_output_for_test(ui));
    });
    platform_output.textures_delta.clear();
    let output = output
        .expect("root renders")
        .expect("root output is available");
    output
        .events()
        .forward_once(&mut SessionForwarder)
        .expect("one-shot scenario event transport forwards");
    output
}

fn render_current(
    context: &egui::Context,
    root: &mut crate::text_command_surface::EguiTextCommandSurfaceHostRoot,
    input: egui::RawInput,
) -> crate::text_command_surface::EguiTextCommandSurfaceRootOutput {
    let mut output = None;
    crate::run_ui_discard(context, input, |ui| {
        output = Some(root.show_output_for_test(ui));
    });
    output
        .expect("root renders")
        .expect("root output is available")
}
