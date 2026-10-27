use super::drag_feedback::{TabStripDragFeedback, TabStripDropIndicator, drop_indicator};
use super::{
    TabStripDropCandidateKind, TabStripGroupDescriptor, TabStripGroupTarget,
    TabStripProposalOperation, TabStripResolvedGroupDrop, TabStripRetainedError,
    TabStripRetainedState,
};

#[cfg(test)]
#[path = "group_drag_tests.rs"]
mod tests;

impl TabStripRetainedState {
    pub(super) fn start_group_drag(
        &mut self,
        group: &TabStripGroupDescriptor,
        bounds: egui::Rect,
        ui: &egui::Ui,
    ) -> Result<(), TabStripRetainedError> {
        if self.group_drag.is_some() || self.drag.is_some() {
            return Ok(());
        }
        self.group_drag = Some(super::TabStripGroupDragState {
            source: group.target.copy_for_route(),
            label: super::TabStripText::new(group.label.value.as_str()),
            pointer: ui
                .input(|input| input.pointer.latest_pos())
                .unwrap_or(bounds.center()),
        });
        Ok(())
    }

    pub(super) fn resolve_group_drag(
        &mut self,
        ui: &egui::Ui,
        strip_bounds: egui::Rect,
        operations: &mut Vec<super::TabStripPaintOperation>,
    ) -> Result<(), TabStripRetainedError> {
        let Some(mut drag) = self.group_drag.take() else {
            self.group_drag_release_pending = false;
            return Ok(());
        };
        drag.pointer = ui
            .input(|input| input.pointer.latest_pos())
            .unwrap_or(drag.pointer);
        let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
        let pointer_released = ui.input(|input| {
            input.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::PointerButton {
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        ..
                    }
                )
            })
        });
        let release = std::mem::take(&mut self.group_drag_release_pending) || pointer_released;
        if escape || release {
            let destination = (!escape)
                .then(|| self.resolve_group_drop(drag.pointer, &drag.source, strip_bounds))
                .flatten()
                .map(|drop| drop.destination);
            let operation = match (escape, destination) {
                (true, _) | (false, None) => TabStripProposalOperation::CancelDrag,
                (false, Some(destination)) => TabStripProposalOperation::ReorderGroup {
                    source: drag.source,
                    destination,
                },
            };
            self.forward_proposal_with_origin(
                self.routes.correlation_for_proposal(),
                operation,
                super::tab_strip_proposal_port::TabStripProposalOrigin::DragTerminal,
            )?;
            return Ok(());
        }
        let drop = self.resolve_group_drop(drag.pointer, &drag.source, strip_bounds);
        self.paint_drag_feedback(
            ui,
            TabStripDragFeedback {
                label: &drag.label,
                pointer: drag.pointer,
                indicator: drop.map(|value| value.indicator),
                ghost_width: None,
            },
            strip_bounds,
            operations,
        )?;
        self.group_drag = Some(drag);
        Ok(())
    }

    fn resolve_group_drop(
        &self,
        pointer: egui::Pos2,
        source: &TabStripGroupTarget,
        strip_bounds: egui::Rect,
    ) -> Option<TabStripResolvedGroupDrop> {
        for candidate in &self.group_drag_candidates {
            if !candidate.bounds.contains(pointer) {
                continue;
            }
            let TabStripDropCandidateKind::Group(target) = &candidate.kind else {
                continue;
            };
            if source == target {
                continue;
            }
            let ratio = (pointer.x - candidate.bounds.min.x) / candidate.bounds.width();
            if ratio <= super::DROP_LEFT_RATIO {
                return Some(TabStripResolvedGroupDrop {
                    destination:
                        super::super::tab_strip_proposal_port::TabStripGroupPlacement::Before(
                            target.copy_for_route(),
                        ),
                    indicator: drop_indicator(
                        TabStripDropIndicator::Before,
                        candidate.bounds,
                        strip_bounds,
                        pointer.x,
                    ),
                });
            }
            if ratio >= super::DROP_RIGHT_RATIO {
                return Some(TabStripResolvedGroupDrop {
                    destination:
                        super::super::tab_strip_proposal_port::TabStripGroupPlacement::After(
                            target.copy_for_route(),
                        ),
                    indicator: drop_indicator(
                        TabStripDropIndicator::After,
                        candidate.bounds,
                        strip_bounds,
                        pointer.x,
                    ),
                });
            }
        }
        let rightmost = self
            .group_drag_candidates
            .iter()
            .map(|candidate| candidate.bounds.max.x)
            .fold(strip_bounds.min.x, f32::max);
        (strip_bounds.contains(pointer) && pointer.x > rightmost).then_some(
            TabStripResolvedGroupDrop {
                destination:
                    super::super::tab_strip_proposal_port::TabStripGroupPlacement::EndOfStrip,
                indicator: drop_indicator(
                    TabStripDropIndicator::EndOfStrip,
                    strip_bounds,
                    strip_bounds,
                    pointer.x,
                ),
            },
        )
    }
}
