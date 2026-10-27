use super::support::{
    TabStripGroupPopupPrefix, append_text_surface_operations, rename_paint_style,
    rename_raster_style, ui_rect,
};
use super::{
    CORNER_RADIUS_PX, EguiTextSurfaceInputPolicy, OVERLAY_BACKGROUND_RGBA, OVERLAY_PADDING_PX,
    OVERLAY_ROW_HEIGHT_PX, OVERLAY_SWATCH_INSET_PX, OVERLAY_SWATCH_SIZE_PX, OVERLAY_WIDTH_PX,
    TabStripGroupDescriptor, TabStripPaintOperation, TabStripPaintOperationKind,
    TabStripRenameDraft, TabStripRetainedError, TabStripRetainedState, TabStripText, TextAreaEvent,
    TextSurfaceEvent,
};

impl TabStripRetainedState {
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
        let area_id = self
            .routes
            .interaction_id(ui.id(), path)
            .with("tab-strip-group-popup-prefix");
        let parent_enabled = ui.is_enabled();
        let shown = egui::Area::new(area_id)
            .order(egui::Order::Foreground)
            .constrain(false)
            .fixed_pos(anchor)
            .enabled(parent_enabled)
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
                let mut rename_submissions = Vec::new();
                let on_change = group.popup.as_ref().is_some_and(|popup| {
                    popup.rename_policy
                        == super::tab_strip_projection_lease::TabStripRenamePolicy::OnChange
                });
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
                    if draft.requested_start
                        && self.pending_rename_start.is_none()
                        && popup_ui.is_enabled()
                    {
                        if !self
                            .inline_rename_consumed
                            .iter()
                            .any(|target| target == &group.target)
                        {
                            self.inline_rename_consumed.push(group.target.clone());
                        }
                        if let Some(port) = self.port.clone() {
                            let commit = super::pass_commit::TabStripPassCommit::committed();
                            commit.watch(popup_ui.ctx());
                            popup_ui.ctx().request_repaint();
                            self.pending_rename_start = Some(super::TabStripPendingRenameStart {
                                target: group.target.clone(),
                                correlation: self.routes.correlation_for_proposal(),
                                port,
                                commit,
                            });
                        }
                        draft.requested_start = false;
                    }
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
                    if on_change {
                        collect_rename_changes(draft, &output.events, &mut rename_submissions);
                        if cancel || commit {
                            closed = true;
                        }
                    } else if cancel {
                        closed = true;
                    } else if commit && !draft.value().is_empty() && draft.changed() {
                        rename_submissions.push(TabStripText::new(draft.value()));
                        closed = true;
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
                        let route_path = format!("{path}-popup-swatch-{index}");
                        let response = popup_ui.interact(
                            rect,
                            self.routes.interaction_id(area_id, &route_path),
                            egui::Sense::click(),
                        );
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
                if (on_change || !swatch_activated) && !rename_submissions.is_empty() {
                    if on_change {
                        let port = self
                            .port
                            .clone()
                            .ok_or(TabStripRetainedError::MissingPort)?;
                        let commit = super::pass_commit::TabStripPassCommit::committed();
                        commit.watch(popup_ui.ctx());
                        popup_ui.ctx().request_repaint();
                        self.pending_rename_changes = Some(super::TabStripPendingRenameChanges {
                            target: group.target.clone(),
                            correlation: self.routes.correlation_for_proposal(),
                            port,
                            commit,
                            names: rename_submissions,
                            before: None,
                        });
                    } else {
                        for name in rename_submissions {
                            self.forward_rename_route(&format!("{path}-popup-rename"), name)?;
                        }
                    }
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

fn collect_rename_changes(
    draft: &mut TabStripRenameDraft,
    events: &[TextSurfaceEvent],
    names: &mut Vec<TabStripText>,
) {
    for event in events {
        if let TextSurfaceEvent::TextArea(TextAreaEvent::Change(value)) = event
            && value != &draft.initial
        {
            names.push(TabStripText::new(value));
            draft.initial.clone_from(value);
        }
    }
}
