use super::command_chrome_fixture::{
    floating_fixture, paint_style, raster_style, search_fixture, search_style, toolbar_fixture,
};
use katana_ui_core::egui::command_chrome::{
    EguiCommandChromeAdapter, EguiCommandChromeError, EguiCommandChromeFloatingOutput,
    EguiCommandChromeOutput, EguiCommandChromeSearchOutput,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchStrip, CommandChromeToolbar, FloatingCommandToolbar,
};

const TOOLBAR_TO_FLOATING_GAP_PX: f32 = 12.0;
const FLOATING_TO_SEARCH_GAP_PX: f32 = 72.0;

#[derive(Debug)]
pub(super) struct CommandChromeSurfaceFrame {
    pub(super) toolbar: EguiCommandChromeOutput,
    pub(super) floating: EguiCommandChromeFloatingOutput,
    pub(super) search: EguiCommandChromeSearchOutput,
}

pub(super) struct CommandChromeSurfaceFixture {
    pub(super) adapter: EguiCommandChromeAdapter,
    pub(super) toolbar: CommandChromeToolbar,
    pub(super) floating: FloatingCommandToolbar,
    pub(super) search: CommandChromeSearchStrip,
}

pub(super) struct CommandChromeSurface {
    adapter: EguiCommandChromeAdapter,
    toolbar: CommandChromeToolbar,
    floating: FloatingCommandToolbar,
    search: CommandChromeSearchStrip,
}

pub(super) fn command_chrome_surface_fixture(disable_regex: bool) -> CommandChromeSurfaceFixture {
    CommandChromeSurfaceFixture {
        adapter: EguiCommandChromeAdapter::default(),
        toolbar: toolbar_fixture(),
        floating: floating_fixture(),
        search: search_fixture(disable_regex),
    }
}

pub(super) fn show_command_chrome(
    ui: &mut egui::Ui,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
    floating: &mut FloatingCommandToolbar,
    search: &mut CommandChromeSearchStrip,
) -> Result<CommandChromeSurfaceFrame, EguiCommandChromeError> {
    let toolbar = adapter.show_toolbar(ui, toolbar, &raster_style(), &paint_style())?;
    ui.add_space(TOOLBAR_TO_FLOATING_GAP_PX);
    let floating = adapter.show_floating_toolbar(ui, floating, &raster_style(), &paint_style())?;
    ui.add_space(FLOATING_TO_SEARCH_GAP_PX);
    let search =
        adapter.show_search_strip(ui, search, &raster_style(), &paint_style(), &search_style())?;
    Ok(CommandChromeSurfaceFrame {
        toolbar,
        floating,
        search,
    })
}

impl CommandChromeSurface {
    pub(super) fn new(disable_regex: bool) -> Self {
        let CommandChromeSurfaceFixture {
            adapter,
            toolbar,
            floating,
            search,
        } = command_chrome_surface_fixture(disable_regex);
        Self {
            adapter,
            toolbar,
            floating,
            search,
        }
    }

    pub(super) fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<CommandChromeSurfaceFrame, EguiCommandChromeError> {
        show_command_chrome(
            ui,
            &mut self.adapter,
            &mut self.toolbar,
            &mut self.floating,
            &mut self.search,
        )
    }
}

#[cfg(test)]
#[path = "command_chrome_surface_integration_tests.rs"]
mod command_chrome_surface_integration_tests;
