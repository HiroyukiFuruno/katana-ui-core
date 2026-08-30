use super::{EguiTextSurfaceInputPolicy, EguiTextSurfaceKey};

#[test]
fn input_policy_options_preserve_context_and_text_contracts() {
    let policy = EguiTextSurfaceInputPolicy::context_menu()
        .without_context_target()
        .with_text_input_target()
        .with_retained_pointer_focus()
        .suppress(EguiTextSurfaceKey::Enter);

    assert!(!policy.publish_context_target);
    assert!(policy.publish_text_input_target);
    assert!(policy.retain_pointer_focus);
    assert!(policy.suppressed_keys.contains(&EguiTextSurfaceKey::Enter));
}

#[test]
fn input_policy_default_does_not_filter_text_without_text_input_mode() {
    let policy = EguiTextSurfaceInputPolicy::default();

    assert!(!policy.suppress_text_input);
    assert!(!policy.suppresses_event(&egui::Event::Text("x".to_owned())));
    assert!(!policy.suppresses_event(&egui::Event::Key {
        key: egui::Key::Enter,
        pressed: true,
        physical_key: None,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }));
}

#[test]
fn input_policy_context_menu_suppresses_expected_keys_and_ignores_unknown_inputs() {
    let policy = EguiTextSurfaceInputPolicy::context_menu();

    assert!(policy.suppress_text_input);
    assert!(policy.suppresses_event(&egui::Event::Text("x".to_owned())));
    assert!(
        policy.suppresses_event(&egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "x".to_string(),
            active_range_chars: None,
        }))
    );
    assert!(policy.suppresses_event(&egui::Event::Ime(egui::ImeEvent::Commit("x".to_owned()))));
    assert!(policy.suppresses_event(&egui::Event::Key {
        key: egui::Key::ArrowUp,
        pressed: true,
        physical_key: None,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }));
    assert!(!policy.suppresses_event(&egui::Event::Key {
        key: egui::Key::A,
        pressed: true,
        physical_key: None,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }));
}

#[test]
fn suppressing_same_key_multiple_times_keeps_single_entry() {
    let policy = EguiTextSurfaceInputPolicy::default()
        .suppress(EguiTextSurfaceKey::Enter)
        .suppress(EguiTextSurfaceKey::Enter);

    assert_eq!(policy.suppressed_keys, vec![EguiTextSurfaceKey::Enter]);
}

#[test]
fn default_input_policy_does_not_filter_ime_text_or_nonmatching_keys() {
    let policy = EguiTextSurfaceInputPolicy::default();

    assert!(!policy.suppresses_event(&egui::Event::Text("x".to_owned())));
    assert!(!policy.suppresses_event(&egui::Event::Ime(egui::ImeEvent::Commit("x".to_owned()))));
    assert!(!policy.suppresses_event(&egui::Event::Key {
        key: egui::Key::Tab,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }));
}

#[test]
fn default_policy_suppresses_only_configured_pressed_keys_and_releases_pass_through() {
    let policy = EguiTextSurfaceInputPolicy::default().suppress(EguiTextSurfaceKey::Enter);

    assert!(policy.suppresses_event(&egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }));
    assert!(!policy.suppresses_event(&egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: None,
        pressed: false,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }));
}

#[test]
fn input_policy_maps_all_supported_navigation_keys_and_retains_pointer_focus() {
    let policy = EguiTextSurfaceInputPolicy::default()
        .with_retained_pointer_focus()
        .suppress(EguiTextSurfaceKey::Escape)
        .suppress(EguiTextSurfaceKey::ArrowUp)
        .suppress(EguiTextSurfaceKey::ArrowDown)
        .suppress(EguiTextSurfaceKey::ArrowLeft)
        .suppress(EguiTextSurfaceKey::ArrowRight);

    assert!(policy.retain_pointer_focus);
    for key in [
        egui::Key::Escape,
        egui::Key::ArrowUp,
        egui::Key::ArrowDown,
        egui::Key::ArrowLeft,
        egui::Key::ArrowRight,
    ] {
        assert!(policy.suppresses_event(&egui::Event::Key {
            key,
            pressed: true,
            physical_key: None,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }));
    }
}
