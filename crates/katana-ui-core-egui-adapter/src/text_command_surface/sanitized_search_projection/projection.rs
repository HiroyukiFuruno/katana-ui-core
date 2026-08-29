use super::operation::SanitizedSearchOperation;
use super::presentation::SanitizedSearchLocalizedPresentation;
use super::types::SEARCH_SIGNATURE_BYTES;
use sha2::{Digest, Sha256};

pub struct SanitizedSearchProjection {
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
    pub(crate) presentation: SanitizedSearchLocalizedPresentation,
}

impl SanitizedSearchProjection {
    #[cfg(test)]
    pub(crate) fn same_as(&self, other: &Self) -> bool {
        self.stable_fingerprint() == other.stable_fingerprint()
    }

    pub(crate) fn stable_fingerprint(&self) -> [u8; SEARCH_SIGNATURE_BYTES] {
        let mut hasher = Sha256::new();
        self.query.hash_into(&mut hasher);
        self.replacement.hash_into(&mut hasher);
        self.match_case.hash_into(&mut hasher);
        self.whole_word.hash_into(&mut hasher);
        self.regex.hash_into(&mut hasher);
        self.close.hash_into(&mut hasher);
        self.next.hash_into(&mut hasher);
        self.previous.hash_into(&mut hasher);
        self.replace.hash_into(&mut hasher);
        self.replace_all.hash_into(&mut hasher);
        self.presentation.hash_into(&mut hasher);
        hasher.finalize().into()
    }
}

impl std::fmt::Debug for SanitizedSearchProjection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}
