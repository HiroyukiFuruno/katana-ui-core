#[path = "command_chrome_artifact_writer_composite.rs"]
mod command_chrome_artifact_writer_composite;
#[path = "command_chrome_artifact_writer_io.rs"]
mod command_chrome_artifact_writer_io;

use super::command_chrome_artifact::render_command_chrome_plan;
use super::command_chrome_fixture::{FRAME_HEIGHT, FRAME_WIDTH};
use super::command_chrome_script::run_scripted_sequence;
use super::command_chrome_script_types::{
    CommandChromeArtifactError, CommandChromeArtifactFrame, CommandChromeArtifactSequence,
    StorybookCommandChromeManifest,
};
use command_chrome_artifact_writer_composite as composite;
use command_chrome_artifact_writer_io as io;
use katana_ui_core::render_model::UiRect;
use std::fs;
use std::path::Path;

pub(super) const ARTIFACT_GIF_FILE: &str = "command-chrome-motion.gif";
pub(super) const ARTIFACT_MANIFEST_FILE: &str = "command-chrome-manifest.json";

pub(super) fn write_scripted_artifact(output_dir: &Path) -> Result<(), CommandChromeArtifactError> {
    let sequence = run_command_chrome_artifact_sequence()?;
    assert_command_chrome_sequence_contract(&sequence)?;

    fs::create_dir_all(output_dir).map_err(CommandChromeArtifactError::Io)?;
    io::clear_previous_artifact_files(output_dir)?;

    for frame in &sequence.frames {
        let manifest_entry = frame.manifest_entry();
        io::write_png(
            &frame.composite_pixels,
            &output_dir.join(manifest_entry.png),
        )
        .map_err(CommandChromeArtifactError::Image)?;
    }

    io::write_gif(
        &sequence
            .frames
            .iter()
            .map(|frame| frame.composite_pixels.clone())
            .collect::<Vec<_>>(),
        &output_dir.join(ARTIFACT_GIF_FILE),
    )
    .map_err(CommandChromeArtifactError::Image)?;

    let manifest = StorybookCommandChromeManifest::from_frames(
        sequence
            .frames
            .iter()
            .map(|frame| frame.manifest_entry())
            .collect(),
    );
    let json = serde_json::to_vec_pretty(&manifest).map_err(CommandChromeArtifactError::Json)?;
    fs::write(output_dir.join(ARTIFACT_MANIFEST_FILE), json)
        .map_err(CommandChromeArtifactError::Io)?;

    Ok(())
}

pub(super) fn run_command_chrome_artifact_sequence()
-> Result<CommandChromeArtifactSequence, CommandChromeArtifactError> {
    let source = run_scripted_sequence().map_err(CommandChromeArtifactError::from)?;
    let mut frames = Vec::with_capacity(source.frames.len());

    for (index, frame) in source.frames.into_iter().enumerate() {
        let toolbar_plan = &frame.toolbar.artifact.paint_plan;
        let floating_plan = frame
            .floating
            .artifact
            .as_ref()
            .map(|artifact| &artifact.paint_plan);
        let search_plan = &frame.search.artifact.paint_plan;

        let canvas = actual_root_canvas();
        let toolbar_pixels = render_command_chrome_plan(toolbar_plan, canvas)
            .map_err(CommandChromeArtifactError::Contract)?;
        let floating_pixels = floating_plan
            .map(|plan| render_command_chrome_plan(plan, canvas))
            .transpose()
            .map_err(CommandChromeArtifactError::Contract)?;
        let search_pixels = render_command_chrome_plan(search_plan, canvas)
            .map_err(CommandChromeArtifactError::Contract)?;
        let composite_pixels =
            composite::render_composite_pixels(canvas, toolbar_plan, floating_plan, search_plan)?;

        let floating = if frame.floating.record.is_some()
            || frame.floating.artifact.is_some()
            || !frame.floating.events.is_empty()
        {
            Some(frame.floating)
        } else {
            None
        };

        frames.push(CommandChromeArtifactFrame {
            name: frame_name(index),
            index,
            toolbar: frame.toolbar,
            floating,
            search: frame.search,
            accesskit_labels: frame.accesskit_labels,
            toolbar_pixels,
            floating_pixels,
            search_pixels,
            composite_pixels,
            frame_width: canvas.width,
            frame_height: canvas.height,
        });
    }

    finish_artifact_sequence(frames)
}

fn finish_artifact_sequence(
    frames: Vec<CommandChromeArtifactFrame>,
) -> Result<CommandChromeArtifactSequence, CommandChromeArtifactError> {
    if frames.is_empty() {
        return Err(CommandChromeArtifactError::Contract(
            "command-chrome artifact sequence was empty".to_string(),
        ));
    }

    Ok(CommandChromeArtifactSequence { frames })
}

fn actual_root_canvas() -> UiRect {
    UiRect::new(0, 0, FRAME_WIDTH as u32, FRAME_HEIGHT as u32)
}

fn frame_name(index: usize) -> String {
    format!("frame-{index:02}")
}

fn assert_command_chrome_sequence_contract(
    sequence: &CommandChromeArtifactSequence,
) -> Result<(), CommandChromeArtifactError> {
    for frame in &sequence.frames {
        frame
            .manifest_entry()
            .validate_contract()
            .map_err(CommandChromeArtifactError::Contract)?;

        if frame.toolbar_pixels.paint_plan_hash != frame.toolbar.artifact.paint_plan_hash {
            return Err(CommandChromeArtifactError::Contract(format!(
                "toolbar paint plan hash mismatch on frame {}",
                frame.index
            )));
        }

        if frame.search_pixels.paint_plan_hash != frame.search.artifact.paint_plan_hash {
            return Err(CommandChromeArtifactError::Contract(format!(
                "search paint plan hash mismatch on frame {}",
                frame.index
            )));
        }

        if let (Some(pixels), Some(floating_artifact)) =
            (&frame.floating_pixels, frame.floating.as_ref())
        {
            let artifact = floating_artifact.artifact.as_ref().ok_or_else(|| {
                CommandChromeArtifactError::Contract(format!(
                    "floating pixels exist but artifact missing on frame {}",
                    frame.index
                ))
            })?;
            if pixels.paint_plan_hash != artifact.paint_plan_hash {
                return Err(CommandChromeArtifactError::Contract(format!(
                    "floating paint plan hash mismatch on frame {}",
                    frame.index
                )));
            }
        }

        if frame.toolbar_pixels.width == 0
            || frame.toolbar_pixels.height == 0
            || frame.toolbar_pixels.pixel_hash.is_empty()
            || frame.toolbar_pixels.rgba.is_empty()
        {
            return Err(CommandChromeArtifactError::Contract(format!(
                "toolbar pixels invalid on frame {}",
                frame.index
            )));
        }
        if frame.search_pixels.width == 0
            || frame.search_pixels.height == 0
            || frame.search_pixels.pixel_hash.is_empty()
            || frame.search_pixels.rgba.is_empty()
        {
            return Err(CommandChromeArtifactError::Contract(format!(
                "search pixels invalid on frame {}",
                frame.index
            )));
        }

        if frame.search.record.bounds.width == 0 || frame.search.record.bounds.height == 0 {
            return Err(CommandChromeArtifactError::Contract(format!(
                "search bounds invalid on frame {}",
                frame.index
            )));
        }
        if !composite::has_non_zero_pixel(&frame.composite_pixels.rgba) {
            return Err(CommandChromeArtifactError::Contract(format!(
                "frame {} is fully blank",
                frame.index
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::command_chrome_script::run_scripted_sequence;
    use super::{
        ARTIFACT_GIF_FILE, ARTIFACT_MANIFEST_FILE, assert_command_chrome_sequence_contract,
        finish_artifact_sequence, run_command_chrome_artifact_sequence, write_scripted_artifact,
    };
    use serde::Deserialize;
    use sha2::{Digest, Sha256};
    use std::error::Error;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    const EXPECTED_FRAME_COUNT: usize = 34;

    #[derive(Deserialize)]
    struct TestManifest {
        frames: Vec<TestManifestFrame>,
    }

    #[derive(Deserialize)]
    struct TestManifestFrame {
        png: String,
        frame_bounds: katana_ui_core::render_model::UiRect,
    }

    #[test]
    fn command_chrome_artifact_sequence_rejects_empty_input() {
        assert!(finish_artifact_sequence(Vec::new()).is_err());
    }

    #[test]
    fn command_chrome_artifact_sequence_rejects_missing_composite_payload()
    -> Result<(), Box<dyn Error>> {
        let mut sequence = run_command_chrome_artifact_sequence()?;
        sequence.frames[0].composite_pixels.pixel_hash.clear();
        assert!(assert_command_chrome_sequence_contract(&sequence).is_err());

        let mut sequence = run_command_chrome_artifact_sequence()?;
        sequence.frames[0].composite_pixels.rgba.clear();
        assert!(assert_command_chrome_sequence_contract(&sequence).is_err());

        let mut sequence = run_command_chrome_artifact_sequence()?;
        sequence.frames[0].composite_pixels.rgba = vec![1];
        assert!(assert_command_chrome_sequence_contract(&sequence).is_err());
        Ok(())
    }

    #[test]
    fn command_chrome_manifest_frame_rejects_missing_required_identity_fields()
    -> Result<(), Box<dyn Error>> {
        let sequence = run_command_chrome_artifact_sequence()?;
        let valid = sequence.frames[0].manifest_entry();

        let mut missing_name = valid.clone();
        missing_name.name.clear();
        assert!(missing_name.validate_contract().is_err());

        let mut missing_toolbar_hash = valid.clone();
        missing_toolbar_hash.toolbar_pixel_hash.clear();
        assert!(missing_toolbar_hash.validate_contract().is_err());

        let mut missing_search_hash = valid.clone();
        missing_search_hash.search_pixel_hash.clear();
        assert!(missing_search_hash.validate_contract().is_err());

        let mut missing_composite_hash = valid;
        missing_composite_hash.composite_pixel_hash.clear();
        assert!(missing_composite_hash.validate_contract().is_err());
        Ok(())
    }

    #[test]
    fn command_chrome_artifact_sequence_has_full_length_and_visible_frames()
    -> Result<(), Box<dyn Error>> {
        let output_dir = test_temp_dir("command_chrome_artifact_sequence")?;
        write_scripted_artifact(&output_dir)?;

        let manifest = read_manifest(&output_dir)?;
        let frames = &manifest.frames;
        assert_eq!(EXPECTED_FRAME_COUNT, frames.len());

        let frame_png_count = fs::read_dir(&output_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "png")
            })
            .count();
        assert_eq!(EXPECTED_FRAME_COUNT, frame_png_count);

        for frame in frames {
            let image = image::open(output_dir.join(&frame.png))?;
            assert!(
                image.to_rgba8().pixels().any(|pixel| pixel[3] > 0),
                "composed frame pixels should be non-blank",
            );
            assert!(frame.frame_bounds.width > 0);
            assert!(frame.frame_bounds.height > 0);
        }
        Ok(())
    }

    #[test]
    fn command_chrome_artifact_output_is_deterministic() -> Result<(), Box<dyn Error>> {
        let first = test_temp_dir("command_chrome_artifact_deterministic_first")?;
        let second = test_temp_dir("command_chrome_artifact_deterministic_second")?;

        write_scripted_artifact(&first)?;
        write_scripted_artifact(&second)?;

        let first_gif = fs::read(first.join(ARTIFACT_GIF_FILE))?;
        let second_gif = fs::read(second.join(ARTIFACT_GIF_FILE))?;
        assert_eq!(Sha256::digest(&first_gif), Sha256::digest(&second_gif));

        let first_manifest = fs::read(first.join(ARTIFACT_MANIFEST_FILE))?;
        let second_manifest = fs::read(second.join(ARTIFACT_MANIFEST_FILE))?;
        assert_eq!(
            Sha256::digest(&first_manifest),
            Sha256::digest(&second_manifest)
        );
        Ok(())
    }

    #[test]
    fn command_chrome_artifact_preserves_disabled_no_command_and_close_reasons()
    -> Result<(), Box<dyn Error>> {
        let sequence = run_command_chrome_artifact_sequence()?;
        let manifest_frames = sequence
            .frames
            .iter()
            .map(|frame| frame.manifest_entry())
            .collect::<Vec<_>>();

        let disabled_frame = &manifest_frames[4];
        assert_eq!(disabled_frame.name, "frame-04");
        assert!(
            disabled_frame.typed_events.is_empty(),
            "disabled frame should not emit typed events"
        );

        assert!(
            manifest_frames
                .iter()
                .any(|frame| frame.toolbar_dropdown_close_reason.is_some()
                    || frame.floating_close_reason.is_some())
        );
        assert!(
            manifest_frames
                .iter()
                .any(|frame| !frame.accessibility_labels.is_empty())
        );
        Ok(())
    }

    #[test]
    fn command_chrome_artifact_plan_hashes_match_rendered_pixels() -> Result<(), Box<dyn Error>> {
        let sequence = run_command_chrome_artifact_sequence()?;

        for frame in &sequence.frames {
            assert_eq!(
                frame.toolbar_pixels.paint_plan_hash,
                frame.toolbar.artifact.paint_plan_hash
            );
            assert_eq!(
                frame.search_pixels.paint_plan_hash,
                frame.search.artifact.paint_plan_hash
            );
            if let Some(pixels) = &frame.floating_pixels {
                assert_eq!(
                    Some(pixels.paint_plan_hash.as_str()),
                    frame
                        .floating
                        .as_ref()
                        .and_then(|floating| floating.artifact.as_ref())
                        .map(|artifact| artifact.paint_plan_hash.as_str())
                );
            }
        }
        Ok(())
    }

    #[test]
    fn command_chrome_artifact_records_language_and_star_variation_facts()
    -> Result<(), Box<dyn Error>> {
        let sequence = run_command_chrome_artifact_sequence()?;
        let manifest_frames = sequence
            .frames
            .iter()
            .map(|frame| frame.manifest_entry())
            .collect::<Vec<_>>();

        assert!(
            manifest_frames
                .iter()
                .any(|frame| frame.star_variation_selector_present),
            "should record variation-selector based star rendering",
        );
        assert!(
            manifest_frames
                .iter()
                .any(|frame| frame.color_emoji_texture_present),
            "should record colored star texture",
        );
        assert!(
            manifest_frames
                .iter()
                .any(|frame| !frame.palette_identities.is_empty()),
            "should record texture identities",
        );
        Ok(())
    }

    #[test]
    fn command_chrome_artifact_sequence_tracks_raw_script_contract_hashes()
    -> Result<(), Box<dyn Error>> {
        let script = run_scripted_sequence()?;
        let artifact_sequence = run_command_chrome_artifact_sequence()?;

        assert_eq!(script.frames.len(), artifact_sequence.frames.len());
        for (index, (script_frame, artifact_frame)) in script
            .frames
            .iter()
            .zip(&artifact_sequence.frames)
            .enumerate()
        {
            assert_eq!(index, artifact_frame.index);
            assert_eq!(artifact_frame.name, format!("frame-{index:02}"));

            let manifest = artifact_frame.manifest_entry();
            assert_eq!(
                manifest.toolbar_frame_record_hash,
                script_frame.toolbar.artifact.frame_record_hash
            );
            assert_eq!(
                manifest.toolbar_paint_plan_hash,
                script_frame.toolbar.artifact.paint_plan_hash
            );
            assert_eq!(
                manifest.search_frame_record_hash,
                script_frame.search.artifact.frame_record_hash
            );
            assert_eq!(
                manifest.search_paint_plan_hash,
                script_frame.search.artifact.paint_plan_hash
            );

            let has_floating_fidelity = script_frame.floating.record.is_some()
                || script_frame.floating.artifact.is_some()
                || !script_frame.floating.events.is_empty();
            assert_eq!(
                has_floating_fidelity,
                artifact_frame.floating.is_some(),
                "frame {index} floating artifact visibility mismatch"
            );
            if let (Some(script_floating_artifact), Some(artifact_floating)) = (
                script_frame.floating.artifact.as_ref(),
                artifact_frame.floating.as_ref(),
            ) {
                assert_eq!(
                    Some(script_floating_artifact.frame_record_hash.as_str()),
                    artifact_floating
                        .artifact
                        .as_ref()
                        .map(|artifact| artifact.frame_record_hash.as_str())
                );
                assert_eq!(
                    Some(script_floating_artifact.frame_record_hash.as_str()),
                    manifest.floating_frame_record_hash.as_deref()
                );
                assert_eq!(
                    Some(script_floating_artifact.paint_plan_hash.as_str()),
                    manifest.floating_paint_plan_hash.as_deref()
                );
            }

            let mut script_labels = script_frame.accesskit_labels.clone();
            script_labels.sort();
            script_labels.dedup();
            assert_eq!(script_labels, manifest.accessibility_labels);

            assert!(!manifest.toolbar_pixel_hash.is_empty());
            assert!(!manifest.search_pixel_hash.is_empty());
            assert!(!manifest.composite_pixel_hash.is_empty());
        }

        Ok(())
    }

    #[test]
    fn command_chrome_sequence_contract_rejects_corrupted_render_evidence()
    -> Result<(), Box<dyn Error>> {
        let sequence = run_command_chrome_artifact_sequence()?;

        let mut corrupted = sequence.clone();
        corrupted.frames[0].toolbar_pixels.paint_plan_hash = "mismatch".to_owned();
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

        let mut corrupted = sequence.clone();
        corrupted.frames[0].search_pixels.paint_plan_hash = "mismatch".to_owned();
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

        let floating_indices = sequence
            .frames
            .iter()
            .enumerate()
            .filter_map(|(index, frame)| {
                (frame.floating_pixels.is_some()
                    && frame
                        .floating
                        .as_ref()
                        .and_then(|floating| floating.artifact.as_ref())
                        .is_some())
                .then_some(index)
            })
            .collect::<Vec<_>>();
        assert!(!floating_indices.is_empty());
        for floating_index in floating_indices {
            let mut corrupted = sequence.clone();
            corrupted.frames[floating_index]
                .floating
                .iter_mut()
                .for_each(|floating| floating.artifact = None);
            assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

            let mut corrupted = sequence.clone();
            corrupted.frames[floating_index]
                .floating_pixels
                .iter_mut()
                .for_each(|pixels| pixels.paint_plan_hash = "mismatch".to_owned());
            assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());
        }

        let mut corrupted = sequence.clone();
        corrupted.frames[0].toolbar_pixels.width = 0;
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

        let mut corrupted = sequence.clone();
        corrupted.frames[0].search_pixels.height = 0;
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

        let mut corrupted = sequence.clone();
        corrupted.frames[0].toolbar_pixels.rgba.clear();
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

        let mut corrupted = sequence.clone();
        corrupted.frames[0].search_pixels.rgba.clear();
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

        let mut corrupted = sequence.clone();
        corrupted.frames[0].search.record.bounds.width = 0;
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

        let mut corrupted = sequence.clone();
        corrupted.frames[0].composite_pixels.pixel_hash.clear();
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

        let mut corrupted = sequence.clone();
        corrupted.frames[0].composite_pixels.rgba.fill(0);
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());

        let mut corrupted = sequence;
        corrupted.frames[0].composite_pixels.rgba = vec![1];
        assert!(assert_command_chrome_sequence_contract(&corrupted).is_err());
        Ok(())
    }

    fn read_manifest(path: &Path) -> Result<TestManifest, Box<dyn Error>> {
        let manifest_file = std::fs::File::open(path.join(ARTIFACT_MANIFEST_FILE))?;
        Ok(serde_json::from_reader(manifest_file)?)
    }

    fn test_temp_dir(prefix: &str) -> Result<PathBuf, Box<dyn Error>> {
        let mut path = std::env::temp_dir();
        path.push("katana-storybook-command-chrome-artifact-tests");
        path.push(format!(
            "{prefix}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));
        fs::create_dir_all(&path)?;
        let mut file = std::fs::File::create(path.join("lock"))?;
        file.write_all(b"command-chrome")?;
        Ok(path)
    }
}
