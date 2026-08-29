//! Root composition helpers for text command surface child adapters.

#[cfg(test)]
use crate::command_chrome::EguiCommandChromeOutput;
use crate::command_chrome::{EguiCommandChromeAdapter, EguiCommandChromeError};
use crate::context_menu::EguiContextMenuAdapter;
use crate::text_command_surface::accesskit_evidence::AccessKitEvidenceLedger;
use crate::text_command_surface::artifact;
use crate::text_command_surface::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
    EguiTextCommandSurfaceOutput, TextCommandSurfaceStyle,
};
use crate::text_surface::{EguiTextSurfaceInputPolicy, EguiTextSurfaceOutput};

use katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::{TextSurface, TextSurfaceAction};

const SEARCH_INPUT_FRAME_HEIGHT_PX: u32 = 4;

impl EguiTextCommandSurfaceAdapter {
    /// Composes all generic children inside the supplied actual root `egui::Ui`.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut EguiTextCommandSurface,
        style: &TextCommandSurfaceStyle,
    ) -> Result<EguiTextCommandSurfaceOutput, EguiTextCommandSurfaceError> {
        if let (Some(primary), Some(floating)) = (
            surface.toolbar.as_ref(),
            surface
                .deferred_floating_toolbar
                .as_ref()
                .or_else(|| surface.floating.as_ref().map(|value| value.toolbar_model())),
        ) && primary.command_family_id() == floating.command_family_id()
        {
            return Err(EguiTextCommandSurfaceError::DuplicateCommandFamilyMount {
                family: primary.command_family_id().clone(),
            });
        }
        AccessKitEvidenceLedger::begin_frame(ui.ctx());
        let root = ui.available_rect_before_wrap();
        let toolbar_height = surface
            .toolbar
            .as_ref()
            .map(|toolbar| {
                self.chrome
                    .measure_toolbar(ui, toolbar, &style.chrome_raster)
                    .map(|size| size.height.max(1) as f32)
            })
            .transpose()?
            .unwrap_or(0.0);
        /* WHY: The retained strip owns an input frame in addition to its requested content. */
        let search_height = surface.search.as_ref().map_or(0.0, |_| {
            style
                .search
                .input_height_px
                .saturating_add(SEARCH_INPUT_FRAME_HEIGHT_PX)
                .max(1) as f32
        });
        let text_height = (root.height() - toolbar_height - search_height).max(1.0);
        let toolbar_rect =
            egui::Rect::from_min_size(root.min, egui::vec2(root.width(), toolbar_height));
        let text_rect = egui::Rect::from_min_size(
            egui::pos2(root.min.x, toolbar_rect.max.y),
            egui::vec2(root.width(), text_height),
        );
        let search_rect = egui::Rect::from_min_size(
            egui::pos2(root.min.x, text_rect.max.y),
            egui::vec2(root.width(), search_height),
        );

        let toolbar = surface
            .toolbar
            .as_mut()
            .map(|toolbar| {
                self.show_in(ui, toolbar_rect, |child, adapter| {
                    adapter.show_toolbar(child, toolbar, &style.chrome_raster, &style.chrome_paint)
                })
            })
            .transpose()?;
        #[cfg(test)]
        let mut toolbar = toolbar;
        /* WHY: Evaluate focused search controls before the body so one RawInput text event
        cannot be dispatched to both retained children in the same root frame. */
        let search = surface
            .search
            .as_mut()
            .map(|search| {
                self.show_in(ui, search_rect, |child, adapter| {
                    adapter.show_search_strip(
                        child,
                        search,
                        &style.chrome_raster,
                        &style.chrome_paint,
                        &style.search,
                    )
                })
            })
            .transpose()?;
        if search
            .as_ref()
            .is_some_and(|value| value.record.focused_target.is_some())
        {
            let _ = surface
                .text
                .apply_action(TextSurfaceAction::SetFocus(false));
        }
        let menu_open = self
            .context_menu
            .as_ref()
            .is_some_and(EguiContextMenuAdapter::is_open);
        let text = self.show_text_in(ui, text_rect, &mut surface.text, style, menu_open)?;
        let selection = selection_for(surface);
        self.synchronize_floating_for_frame(surface, &text, selection);
        let floating = surface
            .floating
            .as_mut()
            .map(|floating| {
                self.chrome.show_floating_toolbar(
                    ui,
                    floating,
                    &style.chrome_raster,
                    &style.chrome_paint,
                )
            })
            .transpose()?;
        let context_menu = self.show_context_menu(ui, surface, &text, style)?;
        #[cfg(test)]
        inject_same_bounds_test_overlay(ui, toolbar.as_mut(), &mut self.chrome, style)?;
        if context_menu.as_ref().is_some_and(|output| {
            output.events.iter().any(|event| {
                matches!(
                    event,
                    katana_ui_core::molecule::selection::ContextMenuEvent::Closed { .. }
                )
            })
        }) {
            /* WHY: Root-owned dismissal restores the retained TextSurface focus state. */
            let _ = surface.text.apply_action(TextSurfaceAction::SetFocus(true));
            self.text.request_focus(true);
        }
        if floating.as_ref().is_some_and(|value| {
            value
                .events
                .iter()
                .any(|event| matches!(event, FloatingCommandToolbarEvent::Closed { .. }))
        }) {
            self.closed_selection = Some(selection);
        }
        if floating.as_ref().is_some_and(|value| {
            value.events.iter().any(|event| {
                matches!(
                    event,
                    FloatingCommandToolbarEvent::FocusReturnRequested { .. }
                        | FloatingCommandToolbarEvent::Closed { .. }
                )
            })
        }) {
            /* WHY: The root compositor owns focus hand-back; consumers never need egui ids. */
            let _ = surface.text.apply_action(TextSurfaceAction::SetFocus(true));
            self.text.request_focus(true);
        }
        ui.allocate_rect(root, egui::Sense::hover());
        let artifact_order = artifact::artifact_order_for_root(
            toolbar.is_some(),
            search.is_some(),
            floating
                .as_ref()
                .is_some_and(|value| value.artifact.is_some()),
            context_menu
                .as_ref()
                .is_some_and(|value| value.artifact.is_some()),
        );

        Ok(EguiTextCommandSurfaceOutput::from_root(
            ui_rect(root),
            text,
            toolbar,
            floating,
            search,
            context_menu,
            AccessKitEvidenceLedger::finish_frame(ui.ctx()),
            artifact_order,
        ))
    }

    fn show_text_in(
        &mut self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        surface: &mut TextSurface,
        style: &TextCommandSurfaceStyle,
        context_menu_open: bool,
    ) -> Result<EguiTextSurfaceOutput, EguiTextCommandSurfaceError> {
        surface.use_adapter_measured_viewport();
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect));
        if context_menu_open {
            self.text
                .show_with_input_policy(
                    &mut child,
                    surface,
                    &style.text_raster,
                    &style.text_paint,
                    &EguiTextSurfaceInputPolicy::context_menu(),
                )
                .map_err(Into::into)
        } else {
            self.text
                .show(&mut child, surface, &style.text_raster, &style.text_paint)
                .map_err(Into::into)
        }
    }

    fn show_in<T>(
        &mut self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        render: impl FnOnce(
            &mut egui::Ui,
            &mut EguiCommandChromeAdapter,
        ) -> Result<T, EguiCommandChromeError>,
    ) -> Result<T, EguiTextCommandSurfaceError> {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect));
        render(&mut child, &mut self.chrome).map_err(Into::into)
    }
}

#[cfg(test)]
fn inject_same_bounds_test_overlay(
    ui: &mut egui::Ui,
    toolbar: Option<&mut EguiCommandChromeOutput>,
    chrome: &mut EguiCommandChromeAdapter,
    style: &TextCommandSurfaceStyle,
) -> Result<(), EguiTextCommandSurfaceError> {
    let Some(toolbar) = toolbar else {
        return Ok(());
    };
    let bounds = toolbar.record.bounds;
    let rect = egui::Rect::from_min_size(
        egui::pos2(bounds.x as f32, bounds.y as f32),
        egui::vec2(bounds.width as f32, bounds.height as f32),
    );
    let mut render = |id: &str| -> Result<crate::command_chrome::EguiCommandChromeOutput, _> {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect));
        let mut overlay = katana_ui_core::molecule::command_chrome::CommandChromeToolbar::new();
        overlay = overlay.action(
            katana_ui_core::molecule::command_chrome::CommandChromeAction::new(id, "同一")
                .accessibility_label("同一 ⭐️"),
        );
        chrome.show_toolbar(
            &mut child,
            &mut overlay,
            &style.chrome_raster,
            &style.chrome_paint,
        )
    };
    let first = render("collision-left")?;
    let second = render("collision-right")?;
    toolbar.record.actions.extend(first.record.actions);
    toolbar.record.actions.extend(second.record.actions);
    toolbar.events.extend(first.events);
    toolbar.events.extend(second.events);
    Ok(())
}

fn selection_for(surface: &EguiTextCommandSurface) -> (usize, usize) {
    (
        surface.text.state().text_area.selection.start,
        surface.text.state().text_area.selection.end,
    )
}

fn ui_rect(rect: egui::Rect) -> UiRect {
    UiRect::new(
        rect.min.x.round() as i32,
        rect.min.y.round() as i32,
        rect.width().round().max(0.0) as u32,
        rect.height().round().max(0.0) as u32,
    )
}
