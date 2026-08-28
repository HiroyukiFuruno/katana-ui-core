use katana_ui_core::atom::{
    TextAreaAction, TextAreaCompositionPhase, TextAreaKey, TextAreaKeyChord, TextAreaSelection,
};
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceClipboardOperation, TextSurfaceEvent,
    TextSurfaceHistoryOperation, TextSurfaceLayout,
};

pub(super) fn input_event(
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
    let target_line = &layout.lines[target_index];
    let target_y = target_line
        .bounds
        .y
        .saturating_add_unsigned(target_line.bounds.height / 2);
    layout.hit_test(current_rect.x, target_y).caret_position()
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::atom::{TextArea, TextAreaEvent};
    use katana_ui_core::render_model::{UiRect, UiTextSpan};
    use katana_ui_core::text_surface::{
        TextSurfaceGraphemeBox, TextSurfaceProps, TextSurfaceViewport,
    };

    fn fixture() -> (TextSurface, TextSurfaceLayout) {
        let text = "ab\ncd";
        let surface = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("keyboard-fixture")
                .value(text)
                .ime_enabled(true),
            vec![UiTextSpan::plain(text)],
            TextSurfaceViewport::new(0, 0, 80, 40),
        ));
        let layout = TextSurfaceLayout::from_grapheme_boxes(
            "keyboard-layout",
            UiRect::new(0, 0, 40, 40),
            text,
            vec![
                TextSurfaceGraphemeBox {
                    grapheme_index: 0,
                    byte_start: 0,
                    byte_end: 1,
                    bounds: UiRect::new(0, 0, 10, 20),
                },
                TextSurfaceGraphemeBox {
                    grapheme_index: 1,
                    byte_start: 1,
                    byte_end: 2,
                    bounds: UiRect::new(10, 0, 10, 20),
                },
                TextSurfaceGraphemeBox {
                    grapheme_index: 2,
                    byte_start: 2,
                    byte_end: 3,
                    bounds: UiRect::new(20, 0, 10, 20),
                },
                TextSurfaceGraphemeBox {
                    grapheme_index: 3,
                    byte_start: 3,
                    byte_end: 4,
                    bounds: UiRect::new(0, 20, 10, 20),
                },
                TextSurfaceGraphemeBox {
                    grapheme_index: 4,
                    byte_start: 4,
                    byte_end: 5,
                    bounds: UiRect::new(10, 20, 10, 20),
                },
            ],
        );
        (surface, layout)
    }

    fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    #[test]
    fn input_events_cover_clipboard_ime_text_history_and_ignored_boundaries() {
        let (mut surface, layout) = fixture();
        let emitted = [
            egui::Event::Copy,
            egui::Event::Cut,
            egui::Event::Paste("x".to_string()),
            egui::Event::Text("x".to_string()),
            egui::Event::Ime(egui::ImeEvent::Preedit {
                text: "かな".to_string(),
                active_range_chars: None,
            }),
            egui::Event::Ime(egui::ImeEvent::Preedit {
                text: String::new(),
                active_range_chars: None,
            }),
            egui::Event::Ime(egui::ImeEvent::Commit("確定".to_string())),
            key(
                egui::Key::A,
                egui::Modifiers {
                    command: true,
                    ..egui::Modifiers::default()
                },
            ),
            key(
                egui::Key::Z,
                egui::Modifiers {
                    command: true,
                    ..egui::Modifiers::default()
                },
            ),
            key(
                egui::Key::Z,
                egui::Modifiers {
                    command: true,
                    shift: true,
                    ..egui::Modifiers::default()
                },
            ),
            key(egui::Key::Delete, egui::Modifiers::default()),
            key(egui::Key::Enter, egui::Modifiers::default()),
            key(egui::Key::Tab, egui::Modifiers::default()),
        ]
        .into_iter()
        .flat_map(|event| input_event(&mut surface, &layout, event))
        .collect::<Vec<_>>();
        assert!(input_event(&mut surface, &layout, egui::Event::Text("\n".to_string())).is_empty());
        assert!(
            input_event(
                &mut surface,
                &layout,
                egui::Event::Key {
                    key: egui::Key::Escape,
                    physical_key: None,
                    pressed: false,
                    repeat: false,
                    modifiers: egui::Modifiers::default(),
                }
            )
            .is_empty()
        );
        assert!(
            input_event(
                &mut surface,
                &layout,
                key(egui::Key::Escape, egui::Modifiers::default())
            )
            .is_empty()
        );
        assert!(
            emitted
                .iter()
                .any(|event| matches!(event, TextSurfaceEvent::TextArea(TextAreaEvent::Change(_))))
        );
    }

    #[test]
    fn directional_targets_cover_horizontal_vertical_line_edges_and_fallback() {
        let (mut surface, layout) = fixture();
        let selection = UiTextSelectionRange::caret(1);
        for navigation_key in [
            egui::Key::ArrowLeft,
            egui::Key::ArrowRight,
            egui::Key::Home,
            egui::Key::End,
            egui::Key::ArrowUp,
            egui::Key::ArrowDown,
            egui::Key::Escape,
        ] {
            let target = target_grapheme(&layout, selection, navigation_key);
            assert!(target <= layout.graphemes.len());
            if navigation_key != egui::Key::Escape {
                let _ = input_event(
                    &mut surface,
                    &layout,
                    key(navigation_key, egui::Modifiers::default()),
                );
            }
        }
        let _ = input_event(
            &mut surface,
            &layout,
            key(
                egui::Key::ArrowLeft,
                egui::Modifiers {
                    shift: true,
                    ..egui::Modifiers::default()
                },
            ),
        );

        let empty = TextSurfaceLayout::from_grapheme_boxes(
            "empty-keyboard-layout",
            UiRect::new(0, 0, 1, 1),
            "",
            Vec::new(),
        );
        assert_eq!(
            0,
            vertical_target_grapheme(&empty, 0, UiRect::new(0, 0, 1, 1), true)
        );
    }
}
