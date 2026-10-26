use super::SelectionDialogRetainedState;

#[test]
fn focus_defers_only_text_input_events_until_modal_layer_retires() {
    let key = egui::Event::Key {
        key: egui::Key::A,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    let ime = egui::Event::Ime(egui::ImeEvent::Preedit {
        text: "入力".into(),
        active_range_chars: None,
    });
    let deferred = [
        egui::Event::Text("入力".into()),
        ime,
        key,
        egui::Event::Paste("貼付".into()),
        egui::Event::Copy,
        egui::Event::Cut,
    ];
    assert!(
        deferred
            .iter()
            .all(SelectionDialogRetainedState::defer_until_focus_returns)
    );
    assert!(!SelectionDialogRetainedState::defer_until_focus_returns(
        &egui::Event::PointerMoved(egui::pos2(1.0, 1.0))
    ));
}
