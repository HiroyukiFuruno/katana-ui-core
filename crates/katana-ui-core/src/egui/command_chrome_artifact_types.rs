use super::command_chrome_types::{
    EguiCommandChromeDrawLayer, EguiCommandChromeFloatingFrameRecord, EguiCommandChromeFrameRecord,
    EguiCommandChromeSearchFrameRecord,
};
use crate::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use crate::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use crate::text_surface::TextSurfaceEvent;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiCommandChromeFloatingArtifactFrame {
    pub record: EguiCommandChromeFloatingFrameRecord,
    pub paint_plan: CommandChromePaintPlan,
    pub events: Vec<FloatingCommandToolbarEvent>,
    pub frame_record_hash: String,
    pub paint_plan_hash: String,
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
