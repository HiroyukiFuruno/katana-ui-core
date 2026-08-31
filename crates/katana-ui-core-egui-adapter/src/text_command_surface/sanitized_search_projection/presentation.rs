use super::types::SanitizedSearchProjectionBuildError;
use sha2::{Digest, Sha256};

fn hash_text(hasher: &mut Sha256, value: &str) {
    hasher.update(value.len().to_le_bytes());
    hasher.update(value.as_bytes());
}

#[derive(Clone, PartialEq, Eq)]
pub struct SanitizedSearchTextPresentation {
    pub(crate) visible: String,
    pub(crate) tooltip: String,
    pub(crate) accessibility_label: String,
}

impl SanitizedSearchTextPresentation {
    #[must_use]
    pub fn new(
        visible: impl Into<String>,
        tooltip: impl Into<String>,
        accessibility_label: impl Into<String>,
    ) -> Self {
        Self {
            visible: visible.into(),
            tooltip: tooltip.into(),
            accessibility_label: accessibility_label.into(),
        }
    }

    pub(crate) fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        if self.visible.is_empty() || self.tooltip.is_empty() || self.accessibility_label.is_empty()
        {
            return Err(SanitizedSearchProjectionBuildError::EmptyPresentationText);
        }
        Ok(())
    }

    pub(crate) fn hash_into(&self, hasher: &mut Sha256) {
        hash_text(hasher, &self.visible);
        hash_text(hasher, &self.tooltip);
        hash_text(hasher, &self.accessibility_label);
    }
}

impl std::fmt::Debug for SanitizedSearchTextPresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SanitizedSearchResultSummaryPresentation {
    pub(crate) empty: String,
    pub(crate) zero_results: String,
    pub(crate) single_result: String,
    pub(crate) indexed_result: String,
    pub(crate) count_results: String,
}

impl SanitizedSearchResultSummaryPresentation {
    #[must_use]
    pub fn new(
        empty: impl Into<String>,
        zero_results: impl Into<String>,
        single_result: impl Into<String>,
        indexed_result: impl Into<String>,
        count_results: impl Into<String>,
    ) -> Self {
        Self {
            empty: empty.into(),
            zero_results: zero_results.into(),
            single_result: single_result.into(),
            indexed_result: indexed_result.into(),
            count_results: count_results.into(),
        }
    }

    pub(crate) fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        if self.empty.is_empty()
            || self.zero_results.is_empty()
            || self.single_result.is_empty()
            || self.indexed_result.is_empty()
            || self.count_results.is_empty()
        {
            return Err(SanitizedSearchProjectionBuildError::EmptyPresentationText);
        }
        Ok(())
    }

    pub(crate) fn hash_into(&self, hasher: &mut Sha256) {
        hash_text(hasher, &self.empty);
        hash_text(hasher, &self.zero_results);
        hash_text(hasher, &self.single_result);
        hash_text(hasher, &self.indexed_result);
        hash_text(hasher, &self.count_results);
    }
}

impl std::fmt::Debug for SanitizedSearchResultSummaryPresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SanitizedSearchControlPresentation {
    pub(crate) strip: SanitizedSearchTextPresentation,
    pub(crate) query: SanitizedSearchTextPresentation,
    pub(crate) replacement: SanitizedSearchTextPresentation,
    pub(crate) match_case: SanitizedSearchTextPresentation,
    pub(crate) whole_word: SanitizedSearchTextPresentation,
    pub(crate) regex: SanitizedSearchTextPresentation,
}

impl SanitizedSearchControlPresentation {
    #[must_use]
    pub fn new(
        strip: SanitizedSearchTextPresentation,
        query: SanitizedSearchTextPresentation,
        replacement: SanitizedSearchTextPresentation,
        match_case: SanitizedSearchTextPresentation,
        whole_word: SanitizedSearchTextPresentation,
        regex: SanitizedSearchTextPresentation,
    ) -> Self {
        Self {
            strip,
            query,
            replacement,
            match_case,
            whole_word,
            regex,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        self.strip.validate()?;
        self.query.validate()?;
        self.replacement.validate()?;
        self.match_case.validate()?;
        self.whole_word.validate()?;
        self.regex.validate()?;
        Ok(())
    }

    pub(crate) fn hash_into(&self, hasher: &mut Sha256) {
        self.strip.hash_into(hasher);
        self.query.hash_into(hasher);
        self.replacement.hash_into(hasher);
        self.match_case.hash_into(hasher);
        self.whole_word.hash_into(hasher);
        self.regex.hash_into(hasher);
    }
}

impl std::fmt::Debug for SanitizedSearchControlPresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SanitizedSearchOperationPresentation {
    pub(crate) previous: SanitizedSearchTextPresentation,
    pub(crate) next: SanitizedSearchTextPresentation,
    pub(crate) replace: SanitizedSearchTextPresentation,
    pub(crate) replace_all: SanitizedSearchTextPresentation,
    pub(crate) close: SanitizedSearchTextPresentation,
}

impl SanitizedSearchOperationPresentation {
    #[must_use]
    pub fn new(
        previous: SanitizedSearchTextPresentation,
        next: SanitizedSearchTextPresentation,
        replace: SanitizedSearchTextPresentation,
        replace_all: SanitizedSearchTextPresentation,
        close: SanitizedSearchTextPresentation,
    ) -> Self {
        Self {
            previous,
            next,
            replace,
            replace_all,
            close,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        self.previous.validate()?;
        self.next.validate()?;
        self.replace.validate()?;
        self.replace_all.validate()?;
        self.close.validate()?;
        Ok(())
    }

    pub(crate) fn hash_into(&self, hasher: &mut Sha256) {
        self.previous.hash_into(hasher);
        self.next.hash_into(hasher);
        self.replace.hash_into(hasher);
        self.replace_all.hash_into(hasher);
        self.close.hash_into(hasher);
    }
}

impl std::fmt::Debug for SanitizedSearchOperationPresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

mod unavailable;
pub use unavailable::*;
