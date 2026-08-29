//! Root composition for text command surface child adapters.

use crate::text_command_surface::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
    EguiTextCommandSurfaceOutput, RootChildOutputs, TextCommandSurfaceStyle,
};
use crate::text_surface::EguiTextSurfaceError;
use katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::TextSurfaceAction;

const SEARCH_INPUT_FRAME_HEIGHT_PX: u32 = 4;
const SOURCE_ADDRESS_HEIGHT_PX: f32 = 36.0;
const STATUS_BAR_HEIGHT_PX: f32 = 28.0;
const DIAGNOSTICS_LIST_HEIGHT_PX: f32 = 166.0;

impl EguiTextCommandSurfaceAdapter {
    pub(crate) fn show_with_tab_strip(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut EguiTextCommandSurface,
        style: &TextCommandSurfaceStyle,
        tab_strip: Option<&mut super::tab_strip_retained::TabStripRetainedState>,
        status_bar: Option<&mut katana_ui_core::molecule::StatusBar>,
        diagnostics_list: Option<&mut katana_ui_core::molecule::DiagnosticsList>,
    ) -> Result<EguiTextCommandSurfaceOutput, EguiTextCommandSurfaceError> {
        if let (Some(primary), Some(floating)) = (
            surface.primary_command_family(),
            surface.floating_command_family(),
        ) && primary == floating
        {
            return Err(EguiTextCommandSurfaceError::DuplicateCommandFamilyMount {
                family: primary.clone(),
            });
        }
        super::accesskit_evidence::begin_frame(ui.ctx());
        self.metrics
            .borrow_mut()
            .begin(ui.ctx().pixels_per_point())
            .map_err(|error| {
                EguiTextCommandSurfaceError::Text(EguiTextSurfaceError::from(error))
            })?;
        let root = ui.available_rect_before_wrap();
        let source_address_height = if surface.source_address.is_some() {
            SOURCE_ADDRESS_HEIGHT_PX
        } else {
            0.0
        };
        let tab_strip_height = tab_strip
            .as_ref()
            .map(|_| super::tab_strip_retained::TAB_STRIP_HEIGHT_PX)
            .unwrap_or(0.0);
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
        let search_visible = surface.search.is_some() && !surface.search_closed_by_interaction;
        let search_height = if search_visible {
            style
                .search
                .input_height_px
                .saturating_add(SEARCH_INPUT_FRAME_HEIGHT_PX)
                .max(1) as f32
        } else {
            0.0
        };
        let status_height = status_bar
            .as_ref()
            .map(|_| STATUS_BAR_HEIGHT_PX)
            .unwrap_or(0.0);
        let diagnostics_height = diagnostics_list
            .as_ref()
            .map(|_| DIAGNOSTICS_LIST_HEIGHT_PX)
            .unwrap_or(0.0);
        let text_height = (root.height()
            - tab_strip_height
            - source_address_height
            - toolbar_height
            - search_height
            - status_height
            - diagnostics_height)
            .max(1.0);
        let tab_strip_rect =
            egui::Rect::from_min_size(root.min, egui::vec2(root.width(), tab_strip_height));
        let source_address_rect = egui::Rect::from_min_size(
            egui::pos2(root.min.x, tab_strip_rect.max.y),
            egui::vec2(root.width(), source_address_height),
        );
        let toolbar_rect = egui::Rect::from_min_size(
            egui::pos2(root.min.x, source_address_rect.max.y),
            egui::vec2(root.width(), toolbar_height),
        );
        let text_rect = egui::Rect::from_min_size(
            egui::pos2(root.min.x, toolbar_rect.max.y),
            egui::vec2(root.width(), text_height),
        );
        let search_rect = egui::Rect::from_min_size(
            egui::pos2(root.min.x, text_rect.max.y),
            egui::vec2(root.width(), search_height),
        );
        let diagnostics_rect = egui::Rect::from_min_size(
            egui::pos2(root.min.x, search_rect.max.y),
            egui::vec2(root.width(), diagnostics_height),
        );
        let status_rect = egui::Rect::from_min_size(
            egui::pos2(root.min.x, diagnostics_rect.max.y),
            egui::vec2(root.width(), status_height),
        );

        let tab_strip = tab_strip
            .map(|state| {
                let mut child = ui.new_child(egui::UiBuilder::new().max_rect(tab_strip_rect));
                state.show(&mut child)
            })
            .transpose()?;
        let source_address = surface
            .source_address
            .as_mut()
            .map(|strip| {
                let mut child = ui.new_child(egui::UiBuilder::new().max_rect(source_address_rect));
                self.source_address.show(&mut child, strip)
            })
            .transpose()?;
        let source_address_paint_plan = source_address
            .as_ref()
            .map(|_| self.source_address.required_paint_plan())
            .transpose()?;
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
        let search = search_visible
            .then_some(surface.search.as_mut())
            .flatten()
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
        if search.as_ref().is_some_and(|value| {
            value.events.iter().any(|event| {
                matches!(
                    event,
                    katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent::CloseRequested
                )
            })
        }) {
            surface.search_closed_by_interaction = true;
            let _ = surface.text.apply_action(TextSurfaceAction::SetFocus(true));
        }
        let text = self.show_text_in(
            ui,
            text_rect,
            &mut surface.text,
            style,
            self.context_menu
                .as_ref()
                .is_some_and(crate::context_menu::EguiContextMenuAdapter::is_open),
        )?;
        let diagnostics_list = diagnostics_list
            .map(|diagnostics| {
                let mut child = ui.new_child(egui::UiBuilder::new().max_rect(diagnostics_rect));
                self.diagnostics_list.show(&mut child, diagnostics)
            })
            .transpose()?;
        let status_bar = status_bar
            .map(|status| {
                let mut child = ui.new_child(egui::UiBuilder::new().max_rect(status_rect));
                self.status_bar.show(&mut child, status)
            })
            .transpose()?;
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
        }
        ui.allocate_rect(root, egui::Sense::hover());
        let artifact_order =
            super::artifact::artifact_order_for_root(super::artifact::RootArtifactChildren {
                tab_strip: tab_strip.is_some(),
                tab_strip_overlay: tab_strip
                    .as_ref()
                    .is_some_and(|value| value.overlay_paint_plan.is_some()),
                source_address: source_address.is_some(),
                toolbar: toolbar.is_some(),
                toolbar_dropdown_open: toolbar
                    .as_ref()
                    .is_some_and(|value| value.record.dropdown.is_some()),
                search: search.is_some(),
                floating_open: floating
                    .as_ref()
                    .is_some_and(|value| value.artifact.is_some()),
                context_menu_open: context_menu
                    .as_ref()
                    .is_some_and(|value| value.artifact.is_some()),
                status_bar: status_bar.is_some(),
                diagnostics_list: diagnostics_list.is_some(),
            });

        Ok(EguiTextCommandSurfaceOutput::from_root(
            ui_rect(root),
            text,
            RootChildOutputs {
                toolbar,
                floating,
                search,
                context_menu,
                source_address: match (source_address, source_address_paint_plan) {
                    (Some(output), Some(paint_plan)) => Some(super::types::SourceAddressRootOutput {
                        output,
                        paint_plan,
                    }),
                    (None, None) => None,
                    _ => return Err(crate::source_address_strip::EguiSourceAddressStripError::PaintPlanNotProduced.into()),
                },
                accesskit_evidence: super::accesskit_evidence::finish_frame(ui.ctx()),
                ordered_artifacts: artifact_order,
                status_bar,
                diagnostics_list,
            },
        )
        .with_tab_strip(tab_strip))
    }
}

mod helpers;

#[cfg(test)]
fn inject_same_bounds_test_overlay(
    ui: &mut egui::Ui,
    toolbar: Option<&mut crate::command_chrome::EguiCommandChromeOutput>,
    chrome: &mut crate::command_chrome::EguiCommandChromeAdapter,
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
