use super::super::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
    EguiTextCommandSurfaceOutput, RootChildOutputs, TextCommandSurfaceStyle,
};
use crate::egui::command_chrome::{EguiCommandChromeAdapter, EguiCommandChromeError};
use crate::egui::text_surface::{EguiTextSurfaceInputPolicy, EguiTextSurfaceOutput};
use crate::interaction::placement::Rect;
use crate::molecule::command_chrome::{
    FloatingCommandToolbarPresentation, FloatingCommandToolbarVisibility,
};
use crate::render_model::UiRect;
use crate::text_surface::TextSurface;

pub(super) fn finish_root_output(
    root: UiRect,
    text: EguiTextSurfaceOutput,
    mut children: RootChildOutputs,
    tab_strip: Option<super::super::tab_strip_retained::TabStripRootOutput>,
) -> EguiTextCommandSurfaceOutput {
    children.ordered_artifacts = super::super::artifact::artifact_order_for_root(
        super::super::artifact::RootArtifactChildren {
            tab_strip: tab_strip.is_some(),
            tab_strip_overlay: tab_strip
                .as_ref()
                .is_some_and(|value| value.overlay_paint_plan.is_some()),
            source_address: children.source_address.is_some(),
            toolbar: children.toolbar.is_some(),
            toolbar_dropdown_open: children
                .toolbar
                .as_ref()
                .is_some_and(|value| value.record.dropdown.is_some()),
            search: children.search.is_some(),
            floating_open: children
                .floating
                .as_ref()
                .is_some_and(|value| value.artifact.is_some()),
            context_menu_open: children
                .context_menu
                .as_ref()
                .is_some_and(|value| value.artifact.is_some()),
            status_bar: children.status_bar.is_some(),
            diagnostics_list: children.diagnostics_list.is_some(),
            preview: children.preview.is_some(),
        },
    );
    EguiTextCommandSurfaceOutput::from_root(root, text, children).with_tab_strip(tab_strip)
}

impl EguiTextCommandSurfaceAdapter {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut EguiTextCommandSurface,
        style: &TextCommandSurfaceStyle,
    ) -> Result<EguiTextCommandSurfaceOutput, EguiTextCommandSurfaceError> {
        self.show_with_tab_strip(ui, surface, style, None, None, None, None)
    }

    pub(super) fn synchronize_floating_for_frame(
        &mut self,
        surface: &mut EguiTextCommandSurface,
        text: &EguiTextSurfaceOutput,
        selection: (usize, usize),
    ) {
        if selection.0 == selection.1 {
            self.floating_selection = None;
            self.closed_selection = None;
            if let Some(floating) = surface.floating.as_mut() {
                let _ = floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
                    rect(text.record.frame.selection.caret),
                    rect(text.record.frame.viewport_bounds),
                    FloatingCommandToolbarVisibility::Closed,
                ));
            }
            return;
        }
        if surface.floating.is_none() {
            let Some(toolbar) = surface.deferred_floating_toolbar.take() else {
                return;
            };
            surface.floating = Some(
                crate::molecule::command_chrome::FloatingCommandToolbar::new_adapter_measured(
                    toolbar,
                    rect(text.record.frame.selection.caret),
                    rect(text.record.frame.viewport_bounds),
                ),
            );
        }
        let visibility = if self.closed_selection == Some(selection) {
            FloatingCommandToolbarVisibility::Closed
        } else if surface.floating_visibility_controlled {
            surface.floating_visibility
        } else {
            FloatingCommandToolbarVisibility::Visible
        };
        let frame = &text.record.frame;
        let anchor = frame
            .selection
            .rects
            .last()
            .copied()
            .unwrap_or(frame.selection.caret);
        if let Some(floating) = surface.floating.as_mut() {
            let _ = floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
                rect(anchor),
                rect(frame.viewport_bounds),
                visibility,
            ));
        }
        self.floating_selection = Some(selection);
    }

    pub(super) fn show_text_in(
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

    pub(super) fn show_in<T>(
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

fn rect(value: UiRect) -> Rect {
    Rect::new(value.x, value.y, value.width, value.height)
}
