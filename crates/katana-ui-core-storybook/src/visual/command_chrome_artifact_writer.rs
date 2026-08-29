#[path = "command_chrome_artifact_writer_composite.rs"]
mod command_chrome_artifact_writer_composite;
#[path = "command_chrome_artifact_writer_io.rs"]
mod command_chrome_artifact_writer_io;

use super::command_chrome_artifact::{RGBA_CHANNELS, render_command_chrome_plan};
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
        if frame.composite_pixels.pixel_hash.is_empty() {
            return Err(CommandChromeArtifactError::Contract(format!(
                "composite pixel hash missing on frame {}",
                frame.index
            )));
        }
        if !composite::has_non_zero_pixel(&frame.composite_pixels.rgba) {
            return Err(CommandChromeArtifactError::Contract(format!(
                "frame {} is fully blank",
                frame.index
            )));
        }
        if frame
            .composite_pixels
            .rgba
            .len()
            .checked_div(RGBA_CHANNELS)
            .is_some_and(|pixels| pixels == 0)
        {
            return Err(CommandChromeArtifactError::Contract(format!(
                "composite rgba invalid on frame {}",
                frame.index
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::command_chrome_script_types::StorybookCommandChromeTypedEvent;
    use super::{
        ARTIFACT_GIF_FILE, ARTIFACT_MANIFEST_FILE, run_command_chrome_artifact_sequence,
        write_scripted_artifact,
    };
    use katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent;
    use sha2::{Digest, Sha256};
    use std::error::Error;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    const EXPECTED_FRAME_COUNT: usize = 34;

    #[test]
    fn command_chrome_artifact_sequence_has_full_length_and_visible_frames()
    -> Result<(), Box<dyn Error>> {
        let output_dir = test_temp_dir("command_chrome_artifact_sequence")?;
        write_scripted_artifact(&output_dir)?;

        let manifest = read_manifest(&output_dir)?;
        let frames = manifest
            .get("frames")
            .and_then(|value| value.as_array())
            .ok_or_else(|| std::io::Error::other("manifest frames should be an array"))?;
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
            let frame_png = frame
                .get("png")
                .and_then(|value| value.as_str())
                .ok_or_else(|| std::io::Error::other("manifest frame should contain png"))?;
            let frame_bounds = frame
                .get("frame_bounds")
                .and_then(|value| value.as_object())
                .ok_or_else(|| std::io::Error::other("manifest frame should contain bounds"))?;
            let image = image::open(output_dir.join(frame_png))?;
            assert!(
                image.to_rgba8().pixels().any(|pixel| pixel[3] > 0),
                "composed frame pixels should be non-blank",
            );
            assert!(
                frame_bounds
                    .get("width")
                    .and_then(|value| value.as_u64())
                    .is_some_and(|width| width > 0)
            );
            assert!(
                frame_bounds
                    .get("height")
                    .and_then(|value| value.as_u64())
                    .is_some_and(|height| height > 0)
            );
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

        let disabled_frame = manifest_frames
            .iter()
            .find(|frame| frame.name == "frame-04")
            .ok_or_else(|| std::io::Error::other("disabled action frame should exist"))?;
        let has_command_activate = disabled_frame.typed_events.iter().any(|event| {
            matches!(
                event,
                StorybookCommandChromeTypedEvent::Toolbar(
                    CommandChromeToolbarEvent::CommandActivated { .. }
                )
            )
        });
        assert!(
            !has_command_activate,
            "disabled frame should not emit command activation events"
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
                let artifact = frame
                    .floating
                    .as_ref()
                    .and_then(|floating| floating.artifact.as_ref())
                    .ok_or_else(|| std::io::Error::other("floating artifact should exist"))?;
                assert_eq!(pixels.paint_plan_hash, artifact.paint_plan_hash);
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

    fn read_manifest(path: &Path) -> Result<serde_json::Value, Box<dyn Error>> {
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
