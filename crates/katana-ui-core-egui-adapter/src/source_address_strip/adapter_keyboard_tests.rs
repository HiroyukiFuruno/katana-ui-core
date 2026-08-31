use super::{
    EguiSourceAddressStripAdapter, EguiSourceAddressStripOutput, SourceAddressFrameEventClass,
};
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressAction, SourceAddressEntry, SourceAddressPresentation, SourceAddressStrip,
};

const SCREEN_SIZE: egui::Vec2 = egui::vec2(420.0, 80.0);

fn strip() -> SourceAddressStrip {
    SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソース",
        "ソースを入力",
        "ソースを入力",
    ))
}

fn keyboard_input(key: egui::Key) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN_SIZE)),
        events: vec![egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        ..egui::RawInput::default()
    }
}

fn render_with_input(
    context: &egui::Context,
    adapter: &mut EguiSourceAddressStripAdapter,
    strip: &mut SourceAddressStrip,
    input: egui::RawInput,
) -> EguiSourceAddressStripOutput {
    let mut output = None;
    crate::run_ui_discard(context, input, |ui| {
        output = Some(adapter.show(ui, strip).expect("source-address renders"));
    });
    output.expect("source-address frame produces output")
}

fn warm(
    context: &egui::Context,
    adapter: &mut EguiSourceAddressStripAdapter,
    strip: &mut SourceAddressStrip,
) {
    let _ = render_with_input(
        context,
        adapter,
        strip,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN_SIZE)),
            ..egui::RawInput::default()
        },
    );
}

fn advance_keyboard_focus(
    context: &egui::Context,
    adapter: &mut EguiSourceAddressStripAdapter,
    strip: &mut SourceAddressStrip,
    tab_count: usize,
) {
    for _ in 0..tab_count {
        let _ = render_with_input(context, adapter, strip, keyboard_input(egui::Key::Tab));
    }
}

fn entry(label: &str) -> SourceAddressEntry {
    SourceAddressEntry::new(
        SourceAddressPresentation::new(label, label, label),
        label.as_bytes(),
    )
}

#[test]
fn focused_history_toggle_activates_once_for_enter_and_space_from_real_egui_input() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let context = egui::Context::default();
        let mut adapter = EguiSourceAddressStripAdapter::new("source-address-history-keyboard")
            .expect("adapter should initialize");
        let mut strip = strip();
        strip.set_history(vec![entry("history")]);
        warm(&context, &mut adapter, &mut strip);
        advance_keyboard_focus(&context, &mut adapter, &mut strip, 2);

        let output = render_with_input(&context, &mut adapter, &mut strip, keyboard_input(key));

        assert_eq!(
            output.event_classes(),
            &[SourceAddressFrameEventClass::HistoryOpened],
            "{key:?} activates the focused history toggle exactly once"
        );
        assert!(strip.history_open());
    }
}

#[test]
fn focused_candidate_toggle_activates_once_for_enter_and_space_from_real_egui_input() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let context = egui::Context::default();
        let mut adapter = EguiSourceAddressStripAdapter::new("source-address-candidate-keyboard")
            .expect("adapter should initialize");
        let mut strip = strip();
        strip.set_candidates(vec![entry("candidate")]);
        warm(&context, &mut adapter, &mut strip);
        advance_keyboard_focus(&context, &mut adapter, &mut strip, 2);

        let output = render_with_input(&context, &mut adapter, &mut strip, keyboard_input(key));

        assert_eq!(
            output.event_classes(),
            &[SourceAddressFrameEventClass::CandidatesOpened],
            "{key:?} activates the focused candidate toggle exactly once"
        );
        assert!(strip.candidates_open());
    }
}

#[test]
fn focused_entry_activates_once_for_enter_and_space_from_real_egui_input() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let context = egui::Context::default();
        let mut adapter = EguiSourceAddressStripAdapter::new("source-address-entry-keyboard")
            .expect("adapter should initialize");
        let mut strip = strip();
        strip.set_history(vec![entry("history")]);
        assert!(matches!(
            strip.apply_action(SourceAddressAction::OpenHistory),
            Some(katana_ui_core::molecule::structured::source_address_strip::SourceAddressEvent::HistoryOpened)
        ));
        warm(&context, &mut adapter, &mut strip);
        advance_keyboard_focus(&context, &mut adapter, &mut strip, 4);

        let output = render_with_input(&context, &mut adapter, &mut strip, keyboard_input(key));

        assert_eq!(
            output.event_classes(),
            &[SourceAddressFrameEventClass::HistorySelected],
            "{key:?} activates the focused entry exactly once"
        );
        assert_eq!(strip.selected_history(), Some(0));
    }
}

#[test]
fn focused_submit_activates_once_for_enter_and_space_from_real_egui_input() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let context = egui::Context::default();
        let mut adapter = EguiSourceAddressStripAdapter::new("source-address-submit-keyboard")
            .expect("adapter should initialize");
        let mut strip = strip();
        assert!(
            strip
                .apply_action(SourceAddressAction::SetDraft("source".to_owned()))
                .is_some()
        );
        warm(&context, &mut adapter, &mut strip);
        advance_keyboard_focus(&context, &mut adapter, &mut strip, 2);

        let mut output = render_with_input(&context, &mut adapter, &mut strip, keyboard_input(key));

        assert_eq!(
            output.event_classes(),
            &[SourceAddressFrameEventClass::Submitted],
            "{key:?} activates the focused submit button exactly once"
        );
        assert_eq!(output.take_submissions().len(), 1);
    }
}

#[test]
fn source_address_keyboard_activation_ignores_enter_and_space_when_another_control_has_focus() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let context = egui::Context::default();
        let mut adapter = EguiSourceAddressStripAdapter::new("source-address-other-focus")
            .expect("adapter should initialize");
        let mut strip = strip();
        strip.set_history(vec![entry("history")]);
        strip.set_candidates(vec![entry("candidate")]);
        warm(&context, &mut adapter, &mut strip);
        context.memory_mut(|memory| memory.request_focus(egui::Id::new("other-control")));

        let output = render_with_input(&context, &mut adapter, &mut strip, keyboard_input(key));

        assert!(
            output.event_classes().is_empty(),
            "{key:?} must not activate an unfocused source-address control"
        );
    }
}
