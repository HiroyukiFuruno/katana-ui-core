use crate::context_menu::{EguiContextMenuAdapter, EguiContextMenuOutput};
use crate::text_command_surface::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
    TextCommandSurfaceStyle,
};
use crate::text_surface::EguiTextSurfaceOutput;

impl EguiTextCommandSurfaceAdapter {
    pub(super) fn show_context_menu(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut EguiTextCommandSurface,
        text: &EguiTextSurfaceOutput,
        style: &TextCommandSurfaceStyle,
    ) -> Result<Option<EguiContextMenuOutput>, EguiTextCommandSurfaceError> {
        let Some(presentation) = surface.context_menu.clone() else {
            self.context_menu = None;
            self.context_target = None;
            return Ok(None);
        };
        let adapter = self.context_menu.get_or_insert_with(|| {
            EguiContextMenuAdapter::with_catalog(
                std::sync::Arc::clone(&self.catalog),
                self.text_raster_config.clone(),
            )
        });
        adapter.synchronize_presentation(presentation);
        if let Some(target) = text.context_target.clone() {
            self.context_target = Some(target);
        }
        if let Some(target) = self.context_target.clone() {
            adapter.request_open(target);
        }
        let raster = style.context_menu_raster_style();
        let paint = style.context_menu_paint_style();
        let output = adapter.show(ui, &raster, &paint)?;
        if output.events.iter().any(|event| {
            matches!(
                event,
                katana_ui_core::molecule::selection::ContextMenuEvent::Closed { .. }
            )
        }) {
            self.context_target = None;
        }
        Ok(Some(output))
    }
}
