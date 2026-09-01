use super::types::{ContextMenuPaintPlan, EguiContextMenuFrameRecord, EguiContextMenuItemFrame};
use super::{
    ContextMenuPaintStyle, ContextMenuPresentation, ContextMenuPresentationItem,
    ContextMenuRasterStyle, EguiContextMenuAdapter,
};
use crate::egui::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use crate::egui::context_menu::ContextMenuAdapterError;
use crate::egui::text_surface::TextSurfaceContextTargetAnchor;
use crate::molecule::selection::{
    ContextMenuAction, ContextMenuItemKind, ContextMenuTypeAheadBuffer,
};
use crate::molecule::selection::{ContextMenuAnchor, ContextMenuSize, ContextMenuViewport};
use crate::render_model::UiRect;
use crate::text_selection::UiTextSelectionRange;
use crate::theme::{FontFamily, FontToken};

const FRAME_WIDTH_PX: f32 = 640.0;
const FRAME_HEIGHT_PX: f32 = 360.0;
const CONTEXT_X: i32 = 620;
const CONTEXT_Y: i32 = 340;
const FONT_SIZE_PX: f32 = 15.0;
const FONT_WEIGHT: u16 = 400;
const LINE_HEIGHT_PX: f32 = 22.0;
const KEYBOARD_INPUT_WIDTH_PX: f32 = 160.0;
const KEYBOARD_INPUT_HEIGHT_PX: f32 = 40.0;
const TEXT_RGBA: [u8; 4] = [230, 230, 230, 255];
const MENU_BACKGROUND_RGBA: [u8; 4] = [30, 30, 32, 255];
const MENU_HIGHLIGHTED_RGBA: [u8; 4] = [60, 80, 112, 255];
const MENU_DISABLED_RGBA: [u8; 4] = [45, 45, 48, 255];

fn pressed_key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}

fn collect_keyboard_actions(
    context: &egui::Context,
    events: Vec<egui::Event>,
    items: &[ContextMenuPresentationItem],
    submenu_path: &mut Vec<usize>,
    highlighted_path: &[usize],
    type_ahead: &mut ContextMenuTypeAheadBuffer,
) -> Vec<ContextMenuAction> {
    let mut actions = Vec::new();
    let mut frame_output = context.run_ui(
        egui::RawInput {
            events,
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(KEYBOARD_INPUT_WIDTH_PX, KEYBOARD_INPUT_HEIGHT_PX),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            actions = ui.input(|input| {
                super::interaction::keyboard_actions(
                    input,
                    items,
                    submenu_path,
                    highlighted_path,
                    type_ahead,
                )
            });
        },
    );
    frame_output.textures_delta.clear();
    actions
}

fn require_ok<T, E: std::fmt::Debug>(result: Result<T, E>, context: &str) -> Option<T> {
    if let Err(error) = &result {
        assert!(result.is_ok(), "{context}: {error:?}");
    }
    result.ok()
}

fn frame_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(FRAME_WIDTH_PX, FRAME_HEIGHT_PX),
        )),
        ..egui::RawInput::default()
    }
}

fn raster_style() -> ContextMenuRasterStyle {
    ContextMenuRasterStyle {
        font: FontToken {
            name: "context-menu-test".to_string(),
            family: FontFamily::Proportional,
            size: FONT_SIZE_PX,
            weight: FONT_WEIGHT,
        },
        text_color_rgba: TEXT_RGBA,
        icon_color_rgba: TEXT_RGBA,
        line_height_px: LINE_HEIGHT_PX,
    }
}

const fn paint_style() -> ContextMenuPaintStyle {
    ContextMenuPaintStyle {
        background_rgba: MENU_BACKGROUND_RGBA,
        highlighted_rgba: MENU_HIGHLIGHTED_RGBA,
        disabled_rgba: MENU_DISABLED_RGBA,
    }
}

#[path = "tests/adapter.rs"]
mod adapter;
#[path = "tests/interaction.rs"]
mod interaction;
#[path = "tests/serialization.rs"]
mod serialization;
