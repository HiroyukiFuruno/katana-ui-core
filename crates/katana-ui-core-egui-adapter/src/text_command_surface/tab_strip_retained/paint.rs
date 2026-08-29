use super::support::ui_rect;
use super::{
    CORNER_RADIUS_PX, DISABLED_TAB_RGBA, ICON_DISABLED_RGB, ICON_ENABLED_RGB, INACTIVE_TAB_RGBA,
    NAVIGATION_CONTROL_WIDTH_PX, PRIMARY_TEXT_RGBA, RGBA_ALPHA_INDEX, RgbaColor, TAB_GAP_PX,
    TabStripDropCandidate, TabStripDropCandidateKind, TabStripGroupDescriptor, TabStripIconControl,
    TabStripLabelInteraction, TabStripLabelRenderRequest, TabStripOverlayState,
    TabStripPaintOperation, TabStripPaintOperationKind, TabStripRenameDraft, TabStripRetainedError,
    TabStripRetainedState, TabStripTabDescriptor, TabStripTrailingControl,
};

impl TabStripRetainedState {
    pub(super) fn render_group(
        &mut self,
        ui: &mut egui::Ui,
        group: &TabStripGroupDescriptor,
        path: String,
        x: &mut f32,
        bounds: egui::Rect,
        operations: &mut Vec<TabStripPaintOperation>,
        active_reveal_pending: &mut bool,
    ) -> Result<(), TabStripRetainedError> {
        let label = self.render_label(
            ui,
            operations,
            TabStripLabelRenderRequest {
                text: &group.label,
                path: format!("{path}-header"),
                x: *x,
                bounds,
                active: group.capabilities.collapsed,
                active_reveal_pending,
                interaction: TabStripLabelInteraction {
                    route_path: group
                        .capabilities
                        .collapsible
                        .then(|| format!("{path}-header")),
                },
                draggable: false,
            },
        )?;
        if group.capabilities.accepts_tab_drop {
            self.drag_candidates.push(TabStripDropCandidate {
                bounds: label.bounds,
                kind: TabStripDropCandidateKind::Group(group.target.copy_for_route()),
            });
        }
        if label.secondary_clicked && group.popup.is_some() {
            self.overlay = TabStripOverlayState::GroupPopup {
                path: path.clone(),
                anchor: egui::pos2(label.bounds.min.x, label.bounds.max.y + TAB_GAP_PX),
                submenu_path: Vec::new(),
                rename: group.popup.as_ref().and_then(|popup| {
                    popup.rename_placeholder.as_ref().map(|placeholder| {
                        Box::new(TabStripRenameDraft::new(
                            &group.label.value,
                            &placeholder.value,
                        ))
                    })
                }),
            };
        }
        *x += label.advance;
        if !group.capabilities.collapsed {
            for (index, tab) in group.tabs.iter().enumerate() {
                self.render_tab(
                    ui,
                    tab,
                    format!("{path}-tab-{index}"),
                    x,
                    bounds,
                    operations,
                    active_reveal_pending,
                )?;
            }
            for (index, child) in group.groups.iter().enumerate() {
                self.render_group(
                    ui,
                    child,
                    format!("{path}-group-{index}"),
                    x,
                    bounds,
                    operations,
                    active_reveal_pending,
                )?;
            }
        }
        Ok(())
    }

    pub(super) fn render_icon_control(
        &mut self,
        ui: &mut egui::Ui,
        control: TabStripIconControl<'_>,
        x: &mut f32,
        bounds: egui::Rect,
        operations: &mut Vec<TabStripPaintOperation>,
    ) -> Result<(), TabStripRetainedError> {
        let rect = egui::Rect::from_min_size(
            egui::pos2(*x, bounds.min.y + TAB_GAP_PX),
            egui::vec2(
                NAVIGATION_CONTROL_WIDTH_PX,
                (bounds.height() - TAB_GAP_PX * 2.0).max(1.0),
            ),
        );
        let response = ui.interact(
            rect,
            ui.id().with(control.path),
            if control.enabled {
                egui::Sense::click()
            } else {
                egui::Sense::hover()
            },
        );
        let response_id = response.id;
        self.routes.register_response(
            control.path,
            response_id,
            ui_rect(rect),
            &control.presentation.accessibility_label.value,
            !control.enabled,
        );
        self.publish_response_accesskit(ui, response_id);
        self.forward_response_activation(ui, &response)?;
        response.on_hover_text(&control.presentation.tooltip.value);
        let texture = self.raster_icon(
            control.icon,
            if control.enabled {
                RgbaColor::new(
                    ICON_ENABLED_RGB[0],
                    ICON_ENABLED_RGB[1],
                    ICON_ENABLED_RGB[2],
                    PRIMARY_TEXT_RGBA[RGBA_ALPHA_INDEX],
                )
            } else {
                RgbaColor::new(
                    ICON_DISABLED_RGB[0],
                    ICON_DISABLED_RGB[1],
                    ICON_DISABLED_RGB[2],
                    PRIMARY_TEXT_RGBA[RGBA_ALPHA_INDEX],
                )
            },
        )?;
        let texture_handle = self.textures.texture_for_rgba(
            ui.ctx(),
            &texture.identity,
            texture.width as usize,
            texture.height as usize,
            &texture.rgba_pixels,
        );
        let background = if control.enabled {
            INACTIVE_TAB_RGBA
        } else {
            DISABLED_TAB_RGBA
        };
        ui.painter().rect_filled(
            rect,
            CORNER_RADIUS_PX,
            egui::Color32::from_rgba_unmultiplied(
                background[0],
                background[1],
                background[2],
                background[RGBA_ALPHA_INDEX],
            ),
        );
        let icon_rect = egui::Rect::from_center_size(
            rect.center(),
            egui::vec2(texture.width as f32, texture.height as f32),
        );
        ui.painter().image(
            texture_handle.id(),
            icon_rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
        operations.push(TabStripPaintOperation {
            clip_bounds: ui_rect(bounds),
            kind: TabStripPaintOperationKind::Fill {
                bounds: ui_rect(rect),
                color_rgba: background,
            },
        });
        operations.push(TabStripPaintOperation {
            clip_bounds: ui_rect(bounds),
            kind: TabStripPaintOperationKind::Texture {
                bounds: ui_rect(icon_rect),
                texture,
            },
        });
        *x += NAVIGATION_CONTROL_WIDTH_PX + TAB_GAP_PX;
        Ok(())
    }

    pub(super) fn render_tab(
        &mut self,
        ui: &mut egui::Ui,
        tab: &TabStripTabDescriptor,
        path: String,
        x: &mut f32,
        bounds: egui::Rect,
        operations: &mut Vec<TabStripPaintOperation>,
        active_reveal_pending: &mut bool,
    ) -> Result<(), TabStripRetainedError> {
        let label = self.render_label(
            ui,
            operations,
            TabStripLabelRenderRequest {
                text: &tab.label,
                path: format!("{path}-label"),
                x: *x,
                bounds,
                active: tab.capabilities.active,
                active_reveal_pending,
                interaction: TabStripLabelInteraction {
                    route_path: tab.capabilities.selectable.then(|| format!("{path}-label")),
                },
                draggable: tab.capabilities.draggable,
            },
        )?;
        if tab.capabilities.accepts_tab_drop {
            self.drag_candidates.push(TabStripDropCandidate {
                bounds: label.bounds,
                kind: TabStripDropCandidateKind::Tab(tab.target.copy_for_route()),
            });
        }
        if label.drag_started {
            self.start_tab_drag(tab, label.bounds, ui)?;
        }
        if label.drag_stopped
            && self
                .drag
                .as_ref()
                .is_some_and(|drag| drag.source.same_target(&tab.target))
        {
            self.drag_release_pending = true;
        }
        if label.secondary_clicked && tab.context_menu.is_some() {
            self.overlay = TabStripOverlayState::TabMenu {
                path: path.clone(),
                anchor: egui::pos2(label.bounds.min.x, label.bounds.max.y + TAB_GAP_PX),
                submenu_path: Vec::new(),
            };
        }
        *x += label.advance;
        if (tab.capabilities.closeable || tab.capabilities.pinned)
            && let Some(presentation) = tab.trailing_control.as_ref()
        {
            self.render_tab_trailing_control(
                ui,
                TabStripTrailingControl {
                    tab,
                    presentation,
                    path: format!("{path}-trailing"),
                },
                x,
                bounds,
                operations,
            )?;
        }
        Ok(())
    }
}
