use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use katana_ui_widget::layout::modal::{ModalParentInteraction, ModalSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FooterSample {
    Confirm,
    Form,
    Detail,
}

impl FooterSample {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Confirm => "confirm",
            Self::Form => "form",
            Self::Detail => "detail",
        }
    }

    pub(crate) fn body(self) -> &'static str {
        match self {
            Self::Confirm => "保存前に内容を確認します。",
            Self::Form => "必須項目を入力してから実行してください。",
            Self::Detail => "詳細確認の完了結果を表示します。",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ModalSettingAction {
    SizeSm,
    SizeLg,
    SizeCustom,
    EscEnabled,
    EscDisabled,
    ParentBlock,
    ParentAllow,
    FooterConfirm,
    FooterForm,
    FooterDetail,
}

impl ModalSettingAction {
    pub(crate) const ALL: [Self; 10] = [
        Self::SizeSm,
        Self::SizeLg,
        Self::SizeCustom,
        Self::EscEnabled,
        Self::EscDisabled,
        Self::ParentBlock,
        Self::ParentAllow,
        Self::FooterConfirm,
        Self::FooterForm,
        Self::FooterDetail,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::SizeSm => "Sm を選択",
            Self::SizeLg => "Lg を選択",
            Self::SizeCustom => "Custom を選択",
            Self::EscEnabled => "Esc で閉じる",
            Self::EscDisabled => "Esc 無効",
            Self::ParentBlock => "親操作をブロック",
            Self::ParentAllow => "親操作を許可",
            Self::FooterConfirm => "footer: confirm",
            Self::FooterForm => "footer: form",
            Self::FooterDetail => "footer: detail",
        }
    }

    pub(crate) fn interaction(self) -> &'static str {
        match self {
            Self::SizeSm => "setting-size-sm",
            Self::SizeLg => "setting-size-lg",
            Self::SizeCustom => "setting-size-custom",
            Self::EscEnabled => "setting-esc-enabled",
            Self::EscDisabled => "setting-esc-disabled",
            Self::ParentBlock => "setting-parent-block",
            Self::ParentAllow => "setting-parent-allow",
            Self::FooterConfirm => "setting-footer-confirm",
            Self::FooterForm => "setting-footer-form",
            Self::FooterDetail => "setting-footer-detail",
        }
    }

    pub(crate) fn detail(self) -> &'static str {
        match self {
            Self::SizeSm => "size-sm-window-created",
            Self::SizeLg => "size-lg-window-created",
            Self::SizeCustom => "size-custom-window-created",
            Self::EscEnabled => "esc-enabled-window-created",
            Self::EscDisabled => "esc-disabled-window-created",
            Self::ParentBlock => "parent-block-window-created",
            Self::ParentAllow => "parent-allow-window-created",
            Self::FooterConfirm => "footer-confirm-window-created",
            Self::FooterForm => "footer-form-window-created",
            Self::FooterDetail => "footer-detail-window-created",
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ModalStateSignals {
    pub(crate) size: RwSignal<ModalSize>,
    pub(crate) title: RwSignal<String>,
    pub(crate) body: RwSignal<String>,
    pub(crate) footer: RwSignal<FooterSample>,
    pub(crate) dismiss_on_backdrop: RwSignal<bool>,
    pub(crate) dismiss_on_esc: RwSignal<bool>,
    pub(crate) parent_interaction: RwSignal<ModalParentInteraction>,
}

#[derive(Clone, Debug, PartialEq)]
struct ModalStateSnapshot {
    size: ModalSize,
    title: String,
    body: String,
    footer: FooterSample,
    dismiss_on_backdrop: bool,
    dismiss_on_esc: bool,
    parent_interaction: ModalParentInteraction,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ModalOpenSnapshot {
    pub(crate) size: ModalSize,
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) footer: FooterSample,
    pub(crate) footer_body: String,
    pub(crate) dismiss_on_backdrop: bool,
    pub(crate) dismiss_on_esc: bool,
    pub(crate) parent_interaction: ModalParentInteraction,
}

pub(crate) fn state_signals(
    size: RwSignal<ModalSize>,
    title: RwSignal<String>,
    body: RwSignal<String>,
    footer: RwSignal<FooterSample>,
    dismiss_on_backdrop: RwSignal<bool>,
    dismiss_on_esc: RwSignal<bool>,
    parent_interaction: RwSignal<ModalParentInteraction>,
) -> ModalStateSignals {
    ModalStateSignals {
        size,
        title,
        body,
        footer,
        dismiss_on_backdrop,
        dismiss_on_esc,
        parent_interaction,
    }
}

pub(crate) fn size_label(size: &ModalSize) -> &'static str {
    match size {
        ModalSize::Sm => "Sm",
        ModalSize::Md => "Md",
        ModalSize::Lg => "Lg",
        ModalSize::Custom(_) => "Custom",
    }
}

pub(crate) fn parent_interaction_label(value: &ModalParentInteraction) -> &'static str {
    match value {
        ModalParentInteraction::Block => "親ウィンドウ操作をブロック",
        ModalParentInteraction::Allow => "親ウィンドウ操作を許可",
    }
}

pub(crate) fn bool_label(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

pub(crate) fn modal_setting_button_label(
    action: ModalSettingAction,
    state: ModalStateSignals,
) -> String {
    if is_modal_setting_active(action, state) {
        format!("✓ {}", action.label())
    } else {
        action.label().to_string()
    }
}

pub(crate) fn apply_modal_setting(action: ModalSettingAction, state: ModalStateSignals) {
    let mut snapshot = snapshot_from_signals(state);
    apply_modal_setting_to_snapshot(action, &mut snapshot);
    write_snapshot_to_signals(snapshot, state);
}

pub(crate) fn open_snapshot_from_signals(state: ModalStateSignals) -> ModalOpenSnapshot {
    let snapshot = snapshot_from_signals(state);
    ModalOpenSnapshot {
        size: snapshot.size,
        title: snapshot.title,
        body: snapshot.body,
        footer: snapshot.footer,
        footer_body: snapshot.footer.body().to_string(),
        dismiss_on_backdrop: snapshot.dismiss_on_backdrop,
        dismiss_on_esc: snapshot.dismiss_on_esc,
        parent_interaction: snapshot.parent_interaction,
    }
}

pub(crate) fn modal_open_matches_action(
    action: ModalSettingAction,
    snapshot: &ModalOpenSnapshot,
) -> bool {
    match action {
        ModalSettingAction::SizeSm => snapshot.size == ModalSize::Sm,
        ModalSettingAction::SizeLg => snapshot.size == ModalSize::Lg,
        ModalSettingAction::SizeCustom => matches!(snapshot.size, ModalSize::Custom(560.0)),
        ModalSettingAction::EscEnabled => snapshot.dismiss_on_esc,
        ModalSettingAction::EscDisabled => !snapshot.dismiss_on_esc,
        ModalSettingAction::ParentBlock => {
            snapshot.parent_interaction == ModalParentInteraction::Block
        }
        ModalSettingAction::ParentAllow => {
            snapshot.parent_interaction == ModalParentInteraction::Allow
        }
        ModalSettingAction::FooterConfirm => footer_body_matches(snapshot, FooterSample::Confirm),
        ModalSettingAction::FooterForm => footer_body_matches(snapshot, FooterSample::Form),
        ModalSettingAction::FooterDetail => footer_body_matches(snapshot, FooterSample::Detail),
    }
}

pub(crate) fn modal_open_summary(snapshot: &ModalOpenSnapshot) -> String {
    format!(
        "size={} footer={} footer_body={} esc={} parent={}",
        size_label(&snapshot.size),
        snapshot.footer.label(),
        snapshot.footer_body,
        bool_label(snapshot.dismiss_on_esc),
        parent_interaction_label(&snapshot.parent_interaction),
    )
}

fn footer_body_matches(snapshot: &ModalOpenSnapshot, footer: FooterSample) -> bool {
    snapshot.footer == footer && snapshot.footer_body == footer.body()
}

fn is_modal_setting_active(action: ModalSettingAction, state: ModalStateSignals) -> bool {
    let snapshot = snapshot_from_signals(state);
    is_modal_setting_active_in_snapshot(action, &snapshot)
}

fn snapshot_from_signals(state: ModalStateSignals) -> ModalStateSnapshot {
    ModalStateSnapshot {
        size: state.size.get(),
        title: state.title.get(),
        body: state.body.get(),
        footer: state.footer.get(),
        dismiss_on_backdrop: state.dismiss_on_backdrop.get(),
        dismiss_on_esc: state.dismiss_on_esc.get(),
        parent_interaction: state.parent_interaction.get(),
    }
}

fn write_snapshot_to_signals(snapshot: ModalStateSnapshot, state: ModalStateSignals) {
    state.size.set(snapshot.size);
    state.title.set(snapshot.title);
    state.body.set(snapshot.body);
    state.footer.set(snapshot.footer);
    state.dismiss_on_backdrop.set(snapshot.dismiss_on_backdrop);
    state.dismiss_on_esc.set(snapshot.dismiss_on_esc);
    state.parent_interaction.set(snapshot.parent_interaction);
}

fn apply_modal_setting_to_snapshot(
    action: ModalSettingAction,
    state: &mut ModalStateSnapshot,
) {
    match action {
        ModalSettingAction::SizeSm => {
            state.size = ModalSize::Sm;
            state.title = "Small modal".to_string();
            state.body = "Small サイズで開く別ウィンドウModalです。".to_string();
        }
        ModalSettingAction::SizeLg => {
            state.size = ModalSize::Lg;
            state.title = "Large modal".to_string();
            state.body = "Large サイズで開く別ウィンドウModalです。".to_string();
        }
        ModalSettingAction::SizeCustom => {
            state.size = ModalSize::Custom(560.0);
            state.title = "Custom modal".to_string();
            state.body = "Custom 幅で開く別ウィンドウModalです。".to_string();
        }
        ModalSettingAction::EscEnabled => state.dismiss_on_esc = true,
        ModalSettingAction::EscDisabled => state.dismiss_on_esc = false,
        ModalSettingAction::ParentBlock => {
            state.parent_interaction = ModalParentInteraction::Block;
        }
        ModalSettingAction::ParentAllow => {
            state.parent_interaction = ModalParentInteraction::Allow;
        }
        ModalSettingAction::FooterConfirm => state.footer = FooterSample::Confirm,
        ModalSettingAction::FooterForm => state.footer = FooterSample::Form,
        ModalSettingAction::FooterDetail => state.footer = FooterSample::Detail,
    }
}

fn is_modal_setting_active_in_snapshot(
    action: ModalSettingAction,
    state: &ModalStateSnapshot,
) -> bool {
    match action {
        ModalSettingAction::SizeSm => state.size == ModalSize::Sm,
        ModalSettingAction::SizeLg => state.size == ModalSize::Lg,
        ModalSettingAction::SizeCustom => matches!(state.size, ModalSize::Custom(_)),
        ModalSettingAction::EscEnabled => state.dismiss_on_esc,
        ModalSettingAction::EscDisabled => !state.dismiss_on_esc,
        ModalSettingAction::ParentBlock => {
            state.parent_interaction == ModalParentInteraction::Block
        }
        ModalSettingAction::ParentAllow => {
            state.parent_interaction == ModalParentInteraction::Allow
        }
        ModalSettingAction::FooterConfirm => state.footer == FooterSample::Confirm,
        ModalSettingAction::FooterForm => state.footer == FooterSample::Form,
        ModalSettingAction::FooterDetail => state.footer == FooterSample::Detail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot() -> ModalStateSnapshot {
        ModalStateSnapshot {
            size: ModalSize::Md,
            title: "確認Modal".to_string(),
            body: "別ウィンドウとして開くModalです。".to_string(),
            footer: FooterSample::Confirm,
            dismiss_on_backdrop: true,
            dismiss_on_esc: true,
            parent_interaction: ModalParentInteraction::Block,
        }
    }

    #[test]
    fn size_setting_preserves_behavior_and_footer() {
        let mut state = snapshot();
        state.dismiss_on_esc = false;
        state.parent_interaction = ModalParentInteraction::Allow;
        state.footer = FooterSample::Detail;

        apply_modal_setting_to_snapshot(ModalSettingAction::SizeLg, &mut state);

        assert_eq!(state.size, ModalSize::Lg);
        assert!(!state.dismiss_on_esc);
        assert_eq!(state.parent_interaction, ModalParentInteraction::Allow);
        assert_eq!(state.footer, FooterSample::Detail);
    }

    #[test]
    fn esc_setting_preserves_size_parent_and_footer() {
        let mut state = snapshot();
        apply_modal_setting_to_snapshot(ModalSettingAction::SizeCustom, &mut state);
        apply_modal_setting_to_snapshot(ModalSettingAction::FooterForm, &mut state);
        apply_modal_setting_to_snapshot(ModalSettingAction::ParentAllow, &mut state);

        apply_modal_setting_to_snapshot(ModalSettingAction::EscDisabled, &mut state);

        assert!(matches!(state.size, ModalSize::Custom(560.0)));
        assert_eq!(state.footer, FooterSample::Form);
        assert_eq!(state.parent_interaction, ModalParentInteraction::Allow);
        assert!(!state.dismiss_on_esc);
    }

    #[test]
    fn footer_setting_preserves_size_and_behavior() {
        let mut state = snapshot();
        apply_modal_setting_to_snapshot(ModalSettingAction::SizeSm, &mut state);
        apply_modal_setting_to_snapshot(ModalSettingAction::EscDisabled, &mut state);

        apply_modal_setting_to_snapshot(ModalSettingAction::FooterDetail, &mut state);

        assert_eq!(state.size, ModalSize::Sm);
        assert!(!state.dismiss_on_esc);
        assert_eq!(state.footer, FooterSample::Detail);
    }

    #[test]
    fn active_setting_reflects_current_state() {
        let mut state = snapshot();
        apply_modal_setting_to_snapshot(ModalSettingAction::ParentAllow, &mut state);

        assert!(is_modal_setting_active_in_snapshot(
            ModalSettingAction::ParentAllow,
            &state
        ));
        assert!(!is_modal_setting_active_in_snapshot(
            ModalSettingAction::ParentBlock,
            &state
        ));
    }

    #[test]
    fn open_snapshot_contains_selected_footer_body() {
        let mut state = snapshot();
        apply_modal_setting_to_snapshot(ModalSettingAction::FooterForm, &mut state);
        let opened = ModalOpenSnapshot {
            size: state.size,
            title: state.title,
            body: state.body,
            footer: state.footer,
            footer_body: state.footer.body().to_string(),
            dismiss_on_backdrop: state.dismiss_on_backdrop,
            dismiss_on_esc: state.dismiss_on_esc,
            parent_interaction: state.parent_interaction,
        };

        assert!(modal_open_matches_action(
            ModalSettingAction::FooterForm,
            &opened
        ));
    }

    #[test]
    fn open_snapshot_rejects_wrong_footer_body() {
        let mut state = snapshot();
        apply_modal_setting_to_snapshot(ModalSettingAction::FooterDetail, &mut state);
        let opened = ModalOpenSnapshot {
            size: state.size,
            title: state.title,
            body: state.body,
            footer: state.footer,
            footer_body: FooterSample::Confirm.body().to_string(),
            dismiss_on_backdrop: state.dismiss_on_backdrop,
            dismiss_on_esc: state.dismiss_on_esc,
            parent_interaction: state.parent_interaction,
        };

        assert!(!modal_open_matches_action(
            ModalSettingAction::FooterDetail,
            &opened
        ));
    }
}
