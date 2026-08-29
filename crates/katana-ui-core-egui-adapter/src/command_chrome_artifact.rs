use super::command_chrome_types::{
    EguiCommandChromeDrawLayer, EguiCommandChromeError, EguiCommandChromeFloatingFrameRecord,
    EguiCommandChromeFrameRecord, EguiCommandChromeSearchFrameRecord,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use katana_ui_core::text_surface::TextSurfaceEvent;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromePaintTexture {
    pub identity: String,
    pub width: u32,
    pub height: u32,
    pub rgba_pixels: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromePaintOperationKind {
    Fill {
        bounds: UiRect,
        color_rgba: [u8; RGBA_CHANNEL_COUNT],
    },
    RoundedFill {
        bounds: UiRect,
        color_rgba: [u8; RGBA_CHANNEL_COUNT],
        radius_px: u32,
    },
    Texture {
        bounds: UiRect,
        texture: CommandChromePaintTexture,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromePaintOperation {
    pub layer: EguiCommandChromeDrawLayer,
    pub clip_bounds: UiRect,
    pub kind: CommandChromePaintOperationKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromePaintPlan {
    pub surface_bounds: UiRect,
    pub operations: Vec<CommandChromePaintOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeArtifactFrame {
    pub record: EguiCommandChromeFrameRecord,
    pub paint_plan: CommandChromePaintPlan,
    pub events: Vec<CommandChromeToolbarEvent>,
    pub frame_record_hash: String,
    pub paint_plan_hash: String,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeFloatingArtifactFrame {
    pub record: EguiCommandChromeFloatingFrameRecord,
    pub paint_plan: CommandChromePaintPlan,
    pub events: Vec<FloatingCommandToolbarEvent>,
    pub frame_record_hash: String,
    pub paint_plan_hash: String,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeSearchArtifactFrame {
    pub record: EguiCommandChromeSearchFrameRecord,
    pub paint_plan: CommandChromePaintPlan,
    pub events: Vec<CommandChromeSearchEvent>,
    pub text_events: Vec<TextSurfaceEvent>,
    pub frame_record_hash: String,
    pub paint_plan_hash: String,
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
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
