use super::FULL_ROOT_MANIFEST_FILE_NAME;
use super::model::FullRootArtifactError;
use super::process::{VIDEO_ENCODER, VIDEO_MUXER, VIDEO_PIXEL_FORMAT};
use super::sequence::{frame_sequence_sha256, run_scripted_sequence, validate_sequence};
use super::write_artifact;
use eframe::App as _;
use eframe::egui;
use katana_ui_core::molecule::selection::ContextMenuItemKind;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn full_root_storybook_uses_only_the_public_facade_root() {
    let source = include_str!("text_command_root_storybook.rs");
    for forbidden in [
        "EguiTextCommandSurfaceRoot,",
        "EguiTextCommandSurfaceRootOutput",
        "EguiTextCommandSurfaceAdapter",
        "EguiTextCommandSurface,",
        "ArtifactCompositor",
        "PaintPlan",
        "TextureId",
        "egui::Id",
    ] {
        assert!(
            !source.contains(forbidden),
            "full-root Storybook leaked `{forbidden}`"
        );
    }
    assert!(source.contains("EguiTextCommandSurfaceHostRoot"));
    assert!(source.contains("EguiTextCommandSurfaceRootFactory"));
    assert!(source.contains("root.show(ui)"));
}

#[test]
fn full_root_trace_repeats_closed_evidence() -> Result<(), FullRootArtifactError> {
    let first = run_scripted_sequence()?;
    let second = run_scripted_sequence()?;
    validate_sequence(&first)?;
    validate_sequence(&second)?;
    assert_eq!(
        first
            .steps
            .iter()
            .map(|step| &step.evidence)
            .collect::<Vec<_>>(),
        second
            .steps
            .iter()
            .map(|step| &step.evidence)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        first
            .steps
            .iter()
            .map(|step| step.frame.record().rgba_hash())
            .collect::<Vec<_>>(),
        second
            .steps
            .iter()
            .map(|step| step.frame.record().rgba_hash())
            .collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn full_root_trace_contains_expected_contract_steps_in_order() -> Result<(), FullRootArtifactError>
{
    let sequence = run_scripted_sequence()?;
    validate_sequence(&sequence)?;

    let step_names: Vec<_> = sequence.steps.iter().map(|step| step.name).collect();
    assert_eq!("initial-root", step_names[0]);

    let required = [
        "focus-and-multiline-input",
        "ime-preedit",
        "ime-commit",
        "selection-anchored-floating-toolbar",
        "heading-and-dropdown",
        "context-menu-open-and-dismiss",
        "search-query-next-previous",
        "replace-and-replace-all",
    ];
    let mut cursor = 1;
    for required_step in required {
        let position = step_names
            .iter()
            .position(|name| *name == required_step)
            .ok_or_else(|| FullRootArtifactError::Contract(format!("missing {required_step}")))?;
        assert!(
            position >= cursor,
            "required step {required_step} appeared out of sequence"
        );
        cursor = position + 1;
    }
    assert_eq!(sequence.steps.len(), 9);
    Ok(())
}

fn sha(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

#[test]
fn full_root_artifact_generates_decodes_and_records_mp4() -> Result<(), FullRootArtifactError> {
    let output = PathBuf::from("target/text-command-root-storybook-test-artifact");
    write_artifact(&output)?;
    let output = fs::canonicalize(output)?;
    let manifest_path = output.join(FULL_ROOT_MANIFEST_FILE_NAME);
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(manifest_path)?)?;
    let sequence = run_scripted_sequence()?;
    let mp4_path = PathBuf::from(
        manifest["mp4"]["path"]
            .as_str()
            .ok_or_else(|| FullRootArtifactError::Contract("manifest mp4 path missing".into()))?,
    );
    assert!(mp4_path.is_absolute());
    assert!(mp4_path.is_file());
    assert_eq!(
        sha(&fs::read(&mp4_path)?),
        manifest["mp4"]["sha256"]
            .as_str()
            .ok_or_else(|| FullRootArtifactError::Contract("manifest mp4 SHA missing".into()))?
    );
    let frame_paths = sequence
        .steps
        .iter()
        .enumerate()
        .map(|(index, _)| output.join(format!("frame-{index:03}.png")))
        .collect::<Vec<_>>();
    assert_eq!(
        frame_sequence_sha256(&frame_paths)?,
        manifest["mp4"]["frame_sequence_sha256"]
            .as_str()
            .ok_or_else(|| {
                FullRootArtifactError::Contract("manifest frame sequence SHA missing".into())
            })?
    );
    assert_eq!(
        sequence.steps.len(),
        manifest["mp4"]["frame_count"]
            .as_u64()
            .ok_or_else(|| FullRootArtifactError::Contract("manifest frame count missing".into()))?
            as usize
    );
    assert_eq!(manifest["mp4"]["fps"]["numerator"], 1_000);
    assert_eq!(manifest["mp4"]["fps"]["denominator"], 180);
    assert_eq!(manifest["mp4"]["container"], VIDEO_MUXER);
    assert_eq!(manifest["mp4"]["codec"], VIDEO_ENCODER);
    assert_eq!(manifest["mp4"]["pixel_format"], VIDEO_PIXEL_FORMAT);
    assert_eq!(manifest["mp4"]["encoder_capability_verified"], true);
    assert_eq!(manifest["mp4"]["muxer_capability_verified"], true);
    assert_eq!(manifest["mp4"]["decoder"]["verified"], true);
    assert_eq!(
        manifest["mp4"]["decoder"]["decoded_frame_count"],
        sequence.steps.len()
    );
    assert_eq!(
        manifest["mp4"]["decoder"]["source_frame_hashes"],
        manifest["mp4"]["decoder"]["decoded_frame_hashes"]
    );
    assert_eq!(
        manifest["mp4"]["decoder"]["decoded_frame_hashes"]
            .as_array()
            .map(Vec::len),
        Some(sequence.steps.len())
    );
    assert!(
        manifest["mp4"]["ffmpeg_path"]
            .as_str()
            .is_some_and(|path| Path::new(path).is_absolute())
    );
    assert!(
        manifest["mp4"]["ffmpeg_version"]
            .as_str()
            .is_some_and(|version| version.starts_with("ffmpeg version "))
    );
    assert!(
        manifest["gif_path"]
            .as_str()
            .is_some_and(|path| Path::new(path).is_absolute())
    );
    assert!(
        manifest["gif_sha256"]
            .as_str()
            .is_some_and(|hash| !hash.is_empty())
    );
    assert!(output.join("motion.gif").is_file());
    for index in 0..sequence.steps.len() {
        assert!(
            output
                .join(format!("frame-{index:03}.manifest.json"))
                .is_file()
        );
    }
    Ok(())
}

#[test]
fn context_menu_fixture_is_stable_and_expected() {
    let menu = super::context_menu_fixture();
    assert!(menu.visible);
    assert_eq!(5, menu.items.len());
    assert_eq!("save", menu.items[0].id);
    assert_eq!("format", menu.items[1].id);
    assert_eq!("copy", menu.items[2].id);
    assert_eq!("paste", menu.items[3].id);
    assert_eq!(ContextMenuItemKind::Submenu, menu.items[1].kind);
    let mut format_children = menu.items[1].children.iter().map(|item| item.id.as_str());
    assert!(format_children.any(|id| id == "format-markdown"));
    assert!(format_children.any(|id| id == "format-plain"));
    assert_eq!(ContextMenuItemKind::Action, menu.items[2].kind);
}

#[test]
fn command_root_helpers_generate_expected_events() {
    let key_event = super::key(egui::Key::Enter, egui::Modifiers::NONE);
    assert!(matches!(
        key_event,
        egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    ));

    let pointer_event =
        super::pointer_button(egui::pos2(5.0, 7.0), egui::PointerButton::Primary, false);
    assert!(matches!(
        pointer_event,
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        } if pos == egui::pos2(5.0, 7.0)
    ));
}

#[test]
fn command_root_app_covers_bounded_and_unbounded_frame_lifecycles()
-> Result<(), super::FullRootArtifactError> {
    for (frames, expected_remaining) in [(0, None), (1, Some(0))] {
        let mut app = super::TextCommandRootStorybookApp::new(frames)?;
        let context = egui::Context::default();
        let mut frame = eframe::Frame::_new_kittest();
        let mut output = context.run_ui(Default::default(), |ui| app.ui(ui, &mut frame));
        output.textures_delta.clear();
        assert_eq!(expected_remaining, app.frames_remaining);
    }
    Ok(())
}
