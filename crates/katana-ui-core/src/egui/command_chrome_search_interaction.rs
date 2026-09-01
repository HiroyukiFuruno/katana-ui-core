use super::command_chrome_types::EguiCommandChromeSearchControlFrame;
use crate::atom::TextAreaEvent;
use crate::egui::text_surface::{
    EguiTextSurfaceInputPolicy, EguiTextSurfaceKey, EguiTextSurfaceOutput,
};
use crate::molecule::command_chrome::{
    CommandChromeSearchAction, CommandChromeSearchEvent, CommandChromeSearchStrip,
};
use crate::molecule::structured::{SearchControlStripAction, SearchNavigationDirection};
use crate::render_model::UiRect;
use crate::text_surface::TextSurfaceEvent;

pub(super) fn apply_text_events(
    strip: &mut CommandChromeSearchStrip,
    events: &[TextSurfaceEvent],
    query: bool,
) -> Vec<CommandChromeSearchEvent> {
    events
        .iter()
        .flat_map(|event| match event {
            TextSurfaceEvent::TextArea(TextAreaEvent::Change(value)) => {
                strip.apply_action(CommandChromeSearchAction::Strip {
                    action: if query {
                        SearchControlStripAction::SetSearchQuery(value.clone())
                    } else {
                        SearchControlStripAction::SetReplaceValue(value.clone())
                    },
                })
            }
            _ => Vec::new(),
        })
        .collect()
}

pub(super) fn query_key_events(
    ui: &egui::Ui,
    strip: &mut CommandChromeSearchStrip,
    query_focused: bool,
) -> Vec<CommandChromeSearchEvent> {
    if !query_focused {
        return Vec::new();
    }
    let input_events = ui.input(|input| input.events.clone());
    let mut routed_events = Vec::new();
    for event in input_events {
        if let egui::Event::Key {
            key,
            pressed: true,
            modifiers,
            ..
        } = event
        {
            routed_events.extend(route_pressed_key(strip, key, modifiers));
        }
    }
    routed_events
}

fn route_pressed_key(
    strip: &mut CommandChromeSearchStrip,
    key: egui::Key,
    modifiers: egui::Modifiers,
) -> Vec<CommandChromeSearchEvent> {
    match key {
        egui::Key::Enter => navigate(strip, modifiers.shift),
        egui::Key::ArrowDown => navigate(strip, false),
        egui::Key::ArrowUp => navigate(strip, true),
        egui::Key::Escape => strip.apply_action(CommandChromeSearchAction::RequestClose),
        _ => Vec::new(),
    }
}

pub(super) fn query_input_policy() -> EguiTextSurfaceInputPolicy {
    EguiTextSurfaceInputPolicy::default()
        .without_context_target()
        .with_text_input_target()
        .suppress(EguiTextSurfaceKey::Enter)
        .suppress(EguiTextSurfaceKey::Escape)
        .suppress(EguiTextSurfaceKey::ArrowUp)
        .suppress(EguiTextSurfaceKey::ArrowDown)
}

pub(super) fn bounds(
    start: egui::Pos2,
    query: &UiRect,
    replace: Option<&EguiTextSurfaceOutput>,
    controls: &[EguiCommandChromeSearchControlFrame],
) -> UiRect {
    let mut values = vec![*query];
    if let Some(replace) = replace {
        values.push(replace.record.frame.content_bounds);
    }
    values.extend(controls.iter().map(|control| control.bounds));
    let right = values
        .iter()
        .map(|value| value.x.saturating_add(value.width as i32))
        .max()
        .unwrap_or(start.x as i32);
    let bottom = values
        .iter()
        .map(|value| value.y.saturating_add(value.height as i32))
        .max()
        .unwrap_or(start.y as i32);
    UiRect::new(
        start.x.round() as i32,
        start.y.round() as i32,
        right.saturating_sub(start.x.round() as i32) as u32,
        bottom.saturating_sub(start.y.round() as i32) as u32,
    )
}

fn navigate(strip: &mut CommandChromeSearchStrip, previous: bool) -> Vec<CommandChromeSearchEvent> {
    strip.apply_action(CommandChromeSearchAction::Strip {
        action: SearchControlStripAction::Navigate(if previous {
            SearchNavigationDirection::Previous
        } else {
            SearchNavigationDirection::Next
        }),
    })
}

#[cfg(test)]
#[path = "command_chrome_search_interaction_tests.rs"]
mod tests;
