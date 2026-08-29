use super::text_surface_app::TextSurfaceStorybookApp;
#[cfg(test)]
use super::text_surface_artifact_writer::{
    ARTIFACT_GIF_FILE, ARTIFACT_MANIFEST_FILE, write_scripted_artifact,
};
#[cfg(test)]
use super::text_surface_fixture::STORY_LINE_COUNT;
use super::text_surface_fixture::{SURFACE_HEIGHT, SURFACE_WIDTH};
#[cfg(test)]
use super::text_surface_script::{assert_sequence_contract, run_scripted_sequence};

const TEXT_SURFACE_PAGE: &str = "text-area";
const TEXT_SURFACE_WINDOW_TITLE: &str = "katana-ui-core Storybook - TextSurface";

pub(super) fn handles_page(page: &str) -> bool {
    page == TEXT_SURFACE_PAGE
}

pub(super) fn open_window(frames: usize) -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(SURFACE_WIDTH, SURFACE_HEIGHT)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        TEXT_SURFACE_WINDOW_TITLE,
        native_options,
        Box::new(move |_| Ok(Box::new(TextSurfaceStorybookApp::new(frames)))),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ARTIFACT_GIF_FILE, ARTIFACT_MANIFEST_FILE, STORY_LINE_COUNT, SURFACE_HEIGHT, SURFACE_WIDTH,
        TextSurfaceStorybookApp, assert_sequence_contract, handles_page, run_scripted_sequence,
        write_scripted_artifact,
    };
    use eframe::App as _;
    use sha2::{Digest, Sha256};
    use std::error::Error;
    use std::fs;
    use std::path::Path;

    #[test]
    fn text_surface_app_covers_bounded_and_unbounded_frame_lifecycles() {
        for (frames, expected_remaining) in [(0, None), (1, Some(0))] {
            let context = egui::Context::default();
            let mut app = TextSurfaceStorybookApp::new(frames);
            let mut frame = eframe::Frame::_new_kittest();
            let mut output = context.run_ui(Default::default(), |ui| app.ui(ui, &mut frame));
            output.textures_delta.clear();
            assert_eq!(expected_remaining, app.frames_remaining);
        }
    }

    #[test]
    fn text_surface_story_fixture_is_rendered_by_the_actual_adapter() -> Result<(), Box<dyn Error>>
    {
        let context = egui::Context::default();
        let mut app = TextSurfaceStorybookApp::new(0);
        let mut full_output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(SURFACE_WIDTH, SURFACE_HEIGHT),
                )),
                ..egui::RawInput::default()
            },
            |ui| app.show(ui),
        );
        full_output.textures_delta.clear();

        assert!(app.last_error.is_none());
        assert!(app.last_record.as_ref().is_some_and(|record| {
            record.frame.surface_bounds == record.frame.accessibility.root.bounds
                && record.raster_identity.contains("⭐️")
                && record.frame.gutter.len() == STORY_LINE_COUNT
                && record.frame.annotations.len() == 1
        }));
        assert_eq!(
            app.last_artifact.as_ref().map(|artifact| &artifact.record),
            app.last_record.as_ref()
        );
        assert_eq!(
            app.last_artifact
                .as_ref()
                .map(|artifact| artifact.paint_plan_hash.as_str()),
            app.last_pixels
                .as_ref()
                .map(|pixels| pixels.paint_plan_hash.as_str())
        );
        assert!(app.last_pixels.as_ref().is_some_and(|pixels| {
            pixels.pixel_hash.len() == 64 && pixels.rgba.iter().any(|component| *component != 0)
        }));
        Ok(())
    }

    #[test]
    fn text_surface_runtime_only_handles_the_text_area_page() {
        assert!(handles_page("text-area"));
        assert!(!handles_page("text-input"));
    }

    #[test]
    fn actual_egui_script_is_deterministic_and_covers_editor_surface_events()
    -> Result<(), Box<dyn Error>> {
        let first = run_scripted_sequence()?;
        let second = run_scripted_sequence()?;
        assert_sequence_contract(&first)?;
        assert_sequence_contract(&second)?;
        assert_eq!(first.steps.len(), second.steps.len());
        for (left, right) in first.steps.iter().zip(second.steps.iter()) {
            assert_eq!(left.name, right.name);
            assert_eq!(
                left.artifact.record, right.artifact.record,
                "{} record",
                left.name
            );
            assert_eq!(
                left.artifact.frame_record_hash, right.artifact.frame_record_hash,
                "{} frame record",
                left.name
            );
            assert_eq!(
                left.artifact.paint_plan_hash, right.artifact.paint_plan_hash,
                "{} paint plan",
                left.name
            );
            assert_eq!(
                left.pixels.pixel_hash, right.pixels.pixel_hash,
                "{} pixels",
                left.name
            );
            assert_eq!(left.events, right.events);
        }
        Ok(())
    }

    #[test]
    fn text_surface_runtime_first_frame_matches_scripted_contract() -> Result<(), Box<dyn Error>> {
        let sequence = run_scripted_sequence()?;
        let scripted_first = sequence.steps.first();
        assert!(scripted_first.is_some());

        let context = egui::Context::default();
        let mut app = TextSurfaceStorybookApp::new(0);
        let mut full_output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(SURFACE_WIDTH, SURFACE_HEIGHT),
                )),
                ..egui::RawInput::default()
            },
            |ui| app.show(ui),
        );
        full_output.textures_delta.clear();

        assert!(app.last_error.is_none());
        assert_eq!(
            app.last_record.as_ref(),
            scripted_first.map(|step| &step.artifact.record)
        );
        assert_eq!(
            app.last_artifact
                .as_ref()
                .map(|artifact| artifact.frame_record_hash.as_str()),
            scripted_first.map(|step| step.artifact.frame_record_hash.as_str())
        );
        assert_eq!(
            app.last_artifact
                .as_ref()
                .map(|artifact| artifact.paint_plan_hash.as_str()),
            scripted_first.map(|step| step.artifact.paint_plan_hash.as_str())
        );
        assert_eq!(
            app.last_pixels
                .as_ref()
                .map(|pixels| (pixels.pixel_hash.as_str(), pixels.rgba.len())),
            scripted_first.map(|step| (step.pixels.pixel_hash.as_str(), step.pixels.rgba.len()))
        );
        assert_eq!(
            app.last_record
                .as_ref()
                .map(|record| record.frame.accessibility.root.focused),
            scripted_first.map(|step| step.surface_focused)
        );
        Ok(())
    }

    #[test]
    fn scripted_artifact_writes_plan_only_png_gif_and_manifest() -> Result<(), Box<dyn Error>> {
        let output_dir = Path::new("target/text-surface-storybook-test-artifact");
        write_scripted_artifact(output_dir)?;
        let manifest = fs::read_to_string(output_dir.join(ARTIFACT_MANIFEST_FILE))?;
        assert!(manifest.contains("actual-egui-raw-input"));
        assert!(manifest.contains("adapter-paint-plan-only"));
        assert!(manifest.contains("⭐️"));
        assert!(manifest.contains("\"star_variation_selector_present\": true"));
        assert!(manifest.contains("\"color_emoji_texture_present\": true"));
        assert!(output_dir.join(ARTIFACT_GIF_FILE).is_file());
        let png_count = fs::read_dir(output_dir)?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "png")
            })
            .count();
        assert_eq!(17, png_count);
        let gif_hash = digest(&fs::read(output_dir.join(ARTIFACT_GIF_FILE))?);
        write_scripted_artifact(output_dir)?;
        assert_eq!(
            gif_hash,
            digest(&fs::read(output_dir.join(ARTIFACT_GIF_FILE))?)
        );
        Ok(())
    }

    fn digest(bytes: &[u8]) -> String {
        hex::encode(Sha256::digest(bytes))
    }
}
