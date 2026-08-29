use super::support::ui_rect;
use super::{
    CORNER_RADIUS_PX, DRAG_GHOST_OFFSET_PX, DRAG_GHOST_RGBA, DROP_LEFT_RATIO, DROP_RIGHT_RATIO,
    PREEDIT_RGBA, RGBA_ALPHA_INDEX, TAB_GAP_PX, TAB_PADDING_PX, TabStripDragState,
    TabStripDropCandidateKind, TabStripPaintOperation, TabStripPaintOperationKind,
    TabStripPaintTexture, TabStripProposal, TabStripProposalOperation, TabStripResolvedDrop,
    TabStripRetainedError, TabStripRetainedState, TabStripTabDescriptor, TabStripTabPlacement,
    TabStripTabTarget, TabStripText, publish_labeled_button_accesskit,
};

impl TabStripRetainedState {
    pub(super) fn publish_response_accesskit(&self, ui: &egui::Ui, response_id: egui::Id) {
        let Some((bounds, label, disabled)) = self.routes.route_for_response(response_id) else {
            return;
        };
        publish_labeled_button_accesskit(
            ui,
            response_id,
            label,
            disabled,
            *bounds,
            "tab-strip-control",
            crate::text_command_surface::accesskit_evidence::AccessKitTargetClass::TabStripControl,
        );
    }

    pub(super) fn forward_response_activation(
        &mut self,
        ui: &egui::Ui,
        response: &egui::Response,
    ) -> Result<bool, TabStripRetainedError> {
        let primary_pressed = ui.input(|input| {
            input.events.iter().any(|event| {
                matches!(event, egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed: true, .. } if response.rect.contains(*pos))
            })
        });
        if primary_pressed {
            self.overlay_primary_press = Some(response.id);
        }
        let primary_released = ui.input(|input| {
            input.events.iter().any(|event| {
                matches!(event, egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed: false, .. } if response.rect.contains(*pos))
            })
        });
        let captured_pointer = primary_released
            && self.overlay_primary_press == Some(response.id)
            && response.rect.contains(
                ui.input(|input| input.pointer.latest_pos())
                    .unwrap_or(response.rect.center()),
            );
        if primary_released && self.overlay_primary_press == Some(response.id) {
            self.overlay_primary_press = None;
        }
        let pointer = response.clicked() || captured_pointer;
        if pointer {
            response.request_focus();
        }
        let accesskit = ui.input(|input| {
            input.has_accesskit_action_request(response.id, egui::accesskit::Action::Click)
        });
        let keyboard = response.has_focus()
            && ui.input(|input| {
                input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
            });
        if (pointer || accesskit || keyboard) && !self.routes.response_is_disabled(response.id) {
            self.forward_response_route(response.id)?;
            return Ok(true);
        }
        Ok(false)
    }

    pub(super) fn forward_response_route(
        &mut self,
        response_id: egui::Id,
    ) -> Result<(), TabStripRetainedError> {
        let (correlation, operation) = self
            .routes
            .proposal_for_response(response_id)
            .ok_or(TabStripRetainedError::MissingRoute)?;
        self.forward_proposal(correlation, operation)
    }

    pub(super) fn start_tab_drag(
        &mut self,
        tab: &TabStripTabDescriptor,
        bounds: egui::Rect,
        ui: &egui::Ui,
    ) -> Result<(), TabStripRetainedError> {
        if self.drag.is_some() {
            return Ok(());
        }
        self.forward_proposal(
            self.routes.correlation_for_proposal(),
            TabStripProposalOperation::StartDrag(tab.target.copy_for_route()),
        )?;
        self.drag = Some(TabStripDragState {
            source: tab.target.copy_for_route(),
            label: TabStripText::new(tab.label.value.as_str()),
            pointer: ui
                .input(|input| input.pointer.latest_pos())
                .unwrap_or(bounds.center()),
        });
        Ok(())
    }

    pub(super) fn resolve_tab_drag(
        &mut self,
        ui: &egui::Ui,
        strip_bounds: egui::Rect,
        end_drop_available: bool,
        operations: &mut Vec<TabStripPaintOperation>,
    ) -> Result<(), TabStripRetainedError> {
        let Some(mut drag) = self.drag.take() else {
            self.drag_release_pending = false;
            return Ok(());
        };
        let pointer = ui
            .input(|input| input.pointer.latest_pos())
            .unwrap_or(drag.pointer);
        drag.pointer = pointer;
        let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
        let release = std::mem::take(&mut self.drag_release_pending);
        let drop = self.resolve_tab_drop(pointer, &drag.source, strip_bounds, end_drop_available);
        if escape || release {
            let operation = if !escape {
                drop.map_or(TabStripProposalOperation::CancelDrag, |value| {
                    TabStripProposalOperation::FinishDrag {
                        committed: true,
                        destination: Some(value.destination),
                    }
                })
            } else {
                TabStripProposalOperation::CancelDrag
            };
            self.forward_proposal(self.routes.correlation_for_proposal(), operation)?;
            return Ok(());
        }
        self.paint_tab_drag_feedback(ui, &drag, drop.as_ref(), strip_bounds, operations)?;
        self.drag = Some(drag);
        Ok(())
    }

    pub(super) fn resolve_tab_drop(
        &self,
        pointer: egui::Pos2,
        source: &TabStripTabTarget,
        strip_bounds: egui::Rect,
        end_drop_available: bool,
    ) -> Option<TabStripResolvedDrop> {
        for candidate in &self.drag_candidates {
            if !candidate.bounds.contains(pointer) {
                continue;
            }
            match &candidate.kind {
                TabStripDropCandidateKind::Tab(target) if !source.same_target(target) => {
                    let ratio = (pointer.x - candidate.bounds.min.x) / candidate.bounds.width();
                    if ratio <= DROP_LEFT_RATIO {
                        return Some(TabStripResolvedDrop {
                            destination: TabStripTabPlacement::Before(target.copy_for_route()),
                            indicator: egui::Rect::from_min_size(
                                egui::pos2(candidate.bounds.min.x - 1.0, candidate.bounds.min.y),
                                egui::vec2(2.0, candidate.bounds.height()),
                            ),
                        });
                    }
                    if ratio >= DROP_RIGHT_RATIO {
                        return Some(TabStripResolvedDrop {
                            destination: TabStripTabPlacement::After(target.copy_for_route()),
                            indicator: egui::Rect::from_min_size(
                                egui::pos2(candidate.bounds.max.x - 1.0, candidate.bounds.min.y),
                                egui::vec2(2.0, candidate.bounds.height()),
                            ),
                        });
                    }
                }
                TabStripDropCandidateKind::Group(target) => {
                    return Some(TabStripResolvedDrop {
                        destination: TabStripTabPlacement::InGroup(target.copy_for_route()),
                        indicator: candidate.bounds.shrink(2.0),
                    });
                }
                TabStripDropCandidateKind::Tab(_) => {}
            }
        }
        (end_drop_available
            && strip_bounds.contains(pointer)
            && self
                .drag_candidates
                .iter()
                .map(|candidate| candidate.bounds.max.x)
                .fold(strip_bounds.min.x, f32::max)
                <= pointer.x)
            .then(|| TabStripResolvedDrop {
                destination: TabStripTabPlacement::EndOfStrip,
                indicator: egui::Rect::from_min_size(
                    egui::pos2(pointer.x - 1.0, strip_bounds.min.y + TAB_GAP_PX),
                    egui::vec2(2.0, (strip_bounds.height() - TAB_GAP_PX * 2.0).max(1.0)),
                ),
            })
    }

    pub(super) fn paint_tab_drag_feedback(
        &mut self,
        ui: &egui::Ui,
        drag: &TabStripDragState,
        drop: Option<&TabStripResolvedDrop>,
        strip_bounds: egui::Rect,
        operations: &mut Vec<TabStripPaintOperation>,
    ) -> Result<(), TabStripRetainedError> {
        let raster = self
            .rasterizer
            .rasterize(&drag.label, ui.ctx().pixels_per_point())
            .map_err(TabStripRetainedError::Raster)?;
        let texture = TabStripPaintTexture {
            identity: "tab-strip-drag-ghost".to_owned(),
            width: raster.width,
            height: raster.height,
            rgba_pixels: raster.rgba_pixels,
        };
        let ghost = egui::Rect::from_min_size(
            drag.pointer + egui::vec2(DRAG_GHOST_OFFSET_PX, DRAG_GHOST_OFFSET_PX),
            egui::vec2(
                texture.width as f32 + TAB_PADDING_PX * 2.0,
                (strip_bounds.height() - TAB_GAP_PX * 2.0).max(1.0),
            ),
        );
        ui.painter().rect_filled(
            ghost,
            CORNER_RADIUS_PX,
            egui::Color32::from_rgba_unmultiplied(
                DRAG_GHOST_RGBA[0],
                DRAG_GHOST_RGBA[1],
                DRAG_GHOST_RGBA[2],
                DRAG_GHOST_RGBA[RGBA_ALPHA_INDEX],
            ),
        );
        operations.push(TabStripPaintOperation {
            clip_bounds: ui_rect(strip_bounds),
            kind: TabStripPaintOperationKind::Fill {
                bounds: ui_rect(ghost),
                color_rgba: DRAG_GHOST_RGBA,
            },
        });
        let text_bounds = egui::Rect::from_min_size(
            egui::pos2(
                ghost.min.x + TAB_PADDING_PX,
                ghost.center().y - texture.height as f32 / 2.0,
            ),
            egui::vec2(texture.width as f32, texture.height as f32),
        );
        self.paint_overlay_texture(ui, operations, strip_bounds, &texture, text_bounds);
        if let Some(drop) = drop {
            ui.painter().rect_filled(
                drop.indicator,
                1.0,
                egui::Color32::from_rgb(PREEDIT_RGBA[0], PREEDIT_RGBA[1], PREEDIT_RGBA[2]),
            );
            operations.push(TabStripPaintOperation {
                clip_bounds: ui_rect(strip_bounds),
                kind: TabStripPaintOperationKind::Fill {
                    bounds: ui_rect(drop.indicator),
                    color_rgba: PREEDIT_RGBA,
                },
            });
        }
        Ok(())
    }

    pub(super) fn forward_proposal(
        &mut self,
        correlation: super::tab_strip_projection_lease::TabStripCorrelation,
        operation: super::tab_strip_proposal_port::TabStripProposalOperation,
    ) -> Result<(), TabStripRetainedError> {
        self.next_nonce = self.next_nonce.saturating_add(1);
        let proposal = TabStripProposal::new(self.next_nonce, correlation, operation);
        self.port
            .as_mut()
            .ok_or(TabStripRetainedError::MissingPort)?
            .forward_once(proposal)
            .map_err(TabStripRetainedError::Port)
    }

    pub(super) fn forward_rename_route(
        &mut self,
        path: &str,
        name: TabStripText,
    ) -> Result<(), TabStripRetainedError> {
        let (correlation, operation) = self
            .routes
            .rename_proposal_for(path, name)
            .ok_or(TabStripRetainedError::MissingRoute)?;
        self.forward_proposal(correlation, operation)
    }
}

#[cfg(test)]
#[path = "interaction_tests.rs"]
mod tests;
