use super::operation::SanitizedSearchOperation;
use super::presentation::SanitizedSearchLocalizedPresentation;

pub struct SanitizedSearchProjectionBuilder {
    pub(crate) query: SanitizedSearchOperation,
    pub(crate) replacement: SanitizedSearchOperation,
    pub(crate) match_case: SanitizedSearchOperation,
    pub(crate) whole_word: SanitizedSearchOperation,
    pub(crate) regex: SanitizedSearchOperation,
    pub(crate) close: SanitizedSearchOperation,
    pub(crate) next: SanitizedSearchOperation,
    pub(crate) previous: SanitizedSearchOperation,
    pub(crate) replace: SanitizedSearchOperation,
    pub(crate) replace_all: SanitizedSearchOperation,
    pub(crate) presentation: Option<SanitizedSearchLocalizedPresentation>,
}
