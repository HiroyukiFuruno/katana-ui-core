use super::model::SourceAddressStrip;

/// Source Address 制御が扱う汎用操作。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceAddressAction {
    SetDraft(String),
    SetEnabled(bool),
    SetFocused(bool),
    OpenHistory,
    CloseHistory,
    OpenCandidates,
    CloseCandidates,
    SelectHistory(usize),
    SelectCandidate(usize),
    Submit,
}

/// opaque target を含めず、表示上の汎用イベントだけを公開する。
pub enum SourceAddressEvent {
    DraftChanged,
    EnabledChanged,
    Focused,
    Blurred,
    HistoryOpened,
    HistoryClosed,
    CandidatesOpened,
    CandidatesClosed,
    HistorySelected,
    CandidateSelected,
    Submitted(SourceAddressSubmission),
}

/// 複製・検査・整形・シリアライズを許さない一回限りの draft 提出値。
pub struct SourceAddressSubmission(String);

impl SourceAddressSubmission {
    pub(super) fn new(draft: String) -> Self {
        Self(draft)
    }

    /// 提出値を消費して host 境界へ draft を渡す。
    #[must_use]
    pub fn into_draft(self) -> String {
        self.0
    }
}

pub(super) fn apply(
    strip: &mut SourceAddressStrip,
    action: SourceAddressAction,
) -> Option<SourceAddressEvent> {
    match action {
        SourceAddressAction::SetEnabled(enabled) => strip.set_enabled(enabled),
        SourceAddressAction::SetFocused(focused) if strip.enabled() => strip.set_focused(focused),
        SourceAddressAction::SetDraft(draft) if strip.enabled() => strip.set_draft(draft),
        SourceAddressAction::OpenHistory if strip.enabled() => strip.open_history(),
        SourceAddressAction::CloseHistory => strip.close_history(),
        SourceAddressAction::OpenCandidates if strip.enabled() => strip.open_candidates(),
        SourceAddressAction::CloseCandidates => strip.close_candidates(),
        SourceAddressAction::SelectHistory(index) if strip.enabled() => strip.select_history(index),
        SourceAddressAction::SelectCandidate(index) if strip.enabled() => {
            strip.select_candidate(index)
        }
        SourceAddressAction::Submit if strip.enabled() => Some(SourceAddressEvent::Submitted(
            SourceAddressSubmission::new(strip.draft().to_owned()),
        )),
        _ => None,
    }
}

#[cfg(test)]
#[path = "actions_tests.rs"]
mod tests;
