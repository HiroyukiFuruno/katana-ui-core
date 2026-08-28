pub(super) use super::command_chrome_artifact_types::{
    CommandChromeArtifactFrame, CommandChromePaintOperation, CommandChromePaintOperationKind,
    CommandChromePaintPlan, CommandChromePaintTexture, EguiCommandChromeFloatingArtifactFrame,
    EguiCommandChromeSearchArtifactFrame,
};
use super::command_chrome_types::{
    EguiCommandChromeError, EguiCommandChromeFloatingFrameRecord, EguiCommandChromeFrameRecord,
    EguiCommandChromeSearchFrameRecord,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::text_surface::TextSurfaceEvent;
use serde::Serialize;
use sha2::{Digest, Sha256};

impl CommandChromeArtifactFrame {
    pub(super) fn new(
        record: EguiCommandChromeFrameRecord,
        paint_plan: CommandChromePaintPlan,
        events: Vec<CommandChromeToolbarEvent>,
    ) -> Result<Self, EguiCommandChromeError> {
        Ok(Self {
            frame_record_hash: artifact_hash(&record)?,
            paint_plan_hash: artifact_hash(&paint_plan)?,
            record,
            paint_plan,
            events,
        })
    }
}

impl EguiCommandChromeFloatingArtifactFrame {
    pub(super) fn new(
        record: EguiCommandChromeFloatingFrameRecord,
        paint_plan: CommandChromePaintPlan,
        events: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<Self, EguiCommandChromeError> {
        Ok(Self {
            frame_record_hash: artifact_hash(&record)?,
            paint_plan_hash: artifact_hash(&paint_plan)?,
            record,
            paint_plan,
            events,
        })
    }
}

impl EguiCommandChromeSearchArtifactFrame {
    pub(super) fn new(
        record: EguiCommandChromeSearchFrameRecord,
        paint_plan: CommandChromePaintPlan,
        events: Vec<CommandChromeSearchEvent>,
        text_events: Vec<TextSurfaceEvent>,
    ) -> Result<Self, EguiCommandChromeError> {
        Ok(Self {
            frame_record_hash: artifact_hash(&record)?,
            paint_plan_hash: artifact_hash(&paint_plan)?,
            record,
            paint_plan,
            events,
            text_events,
        })
    }
}

fn artifact_hash(value: &impl Serialize) -> Result<String, EguiCommandChromeError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| EguiCommandChromeError::ArtifactSerialization(error.to_string()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

#[cfg(test)]
mod tests {
    use super::CommandChromePaintOperationKind::{Fill, Texture};
    use super::{
        CommandChromePaintOperation, CommandChromePaintPlan, CommandChromePaintTexture,
        EguiCommandChromeError, EguiCommandChromeFloatingFrameRecord, EguiCommandChromeFrameRecord,
        EguiCommandChromeSearchFrameRecord, artifact_hash,
    };
    use crate::command_chrome::EguiCommandChromeDrawLayer;
    use crate::text_surface::EguiTextSurfaceFrameRecord;
    use katana_ui_core::accessibility::AccessibilityLabel as CoreAccessibilityText;
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeSearchEvent, CommandChromeToolbarEvent,
    };
    use katana_ui_core::molecule::toolbar::ToolbarActionId;
    use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};
    use katana_ui_core::text_selection::UiTextSelectionRange;
    use katana_ui_core::text_surface::{
        TextSurfaceAccessibilityAction, TextSurfaceAccessibilityNode,
        TextSurfaceAccessibilityTarget, TextSurfaceAccessibilityTree, TextSurfaceEvent,
        TextSurfaceFrameRecord, TextSurfaceSelectionFrame, TextSurfaceViewport,
    };
    use serde::{Serialize, Serializer, ser::Error as SerdeError};

    #[derive(Debug)]
    struct SerializeFail;

    impl Serialize for SerializeFail {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(SerdeError::custom("failed intentionally"))
        }
    }

    fn frame_record() -> EguiCommandChromeFrameRecord {
        EguiCommandChromeFrameRecord {
            bounds: UiRect::new(0, 0, 10, 20),
            actions: vec![],
            dropdown: None,
            hidden_item_ids: Vec::new(),
            focused_action_id: None,
            layers: Vec::new(),
        }
    }

    fn paint_plan() -> CommandChromePaintPlan {
        CommandChromePaintPlan {
            surface_bounds: UiRect::new(0, 0, 10, 20),
            operations: vec![CommandChromePaintOperation {
                layer: EguiCommandChromeDrawLayer::PanelFill,
                clip_bounds: UiRect::new(0, 0, 10, 10),
                kind: Fill {
                    bounds: UiRect::new(0, 0, 10, 10),
                    color_rgba: [0; RGBA_CHANNEL_COUNT],
                },
            }],
        }
    }

    #[test]
    fn artifact_hash_wraps_serialization_failures_for_closed_artifact_errors() {
        let result = artifact_hash(&SerializeFail);
        assert!(matches!(
            result,
            Err(EguiCommandChromeError::ArtifactSerialization(_))
        ));
    }

    #[test]
    fn normal_frames_compute_hashes_and_preserve_payloads() {
        let frame = super::CommandChromeArtifactFrame::new(
            frame_record(),
            paint_plan(),
            vec![CommandChromeToolbarEvent::CommandActivated {
                action_id: ToolbarActionId::new("format"),
            }],
        )
        .expect("frame can be created");
        assert_eq!(
            frame.events,
            vec![CommandChromeToolbarEvent::CommandActivated {
                action_id: ToolbarActionId::new("format"),
            }],
        );
        assert_eq!(frame.paint_plan_hash, frame.paint_plan_hash);
        assert!(!frame.frame_record_hash.is_empty());
        assert!(!frame.paint_plan_hash.is_empty());
    }

    #[test]
    fn floating_frame_constructor_has_independent_hashes() {
        let floating = EguiCommandChromeFloatingFrameRecord {
            surface_id: "surface".to_string(),
            anchor_bounds: UiRect::new(1, 2, 3, 4),
            panel_bounds: UiRect::new(5, 6, 7, 8),
            toolbar: frame_record(),
            tooltip_bounds: None,
            tooltip_raster_identity: None,
        };
        let frame = super::EguiCommandChromeFloatingArtifactFrame::new(
            floating.clone(),
            paint_plan(),
            Vec::new(),
        )
        .expect("floating frame can be created");
        assert_eq!(frame.record, floating);
        assert!(frame.events.is_empty());
        assert!(!frame.frame_record_hash.is_empty());
        assert!(!frame.paint_plan_hash.is_empty());
    }

    #[test]
    fn search_frame_constructor_hashes_are_stable() {
        let search = EguiCommandChromeSearchFrameRecord {
            bounds: UiRect::new(0, 0, 1, 1),
            query: EguiTextSurfaceFrameRecord {
                frame: TextSurfaceFrameRecord {
                    layout_identity: "layout".to_string(),
                    content_bounds: UiRect::new(0, 0, 1, 1),
                    surface_bounds: UiRect::new(0, 0, 1, 1),
                    viewport_bounds: UiRect::new(0, 0, 1, 1),
                    viewport: TextSurfaceViewport::default(),
                    visible_logical_rows: Vec::new(),
                    caret: 0,
                    selection_start: 0,
                    selection_end: 0,
                    selection: TextSurfaceSelectionFrame {
                        range: UiTextSelectionRange::new(0, 0),
                        rects: Vec::new(),
                        caret: UiRect::new(0, 0, 0, 0),
                    },
                    preedit: None,
                    annotations: Vec::new(),
                    gutter: Vec::new(),
                    accessibility: TextSurfaceAccessibilityTree {
                        root: TextSurfaceAccessibilityNode {
                            target: TextSurfaceAccessibilityTarget::Surface,
                            role: katana_ui_core::accessibility::AccessibilityRole::Text,
                            label: CoreAccessibilityText::new(""),
                            bounds: UiRect::new(0, 0, 1, 1),
                            active: false,
                            hovered: false,
                            focused: false,
                            editable: false,
                            readonly: false,
                            disabled: false,
                            disabled_reason: None,
                            description: None,
                            selection: None,
                        },
                        gutter_targets: Vec::new(),
                        context_target: None,
                        actions: Vec::<TextSurfaceAccessibilityAction>::new(),
                    },
                },
                raster_identity: "raster".to_string(),
                texture_bounds: UiRect::new(0, 0, 1, 1),
                placeholder_raster_identity: None,
                placeholder_texture_bounds: None,
                hit_target: "surface".to_string(),
                layers: Vec::new(),
                scroll_request: None,
                focus_request: None,
            },
            replace: None,
            controls: Vec::new(),
            focused_target: None,
            layers: Vec::new(),
        };
        let frame = super::EguiCommandChromeSearchArtifactFrame::new(
            search.clone(),
            paint_plan(),
            Vec::new(),
            Vec::new(),
        )
        .expect("search frame can be created");
        assert_eq!(frame.record, search);
        assert_eq!(frame.events, Vec::<CommandChromeSearchEvent>::new());
        assert_eq!(frame.text_events, Vec::<TextSurfaceEvent>::new());
        assert!(!frame.frame_record_hash.is_empty());
        assert!(!frame.paint_plan_hash.is_empty());
    }

    #[test]
    fn texture_plan_entries_are_hash_sensitive_to_identity() {
        let mut left = paint_plan();
        let mut right = paint_plan();
        left.operations[0].kind = Texture {
            bounds: UiRect::new(0, 0, 1, 1),
            texture: CommandChromePaintTexture {
                identity: "left".to_string(),
                width: 1,
                height: 1,
                rgba_pixels: vec![255; 4],
            },
        };
        right.operations[0].kind = Texture {
            bounds: UiRect::new(0, 0, 1, 1),
            texture: CommandChromePaintTexture {
                identity: "right".to_string(),
                width: 1,
                height: 1,
                rgba_pixels: vec![255; 4],
            },
        };
        assert_ne!(
            artifact_hash(&left).expect("left hash"),
            artifact_hash(&right).expect("right hash")
        );
    }
}
