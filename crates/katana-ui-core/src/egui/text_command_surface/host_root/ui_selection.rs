#[path = "ui_selection/availability.rs"]
mod availability;
use availability::SelectionAvailability;

use super::host_root_process::HostRootProcess;
use super::{EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceHostRootFrame};
use crate::text_surface::TextSurfaceUiSelection;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// 不透明なUI observationを取得したinput境界。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucUiSelectionPhase {
    BeforeInput,
    AfterInput,
}

/// availabilityは保存したUI stateの等価性とは独立している。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucUiSelectionAvailability {
    FocusedEditable,
    Unfocused,
    Readonly,
    Disabled,
    Hidden,
    Composing,
    Provisional,
}

/// 再利用可能な不透明UI state。document内容とroot bindingはvalidation metadataである。
/// 等価性では方向、wrap affinity、縦移動で希望するcolumnを比較する。
#[derive(Clone)]
pub struct KucUiSelectionSnapshot {
    commit: super::selection_commit::SelectionCommit,
    root: Arc<()>,
    document: Arc<str>,
    document_identity: crate::render_model::UiStateId,
    state: TextSurfaceUiSelection,
}

impl PartialEq for KucUiSelectionSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Eq for KucUiSelectionSnapshot {}

impl std::fmt::Debug for KucUiSelectionSnapshot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucUiSelectionSnapshot(..)")
    }
}

impl KucUiSelectionSnapshot {
    /// 新しいone-shot requestを作成する。同じ保存snapshotは引き続き再利用できる。
    #[must_use]
    pub fn restore_request(&self) -> KucUiSelectionRestoreRequest {
        KucUiSelectionRestoreRequest {
            snapshot: self.clone(),
            applied: AtomicBool::new(false),
        }
    }
}

#[derive(Debug)]
pub struct KucUiSelectionRestoreRequest {
    snapshot: KucUiSelectionSnapshot,
    applied: AtomicBool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucUiSelectionRestoreError {
    DifferentRoot,
    DifferentDocument,
    DuplicateRequest,
    Unavailable(KucUiSelectionAvailability),
}

impl std::fmt::Display for KucUiSelectionRestoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "UI selection restore rejected: {self:?}")
    }
}

impl std::error::Error for KucUiSelectionRestoreError {}

/// 一つのobservation。provisionalまたはdiscardされたpassは再利用可能なsnapshotを生成しない。
#[derive(Debug, Clone)]
pub struct KucUiSelectionObservation {
    frame_number: u64,
    phase: KucUiSelectionPhase,
    availability: KucUiSelectionAvailability,
    snapshot: Option<KucUiSelectionSnapshot>,
}

impl KucUiSelectionObservation {
    #[must_use]
    pub const fn frame_number(&self) -> u64 {
        self.frame_number
    }

    #[must_use]
    pub const fn phase(&self) -> KucUiSelectionPhase {
        self.phase
    }

    #[must_use]
    pub fn availability(&self) -> KucUiSelectionAvailability {
        if self
            .snapshot
            .as_ref()
            .is_some_and(|snapshot| !snapshot.commit.is_committed())
        {
            KucUiSelectionAvailability::Provisional
        } else {
            self.availability
        }
    }

    #[must_use]
    pub fn snapshot(&self) -> Option<&KucUiSelectionSnapshot> {
        self.snapshot
            .as_ref()
            .filter(|snapshot| snapshot.commit.is_committed())
    }

    pub(super) fn finalize_after_pass(&mut self, context: &egui::Context) {
        self.frame_number = context.cumulative_frame_nr().saturating_add(1);
        if let Some(snapshot) = &self.snapshot {
            snapshot.commit.watch(context);
        }
    }
}

impl HostRootProcess {
    pub(super) fn selection_observation(
        &self,
        phase: KucUiSelectionPhase,
        visible: bool,
        provisional: bool,
    ) -> KucUiSelectionObservation {
        let surface = self.root.selection_surface();
        let state = &surface.state().text_area;
        let availability = SelectionAvailability::resolve(state, visible, provisional);
        let snapshot = (availability == KucUiSelectionAvailability::FocusedEditable).then(|| {
            KucUiSelectionSnapshot {
                commit: super::selection_commit::SelectionCommit::committed(),
                root: Arc::clone(&self.selection_identity),
                document: Arc::from(state.value.as_str()),
                document_identity: state.state_id.clone(),
                state: surface.ui_selection(),
            }
        });
        KucUiSelectionObservation {
            frame_number: self
                .selection_logical_frame
                .saturating_add(u64::from(phase == KucUiSelectionPhase::BeforeInput)),
            phase,
            availability,
            snapshot,
        }
    }

    fn restore_selection(
        &mut self,
        request: &KucUiSelectionRestoreRequest,
    ) -> Result<(), KucUiSelectionRestoreError> {
        if !Arc::ptr_eq(&request.snapshot.root, &self.selection_identity) {
            return Err(KucUiSelectionRestoreError::DifferentRoot);
        }
        if !request.snapshot.commit.is_committed() {
            return Err(KucUiSelectionRestoreError::Unavailable(
                KucUiSelectionAvailability::Provisional,
            ));
        }
        let state = &self.root.selection_surface().state().text_area;
        if request.snapshot.document.as_ref() != state.value
            || request.snapshot.document_identity != state.state_id
        {
            return Err(KucUiSelectionRestoreError::DifferentDocument);
        }
        let current = SelectionAvailability::resolve(state, self.selection_visible, false);
        if current != KucUiSelectionAvailability::FocusedEditable {
            return Err(KucUiSelectionRestoreError::Unavailable(current));
        }
        if request.applied.swap(true, Ordering::AcqRel) {
            return Err(KucUiSelectionRestoreError::DuplicateRequest);
        }
        self.root.restore_selection_surface(&request.snapshot.state);
        Ok(())
    }
}

impl EguiTextCommandSurfaceHostRoot {
    /// document projectionを変えずに、KUC所有の汎用dialogを同期する。
    pub fn synchronize_selection_dialog(
        &mut self,
        lease: super::super::SelectionDialogProjectionLease,
    ) -> Result<bool, super::super::SelectionDialogProjectionError> {
        self.process.root.synchronize_selection_dialog(lease)
    }

    /// 次のframeのinputを処理する前にretained stateを観測する。
    #[must_use]
    pub fn ui_selection_before_input(&self) -> KucUiSelectionObservation {
        self.process.selection_observation(
            KucUiSelectionPhase::BeforeInput,
            self.process.selection_visible,
            false,
        )
    }

    /// UI selectionだけを復元する。hostは先にdocument transactionを復元しなければならない。
    /// activeなIME compositionは保存もcancelもせず拒否する。
    pub fn restore_ui_selection_once(
        &mut self,
        request: &KucUiSelectionRestoreRequest,
    ) -> Result<(), KucUiSelectionRestoreError> {
        self.process.restore_selection(request)
    }
}

impl EguiTextCommandSurfaceHostRootFrame {
    #[must_use]
    pub const fn ui_selection_after_input(&self) -> &KucUiSelectionObservation {
        &self.selection_observation
    }
}
