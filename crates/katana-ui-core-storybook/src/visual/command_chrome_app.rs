use super::command_chrome_surface::{CommandChromeSurface, CommandChromeSurfaceFrame};
#[cfg(test)]
use katana_ui_core_egui_adapter::command_chrome::EguiCommandChromeError;

pub(super) struct CommandChromeStorybookApp {
    surface: CommandChromeSurface,
    frames_remaining: Option<usize>,
    #[cfg(test)]
    pub(super) last_frame: Option<CommandChromeSurfaceFrame>,
    #[cfg(test)]
    pub(super) last_error: Option<EguiCommandChromeError>,
}

impl CommandChromeStorybookApp {
    pub(super) fn new(frames: usize) -> Self {
        Self {
            surface: CommandChromeSurface::new(true),
            frames_remaining: (frames > 0).then_some(frames),
            #[cfg(test)]
            last_frame: None,
            #[cfg(test)]
            last_error: None,
        }
    }

    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        match self.surface.show(ui) {
            Ok(frame) => {
                if surface_received_interaction(&frame) {
                    ui.ctx().request_repaint();
                }
                #[cfg(test)]
                {
                    self.last_frame = Some(frame);
                    self.last_error = None;
                }
                #[cfg(not(test))]
                let _ = frame;
            }
            Err(error) => {
                #[cfg(test)]
                {
                    self.last_error = Some(error);
                }
                #[cfg(not(test))]
                eprintln!("CommandChrome Storybook adapter error: {error}");
            }
        }
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
