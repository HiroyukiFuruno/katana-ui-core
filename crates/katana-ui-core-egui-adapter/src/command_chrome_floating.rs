use super::EguiCommandChromeAdapter;
use super::command_chrome_artifact::EguiCommandChromeFloatingArtifactFrame;
use super::command_chrome_floating_paint::{TooltipPaintSource, build_floating_paint_plan};
use super::command_chrome_paint::paint_command_chrome;
use super::command_chrome_types::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeError,
    EguiCommandChromeFloatingFrameRecord, EguiCommandChromeFloatingOutput,
};
use katana_ui_core::molecule::command_chrome::{
    FloatingCommandToolbar, FloatingCommandToolbarAction, FloatingCommandToolbarCloseReason,
    FloatingCommandToolbarEvent,
};
use katana_ui_core::render_model::UiRect;

const TOOLTIP_PADDING_PX: u32 = 6;
const FLOATING_SURFACE_ID: &str = "kuc-floating-command-chrome";
pub(super) const FLOATING_PANEL_PADDING_PX: u32 = 10;
pub(super) const FLOATING_PANEL_BORDER_PX: u32 = 1;
pub(super) const FLOATING_PANEL_RADIUS_PX: u32 = 8;

impl EguiCommandChromeAdapter {
    pub fn show_floating_toolbar(
        &mut self,
        ui: &mut egui::Ui,
        floating: &mut FloatingCommandToolbar,
        raster_style: &CommandChromeRasterStyle,
        paint_style: &CommandChromePaintStyle,
    ) -> Result<EguiCommandChromeFloatingOutput, EguiCommandChromeError> {
        let mut events = escape_dismiss_events(ui, floating);
        if !floating.is_open() {
            return Ok(EguiCommandChromeFloatingOutput {
                record: None,
                events,
                artifact: None,
            });
        }
        let measured_panel = self.measure_toolbar(ui, floating.toolbar_model(), raster_style)?;
        let measured_panel = katana_ui_core::interaction::placement::Size::new(
            measured_panel.width.saturating_add(
                FLOATING_PANEL_PADDING_PX
                    .saturating_add(FLOATING_PANEL_BORDER_PX)
                    .saturating_mul(2),
            ),
            measured_panel.height.saturating_add(
                FLOATING_PANEL_PADDING_PX
                    .saturating_add(FLOATING_PANEL_BORDER_PX)
                    .saturating_mul(2),
            ),
        );
        let _ = floating.synchronize_measured_panel(measured_panel);
        let Some(bounds) = floating.bounds_model() else {
            return Ok(EguiCommandChromeFloatingOutput {
                record: None,
                events,
                artifact: None,
            });
        };
        let panel_bounds = UiRect::new(bounds.x, bounds.y, bounds.width, bounds.height);
        let area_id = ui.id().with(FLOATING_SURFACE_ID);
        let panel_size = egui::vec2(bounds.width as f32, bounds.height as f32);
        let panel_rect = egui_rect(panel_bounds);
        let toolbar_rect = inset_panel(panel_bounds);
        let mut panel_ui = ui.new_child(egui::UiBuilder::new().id(area_id).max_rect(panel_rect));
        panel_ui.set_min_size(panel_size);
        panel_ui = panel_ui.new_child(egui::UiBuilder::new().max_rect(egui_rect(toolbar_rect)));
        let toolbar_output = self.show_toolbar_unpainted(
            &mut panel_ui,
            floating.toolbar_model_mut(),
            raster_style,
            paint_style,
            crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::FloatingToolbar,
        )?;
        if let Some(close_events) = outside_dismiss_events(ui, floating, &toolbar_output.record) {
            return Ok(EguiCommandChromeFloatingOutput {
                record: None,
                events: close_events,
                artifact: None,
            });
        }
        events.extend(floating_toolbar_events(toolbar_output.events));
        let tooltip = hovered_tooltip(ui, floating, &toolbar_output.record.actions)
            .map(|(text, action_bounds)| {
                self.floating_tooltip(ui, &text, action_bounds, raster_style)
            })
            .transpose()?;
        let record = EguiCommandChromeFloatingFrameRecord {
            surface_id: FLOATING_SURFACE_ID.to_string(),
            anchor_bounds: core_rect(floating.layout_model().anchor),
            panel_bounds,
            toolbar: toolbar_output.record,
            tooltip_bounds: tooltip.as_ref().map(TooltipPaintSource::bounds),
            tooltip_raster_identity: tooltip
                .as_ref()
                .map(|value| value.raster_identity().to_string()),
        };
        let paint_plan = build_floating_paint_plan(
            panel_bounds,
            &toolbar_output.artifact.paint_plan,
            tooltip.as_ref(),
            paint_style,
        );
        let artifact = EguiCommandChromeFloatingArtifactFrame::new(
            record.clone(),
            paint_plan,
            events.clone(),
        )?;
        paint_command_chrome(ui, &mut self.textures, &artifact.paint_plan);
        Ok(EguiCommandChromeFloatingOutput {
            record: Some(record),
            events,
            artifact: Some(artifact),
        })
    }

    fn floating_tooltip(
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
        action_bounds: UiRect,
        raster_style: &CommandChromeRasterStyle,
    ) -> Result<TooltipPaintSource, EguiCommandChromeError> {
        let raster = self.raster_label(text, raster_style, ui.ctx().pixels_per_point())?;
        let bounds = UiRect::new(
            action_bounds.x,
            action_bounds
                .y
                .saturating_add_unsigned(action_bounds.height)
                .saturating_add(TOOLTIP_PADDING_PX as i32),
            raster
                .width
                .saturating_add(TOOLTIP_PADDING_PX.saturating_mul(2)),
            raster
                .height
                .saturating_add(TOOLTIP_PADDING_PX.saturating_mul(2)),
        );
        let text_bounds = UiRect::new(
            bounds.x.saturating_add_unsigned(TOOLTIP_PADDING_PX),
            bounds.y.saturating_add_unsigned(TOOLTIP_PADDING_PX),
            raster.width,
            raster.height,
        );
        Ok(TooltipPaintSource::new(bounds, text_bounds, raster))
    }
}

fn escape_dismiss_events(
    ui: &egui::Ui,
    floating: &mut FloatingCommandToolbar,
) -> Vec<FloatingCommandToolbarEvent> {
    let escape = ui.input(|input| {
        input.events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    ..
                }
            )
        })
    });
    if escape {
        return floating.apply_action(FloatingCommandToolbarAction::Dismiss {
            reason: FloatingCommandToolbarCloseReason::Escape,
        });
    }
    Vec::new()
}

fn outside_dismiss_events(
    ui: &egui::Ui,
    floating: &mut FloatingCommandToolbar,
    record: &super::EguiCommandChromeFrameRecord,
) -> Option<Vec<FloatingCommandToolbarEvent>> {
    let panel_bounds = floating.bounds_model().map(core_rect)?;
    let outside_click = ui.input(|input| {
        input.events.iter().any(|event| {
            let egui::Event::PointerButton {
                pos, pressed: true, ..
            } = event
            else {
                return false;
            };
            !floating_interaction_contains(panel_bounds, record, *pos)
        })
    });
    if outside_click {
        Some(
            floating.apply_action(FloatingCommandToolbarAction::Dismiss {
                reason: FloatingCommandToolbarCloseReason::OutsideClick,
            }),
        )
    } else {
        None
    }
}

fn floating_interaction_contains(
    panel_bounds: UiRect,
    record: &super::EguiCommandChromeFrameRecord,
    point: egui::Pos2,
) -> bool {
    contains_ui_rect(panel_bounds, point)
        || contains_ui_rect(record.bounds, point)
        || record.dropdown.as_ref().is_some_and(|dropdown| {
            contains_ui_rect(dropdown.trigger_bounds, point)
                || contains_ui_rect(dropdown.bounds, point)
                || dropdown
                    .items
                    .iter()
                    .any(|item| !item.disabled && contains_ui_rect(item.bounds, point))
        })
}

fn floating_toolbar_events(
    toolbar_events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent>,
) -> Vec<FloatingCommandToolbarEvent> {
    if toolbar_events.is_empty() {
        return Vec::new();
    }
    std::iter::once(FloatingCommandToolbarEvent::FocusRetained)
        .chain(
            toolbar_events
                .into_iter()
                .map(|event| FloatingCommandToolbarEvent::Toolbar { event }),
        )
        .collect()
}

fn hovered_tooltip(
    ui: &egui::Ui,
    floating: &FloatingCommandToolbar,
    actions: &[super::EguiCommandChromeActionFrame],
) -> Option<(String, UiRect)> {
    let pointer = ui.ctx().input(|input| input.pointer.hover_pos())?;
    let action = actions
        .iter()
        .find(|action| contains_ui_rect(action.bounds, pointer))?;
    floating
        .toolbar_model()
        .actions()
        .iter()
        .find(|candidate| candidate.id().as_str() == action.action_id)
        .and_then(|candidate| candidate.tooltip_model())
        .filter(|value| !value.is_empty())
        .map(|value| (value.clone(), action.bounds))
}

fn contains_ui_rect(bounds: UiRect, point: egui::Pos2) -> bool {
    let x = point.x.round() as i32;
    let y = point.y.round() as i32;
    x >= bounds.x
        && x < bounds.x.saturating_add_unsigned(bounds.width)
        && y >= bounds.y
        && y < bounds.y.saturating_add_unsigned(bounds.height)
}

fn egui_rect(bounds: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(bounds.x as f32, bounds.y as f32),
        egui::vec2(bounds.width as f32, bounds.height as f32),
    )
}

fn core_rect(bounds: katana_ui_core::interaction::placement::Rect) -> UiRect {
    UiRect::new(bounds.x, bounds.y, bounds.width, bounds.height)
}

fn inset_panel(bounds: UiRect) -> UiRect {
    let inset = FLOATING_PANEL_PADDING_PX.saturating_add(FLOATING_PANEL_BORDER_PX);
    UiRect::new(
        bounds.x.saturating_add(inset as i32),
        bounds.y.saturating_add(inset as i32),
        bounds.width.saturating_sub(inset.saturating_mul(2)),
        bounds.height.saturating_sub(inset.saturating_mul(2)),
    )
}
