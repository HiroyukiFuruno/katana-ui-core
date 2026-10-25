use crate::egui::context_menu::{EguiContextMenuAdapter, EguiContextMenuOutput};
use crate::egui::text_command_surface::ContextMenuEnabledResolver;
use crate::egui::text_command_surface::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
    TextCommandSurfaceStyle,
};
use crate::egui::text_surface::EguiTextSurfaceOutput;
use crate::molecule::selection::ContextMenuItemKind;

impl EguiTextCommandSurfaceAdapter {
    pub(super) fn show_context_menu(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut EguiTextCommandSurface,
        text: &EguiTextSurfaceOutput,
        style: &TextCommandSurfaceStyle,
    ) -> Result<Option<EguiContextMenuOutput>, EguiTextCommandSurfaceError> {
        let Some(mut presentation) = surface.context_menu.clone() else {
            self.context_menu = None;
            self.context_target = None;
            return Ok(None);
        };
        let adapter = self.context_menu.get_or_insert_with(|| {
            EguiContextMenuAdapter::with_resources_and_metrics(
                &self.text_raster_resources,
                std::rc::Rc::clone(&self.metrics),
            )
        });
        if let Some(output) = adapter.dismiss_from_input(ui, text.context_target.is_none()) {
            self.context_target = None;
            return Ok(Some(output));
        }
        if presentation.visible
            && (text.context_target.is_some() || self.context_target.is_some())
            && let Some(resolver) = self.context_menu_enabled_resolver.as_mut()
        {
            resolve_enabled_items(&mut presentation.items, resolver)
                .map_err(EguiTextCommandSurfaceError::ContextMenuEnabledResolver)?;
        }
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
                crate::molecule::selection::ContextMenuEvent::Closed { .. }
            )
        }) {
            self.context_target = None;
        }
        Ok(Some(output))
    }
}

fn resolve_enabled_items(
    items: &mut [crate::egui::context_menu::ContextMenuPresentationItem],
    resolver: &mut ContextMenuEnabledResolver,
) -> Result<(), crate::egui::text_command_surface::KucContextMenuEnabledResolverError> {
    for item in items {
        if item.kind != ContextMenuItemKind::Divider {
            item.enabled = resolver(&item.id, item.enabled)?;
        }
        resolve_enabled_items(&mut item.children, resolver)?;
    }
    Ok(())
}
