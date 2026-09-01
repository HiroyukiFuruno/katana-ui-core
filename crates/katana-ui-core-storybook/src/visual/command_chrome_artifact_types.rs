use crate::visual::command_chrome_artifact::CommandChromePlanPixels;
use katana_ui_core::egui::command_chrome::{
    EguiCommandChromeFloatingOutput, EguiCommandChromeOutput, EguiCommandChromeSearchOutput,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeDropdownCloseReason, CommandChromeSearchEvent, CommandChromeToolbarEvent,
    FloatingCommandToolbarCloseReason, FloatingCommandToolbarEvent,
};
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::TextSurfaceEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub(crate) struct CommandChromeArtifactFrame {
    pub(crate) name: String,
    pub(crate) index: usize,
    pub(crate) toolbar: EguiCommandChromeOutput,
    pub(crate) floating: Option<EguiCommandChromeFloatingOutput>,
    pub(crate) search: EguiCommandChromeSearchOutput,
    pub(crate) accesskit_labels: Vec<String>,
    pub(crate) toolbar_pixels: CommandChromePlanPixels,
    pub(crate) floating_pixels: Option<CommandChromePlanPixels>,
    pub(crate) search_pixels: CommandChromePlanPixels,
    pub(crate) composite_pixels: CommandChromePlanPixels,
    pub(crate) frame_width: u32,
    pub(crate) frame_height: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct CommandChromeArtifactSequence {
    pub(crate) frames: Vec<CommandChromeArtifactFrame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StorybookCommandChromeManifest {
    pub(crate) schema: &'static str,
    pub(crate) input_origin: &'static str,
    pub(crate) artifact_encoder: &'static str,
    pub(crate) frames: Vec<StorybookCommandChromeManifestFrame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StorybookCommandChromeManifestFrame {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) png: String,
    pub(crate) frame_bounds: UiRect,
    pub(crate) toolbar_bounds: UiRect,
    pub(crate) floating_bounds: Option<UiRect>,
    pub(crate) search_bounds: UiRect,
    pub(crate) toolbar_frame_record_hash: String,
    pub(crate) toolbar_paint_plan_hash: String,
    pub(crate) toolbar_pixel_hash: String,
    pub(crate) floating_frame_record_hash: Option<String>,
    pub(crate) floating_paint_plan_hash: Option<String>,
    pub(crate) floating_pixel_hash: Option<String>,
    pub(crate) search_frame_record_hash: String,
    pub(crate) search_paint_plan_hash: String,
    pub(crate) search_pixel_hash: String,
    pub(crate) composite_pixel_hash: String,
    pub(crate) focused_action_id: Option<String>,
    pub(crate) search_focused_target: Option<String>,
    pub(crate) dropdown_open: bool,
    pub(crate) dropdown_item_count: usize,
    pub(crate) floating_tooltip_bounds: Option<UiRect>,
    pub(crate) toolbar_dropdown_close_reason: Option<CommandChromeDropdownCloseReason>,
    pub(crate) floating_close_reason: Option<FloatingCommandToolbarCloseReason>,
    pub(crate) accessibility_labels: Vec<String>,
    pub(crate) typed_events: Vec<StorybookCommandChromeTypedEvent>,
    pub(crate) star_variation_selector_present: bool,
    pub(crate) color_emoji_texture_present: bool,
    pub(crate) palette_identities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "event")]
pub(crate) enum StorybookCommandChromeTypedEvent {
    Toolbar(CommandChromeToolbarEvent),
    Floating(FloatingCommandToolbarEvent),
    Search(CommandChromeSearchEvent),
    SearchText(TextSurfaceEvent),
}
