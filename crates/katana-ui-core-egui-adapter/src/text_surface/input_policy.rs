use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EguiTextSurfaceKey {
    Enter,
    Escape,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiTextSurfaceInputPolicy {
    pub suppressed_keys: Vec<EguiTextSurfaceKey>,
    pub suppress_text_input: bool,
    pub publish_context_target: bool,
}

impl Default for EguiTextSurfaceInputPolicy {
    fn default() -> Self {
        Self {
            suppressed_keys: Vec::new(),
            suppress_text_input: false,
            publish_context_target: true,
        }
    }
}

impl EguiTextSurfaceInputPolicy {
    #[must_use]
    pub fn suppress(mut self, value: EguiTextSurfaceKey) -> Self {
        if !self.suppressed_keys.contains(&value) {
            self.suppressed_keys.push(value);
        }
        self
    }

    #[must_use]
    pub(crate) fn context_menu() -> Self {
        Self {
            suppressed_keys: vec![
                EguiTextSurfaceKey::Escape,
                EguiTextSurfaceKey::ArrowUp,
                EguiTextSurfaceKey::ArrowDown,
                EguiTextSurfaceKey::ArrowLeft,
                EguiTextSurfaceKey::ArrowRight,
            ],
            suppress_text_input: true,
            publish_context_target: true,
        }
    }

    #[must_use]
    pub(crate) const fn without_context_target(mut self) -> Self {
        self.publish_context_target = false;
        self
    }

    pub(crate) fn suppresses_event(&self, event: &egui::Event) -> bool {
        if self.suppress_text_input && matches!(event, egui::Event::Text(_) | egui::Event::Ime(_)) {
            return true;
        }
        let egui::Event::Key {
            key, pressed: true, ..
        } = event
        else {
            return false;
        };
        let Some(key) = (match key {
            egui::Key::Enter => Some(EguiTextSurfaceKey::Enter),
            egui::Key::Escape => Some(EguiTextSurfaceKey::Escape),
            egui::Key::ArrowUp => Some(EguiTextSurfaceKey::ArrowUp),
            egui::Key::ArrowDown => Some(EguiTextSurfaceKey::ArrowDown),
            egui::Key::ArrowLeft => Some(EguiTextSurfaceKey::ArrowLeft),
            egui::Key::ArrowRight => Some(EguiTextSurfaceKey::ArrowRight),
            _ => None,
        }) else {
            return false;
        };
        self.suppressed_keys.contains(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_menu_policy_suppresses_text_ime_and_escape() {
        let policy = EguiTextSurfaceInputPolicy::context_menu();
        assert!(policy.suppresses_event(&egui::Event::Text("x".into())));
        assert!(
            policy.suppresses_event(&egui::Event::Ime(egui::ImeEvent::Preedit {
                text: "x".into(),
                active_range_chars: None,
            }))
        );
        assert!(policy.suppresses_event(&egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }));
    }

    #[test]
    fn suppress_keeps_each_key_unique() {
        let policy = EguiTextSurfaceInputPolicy::default()
            .suppress(EguiTextSurfaceKey::Enter)
            .suppress(EguiTextSurfaceKey::Enter);
        assert_eq!(policy.suppressed_keys, vec![EguiTextSurfaceKey::Enter]);
    }

    #[test]
    fn every_key_mapping_and_non_key_path_is_explicit() {
        let policy = [
            EguiTextSurfaceKey::Enter,
            EguiTextSurfaceKey::Escape,
            EguiTextSurfaceKey::ArrowUp,
            EguiTextSurfaceKey::ArrowDown,
            EguiTextSurfaceKey::ArrowLeft,
            EguiTextSurfaceKey::ArrowRight,
        ]
        .into_iter()
        .fold(EguiTextSurfaceInputPolicy::default(), |policy, key| {
            policy.suppress(key)
        });
        for key in [
            egui::Key::Enter,
            egui::Key::Escape,
            egui::Key::ArrowUp,
            egui::Key::ArrowDown,
            egui::Key::ArrowLeft,
            egui::Key::ArrowRight,
        ] {
            assert!(policy.suppresses_event(&egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }));
        }
        assert!(!policy.suppresses_event(&egui::Event::Copy));
        assert!(!policy.suppresses_event(&egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }));
    }

    #[test]
    fn default_policy_accepts_ordinary_text_and_ime_input() {
        let policy = EguiTextSurfaceInputPolicy::default();

        assert!(!policy.suppresses_event(&egui::Event::Text("日本語 ⭐️".into())));
        assert!(
            !policy.suppresses_event(&egui::Event::Ime(egui::ImeEvent::Commit(
                "日本語 ⭐️".into(),
            )))
        );
    }

    #[test]
    fn context_menu_policy_maps_supported_keys_and_rejects_releases_and_unknown_keys() {
        let policy = EguiTextSurfaceInputPolicy::context_menu();
        let suppressed = [
            egui::Key::Escape,
            egui::Key::ArrowUp,
            egui::Key::ArrowDown,
            egui::Key::ArrowLeft,
            egui::Key::ArrowRight,
        ];

        for key in suppressed {
            assert!(policy.suppresses_event(&egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }));
            assert!(!policy.suppresses_event(&egui::Event::Key {
                key,
                physical_key: None,
                pressed: false,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }));
        }

        assert!(!policy.suppresses_event(&egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }));
        assert!(!policy.suppresses_event(&egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }));
    }

    #[test]
    fn default_policy_keeps_key_releases_and_non_text_events() {
        let policy = EguiTextSurfaceInputPolicy::default();

        assert!(!policy.suppresses_event(&egui::Event::Key {
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
        assert!(!policy.suppresses_event(&egui::Event::Key {
            key: egui::Key::F1,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }));
    }

    #[test]
    fn default_policy_keeps_ime_commit_and_key_releases() {
        let policy = EguiTextSurfaceInputPolicy::default();

        assert!(!policy.suppresses_event(&egui::Event::Ime(
            egui::ImeEvent::Commit("かな".into())
        )));
        assert!(!policy.suppresses_event(&egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }));
    }

}
