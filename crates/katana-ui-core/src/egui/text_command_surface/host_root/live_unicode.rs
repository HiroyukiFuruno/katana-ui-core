#[path = "live_unicode/capture.rs"]
mod capture;
#[cfg(test)]
#[path = "live_unicode_tests.rs"]
mod live_unicode_tests;
#[cfg(feature = "native-ime-evidence")]
#[path = "live_unicode/native.rs"]
mod native;
#[path = "live_unicode/types.rs"]
mod types;

use super::super::unicode_evidence::{
    KucAccessKitNodeObservation, KucBounds, KucCaretObservation, KucHitTestObservation,
    KucRgbaCropEvidence, KucUnicodeColorGlyphEvidenceError,
};
pub use types::KucLiveUnicodeObservation;

impl KucLiveUnicodeObservation {
    #[must_use]
    pub fn final_text(&self) -> &str {
        &self.final_text
    }
    #[must_use]
    pub const fn observed_range(&self) -> (usize, usize) {
        self.observed_range
    }
    #[must_use]
    pub fn scalars(&self) -> &[u32] {
        &self.scalars
    }
    #[must_use]
    pub const fn measurement(&self) -> KucBounds {
        self.measurement
    }
    #[must_use]
    pub const fn caret(&self) -> KucCaretObservation {
        self.caret
    }
    #[must_use]
    pub const fn hit_test(&self) -> &KucHitTestObservation {
        &self.hit_test
    }
    #[must_use]
    pub const fn crop(&self) -> &KucRgbaCropEvidence {
        &self.crop
    }
    #[must_use]
    pub const fn accesskit(&self) -> Option<&KucAccessKitNodeObservation> {
        self.accesskit.as_ref()
    }
    #[must_use]
    pub fn root_id(&self) -> &str {
        &self.root_id
    }
    #[must_use]
    pub fn frame_id(&self) -> &str {
        &self.frame_id
    }
    #[must_use]
    pub fn root_frame_hash(&self) -> &str {
        &self.root_frame_hash
    }
    #[must_use]
    pub fn root_rgba_hash(&self) -> &str {
        &self.root_rgba_hash
    }
    #[must_use]
    pub fn root_record_hash(&self) -> &str {
        &self.root_record_hash
    }

    /// 指定したroot/frame/artifactとの対応を検証する。観測値は変更しない。
    pub fn validate_binding(
        &self,
        root_id: &str,
        frame_id: &str,
        record_hash: &str,
        rgba_hash: &str,
    ) -> Result<(), KucUnicodeColorGlyphEvidenceError> {
        if self.root_id != root_id {
            return Err(trace("Unicode root mismatch"));
        }
        if self.frame_id != frame_id {
            return Err(trace("Unicode frame mismatch"));
        }
        if self.root_record_hash != record_hash || self.root_rgba_hash != rgba_hash {
            return Err(trace("Unicode artifact mismatch"));
        }
        Ok(())
    }
}

fn trace(message: impl std::fmt::Display) -> KucUnicodeColorGlyphEvidenceError {
    KucUnicodeColorGlyphEvidenceError::RootTrace(message.to_string())
}
