use super::{
    TAB_GAP_PX, TabStripDropCandidate, TabStripDropCandidateKind, TabStripGroupDescriptor,
    TabStripLabelInteraction, TabStripLabelRenderRequest, TabStripOverlayState,
    TabStripPaintOperation, TabStripRenameDraft, TabStripRetainedError, TabStripRetainedState,
    TabStripTabDescriptor, TabStripTrailingControl,
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
                draggable: group.capabilities.draggable,
            },
        )?;
        self.group_drag_candidates.push(TabStripDropCandidate {
            bounds: label.bounds,
            kind: TabStripDropCandidateKind::Group(group.target.copy_for_route()),
        });
        if label.drag_started {
            self.start_group_drag(group, label.bounds, ui)?;
        }
        if label.drag_stopped
            && self
                .group_drag
                .as_ref()
                .is_some_and(|drag| drag.source == group.target)
        {
            self.group_drag_release_pending = true;
        }
        if let Some(popup) = group.popup.as_ref()
            && popup.request_inline_rename
            && (!group.capabilities.renamable || popup.rename_placeholder.is_none())
        {
            return Err(TabStripRetainedError::InvalidInlineRenameRequest);
        }
        if matches!(self.overlay, TabStripOverlayState::Closed)
            && group.capabilities.renamable
            && let Some(popup) = group.popup.as_ref()
            && popup.request_inline_rename
            && let Some(placeholder) = popup.rename_placeholder.as_ref()
            && !self
                .inline_rename_consumed
                .iter()
                .any(|target| target == &group.target)
        {
            self.inline_rename_consumed.push(group.target.clone());
            self.overlay = TabStripOverlayState::GroupPopup {
                path: path.clone(),
                anchor: egui::pos2(label.bounds.min.x, label.bounds.max.y + TAB_GAP_PX),
                submenu_path: Vec::new(),
                rename: Some(Box::new(TabStripRenameDraft::new(
                    &group.label.value,
                    &placeholder.value,
                ))),
            };
        }
        if group.capabilities.accepts_tab_drop {
            self.drag_candidates.push(TabStripDropCandidate {
                bounds: label.bounds,
                kind: TabStripDropCandidateKind::Group(group.target.copy_for_route()),
            });
        }
        if label.secondary_clicked && group.capabilities.menu_available && group.popup.is_some() {
            self.overlay = TabStripOverlayState::GroupPopup {
                path: path.clone(),
                anchor: egui::pos2(label.bounds.min.x, label.bounds.max.y + TAB_GAP_PX),
                submenu_path: Vec::new(),
                rename: group.popup.as_ref().and_then(|popup| {
                    if !group.capabilities.renamable {
                        return None;
                    }
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

#[cfg(test)]
#[path = "paint_tests.rs"]
mod tests;
