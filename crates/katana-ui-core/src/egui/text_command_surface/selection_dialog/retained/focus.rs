use super::SelectionDialogRetainedState;

#[cfg(test)]
#[path = "focus_tests.rs"]
mod tests;

impl SelectionDialogRetainedState {
    pub(crate) fn prepare_input(&mut self, ui: &egui::Ui) {
        if self.lease.open {
            return;
        }
        let Some(id) = self.restore_focus else {
            return;
        };
        let retiring = self
            .modal_layer
            .is_some_and(|layer| ui.memory(|memory| memory.top_modal_layer() == Some(layer)));
        if retiring {
            /* WHY: 前frameのmodal layerを解放する間も、閉じた直後の文字入力を失わない。 */
            ui.input_mut(|input| {
                let (deferred, remaining): (Vec<_>, Vec<_>) = std::mem::take(&mut input.events)
                    .into_iter()
                    .partition(Self::defer_until_focus_returns);
                self.deferred_input.extend(deferred);
                input.events = remaining;
            });
            ui.ctx().request_repaint();
            return;
        }
        ui.memory_mut(|memory| memory.request_focus(id));
        self.restore_focus = None;
        self.modal_layer = None;
        ui.input_mut(|input| {
            self.deferred_input.append(&mut input.events);
            input.events = std::mem::take(&mut self.deferred_input);
        });
    }

    fn defer_until_focus_returns(event: &egui::Event) -> bool {
        matches!(
            event,
            egui::Event::Text(_)
                | egui::Event::Ime(_)
                | egui::Event::Key { .. }
                | egui::Event::Paste(_)
                | egui::Event::Copy
                | egui::Event::Cut
        )
    }
}
