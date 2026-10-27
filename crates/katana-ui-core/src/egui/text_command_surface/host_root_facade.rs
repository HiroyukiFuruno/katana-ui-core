#[cfg(any(test, feature = "storybook-artifacts"))]
use super::super::root::EguiTextCommandSurfaceRootOutput;
use super::super::root::{
    EguiTextCommandSurfaceRootEventBatchForwardError,
    EguiTextCommandSurfaceRootEventForwardingReceipt, KucRootEventBatchForwarder,
};
use super::host_root_record::EguiTextCommandSurfaceHostRootRecord;
use super::{
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfacePresentationToken,
    EguiTextCommandSurfaceRootFactoryError,
};

impl EguiTextCommandSurfaceHostRoot {
    #[cfg(test)]
    pub(crate) fn show_output_for_test(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootFactoryError> {
        self.process.show(ui)
    }

    /// 子モデルやレイアウト情報を公開せず、新しい不透明トークンを適用する。
    pub fn synchronize(
        &mut self,
        token: EguiTextCommandSurfacePresentationToken,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        self.process.synchronize(
            token.revision,
            super::host_root_token_codec::decode_token(&token)?,
        )
    }

    /// 新しいトークンを適用し、非公開のhost効果ルーターを置き換える。
    pub fn synchronize_with_lease(
        &mut self,
        lease: EguiTextCommandSurfaceHostProjectionLease,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        let (token, attachments) = lease.into_parts_with_attachments();
        self.process.synchronize_with_router(
            token.revision,
            super::host_root_token_codec::decode_token(&token)?,
            attachments,
        )
    }

    /// 先に発行したleaseを進め、これまでにdispatchしたイベントによる状態を保持する。
    #[cfg(any(test, feature = "storybook-artifacts"))]
    pub(crate) fn synchronize_with_lease_preserving_state(
        &mut self,
        lease: EguiTextCommandSurfaceHostProjectionLease,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        let (token, attachments) = lease.into_parts_with_attachments();
        self.process.synchronize_router_preserving_state(
            token.revision,
            super::host_root_token_codec::decode_token(&token)?,
            attachments,
        )
    }

    /// このrootと確定済み最新frameを照合し、全イベントを一度だけ配送する。
    pub fn forward_global_events_once<F>(
        &self,
        frame: &EguiTextCommandSurfaceHostRootFrame,
        forwarder: &mut F,
    ) -> Result<
        EguiTextCommandSurfaceRootEventForwardingReceipt,
        EguiTextCommandSurfaceRootEventBatchForwardError<F::Error>,
    >
    where
        F: KucRootEventBatchForwarder,
    {
        let valid = frame.global_generation.upgrade().is_some_and(|generation| {
            std::rc::Rc::ptr_eq(&generation, &self.process.root.global_generation)
                && generation.get() == frame.global_epoch
        }) && frame.global_commit.is_committed();
        if !valid {
            return Err(
                EguiTextCommandSurfaceRootEventBatchForwardError::GlobalAuthorizationFailed,
            );
        }
        let commit = frame.global_commit.clone();
        if !frame.output.events().authorize_global(
            frame.global_generation.clone(),
            frame.global_epoch,
            std::rc::Rc::new(move || commit.is_committed()),
        ) {
            return Err(EguiTextCommandSurfaceRootEventBatchForwardError::AlreadyConsumed);
        }
        frame.output.events().forward_once(forwarder)
    }

    /// retained root全体を一度表示し、閉じたfacade frameだけを返す。
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfaceRootFactoryError> {
        let output = self.process.show(ui)?;
        let record = EguiTextCommandSurfaceHostRootRecord::from_output(
            self.process.identity(),
            self.process.presentation_revision(),
            &output,
        );
        let mut selection_observation = self.process.selection_observation(
            super::ui_selection::KucUiSelectionPhase::AfterInput,
            self.process.selection_visible,
            ui.ctx().will_discard(),
        );
        selection_observation.finalize_after_pass(ui.ctx());
        let global_commit = super::selection_commit::SelectionCommit::committed();
        if output.events().has_global_events() {
            global_commit.watch(ui.ctx());
        }
        Ok(EguiTextCommandSurfaceHostRootFrame {
            global_generation: std::rc::Rc::downgrade(&self.process.root.global_generation),
            global_epoch: self.process.root.global_generation.get(),
            global_commit,
            font_identity: self.process.unicode_font_identity(),
            output,
            record,
            selection_observation,
            frame_serial: self.process.selection_frame_serial(),
        })
    }
}

impl EguiTextCommandSurfaceHostRootFrame {
    pub fn observe_unicode(
        &self,
        range: (usize, usize),
    ) -> Result<
        super::live_unicode::KucLiveUnicodeObservation,
        crate::egui::text_command_surface::unicode_evidence::KucUnicodeColorGlyphEvidenceError,
    > {
        super::live_unicode::KucLiveUnicodeObservation::observe(
            &self.output,
            range,
            self.frame_serial,
            &self.font_identity,
        )
    }

    #[cfg(feature = "storybook-artifacts")]
    pub(crate) const fn artifact_output(&self) -> &EguiTextCommandSurfaceRootOutput {
        &self.output
    }

    #[must_use]
    pub const fn record(&self) -> &EguiTextCommandSurfaceHostRootRecord {
        &self.record
    }

    /// 現在のKUC所有の汎用interaction locatorを返す。
    #[must_use]
    pub const fn interaction_locator(&self) -> &super::super::root::KucInteractionLocator {
        self.output.interaction_locator()
    }

    #[cfg(any(test, feature = "storybook-artifacts"))]
    pub(crate) const fn text_scroll_y(&self) -> i32 {
        self.output.evidence_text.record.frame.viewport.scroll_y
    }

    #[cfg(any(test, feature = "storybook-artifacts"))]
    pub(crate) fn contains_command_activation(
        &self,
        action_identity: &str,
        floating: bool,
    ) -> bool {
        self.output
            .events()
            .contains_command_activation(action_identity, floating)
    }

    #[cfg(any(test, feature = "storybook-artifacts"))]
    pub(crate) const fn context_menu_is_visible(&self) -> bool {
        self.output.context_menu_is_visible()
    }

    #[cfg(any(test, feature = "storybook-artifacts"))]
    pub(crate) fn contains_context_menu_opened(&self) -> bool {
        self.output.events().contains_context_menu_opened()
    }

    /// 閉じたイベントtransportを一度だけforwardする。
    pub fn forward_events_once<Forwarder>(
        &self,
        forwarder: &mut Forwarder,
    ) -> Result<
        EguiTextCommandSurfaceRootEventForwardingReceipt,
        EguiTextCommandSurfaceRootEventBatchForwardError<Forwarder::Error>,
    >
    where
        Forwarder: KucRootEventBatchForwarder,
    {
        if self.output.events().has_global_events() {
            return Err(
                EguiTextCommandSurfaceRootEventBatchForwardError::GlobalAuthorizationRequired,
            );
        }
        self.output.events().forward_once(forwarder)
    }
}

impl std::fmt::Debug for EguiTextCommandSurfaceHostRootFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EguiTextCommandSurfaceHostRootFrame")
            .field("record", &self.record)
            .finish_non_exhaustive()
    }
}
