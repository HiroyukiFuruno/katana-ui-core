use crate::visual::command_chrome_fixture::{
    FRAME_HEIGHT, FRAME_WIDTH, floating_toolbar_fixture, paint_style as chrome_paint,
    raster_style as chrome_raster, search_fixture, search_style, toolbar_fixture,
};
use crate::visual::text_surface_fixture::{
    paint_style as text_paint, raster_style as text_raster, text_surface_fixture,
};

use super::facts;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceOutput,
    TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::command_chrome::CommandChromeFamilyId;

pub(crate) struct Harness;

impl Harness {
    pub(crate) fn style() -> TextCommandSurfaceStyle {
        TextCommandSurfaceStyle {
            text_raster: text_raster(),
            text_paint: text_paint(),
            chrome_raster: chrome_raster(),
            chrome_paint: chrome_paint(),
            search: search_style(),
        }
    }

    pub(crate) fn run_frame_for_fact(
        events: Vec<egui::Event>,
    ) -> Result<facts::FrameFacts, Box<dyn std::error::Error>> {
        let raster_config = katana_ui_core::text_raster::PlatformTextRasterConfig::default();
        let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(raster_config)?;
        let mut surface = EguiTextCommandSurface::new(text_surface_fixture())
            .with_toolbar(toolbar_fixture().command_family(CommandChromeFamilyId::new("primary")))
            .with_floating_toolbar(
                floating_toolbar_fixture().command_family(CommandChromeFamilyId::new("floating")),
                katana_ui_core::molecule::command_chrome::FloatingCommandToolbarVisibility::Closed,
            )
            .with_search_strip(search_fixture(false));
        let style = Self::style();
        let context = egui::Context::default();
        context.enable_accesskit();
        let (full, output) = Self::run_frame(&context, &mut adapter, &mut surface, &style, events)?;
        facts::FrameFacts::collect(&full, &output)
    }

    pub(crate) fn run_frame(
        context: &egui::Context,
        adapter: &mut EguiTextCommandSurfaceAdapter,
        surface: &mut EguiTextCommandSurface,
        style: &TextCommandSurfaceStyle,
        events: Vec<egui::Event>,
    ) -> Result<(egui::FullOutput, EguiTextCommandSurfaceOutput), Box<dyn std::error::Error>> {
        let mut output = None;
        let mut full = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(FRAME_WIDTH, FRAME_HEIGHT),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ui| output = Some(adapter.show(ui, surface, style)),
        );
        full.textures_delta.clear();
        let output = output.ok_or(std::io::Error::other("actual root did not produce a frame"))?;
        Ok((full, output?))
    }

    pub(crate) fn button(point: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
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
}
