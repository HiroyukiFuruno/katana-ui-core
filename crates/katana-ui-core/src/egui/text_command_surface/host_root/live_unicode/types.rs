use super::{
    KucAccessKitNodeObservation, KucBounds, KucCaretObservation, KucHitTestObservation,
    KucRgbaCropEvidence,
};

/// 同一の実root/frameから取得した、変更できないUnicode観測値。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucLiveUnicodeObservation {
    pub(super) final_text: String,
    pub(super) observed_range: (usize, usize),
    pub(super) scalars: Vec<u32>,
    pub(super) measurement: KucBounds,
    pub(super) caret: KucCaretObservation,
    pub(super) hit_test: KucHitTestObservation,
    pub(super) crop: KucRgbaCropEvidence,
    pub(super) accesskit: Option<KucAccessKitNodeObservation>,
    pub(super) accesskit_node_id: Option<u64>,
    pub(super) root_id: String,
    pub(super) frame_id: String,
    pub(super) root_frame_hash: String,
    pub(super) root_rgba_hash: String,
    pub(super) root_record_hash: String,
    pub(super) font_identity: (Option<String>, String),
    pub(super) frame_serial: u64,
    pub(super) state_revision: u64,
    pub(super) event_correlation: String,
    pub(super) canvas: crate::render_model::UiRect,
}
