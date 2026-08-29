mod capture;
mod constants;
mod crop_observation;
mod model;
mod runner;
mod surface;
mod types;
mod validation;

pub use capture::KucUnicodeColorGlyphEvidenceCapture;
pub use constants::{
    CONTROL_STAR_TEXT, IME_COMMIT_TEXT, IME_PREEDIT_TEXT, STAR_TEXT, UNICODE_EVIDENCE_SCHEMA,
    UNICODE_EVIDENCE_SCHEMA_VERSION, ZWJ_TEXT,
};
pub use model::{
    KucAccessKitNodeObservation, KucBounds, KucCaretObservation, KucHitTestObservation,
    KucImeTraceEvidence, KucRgbaCropEvidence, KucUnicodeColorGlyphEvidence,
    KucUnicodeColorGlyphEvidenceInput, KucUnicodeColorGlyphEvidenceOptions,
    KucUnicodeColorGlyphEvidenceProfile,
};
pub use types::KucUnicodeColorGlyphEvidenceError;
pub use validation::KucUnicodeColorGlyphEvidenceBuilder;
