use super::command_chrome_app::CommandChromeStorybookApp;
use super::command_chrome_fixture::{FRAME_HEIGHT, FRAME_WIDTH};

const COMMAND_CHROME_PAGE: &str = "command-chrome";
const COMMAND_CHROME_WINDOW_TITLE: &str = "katana-ui-core Storybook - CommandChrome";

pub(super) fn handles_page(page: &str) -> bool {
    page == COMMAND_CHROME_PAGE
}

pub(super) fn open_window(frames: usize) -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(FRAME_WIDTH, FRAME_HEIGHT)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        COMMAND_CHROME_WINDOW_TITLE,
        native_options,
        Box::new(move |_| Ok(Box::new(CommandChromeStorybookApp::new(frames)))),
    )
}

#[cfg(test)]
mod tests {
    use super::super::command_chrome_script::run_scripted_sequence;
    use super::super::command_chrome_surface::CommandChromeSurfaceFrame;
    use super::{CommandChromeStorybookApp, FRAME_HEIGHT, FRAME_WIDTH, handles_page};
    use eframe::App as _;
    use std::error::Error;

    #[test]
    fn command_chrome_runtime_only_handles_its_required_page() {
        assert!(handles_page("command-chrome"));
        assert!(!handles_page("text-area"));
    }

    #[test]
    fn eframe_app_covers_bounded_and_unbounded_frame_lifecycles() {
        for (frames, expected_remaining) in [(0, None), (1, Some(0))] {
            let context = egui::Context::default();
            let mut app = CommandChromeStorybookApp::new(frames);
            let mut frame = eframe::Frame::_new_kittest();
            let mut output = context.run_ui(Default::default(), |ui| app.ui(ui, &mut frame));
            output.textures_delta.clear();
            assert_eq!(expected_remaining, app.frames_remaining);
        }
    }

    #[test]
    fn eframe_app_uses_the_same_surface_output_as_the_actual_raw_input_script()
    -> Result<(), Box<dyn Error>> {
        let context = egui::Context::default();
        context.enable_accesskit();
        let mut app = CommandChromeStorybookApp::new(0);
        let mut full_output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(FRAME_WIDTH, FRAME_HEIGHT),
                )),
                ..egui::RawInput::default()
            },
            |ui| app.show(ui),
        );
        full_output.textures_delta.clear();

        assert!(app.last_error.is_none());
        let scripted = run_scripted_sequence()?;
        assert!(app.last_frame.is_some());
        assert!(!scripted.frames.is_empty());
        app.last_frame.iter().zip(scripted.frames.first()).for_each(
            |(app_frame, scripted_frame)| {
                assert_same_artifacts(app_frame, scripted_frame);
            },
        );
        Ok(())
    }

    fn assert_same_artifacts(
        app: &CommandChromeSurfaceFrame,
        scripted: &super::super::command_chrome_script::CommandChromeScriptFrame,
    ) {
        assert_eq!(
            app.toolbar.artifact.frame_record_hash,
            scripted.toolbar.artifact.frame_record_hash
        );
        assert_eq!(
            app.toolbar.artifact.paint_plan_hash,
            scripted.toolbar.artifact.paint_plan_hash
        );
        assert_eq!(
            app.search.artifact.frame_record_hash,
            scripted.search.artifact.frame_record_hash
        );
        assert_eq!(
            app.search.artifact.paint_plan_hash,
            scripted.search.artifact.paint_plan_hash
        );
        assert_eq!(
            app.floating
                .artifact
                .as_ref()
                .map(|artifact| artifact.frame_record_hash.as_str()),
            scripted
                .floating
                .artifact
                .as_ref()
                .map(|artifact| artifact.frame_record_hash.as_str())
        );
        assert_eq!(
            app.floating
                .artifact
                .as_ref()
                .map(|artifact| artifact.paint_plan_hash.as_str()),
            scripted
                .floating
                .artifact
                .as_ref()
                .map(|artifact| artifact.paint_plan_hash.as_str())
        );
        assert!(app.floating.artifact.is_some());
    }
}
