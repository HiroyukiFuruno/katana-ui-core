use super::builder_types::SanitizedSearchProjectionBuilder;
use super::operation::SanitizedSearchOperation;
use super::presentation::SanitizedSearchLocalizedPresentation;
use super::projection::SanitizedSearchProjection;
use super::types::{
    SanitizedSearchOperationSlot, SanitizedSearchProjectionBuildError, SanitizedSearchTarget,
};

impl SanitizedSearchProjectionBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            query: SanitizedSearchOperation::new(),
            replacement: SanitizedSearchOperation::new(),
            match_case: SanitizedSearchOperation::new(),
            whole_word: SanitizedSearchOperation::new(),
            regex: SanitizedSearchOperation::new(),
            close: SanitizedSearchOperation::new(),
            next: SanitizedSearchOperation::new(),
            previous: SanitizedSearchOperation::new(),
            replace: SanitizedSearchOperation::new(),
            replace_all: SanitizedSearchOperation::new(),
            presentation: None,
        }
    }

    #[must_use]
    pub fn localized_presentation(
        mut self,
        presentation: SanitizedSearchLocalizedPresentation,
    ) -> Self {
        self.presentation = Some(presentation);
        self
    }

    #[must_use]
    pub fn query_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.query.enabled = true;
        self.query.target = Some(target);
        self
    }

    #[must_use]
    pub fn replacement_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.replacement.enabled = true;
        self.replacement.target = Some(target);
        self
    }

    #[must_use]
    pub fn match_case_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.match_case.enabled = true;
        self.match_case.target = Some(target);
        self
    }

    #[must_use]
    pub const fn match_case_state(mut self, value: bool) -> Self {
        self.match_case.current = value;
        self
    }

    #[must_use]
    pub fn whole_word_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.whole_word.enabled = true;
        self.whole_word.target = Some(target);
        self
    }

    #[must_use]
    pub const fn whole_word_state(mut self, value: bool) -> Self {
        self.whole_word.current = value;
        self
    }

    #[must_use]
    pub fn regex_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.regex.enabled = true;
        self.regex.target = Some(target);
        self
    }

    #[must_use]
    pub const fn regex_state(mut self, value: bool) -> Self {
        self.regex.current = value;
        self
    }

    #[must_use]
    pub const fn close_enabled(mut self, enabled: bool) -> Self {
        self.close.enabled = enabled;
        self
    }

    #[must_use]
    pub fn close_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.close.target = Some(target);
        self
    }

    #[must_use]
    pub const fn next_enabled(mut self, enabled: bool) -> Self {
        self.next.enabled = enabled;
        self
    }

    #[must_use]
    pub fn next_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.next.target = Some(target);
        self
    }

    #[must_use]
    pub const fn previous_enabled(mut self, enabled: bool) -> Self {
        self.previous.enabled = enabled;
        self
    }

    #[must_use]
    pub fn previous_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.previous.target = Some(target);
        self
    }

    #[must_use]
    pub const fn replace_enabled(mut self, enabled: bool) -> Self {
        self.replace.enabled = enabled;
        self
    }

    #[must_use]
    pub fn replace_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.replace.target = Some(target);
        self
    }

    #[must_use]
    pub const fn replace_all_enabled(mut self, enabled: bool) -> Self {
        self.replace_all.enabled = enabled;
        self
    }

    #[must_use]
    pub fn replace_all_target(mut self, target: SanitizedSearchTarget) -> Self {
        self.replace_all.target = Some(target);
        self
    }

    pub fn build(self) -> Result<SanitizedSearchProjection, SanitizedSearchProjectionBuildError> {
        let presentation = self
            .presentation
            .ok_or(SanitizedSearchProjectionBuildError::MissingPresentation)?;
        presentation.validate()?;
        self.query.validate(SanitizedSearchOperationSlot::Query)?;
        self.replacement
            .validate(SanitizedSearchOperationSlot::Replacement)?;
        self.match_case
            .validate(SanitizedSearchOperationSlot::MatchCase)?;
        self.whole_word
            .validate(SanitizedSearchOperationSlot::WholeWord)?;
        self.regex.validate(SanitizedSearchOperationSlot::Regex)?;
        self.close.validate(SanitizedSearchOperationSlot::Close)?;
        self.next.validate(SanitizedSearchOperationSlot::Next)?;
        self.previous
            .validate(SanitizedSearchOperationSlot::Previous)?;
        self.replace
            .validate(SanitizedSearchOperationSlot::Replace)?;
        self.replace_all
            .validate(SanitizedSearchOperationSlot::ReplaceAll)?;
        Ok(SanitizedSearchProjection {
            query: self.query,
            replacement: self.replacement,
            match_case: self.match_case,
            whole_word: self.whole_word,
            regex: self.regex,
            close: self.close,
            next: self.next,
            previous: self.previous,
            replace: self.replace,
            replace_all: self.replace_all,
            presentation,
        })
    }
}

impl Default for SanitizedSearchProjectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for SanitizedSearchProjectionBuilder {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.presentation.is_some();
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}
