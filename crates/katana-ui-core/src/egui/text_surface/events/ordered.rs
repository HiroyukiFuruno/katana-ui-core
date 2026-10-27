use super::super::keyboard::input_event;
use super::super::model::EguiTextSurfaceInputPolicy;
use crate::text_surface::{TextSurface, TextSurfaceEvent, TextSurfaceLayout};

pub(crate) trait OrderedKeyboardInput {
    fn before(&mut self, surface: &mut TextSurface, event: &egui::Event) -> bool;
    fn after(&mut self, surface: &TextSurface, events: &[TextSurfaceEvent]);
}

pub(crate) struct OrderedKeyboardRouting<'a> {
    pub(crate) handler: Option<&'a mut dyn OrderedKeyboardInput>,
    pub(crate) refresh: &'a mut dyn FnMut(&TextSurface) -> Option<TextSurfaceLayout>,
}

pub(super) fn apply_keyboard(
    ui: &egui::Ui,
    surface: &mut TextSurface,
    layout: &TextSurfaceLayout,
    input_policy: &EguiTextSurfaceInputPolicy,
    mut ordered: Option<&mut dyn OrderedKeyboardInput>,
    refresh: &mut dyn FnMut(&TextSurface) -> Option<TextSurfaceLayout>,
) -> Vec<TextSurfaceEvent> {
    let mut events = Vec::new();
    let mut current_layout = None;
    let mut consumed = Vec::new();
    for (index, event) in ui
        .input(|input| input.events.clone())
        .into_iter()
        .enumerate()
    {
        let before_value = ordered
            .as_ref()
            .map(|_| surface.state().text_area.value.clone());
        if ordered
            .as_deref_mut()
            .is_some_and(|handler| handler.before(surface, &event))
        {
            consumed.push(index);
        } else if !input_policy.suppresses_event(&event) {
            let produced = input_event(surface, current_layout.as_ref().unwrap_or(layout), event);
            if let Some(handler) = ordered.as_deref_mut() {
                handler.after(surface, &produced);
            }
            events.extend(produced);
        }
        if before_value.is_some_and(|value| value != surface.state().text_area.value) {
            current_layout = refresh(surface);
        }
    }
    if !consumed.is_empty() {
        ui.input_mut(|input| {
            let mut index = 0;
            input.events.retain(|_| {
                let keep = !consumed.contains(&index);
                index += 1;
                keep
            });
        });
    }
    events
}
