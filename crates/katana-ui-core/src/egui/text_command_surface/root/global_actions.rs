use serde::Serialize;

/// hostが発行するグローバル操作の不透明な識別子。
#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct KucGlobalActionTarget(Box<[u8]>);
impl KucGlobalActionTarget {
    #[must_use]
    pub fn from_opaque_bytes(value: impl Into<Vec<u8>>) -> Self {
        Self(value.into().into_boxed_slice())
    }

    pub(super) fn identity(&self) -> String {
        use sha2::{Digest, Sha256};
        format!("kuc.global-action.{}", hex::encode(Sha256::digest(&self.0)))
    }

    /// 現在のroot frameからこの操作を探す不透明なselectorを返す。
    #[must_use]
    pub fn interaction_selector(&self) -> super::KucInteractionSelector {
        super::KucInteractionSelector::new(
            self.identity(),
            super::KucInteractionActionClass::GlobalAction,
        )
    }
}
impl std::fmt::Debug for KucGlobalActionTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("KucGlobalActionTarget(..)")
    }
}

/// 修飾キーを完全一致で照合するショートカット。primaryはcommandを使う。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KucGlobalShortcut {
    key: egui::Key,
    modifiers: egui::Modifiers,
}
impl KucGlobalShortcut {
    #[must_use]
    pub const fn new(key: egui::Key, modifiers: egui::Modifiers) -> Self {
        Self { key, modifiers }
    }
}

/// グローバルショートカットと入力focusの競合方針。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KucGlobalActionFocusPolicy {
    /// 他の入力欄やmodal上でもショートカットを処理する。
    #[default]
    Any,
    /// 他入力へのfocus時や同frameのprimary押下時は処理しない。
    DocumentOnly,
}

/// 同じrootのグローバル操作。表示とキー処理はKUCが保持する。
pub struct KucGlobalAction {
    pub(super) target: KucGlobalActionTarget,
    pub(super) label: String,
    pub(super) shortcut: KucGlobalShortcut,
    pub(super) enabled: bool,
    focus_policy: KucGlobalActionFocusPolicy,
}
impl KucGlobalAction {
    #[must_use]
    pub fn new(
        target: KucGlobalActionTarget,
        label: impl Into<String>,
        shortcut: KucGlobalShortcut,
    ) -> Self {
        Self {
            target,
            label: label.into(),
            shortcut,
            enabled: true,
            focus_policy: KucGlobalActionFocusPolicy::Any,
        }
    }
    /// 他入力欄のfocus時にショートカットを処理するかを指定する。
    #[must_use]
    pub const fn focus_policy(mut self, value: KucGlobalActionFocusPolicy) -> Self {
        self.focus_policy = value;
        self
    }

    #[must_use]
    pub const fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// hostが投影するグローバルメニューと操作。
pub struct KucGlobalActionsProjection {
    pub(super) menu_label: String,
    pub(super) actions: Vec<KucGlobalAction>,
}
impl KucGlobalActionsProjection {
    #[must_use]
    pub fn new(menu_label: impl Into<String>) -> Self {
        Self {
            menu_label: menu_label.into(),
            actions: Vec::new(),
        }
    }
    /// 現在のroot frameからメニューを探すselectorを返す。
    #[must_use]
    pub fn menu_selector(&self) -> super::KucInteractionSelector {
        super::KucInteractionSelector::new(
            "kuc.global-menu",
            super::KucInteractionActionClass::GlobalActionMenu,
        )
    }

    #[must_use]
    pub fn action(mut self, action: KucGlobalAction) -> Self {
        self.actions.push(action);
        self
    }
}

/// hostの識別子を不透明なまま返す操作通知。
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KucGlobalActionEvent {
    pub(super) target: KucGlobalActionTarget,
}
impl KucGlobalActionEvent {
    #[must_use]
    pub fn target(&self) -> KucGlobalActionTarget {
        self.target.clone()
    }
}

impl KucGlobalActionsProjection {
    pub(super) fn consume_shortcut(
        &self,
        ui: &mut egui::Ui,
        blocked: bool,
        document_focus: Option<egui::Id>,
    ) -> Vec<KucGlobalActionEvent> {
        if blocked
            || ui.input(|i| {
                i.events
                    .iter()
                    .any(|event| matches!(event, egui::Event::Ime(_)))
            })
        {
            return Vec::new();
        }
        let pointer_focus_change = ui.input(|input| {
            input.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::PointerButton {
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        ..
                    }
                )
            })
        });
        self.actions
            .iter()
            .filter(|action| {
                action.enabled
                    && (action.focus_policy == KucGlobalActionFocusPolicy::Any
                        || (!pointer_focus_change && ui.memory(|m| m.focused().is_none_or(|id| Some(id) == document_focus))))
                    && ui.input_mut(|input| {
                        let mut matched = false;
                        input.events.retain(|event| {
                            let selected = matches!(event, egui::Event::Key { key, modifiers, pressed: true, .. } if *key == action.shortcut.key && modifiers.matches_exact(action.shortcut.modifiers));
                            matched |= selected;
                            !selected
                        });
                        matched
                    })
            })
            .map(|action| KucGlobalActionEvent {
                    target: action.target.clone(),
            })
            .collect()
    }
}
