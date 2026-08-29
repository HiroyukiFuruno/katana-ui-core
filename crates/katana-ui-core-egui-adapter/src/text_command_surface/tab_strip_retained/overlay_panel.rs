use super::support::{TabStripOverlayOutcome, TabStripOverlayPanel, ui_rect, union_bounds};
use super::{
    CHECKMARK_INSET_PX, CORNER_RADIUS_PX, OVERLAY_BACKGROUND_RGBA, OVERLAY_PADDING_PX,
    OVERLAY_ROW_HEIGHT_PX, OVERLAY_SEPARATOR_HEIGHT_PX, OVERLAY_SEPARATOR_RGBA,
    OVERLAY_SWATCH_GAP_PX, OVERLAY_SWATCH_INSET_PX, OVERLAY_WIDTH_PX, PRIMARY_TEXT_RGBA,
    RGBA_ALPHA_INDEX, SELECTION_RGBA, TAB_GAP_PX, TabStripMenuEntry, TabStripPaintOperation,
    TabStripPaintOperationKind, TabStripPaintPlan, TabStripRetainedError, TabStripRetainedState,
    UiRect,
};

impl TabStripRetainedState {
    pub(super) fn render_overlay_tree(
        &mut self,
        ui: &mut egui::Ui,
        entries: &[TabStripMenuEntry],
        route_prefix: &str,
        anchor: egui::Pos2,
        mut submenu_path: Vec<usize>,
        protected_bounds: &[egui::Rect],
    ) -> Result<TabStripOverlayOutcome, TabStripRetainedError> {
        let mut current_entries = entries;
        let mut current_prefix = route_prefix.to_owned();
        let mut position = anchor;
        let mut depth = 0_usize;
        let mut operations = Vec::new();
        let mut panel_bounds = Vec::new();
        let mut closed = false;
        loop {
            let panel = self.render_overlay_panel(
                ui,
                current_entries,
                &current_prefix,
                position,
                depth,
                &mut operations,
            )?;
            panel_bounds.push(panel.bounds);
            if panel.closed {
                closed = true;
                break;
            }
            if let Some(index) = panel.open_submenu {
                submenu_path.truncate(depth);
                submenu_path.push(index);
            }
            let Some(index) = submenu_path.get(depth).copied() else {
                break;
            };
            let Some(entry) = current_entries.get(index) else {
                submenu_path.truncate(depth);
                break;
            };
            if entry.children.is_empty() {
                submenu_path.truncate(depth);
                break;
            }
            position = egui::pos2(panel.bounds.max.x + TAB_GAP_PX, panel.row_y(index));
            current_prefix = format!("{current_prefix}-{index}");
            current_entries = &entry.children;
            depth = depth.saturating_add(1);
        }
        let Some(surface) = union_bounds(&panel_bounds) else {
            return Ok(TabStripOverlayOutcome {
                closed: true,
                submenu_path: Vec::new(),
                paint_plan: TabStripPaintPlan {
                    surface_bounds: UiRect::new(0, 0, 0, 0),
                    operations,
                },
            });
        };
        if !closed
            && ui.input(|input| {
                input.events.iter().any(|event| {
                    let egui::Event::PointerButton {
                        pos, pressed: true, ..
                    } = event
                    else {
                        return false;
                    };
                    !panel_bounds.iter().any(|bounds| bounds.contains(*pos))
                        && !protected_bounds.iter().any(|bounds| bounds.contains(*pos))
                })
            })
        {
            closed = true;
        }
        Ok(TabStripOverlayOutcome {
            closed,
            submenu_path,
            paint_plan: TabStripPaintPlan {
                surface_bounds: ui_rect(surface),
                operations,
            },
        })
    }

    pub(super) fn render_overlay_panel(
        &mut self,
        ui: &mut egui::Ui,
        entries: &[TabStripMenuEntry],
        route_prefix: &str,
        position: egui::Pos2,
        depth: usize,
        operations: &mut Vec<TabStripPaintOperation>,
    ) -> Result<TabStripOverlayPanel, TabStripRetainedError> {
        let height = OVERLAY_PADDING_PX * 2.0
            + entries
                .iter()
                .map(|entry| {
                    if entry.separator {
                        OVERLAY_SEPARATOR_HEIGHT_PX
                    } else {
                        OVERLAY_ROW_HEIGHT_PX
                    }
                })
                .sum::<f32>();
        let area_id = ui.id().with(("tab-strip-overlay", route_prefix, depth));
        let response = egui::Area::new(area_id)
            .order(egui::Order::Foreground)
            .constrain(false)
            .fixed_pos(position)
            .show(ui.ctx(), |menu_ui| {
                let (bounds, _) = menu_ui.allocate_exact_size(
                    egui::vec2(OVERLAY_WIDTH_PX, height),
                    egui::Sense::empty(),
                );
                menu_ui.painter().rect_filled(
                    bounds,
                    OVERLAY_SWATCH_INSET_PX,
                    egui::Color32::from_rgb(
                        OVERLAY_BACKGROUND_RGBA[0],
                        OVERLAY_BACKGROUND_RGBA[1],
                        OVERLAY_BACKGROUND_RGBA[2],
                    ),
                );
                operations.push(TabStripPaintOperation {
                    clip_bounds: ui_rect(bounds),
                    kind: TabStripPaintOperationKind::Fill {
                        bounds: ui_rect(bounds),
                        color_rgba: OVERLAY_BACKGROUND_RGBA,
                    },
                });
                let mut y = bounds.min.y + OVERLAY_PADDING_PX;
                let mut open_submenu = None;
                let mut closed = false;
                let mut row_positions = Vec::with_capacity(entries.len());
                for (index, entry) in entries.iter().enumerate() {
                    if entry.separator {
                        let separator = egui::Rect::from_min_size(
                            egui::pos2(
                                bounds.min.x + OVERLAY_PADDING_PX,
                                y + OVERLAY_SWATCH_INSET_PX,
                            ),
                            egui::vec2(bounds.width() - OVERLAY_PADDING_PX * 2.0, 1.0),
                        );
                        menu_ui.painter().rect_filled(
                            separator,
                            0.0,
                            egui::Color32::from_rgb(
                                OVERLAY_SEPARATOR_RGBA[0],
                                OVERLAY_SEPARATOR_RGBA[1],
                                OVERLAY_SEPARATOR_RGBA[2],
                            ),
                        );
                        operations.push(TabStripPaintOperation {
                            clip_bounds: ui_rect(bounds),
                            kind: TabStripPaintOperationKind::Fill {
                                bounds: ui_rect(separator),
                                color_rgba: OVERLAY_SEPARATOR_RGBA,
                            },
                        });
                        row_positions.push(y);
                        y += OVERLAY_SEPARATOR_HEIGHT_PX;
                        continue;
                    }
                    let row = egui::Rect::from_min_size(
                        egui::pos2(bounds.min.x + OVERLAY_PADDING_PX, y),
                        egui::vec2(
                            bounds.width() - OVERLAY_PADDING_PX * 2.0,
                            OVERLAY_ROW_HEIGHT_PX,
                        ),
                    );
                    let response = menu_ui.interact(
                        row,
                        area_id.with(index),
                        if entry.enabled {
                            egui::Sense::click()
                        } else {
                            egui::Sense::hover()
                        },
                    );
                    let row_color = if response.hovered() && entry.enabled {
                        SELECTION_RGBA
                    } else {
                        OVERLAY_BACKGROUND_RGBA
                    };
                    menu_ui.painter().rect_filled(
                        row,
                        CORNER_RADIUS_PX,
                        egui::Color32::from_rgba_unmultiplied(
                            row_color[0],
                            row_color[1],
                            row_color[2],
                            row_color[RGBA_ALPHA_INDEX],
                        ),
                    );
                    operations.push(TabStripPaintOperation {
                        clip_bounds: ui_rect(bounds),
                        kind: TabStripPaintOperationKind::Fill {
                            bounds: ui_rect(row),
                            color_rgba: row_color,
                        },
                    });
                    if entry.checked {
                        let check = egui::Rect::from_center_size(
                            egui::pos2(row.max.x - CHECKMARK_INSET_PX, row.center().y),
                            egui::vec2(OVERLAY_SWATCH_GAP_PX, OVERLAY_SWATCH_GAP_PX),
                        );
                        menu_ui.painter().rect_filled(
                            check,
                            1.0,
                            egui::Color32::from_rgb(
                                PRIMARY_TEXT_RGBA[0],
                                PRIMARY_TEXT_RGBA[1],
                                PRIMARY_TEXT_RGBA[2],
                            ),
                        );
                        operations.push(TabStripPaintOperation {
                            clip_bounds: ui_rect(bounds),
                            kind: TabStripPaintOperationKind::Fill {
                                bounds: ui_rect(check),
                                color_rgba: PRIMARY_TEXT_RGBA,
                            },
                        });
                    }
                    let route_path = format!("{route_prefix}-{index}");
                    if entry.operation.is_some() {
                        self.routes.register_response(
                            &route_path,
                            response.id,
                            ui_rect(row),
                            &entry.accessibility_label.value,
                            !entry.enabled,
                        );
                        self.publish_response_accesskit(menu_ui, response.id);
                        if self.forward_response_activation(menu_ui, &response)? {
                            closed = true;
                        }
                    }
                    self.paint_overlay_label(
                        menu_ui,
                        operations,
                        bounds,
                        &entry.label,
                        row,
                        route_prefix,
                        index,
                    )?;
                    if !entry.children.is_empty() && response.hovered() {
                        open_submenu = Some(index);
                    }
                    row_positions.push(y);
                    y += OVERLAY_ROW_HEIGHT_PX;
                }
                Ok(TabStripOverlayPanel {
                    bounds,
                    row_positions,
                    open_submenu,
                    closed,
                })
            });
        response.inner
    }
}
