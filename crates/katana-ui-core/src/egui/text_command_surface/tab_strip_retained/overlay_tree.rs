use super::support::{TabStripOverlayOutcome, ui_rect, union_bounds};
use super::{
    TAB_GAP_PX, TabStripMenuEntry, TabStripPaintPlan, TabStripRetainedError, TabStripRetainedState,
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
        while !current_entries.is_empty() {
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
        let opening_bounds =
            self.overlay_input
                .opened_by_secondary
                .as_ref()
                .and_then(|(path, bounds)| {
                    (route_prefix == format!("{path}-menu")
                        || route_prefix == format!("{path}-popup"))
                    .then_some(*bounds)
                });
        let Some(surface) = union_bounds(&panel_bounds) else {
            let closed = protected_bounds.is_empty()
                || ui.input(|input| {
                    input
                        .events
                        .iter()
                        .any(|event| is_outside_press(event, &[], protected_bounds, opening_bounds))
                });
            return Ok(TabStripOverlayOutcome {
                closed,
                submenu_path: Vec::new(),
                paint_plan: TabStripPaintPlan {
                    surface_bounds: union_bounds(protected_bounds)
                        .map_or(UiRect::new(0, 0, 0, 0), ui_rect),
                    operations,
                },
            });
        };
        if !closed
            && ui.input(|input| {
                input.events.iter().any(|event| {
                    is_outside_press(event, &panel_bounds, protected_bounds, opening_bounds)
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
}

fn is_outside_press(
    event: &egui::Event,
    panel_bounds: &[egui::Rect],
    protected_bounds: &[egui::Rect],
    opening_bounds: Option<egui::Rect>,
) -> bool {
    let egui::Event::PointerButton {
        pos,
        button,
        pressed: true,
        ..
    } = event
    else {
        return false;
    };
    if *button == egui::PointerButton::Secondary
        && opening_bounds.is_some_and(|bounds| bounds.contains(*pos))
    {
        return false;
    }
    !panel_bounds
        .iter()
        .chain(protected_bounds)
        .any(|bounds| bounds.contains(*pos))
}
