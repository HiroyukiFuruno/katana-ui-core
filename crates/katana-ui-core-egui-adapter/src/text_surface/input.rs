use super::model::EguiTextSurfaceInputPolicy;
use katana_ui_core::atom::{
    TextAreaAction, TextAreaCompositionPhase, TextAreaKey, TextAreaKeyChord, TextAreaSelection,
};
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceClipboardOperation, TextSurfaceEvent,
    TextSurfaceFrameRecord, TextSurfaceHistoryOperation, TextSurfaceLayout,
};
mod pointer;

pub(super) fn apply_interactions(
    ui: &egui::Ui,
    response: &egui::Response,
    surface: &mut TextSurface,
    layout: &TextSurfaceLayout,
    frame: &TextSurfaceFrameRecord,
    input_policy: &EguiTextSurfaceInputPolicy,
    pending_focus_request: Option<bool>,
) -> Vec<TextSurfaceEvent> {
    let accepts_text_input =
        response.has_focus() || ui.ctx().memory(|memory| memory.focused().is_none());
    let mut events = pointer::focus_events(
        ui,
        surface,
        response,
        frame,
        pending_focus_request,
        input_policy.retain_pointer_focus,
    );
    events.extend(pointer::pointer_events(
        ui, response, surface, layout, frame,
    ));
    if !surface.state().text_area.focused || !accepts_text_input {
        return events;
    }
    for event in ui.input(|input| input.events.clone()) {
        if input_policy.suppresses_event(&event) {
            continue;
        }
        events.extend(input_event(surface, layout, event));
    }
    let scroll = ui.input(|input| input.smooth_scroll_delta());
    if scroll != egui::Vec2::ZERO && (response.hovered() || response.has_focus()) {
        events.extend(
            surface
                .apply_action(TextSurfaceAction::ScrollBy {
                    delta_x: scroll.x.round() as i32,
                    delta_y: scroll.y.round() as i32,
                })
                .events,
        );
    }
    events
}

fn input_event(
    surface: &mut TextSurface,
    layout: &TextSurfaceLayout,
    event: egui::Event,
) -> Vec<TextSurfaceEvent> {
    let action = match event {
        egui::Event::Copy => {
            TextSurfaceAction::ClipboardRequest(TextSurfaceClipboardOperation::Copy)
        }
        egui::Event::Cut => TextSurfaceAction::ClipboardRequest(TextSurfaceClipboardOperation::Cut),
        egui::Event::Paste(_) => {
            TextSurfaceAction::ClipboardRequest(TextSurfaceClipboardOperation::Paste)
        }
        egui::Event::Text(text) if text != "\n" && text != "\r" => {
            TextSurfaceAction::TextArea(TextAreaAction::Type(text))
        }
        egui::Event::Ime(egui::ImeEvent::Preedit { text, .. }) if text.is_empty() => {
            TextSurfaceAction::CancelComposition
        }
        egui::Event::Ime(egui::ImeEvent::Preedit { text, .. }) => TextSurfaceAction::TextArea(
            TextAreaAction::composition(TextAreaCompositionPhase::Update, &text, text.len()),
        ),
        egui::Event::Ime(egui::ImeEvent::Commit(text)) => {
            TextSurfaceAction::TextArea(TextAreaAction::ime_commit(text))
        }
        egui::Event::Key {
            key,
            pressed: true,
            modifiers,
            ..
        } => {
            let Some(action) = key_action(surface, layout, key, modifiers) else {
                return Vec::new();
            };
            action
        }
        _ => return Vec::new(),
    };
    surface.apply_action(action).events
}

fn key_action(
    surface: &TextSurface,
    layout: &TextSurfaceLayout,
    key: egui::Key,
    modifiers: egui::Modifiers,
) -> Option<TextSurfaceAction> {
    match key {
        egui::Key::ArrowLeft
        | egui::Key::ArrowRight
        | egui::Key::ArrowUp
        | egui::Key::ArrowDown
        | egui::Key::Home
        | egui::Key::End => Some(select_action(surface, layout, key, modifiers.shift)),
        egui::Key::A if modifiers.command => Some(select_all_action(layout)),
        egui::Key::Z if modifiers.command && modifiers.shift => Some(
            TextSurfaceAction::HistoryRequest(TextSurfaceHistoryOperation::Redo),
        ),
        egui::Key::Z if modifiers.command => Some(TextSurfaceAction::HistoryRequest(
            TextSurfaceHistoryOperation::Undo,
        )),
        egui::Key::Backspace | egui::Key::Delete => {
            Some(TextSurfaceAction::TextArea(TextAreaAction::DeleteBackward))
        }
        egui::Key::Enter | egui::Key::Tab => Some(TextSurfaceAction::Key(TextAreaKeyChord {
            key: if key == egui::Key::Enter {
                TextAreaKey::Enter
            } else {
                TextAreaKey::Tab
            },
            shift: modifiers.shift,
            primary_modifier: modifiers.command,
        })),
        _ => None,
    }
}

fn select_action(
    surface: &TextSurface,
    layout: &TextSurfaceLayout,
    key: egui::Key,
    extend_selection: bool,
) -> TextSurfaceAction {
    let current = layout.grapheme_range_for_byte_offsets(
        surface.state().text_area.selection.start,
        surface.state().text_area.selection.end,
    );
    let target = target_grapheme(layout, current, key);
    let range = if extend_selection {
        UiTextSelectionRange::new(current.anchor, target)
    } else {
        UiTextSelectionRange::caret(target)
    };
    let (start, end) = layout.byte_offsets_for_grapheme_range(range);
    TextSurfaceAction::TextArea(TextAreaAction::Select(TextAreaSelection { start, end }))
}

fn select_all_action(layout: &TextSurfaceLayout) -> TextSurfaceAction {
    let (start, end) = layout
        .byte_offsets_for_grapheme_range(UiTextSelectionRange::new(0, layout.graphemes.len()));
    TextSurfaceAction::TextArea(TextAreaAction::Select(TextAreaSelection { start, end }))
}

fn target_grapheme(
    layout: &TextSurfaceLayout,
    selection: UiTextSelectionRange,
    key: egui::Key,
) -> usize {
    let caret = selection.caret_position();
    let current_rect = layout.caret_rect(UiTextSelectionRange::caret(caret));
    match key {
        egui::Key::ArrowLeft => caret.saturating_sub(1),
        egui::Key::ArrowRight => caret.saturating_add(1).min(layout.graphemes.len()),
        egui::Key::Home => layout
            .hit_test(layout.content_bounds.x, current_rect.y)
            .caret_position(),
        egui::Key::End => layout
            .hit_test(
                layout
                    .content_bounds
                    .x
                    .saturating_add(layout.content_bounds.width as i32),
                current_rect.y,
            )
            .caret_position(),
        egui::Key::ArrowUp => vertical_target_grapheme(layout, caret, current_rect, false),
        egui::Key::ArrowDown => vertical_target_grapheme(layout, caret, current_rect, true),
        _ => caret,
    }
}

fn vertical_target_grapheme(
    layout: &TextSurfaceLayout,
    caret: usize,
    current_rect: katana_ui_core::render_model::UiRect,
    down: bool,
) -> usize {
    let Some(current_index) = layout.lines.iter().position(|line| {
        current_rect.y >= line.bounds.y
            && current_rect.y < line.bounds.y.saturating_add_unsigned(line.bounds.height)
    }) else {
        return caret;
    };
    let target_index = if down {
        current_index
            .saturating_add(1)
            .min(layout.lines.len().saturating_sub(1))
    } else {
        current_index.saturating_sub(1)
    };
    let Some(target_line) = layout.lines.get(target_index) else {
        return caret;
    };
    let target_y = target_line
        .bounds
        .y
        .saturating_add_unsigned(target_line.bounds.height / 2);
    layout.hit_test(current_rect.x, target_y).caret_position()
}
