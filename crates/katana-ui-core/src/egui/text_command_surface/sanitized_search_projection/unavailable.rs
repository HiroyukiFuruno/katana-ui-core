use super::super::types::SanitizedSearchProjectionBuildError;
use super::{
    SanitizedSearchControlPresentation, SanitizedSearchOperationPresentation,
    SanitizedSearchResultSummaryPresentation,
};
use sha2::{Digest, Sha256};

fn hash_text(hasher: &mut Sha256, value: &str) {
    hasher.update(value.len().to_le_bytes());
    hasher.update(value.as_bytes());
}

#[derive(Clone, PartialEq, Eq)]
pub struct SanitizedSearchUnavailablePresentation {
    pub(crate) regex: String,
    pub(crate) replace: String,
    pub(crate) navigation: String,
    pub(crate) close: String,
}

impl SanitizedSearchUnavailablePresentation {
    #[must_use]
    pub fn new(
        regex: impl Into<String>,
        replace: impl Into<String>,
        navigation: impl Into<String>,
        close: impl Into<String>,
    ) -> Self {
        Self {
            regex: regex.into(),
            replace: replace.into(),
            navigation: navigation.into(),
            close: close.into(),
        }
    }

    pub(crate) fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        if self.regex.is_empty()
            || self.replace.is_empty()
            || self.navigation.is_empty()
            || self.close.is_empty()
        {
            return Err(SanitizedSearchProjectionBuildError::EmptyPresentationText);
        }
        Ok(())
    }

    pub(crate) fn hash_into(&self, hasher: &mut Sha256) {
        hash_text(hasher, &self.regex);
        hash_text(hasher, &self.replace);
        hash_text(hasher, &self.navigation);
        hash_text(hasher, &self.close);
    }
}

impl std::fmt::Debug for SanitizedSearchUnavailablePresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SanitizedSearchLocalizedPresentation {
    pub(crate) controls: SanitizedSearchControlPresentation,
    pub(crate) operations: SanitizedSearchOperationPresentation,
    pub(crate) result_summary: SanitizedSearchResultSummaryPresentation,
    pub(crate) unavailable: SanitizedSearchUnavailablePresentation,
}

impl SanitizedSearchLocalizedPresentation {
    #[must_use]
    pub fn new(
        controls: SanitizedSearchControlPresentation,
        operations: SanitizedSearchOperationPresentation,
        result_summary: SanitizedSearchResultSummaryPresentation,
        unavailable: SanitizedSearchUnavailablePresentation,
    ) -> Self {
        Self {
            controls,
            operations,
            result_summary,
            unavailable,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        self.controls.validate()?;
        self.operations.validate()?;
        self.result_summary.validate()?;
        self.unavailable.validate()?;
        Ok(())
    }

    pub(crate) fn hash_into(&self, hasher: &mut Sha256) {
        self.controls.hash_into(hasher);
        self.operations.hash_into(hasher);
        self.result_summary.hash_into(hasher);
        self.unavailable.hash_into(hasher);
    }
}

impl std::fmt::Debug for SanitizedSearchLocalizedPresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}
