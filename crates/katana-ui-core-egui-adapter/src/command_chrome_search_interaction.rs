use super::command_chrome_types::EguiCommandChromeSearchControlFrame;
use crate::text_surface::{EguiTextSurfaceInputPolicy, EguiTextSurfaceKey, EguiTextSurfaceOutput};
use katana_ui_core::atom::TextAreaEvent;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchAction, CommandChromeSearchEvent, CommandChromeSearchStrip,
};
use katana_ui_core::molecule::structured::{SearchControlStripAction, SearchNavigationDirection};
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::TextSurfaceEvent;

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
    ui.input(|input| input.events.clone())
        .into_iter()
        .filter_map(|event| match event {
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } => query_key_action(key, modifiers.shift),
            _ => None,
        })
        .flat_map(|action| strip.apply_action(action))
        .collect()
}

fn query_key_action(key: egui::Key, shift: bool) -> Option<CommandChromeSearchAction> {
    match key {
        egui::Key::Enter => Some(navigate_action(shift)),
        egui::Key::ArrowDown => Some(navigate_action(false)),
        egui::Key::ArrowUp => Some(navigate_action(true)),
        egui::Key::Escape => Some(CommandChromeSearchAction::RequestClose),
        _ => None,
    }
}

fn navigate_action(previous: bool) -> CommandChromeSearchAction {
    CommandChromeSearchAction::Strip {
        action: SearchControlStripAction::Navigate(if previous {
            SearchNavigationDirection::Previous
        } else {
            SearchNavigationDirection::Next
        }),
    }
}

pub(super) fn query_input_policy() -> EguiTextSurfaceInputPolicy {
    EguiTextSurfaceInputPolicy::default()
        .without_context_target()
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

#[cfg(test)]
mod tests {
    use super::query_key_action;

    #[test]
    fn query_key_classification_covers_navigation_close_and_unhandled_keys() {
        for (key, shift) in [
            (egui::Key::Enter, true),
            (egui::Key::Enter, false),
            (egui::Key::ArrowDown, true),
            (egui::Key::ArrowUp, false),
            (egui::Key::Escape, false),
        ] {
            assert!(query_key_action(key, shift).is_some());
        }
        assert_eq!(query_key_action(egui::Key::Tab, false), None);
    }
}
