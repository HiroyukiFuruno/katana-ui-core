use super::{
    metadata::{FullRootArtifactError, FullRootArtifactManifest},
    validation::{encode_png, sha256_hex},
};
use crate::FullRootArtifactWriter;
use crate::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootFactory, TextCommandSurfaceStyle,
};
use image::GenericImageView;
use katana_ui_core::atom::TextArea;
use katana_ui_core::text_surface::{TextSurfaceProps, TextSurfaceViewport};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const ROOT_FRAME_WIDTH: f32 = 640.0;
const ROOT_FRAME_HEIGHT: f32 = 360.0;
const VIEWPORT_WIDTH: u32 = 10;
const VIEWPORT_HEIGHT: u32 = 12;

#[test]
fn png_encoding_has_header_dimensions_and_non_empty_pixels() {
    let rgba = [12, 34, 56, 255, 0, 0, 0, 255];
    let png = encode_png(&rgba, 2, 1).expect("PNG encoding should succeed");
    assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    let decoded = image::load_from_memory(&png).expect("PNG should decode");
    assert_eq!(decoded.dimensions(), (2, 1));
    assert!(png.iter().any(|byte| *byte != 0));
    assert_eq!(sha256_hex(&png), sha256_hex(&png));
}

#[test]
fn invalid_path_and_stage_are_typed_failures() {
    assert!(matches!(
        super::validation::validate_output_dir(Path::new("")),
        Err(FullRootArtifactError::InvalidPath("path is empty"))
    ));
    assert!(matches!(
        super::validation::validate_stage_id("../escape"),
        Err(FullRootArtifactError::InvalidStageId)
    ));
}

#[test]
fn manifest_contains_only_root_metadata() {
    let manifest = FullRootArtifactManifest::from_test_parts(
        2,
        1,
        "record",
        "pixels",
        "png",
        Path::new("frame.png"),
    );
    let value: serde_json::Value =
        serde_json::from_slice(&serde_json::to_vec(&manifest).expect("manifest should encode"))
            .expect("manifest should be JSON");
    let object = value.as_object().expect("manifest should be an object");
    assert_eq!(object.len(), 6);
    for forbidden in ["rgba", "child", "paint", "palette", "geometry", "accesskit"] {
        assert!(!value.to_string().to_lowercase().contains(forbidden));
    }
}

#[test]
fn write_produces_png_and_manifest_with_expected_metadata() {
    let dir = tempfile_dir("full-root-success");
    let frame = root_frame("frame-000");
    let artifact = FullRootArtifactWriter::new()
        .write(&frame, dir.as_path(), "frame-000")
        .expect("full root write should succeed");
    assert_eq!(artifact.stage_id(), "frame-000");
    assert!(artifact.png_path().is_file());
    assert!(artifact.manifest_path().is_file());
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(artifact.manifest_path()).expect("manifest should read"),
    )
    .expect("manifest should parse");
    assert_eq!(manifest["width"], serde_json::json!(artifact.width()));
    assert_eq!(manifest["height"], serde_json::json!(artifact.height()));
    assert_eq!(
        artifact.png_sha256(),
        sha256_hex(&std::fs::read(artifact.png_path()).expect("png should read"))
    );
}

#[test]
fn write_rejects_bad_stage_and_zero_pixel_frame() {
    let dir = tempfile_dir("full-root-fail");
    let frame = root_frame("frame-000");
    assert!(matches!(
        FullRootArtifactWriter::new().write(&frame, dir.as_path(), "../bad"),
        Err(FullRootArtifactError::InvalidStageId)
    ));
}

fn root_frame(stage_id: &str) -> EguiTextCommandSurfaceHostRootFrame {
    let presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(katana_ui_core::render_model::UiStateId::new(stage_id)),
        text: text_surface_presentation(),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    };
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"full-root-target".to_vec(),
        presentation,
        TextCommandSurfaceStyle::standard(),
    )
    .expect("token should encode");
    let mut root: EguiTextCommandSurfaceHostRoot = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("host root should retain");
    let mut captured = None;
    let context = egui::Context::default();
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(ROOT_FRAME_WIDTH, ROOT_FRAME_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            captured = Some(root.show(ui));
        },
    );
    full_output.textures_delta.clear();
    captured
        .expect("root should render")
        .expect("host root should render")
}

fn text_surface_presentation() -> katana_ui_core::text_surface::TextSurfacePresentation {
    let props = text_surface();
    katana_ui_core::text_surface::TextSurfacePresentation::from_props(&props)
}

fn text_surface() -> TextSurfaceProps {
    let mut props = TextSurfaceProps::new(
        TextArea::new("full-root-surface").value("Hello"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
    );
    props.accessibility_label = "root".to_owned();
    props
}

fn tempfile_dir(label: &str) -> PathBuf {
    let pid = std::process::id();
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should work")
        .as_millis();
    let path = std::env::temp_dir().join(format!("kuc-full-root-{label}-{pid}-{millis}"));
    std::fs::create_dir_all(&path).expect("test directory should create");
    path
}
