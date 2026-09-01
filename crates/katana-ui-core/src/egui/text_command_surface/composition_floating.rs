//! Floating toolbar synchronization for text command surface composition.

use crate::egui::text_command_surface::types::{EguiTextCommandSurface, EguiTextCommandSurfaceAdapter};
use crate::egui::text_surface::EguiTextSurfaceOutput;

use crate::interaction::placement::Rect;
use crate::molecule::command_chrome::{
    FloatingCommandToolbar, FloatingCommandToolbarPresentation, FloatingCommandToolbarVisibility,
};
use crate::render_model::UiRect;

impl EguiTextCommandSurfaceAdapter {
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
            surface.floating = Some(FloatingCommandToolbar::new_adapter_measured(
                toolbar,
                rect(text.record.frame.selection.caret),
                rect(text.record.frame.viewport_bounds),
            ));
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
}

fn rect(value: UiRect) -> Rect {
    Rect::new(value.x, value.y, value.width, value.height)
}
