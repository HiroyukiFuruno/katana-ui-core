use super::*;

#[test]
fn artifact_hash_fingerprints_a_real_command_chrome_value_deterministically() {
    let plan = CommandChromePaintPlan {
        surface_bounds: UiRect::new(0, 0, 1, 1),
        operations: vec![CommandChromePaintOperation {
            layer: EguiCommandChromeDrawLayer::PanelFill,
            clip_bounds: UiRect::new(0, 0, 1, 1),
            kind: CommandChromePaintOperationKind::Fill {
                bounds: UiRect::new(0, 0, 1, 1),
                color_rgba: [1, 2, 3, 255],
            },
        }],
    };
    let first = paint_plan_hash(&plan);
    let second = paint_plan_hash(&plan);

    assert_eq!(first.len(), 64);
    assert_eq!(first, second);
}

#[test]
fn command_chrome_artifact_frames_preserve_distinct_payload_hashes() {
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
        focused_action_id: Some("focused".to_string()),
        layers: vec![EguiCommandChromeDrawLayer::ActionFill],
    };
    let frame = CommandChromeArtifactFrame::new(
        record.clone(),
        plan.clone(),
        vec![CommandChromeToolbarEvent::CommandActivated {
            action_id: "focused".to_string().into(),
        }],
    );

    assert_eq!(frame.frame_record_hash, frame_record_hash(&record));
    assert_eq!(frame.paint_plan_hash, paint_plan_hash(&plan));
    assert_eq!(frame.frame_record_hash.len(), 64);
    assert_eq!(frame.paint_plan_hash.len(), 64);
    assert_ne!(frame.frame_record_hash, frame.paint_plan_hash);
    assert_eq!(frame.record, record);
    assert_eq!(frame.paint_plan, plan);
}

#[test]
fn paint_plan_json_matches_existing_serde_wire_for_all_layers_and_kinds()
-> Result<(), serde_json::Error> {
    let layers = [
        EguiCommandChromeDrawLayer::PanelFill,
        EguiCommandChromeDrawLayer::PanelBorder,
        EguiCommandChromeDrawLayer::ActionFill,
        EguiCommandChromeDrawLayer::IconTexture,
        EguiCommandChromeDrawLayer::TextTexture,
        EguiCommandChromeDrawLayer::FocusRing,
        EguiCommandChromeDrawLayer::TooltipFill,
        EguiCommandChromeDrawLayer::TooltipTexture,
    ];
    let mut operations = Vec::new();
    for (index, layer) in layers.into_iter().enumerate() {
        let bounds = UiRect::new(-(index as i32), index as i32, index as u32 + 1, 2);
        let kind = match index % 3 {
            0 => CommandChromePaintOperationKind::Fill {
                bounds,
                color_rgba: [0, 1, 2, 255],
            },
            1 => CommandChromePaintOperationKind::RoundedFill {
                bounds,
                color_rgba: [3, 4, 5, 255],
                radius_px: index as u32,
            },
            _ => CommandChromePaintOperationKind::Texture {
                bounds,
                texture: CommandChromePaintTexture {
                    identity: "quote:\" slash:\\ controls:\u{8}\u{c}\n\r\t\u{1} unicode:⭐️"
                        .to_string(),
                    width: 1,
                    height: 2,
                    rgba_pixels: vec![0, 127, 255],
                },
            },
        };
        operations.push(CommandChromePaintOperation {
            layer,
            clip_bounds: bounds,
            kind,
        });
    }
    let plan = CommandChromePaintPlan {
        surface_bounds: UiRect::new(-10, 20, 300, 400),
        operations,
    };

    assert_eq!(paint_plan_json(&plan), serde_json::to_vec(&plan)?);
    Ok(())
}
