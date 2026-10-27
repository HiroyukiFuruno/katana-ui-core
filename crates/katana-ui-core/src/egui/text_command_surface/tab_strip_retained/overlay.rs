use super::support::{group_for_path, rect_from_ui_rect, tab_menu_for_path, ui_rect, union_bounds};
use super::{
    TabStripOverlayState, TabStripPaintPlan, TabStripProjection, TabStripRetainedError,
    TabStripRetainedState,
};

impl TabStripRetainedState {
    pub(super) fn render_overlay(
        &mut self,
        ui: &mut egui::Ui,
        projection: &TabStripProjection,
    ) -> Result<Option<TabStripPaintPlan>, TabStripRetainedError> {
        let state = std::mem::replace(&mut self.overlay, TabStripOverlayState::Closed);
        let before = state.clone();
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
                if let Some(pending) = self.pending_rename_changes.as_mut() {
                    pending.before = Some(before.clone());
                }
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
}

#[cfg(test)]
#[path = "overlay_tests.rs"]
mod tests;
