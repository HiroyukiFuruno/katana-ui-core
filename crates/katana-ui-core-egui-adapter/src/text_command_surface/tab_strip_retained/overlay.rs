use super::support::{
    TabStripGroupPopupPrefix, append_text_surface_operations, group_for_path, rect_from_ui_rect,
    rename_paint_style, rename_raster_style, tab_menu_for_path, ui_rect, union_bounds,
};
use super::{
    CORNER_RADIUS_PX, EguiTextSurfaceInputPolicy, OVERLAY_BACKGROUND_RGBA, OVERLAY_PADDING_PX,
    OVERLAY_ROW_HEIGHT_PX, OVERLAY_SWATCH_INSET_PX, OVERLAY_SWATCH_SIZE_PX, OVERLAY_WIDTH_PX,
    TabStripGroupDescriptor, TabStripOverlayState, TabStripPaintOperation,
    TabStripPaintOperationKind, TabStripPaintPlan, TabStripProjection, TabStripRenameDraft,
    TabStripRetainedError, TabStripRetainedState, TabStripText, TextAreaEvent, TextSurfaceEvent,
};

impl TabStripRetainedState {
    pub(super) fn render_overlay(
        &mut self,
        ui: &mut egui::Ui,
        projection: &TabStripProjection,
    ) -> Result<Option<TabStripPaintPlan>, TabStripRetainedError> {
        let state = std::mem::replace(&mut self.overlay, TabStripOverlayState::Closed);
        let mut prefix = None;
        let (path, anchor, submenu_path, entries, route_prefix, rename) = match state {
            TabStripOverlayState::Closed => return Ok(None),
            TabStripOverlayState::TabMenu {
                path,
                anchor,
                submenu_path,
            } => {
                let Some(menu) = tab_menu_for_path(projection, &path) else {
                    return Ok(None);
                };
                (
                    path.clone(),
                    anchor,
                    submenu_path,
                    &menu.entries,
                    format!("{path}-menu"),
                    None,
                )
            }
            TabStripOverlayState::GroupPopup {
                path,
                anchor,
                submenu_path,
                rename,
            } => {
                let Some(group) = group_for_path(projection, &path) else {
                    return Ok(None);
                };
                let Some(popup) = group.popup.as_ref() else {
                    return Ok(None);
                };
                let mut rendered =
                    self.render_group_popup_prefix(ui, &path, anchor, group, rename)?;
                if rendered.closed {
                    return Ok(None);
                }
                let content_anchor = rendered.content_anchor;
                let rename = rendered.rename.take();
                prefix = Some(rendered);
                (
                    path.clone(),
                    content_anchor,
                    submenu_path,
                    &popup.entries,
                    format!("{path}-popup"),
                    rename,
                )
            }
        };
        let protected_bounds = prefix
            .as_ref()
            .map(|value| vec![value.bounds])
            .unwrap_or_default();
        let mut outcome = self.render_overlay_tree(
            ui,
            entries,
            &route_prefix,
            anchor,
            submenu_path,
            &protected_bounds,
        )?;
        let retained_anchor = prefix.as_ref().map_or(anchor, |value| value.bounds.min);
        if let Some(prefix) = prefix {
            let mut operations = prefix.operations;
            operations.append(&mut outcome.paint_plan.operations);
            outcome.paint_plan.operations = operations;
            outcome.paint_plan.surface_bounds = ui_rect(
                union_bounds(&[
                    prefix.bounds,
                    rect_from_ui_rect(outcome.paint_plan.surface_bounds),
                ])
                .ok_or(TabStripRetainedError::MissingOverlayBounds)?,
            );
        }
        self.overlay = if outcome.closed {
            TabStripOverlayState::Closed
        } else if rename.is_some() {
            TabStripOverlayState::GroupPopup {
                path,
                anchor: retained_anchor,
                submenu_path: outcome.submenu_path,
                rename,
            }
        } else if group_for_path(projection, &path).is_some() {
            TabStripOverlayState::GroupPopup {
                path,
                anchor: retained_anchor,
                submenu_path: outcome.submenu_path,
                rename: None,
            }
        } else {
            TabStripOverlayState::TabMenu {
                path,
                anchor,
                submenu_path: outcome.submenu_path,
            }
        };
        Ok(Some(outcome.paint_plan))
    }

    pub(super) fn render_group_popup_prefix(
        &mut self,
        ui: &mut egui::Ui,
        path: &str,
        anchor: egui::Pos2,
        group: &TabStripGroupDescriptor,
        mut rename: Option<Box<TabStripRenameDraft>>,
    ) -> Result<TabStripGroupPopupPrefix, TabStripRetainedError> {
        let rename_height = rename
            .as_ref()
            .map_or(0.0, |_| OVERLAY_ROW_HEIGHT_PX + OVERLAY_PADDING_PX);
        let swatch_height = if group.swatches.is_empty() {
            0.0
        } else {
            OVERLAY_SWATCH_SIZE_PX + OVERLAY_PADDING_PX * 2.0
        };
        let height = rename_height + swatch_height;
        if height == 0.0 {
            return Ok(TabStripGroupPopupPrefix {
                content_anchor: anchor,
                bounds: egui::Rect::from_min_size(anchor, egui::Vec2::ZERO),
                operations: Vec::new(),
                closed: false,
                rename,
            });
        }
        let area_id = ui.id().with(("tab-strip-group-popup-prefix", path));
        let shown = egui::Area::new(area_id)
            .order(egui::Order::Foreground)
            .constrain(false)
            .fixed_pos(anchor)
            .show(ui.ctx(), |popup_ui| {
                let (bounds, _) = popup_ui.allocate_exact_size(
                    egui::vec2(OVERLAY_WIDTH_PX, height),
                    egui::Sense::empty(),
                );
                let mut operations = vec![TabStripPaintOperation {
                    clip_bounds: ui_rect(bounds),
                    kind: TabStripPaintOperationKind::Fill {
                        bounds: ui_rect(bounds),
                        color_rgba: OVERLAY_BACKGROUND_RGBA,
                    },
                }];
                popup_ui.painter().rect_filled(
                    bounds,
                    OVERLAY_SWATCH_INSET_PX,
                    egui::Color32::from_rgb(
                        OVERLAY_BACKGROUND_RGBA[0],
                        OVERLAY_BACKGROUND_RGBA[1],
                        OVERLAY_BACKGROUND_RGBA[2],
                    ),
                );
                let mut y = bounds.min.y + OVERLAY_PADDING_PX;
                let mut closed = false;
                let mut rename_submission = None;
                if let Some(draft) = rename.as_mut() {
                    let input = egui::Rect::from_min_size(
                        egui::pos2(bounds.min.x + OVERLAY_PADDING_PX, y),
                        egui::vec2(
                            bounds.width() - OVERLAY_PADDING_PX * 2.0,
                            OVERLAY_ROW_HEIGHT_PX,
                        ),
                    );
                    let output = popup_ui
                        .scope_builder(egui::UiBuilder::new().max_rect(input), |input_ui| {
                            self.rename_adapter.show_with_input_policy(
                                input_ui,
                                &mut draft.surface,
                                &rename_raster_style(),
                                &rename_paint_style(),
                                &EguiTextSurfaceInputPolicy::default()
                                    .without_context_target()
                                    .with_text_input_target()
                                    .with_retained_pointer_focus(),
                            )
                        })
                        .inner
                        .map_err(TabStripRetainedError::TextSurface)?;
                    append_text_surface_operations(&mut operations, &output.artifact.paint_plan);
                    let commit = output.events.iter().any(|event| {
                        matches!(event, TextSurfaceEvent::TextArea(TextAreaEvent::Submit(_)))
                    });
                    let cancel = popup_ui.input(|input| {
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
                    if cancel {
                        closed = true;
                    } else if commit && !draft.value().is_empty() && draft.changed() {
                        rename_submission = Some(TabStripText::new(draft.value()));
                    }
                    y += OVERLAY_ROW_HEIGHT_PX + OVERLAY_PADDING_PX;
                }
                let mut swatch_activated = false;
                if !group.swatches.is_empty() {
                    let mut x = bounds.min.x + OVERLAY_PADDING_PX;
                    for (index, swatch) in group.swatches.iter().enumerate() {
                        let rect = egui::Rect::from_min_size(
                            egui::pos2(x, y),
                            egui::vec2(OVERLAY_SWATCH_SIZE_PX, OVERLAY_SWATCH_SIZE_PX),
                        );
                        let response = popup_ui.interact(
                            rect,
                            area_id.with(("swatch", index)),
                            egui::Sense::click(),
                        );
                        let route_path = format!("{path}-popup-swatch-{index}");
                        self.routes.register_response(
                            &route_path,
                            response.id,
                            ui_rect(rect),
                            swatch
                                .accessibility_label
                                .as_ref()
                                .map_or("", |value| value.value.as_str()),
                            false,
                        );
                        self.publish_response_accesskit(popup_ui, response.id);
                        swatch_activated |=
                            self.forward_response_activation(popup_ui, &response)?;
                        let color = swatch.display_color;
                        popup_ui.painter().rect_filled(
                            rect,
                            CORNER_RADIUS_PX,
                            egui::Color32::from_rgba_unmultiplied(
                                color.red,
                                color.green,
                                color.blue,
                                color.alpha,
                            ),
                        );
                        operations.push(TabStripPaintOperation {
                            clip_bounds: ui_rect(bounds),
                            kind: TabStripPaintOperationKind::Fill {
                                bounds: ui_rect(rect),
                                color_rgba: [color.red, color.green, color.blue, color.alpha],
                            },
                        });
                        x += OVERLAY_SWATCH_SIZE_PX + OVERLAY_PADDING_PX;
                    }
                }
                if !closed
                    && !swatch_activated
                    && let Some(name) = rename_submission
                {
                    self.forward_rename_route(&format!("{path}-popup-rename"), name)?;
                    closed = true;
                }
                Ok(TabStripGroupPopupPrefix {
                    content_anchor: egui::pos2(anchor.x, anchor.y + height),
                    bounds,
                    operations,
                    closed,
                    rename,
                })
            });
        shown.inner
    }
}
