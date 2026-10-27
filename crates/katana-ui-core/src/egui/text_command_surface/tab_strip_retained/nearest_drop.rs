use super::{
    TabStripDropCandidateKind, TabStripResolvedDrop, TabStripRetainedState, TabStripTabPlacement,
};

impl TabStripRetainedState {
    pub(super) fn resolve_nearest_tab_drop(
        &self,
        center_x: f32,
        end_drop_available: bool,
        viewport: egui::Rect,
    ) -> Option<TabStripResolvedDrop> {
        let logical_tabs: Vec<_> = self
            .drag_candidates
            .iter()
            .filter_map(|candidate| match &candidate.kind {
                TabStripDropCandidateKind::Tab(target) => Some((target, candidate.bounds)),
                TabStripDropCandidateKind::Group(_) => None,
            })
            .collect();
        let tabs: Vec<_> = logical_tabs
            .iter()
            .enumerate()
            .filter_map(|(index, (target, bounds))| {
                let bounds = bounds.intersect(viewport);
                bounds.is_positive().then_some((index, *target, bounds))
            })
            .collect();
        let first = tabs.first()?;
        let mut best: Option<(f32, f32, TabStripTabPlacement, egui::Rect)> = None;
        for index in 0..=tabs.len() {
            let (x, destination, reference) = if index == tabs.len() {
                let (logical_index, target, bounds) = tabs[index - 1];
                let destination = if logical_index + 1 == logical_tabs.len() {
                    TabStripTabPlacement::EndOfStrip
                } else {
                    TabStripTabPlacement::After(target.copy_for_route())
                };
                (bounds.max.x, destination, bounds)
            } else {
                let (_, target, bounds) = tabs[index];
                let x = if index == 0 {
                    first.2.min.x
                } else {
                    (tabs[index - 1].2.max.x + bounds.min.x) / 2.0
                };
                (
                    x,
                    TabStripTabPlacement::Before(target.copy_for_route()),
                    bounds,
                )
            };
            let distance = (center_x - x).abs();
            if best
                .as_ref()
                .is_none_or(|(best_distance, ..)| distance < *best_distance)
            {
                best = Some((distance, x, destination, reference));
            }
        }
        best.filter(|(_, _, destination, _)| {
            end_drop_available || !matches!(destination, TabStripTabPlacement::EndOfStrip)
        })
        .map(|(_, x, destination, bounds)| TabStripResolvedDrop {
            destination,
            indicator: egui::Rect::from_min_size(
                egui::pos2(x - 1.0, bounds.min.y),
                egui::vec2(2.0, bounds.height()),
            ),
        })
    }
}
