use super::*;

#[test]
fn context_menu_adapter_error_display_and_conversion_cover_paths() {
    use katana_ui_core_svg_raster::UiSvgRasterError;
    use katana_ui_core_text_raster::PlatformTextRasterError;
    let raster = ContextMenuAdapterError::from(PlatformTextRasterError::EmptyText);
    let svg = ContextMenuAdapterError::from(UiSvgRasterError::EmptySource);
    let artifact = ContextMenuAdapterError::ArtifactSerialization("artifact".to_owned());
    assert!(matches!(raster, ContextMenuAdapterError::Raster(_)));
    assert!(matches!(svg, ContextMenuAdapterError::Svg(_)));
    assert!(matches!(
        artifact,
        ContextMenuAdapterError::ArtifactSerialization(_)
    ));
    assert!(format!("{raster}").contains("context menu raster failed"));
    assert!(format!("{svg}").contains("context menu SVG raster failed"));
    assert!(format!("{artifact}").contains("context menu artifact serialization failed"));
}

#[test]
fn context_menu_artifact_frame_generates_separate_payload_hashes()
-> Result<(), ContextMenuAdapterError> {
    let record = EguiContextMenuFrameRecord {
        bounds: UiRect::new(0, 0, 80, 40),
        viewport_bounds: UiRect::new(10, 10, 120, 200),
        highlighted_path: vec![0],
        focused: false,
        items: vec![EguiContextMenuItemFrame {
            id: "item".to_owned(),
            bounds: UiRect::new(0, 0, 80, 20),
            disabled: false,
            checked: false,
        }],
    };
    let paint_plan = ContextMenuPaintPlan {
        surface_bounds: UiRect::new(0, 0, 120, 200),
        operations: vec![],
    };
    let frame = super::super::artifact::artifact_frame(record.clone(), paint_plan, vec![])?;
    assert_eq!(frame.frame_record_hash.len(), 64);
    assert_eq!(frame.paint_plan_hash.len(), 64);
    assert_ne!(frame.frame_record_hash, frame.paint_plan_hash);
    assert_eq!(frame.record, record);
    assert_eq!(frame.paint_plan.surface_bounds, UiRect::new(0, 0, 120, 200));
    Ok(())
}

#[test]
fn context_menu_artifact_hash_propagates_serialization_failure() {
    use serde::{Serialize, Serializer};
    struct FailingSerialization;
    impl Serialize for FailingSerialization {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(serde::ser::Error::custom("intentional failure"))
        }
    }
    let error = super::super::artifact::artifact_hash(&FailingSerialization)
        .err()
        .map(|error| error.to_string());
    assert!(error.as_deref().is_some_and(|message| {
        message.contains("intentional")
            && message.contains("context menu artifact serialization failed")
    }));
}
