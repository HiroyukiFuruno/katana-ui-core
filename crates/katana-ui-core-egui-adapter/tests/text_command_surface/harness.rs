use super::fixtures::{
    FRAME_HEIGHT, FRAME_WIDTH, paint_style as chrome_paint, raster_style as chrome_raster,
    search_style, text_paint, text_raster,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceOutput,
    TextCommandSurfaceStyle,
};

pub(crate) fn adapter() -> Result<EguiTextCommandSurfaceAdapter, Box<dyn std::error::Error>> {
    EguiTextCommandSurfaceAdapter::with_text_raster_config(
        katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
    )
    .map_err(Into::into)
}

pub(crate) fn style() -> TextCommandSurfaceStyle {
    TextCommandSurfaceStyle {
        text_raster: text_raster(),
        text_paint: text_paint(),
        chrome_raster: chrome_raster(),
        chrome_paint: chrome_paint(),
        search: search_style(),
    }
}

pub(crate) fn run_frame(
    context: &egui::Context,
    adapter: &mut EguiTextCommandSurfaceAdapter,
    surface: &mut EguiTextCommandSurface,
    style: &TextCommandSurfaceStyle,
    events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, EguiTextCommandSurfaceOutput), Box<dyn std::error::Error>> {
    run_frame_sized(
        context,
        adapter,
        surface,
        style,
        egui::vec2(FRAME_WIDTH, FRAME_HEIGHT),
        events,
    )
}

pub(crate) fn run_frame_sized(
    context: &egui::Context,
    adapter: &mut EguiTextCommandSurfaceAdapter,
    surface: &mut EguiTextCommandSurface,
    style: &TextCommandSurfaceStyle,
    screen_size: egui::Vec2,
    events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, EguiTextCommandSurfaceOutput), Box<dyn std::error::Error>> {
    let mut result = None;
    let full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, screen_size)),
            events,
            ..egui::RawInput::default()
        },
        |ui| result = Some(adapter.show(ui, surface, style)),
    );
    let output = result.ok_or_else(|| std::io::Error::other("missing adapter output"))?;
    Ok((full, output?))
}

pub(crate) fn button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

pub(crate) fn secondary_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Secondary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

pub(crate) fn key(key: egui::Key, shift: bool) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers {
            shift,
            ..egui::Modifiers::default()
        },
    }
}
