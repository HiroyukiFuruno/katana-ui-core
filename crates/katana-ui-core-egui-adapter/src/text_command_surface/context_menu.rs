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
        let adapter = if let Some(adapter) = self.context_menu.as_mut() {
            adapter
        } else {
            let adapter = EguiContextMenuAdapter::with_catalog_and_metrics(
                std::sync::Arc::clone(&self.catalog),
                self.text_raster_config.clone(),
                std::rc::Rc::clone(&self.metrics),
            )?;
            self.context_menu.insert(adapter)
        };
        adapter.synchronize_presentation(presentation);
        if let Some(target) = text.context_target.clone() {
            self.context_target = Some(target);
        }
        if let Some(target) = self.context_target.clone() {
            adapter.request_open(target);
        }
        let output = adapter.show(
            ui,
            &style.context_menu_raster_style(),
            &style.context_menu_paint_style(),
        )?;
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
