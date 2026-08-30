use super::super::command_chrome_types::CommandChromePaintStyle;
use super::*;
use serde::{Serialize, ser::Serializer};

#[test]
fn artifact_hash_serializes_a_real_command_chrome_value() -> Result<(), EguiCommandChromeError> {
    let paint_style = CommandChromePaintStyle {
        action_rgba: [1, 2, 3, 255],
        hovered_action_rgba: [4, 5, 6, 255],
        disabled_action_rgba: [7, 8, 9, 255],
    };

    let hash = artifact_hash(&paint_style)?;

    assert_eq!(hash.len(), 64);
    Ok(())
}

#[test]
fn artifact_hash_propagates_serialization_error_without_hiding_root_cause() {
    struct FailingSerialization;

    impl Serialize for FailingSerialization {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(serde::ser::Error::custom("intentional failure"))
        }
    }

    let error = artifact_hash(&FailingSerialization)
        .err()
        .map(|error| error.to_string());

    assert!(error.as_deref().is_some_and(|message| {
        message.contains("intentional")
            && message.contains("command chrome artifact serialization failed")
    }));
}

#[test]
fn command_chrome_artifact_frames_preserve_distinct_payload_hashes()
-> Result<(), EguiCommandChromeError> {
    let plan = CommandChromePaintPlan {
        surface_bounds: UiRect::new(0, 0, 10, 10),
        operations: vec![CommandChromePaintOperation {
            layer: EguiCommandChromeDrawLayer::PanelFill,
            clip_bounds: UiRect::new(1, 2, 3, 4),
            kind: CommandChromePaintOperationKind::Fill {
                bounds: UiRect::new(5, 6, 7, 8),
                color_rgba: [10, 11, 12, 13],
            },
        }],
    };
    let record = EguiCommandChromeFrameRecord {
        bounds: UiRect::new(10, 20, 30, 40),
        actions: Vec::new(),
        dropdown: None,
        hidden_item_ids: vec!["hidden".to_string()],
        focused_action_id: Some("focused".to_string().into()),
        layers: vec![EguiCommandChromeDrawLayer::ActionFill],
    };
    let frame = CommandChromeArtifactFrame::new(
        record.clone(),
        plan.clone(),
        vec![CommandChromeToolbarEvent::CommandActivated {
            action_id: "focused".to_string().into(),
        }],
    )?;

    assert_eq!(frame.frame_record_hash, artifact_hash(&record)?);
    assert_eq!(frame.paint_plan_hash, artifact_hash(&plan)?);
    assert_eq!(frame.frame_record_hash.len(), 64);
    assert_eq!(frame.paint_plan_hash.len(), 64);
    assert_ne!(frame.frame_record_hash, frame.paint_plan_hash);
    assert_eq!(frame.record, record);
    assert_eq!(frame.paint_plan, plan);
    Ok(())
}
