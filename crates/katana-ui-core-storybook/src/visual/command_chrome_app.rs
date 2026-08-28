use super::command_chrome_surface::{CommandChromeSurface, CommandChromeSurfaceFrame};
use katana_ui_core_egui_adapter::command_chrome::EguiCommandChromeError;

pub(super) struct CommandChromeStorybookApp {
    surface: CommandChromeSurface,
    pub(super) frames_remaining: Option<usize>,
    pub(super) last_frame: Option<CommandChromeSurfaceFrame>,
    pub(super) last_error: Option<EguiCommandChromeError>,
}

impl CommandChromeStorybookApp {
    pub(super) fn new(frames: usize) -> Self {
        Self {
            surface: CommandChromeSurface::new(true),
            frames_remaining: (frames > 0).then_some(frames),
            last_frame: None,
            last_error: None,
        }
    }

    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        let result = self.surface.show(ui);
        request_repaint_if_interacted(ui.ctx(), self.record_result(result));
    }

    fn record_result(
        &mut self,
        result: Result<CommandChromeSurfaceFrame, EguiCommandChromeError>,
    ) -> bool {
        match result {
            Ok(frame) => {
                let received_interaction = surface_received_interaction(&frame);
                self.last_frame = Some(frame);
                self.last_error = None;
                received_interaction
            }
            Err(error) => {
                self.last_error = Some(error);
                false
            }
        }
    }
}

fn request_repaint_if_interacted(context: &egui::Context, interacted: bool) {
    if interacted {
        context.request_repaint();
    }
}

fn surface_received_interaction(frame: &CommandChromeSurfaceFrame) -> bool {
    !frame.toolbar.events.is_empty()
        || !frame.floating.events.is_empty()
        || !frame.search.events.is_empty()
        || !frame.search.text_events.is_empty()
}

impl eframe::App for CommandChromeStorybookApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show(ui);
        let Some(remaining) = self.frames_remaining.as_mut() else {
            return;
        };
        *remaining = remaining.saturating_sub(1);
        if *remaining == 0 {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

#[cfg(test)]
mod result_tests {
    use super::*;

    #[test]
    fn command_chrome_app_retains_adapter_errors_without_requesting_repaint() {
        let mut app = CommandChromeStorybookApp::new(1);
        let error = EguiCommandChromeError::ArtifactSerialization("failure".to_string());

        assert!(!app.record_result(Err(error)));
        assert!(app.last_frame.is_none());
        assert!(app.last_error.is_some());
    }

    #[test]
    fn interaction_requests_a_repaint() {
        let context = egui::Context::default();
        request_repaint_if_interacted(&context, true);
        assert!(context.has_requested_repaint());
        request_repaint_if_interacted(&context, false);
    }
}
