use super::*;
use crate::svg_raster::UiSvgRasterError;
use crate::text_raster::PlatformTextRasterError;
use serde::ser::Serializer;

#[test]
fn text_surface_error_messages_include_cause_for_user_facing_variants() {
    assert_eq!(
        EguiTextSurfaceError::FrameNotProduced.to_string(),
        "egui did not produce a text surface frame"
    );
    assert_eq!(
        EguiTextSurfaceError::Raster(PlatformTextRasterError::EmptyText).to_string(),
        "text surface raster failed: platform text raster request must not be empty"
    );
    assert_eq!(
        EguiTextSurfaceError::Svg(UiSvgRasterError::InvalidDimensions {
            width_px: 0,
            height_px: 0,
        })
        .to_string(),
        "gutter svg raster failed: InvalidDimensions { width_px: 0, height_px: 0 }"
    );
    assert_eq!(
        EguiTextSurfaceError::ArtifactSerialization("error-detail".to_string()).to_string(),
        "text surface artifact serialization failed: error-detail"
    );
    assert_eq!(
        EguiTextSurfaceError::from(UiSvgRasterError::InvalidDimensions {
            width_px: 0,
            height_px: 0,
        }),
        EguiTextSurfaceError::Svg(UiSvgRasterError::InvalidDimensions {
            width_px: 0,
            height_px: 0,
        })
    );
}

#[test]
fn artifact_hash_propagates_serialization_error_without_hiding_context() {
    struct FailingSerialization;

    impl Serialize for FailingSerialization {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(serde::ser::Error::custom(
                "intentional text-surface artifact hash failure",
            ))
        }
    }

    let error = artifact_hash(&FailingSerialization)
        .expect_err("text surface artifact hash should fail on serialization error");
    assert!(
        matches!(error, EguiTextSurfaceError::ArtifactSerialization(message)
            if message == "intentional text-surface artifact hash failure")
    );
}
