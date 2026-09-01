use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanitizedSearchUnitOperation {
    Close,
    Previous,
    Next,
    MatchCase(bool),
    WholeWord(bool),
    Regex(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanitizedSearchTextOperation {
    Query,
    Replacement,
    Replace,
    ReplaceAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanitizedSearchCapabilityRejection {
    Missing,
    WrongOperation,
    AlreadyConsumed,
    Reentrant,
    CallbackRejected,
}

pub(super) type TextCapability = Rc<
    RefCell<
        Option<
            Box<
                dyn FnOnce(
                    SanitizedSearchTextOperation,
                    String,
                ) -> Result<(), SanitizedSearchCapabilityRejection>,
            >,
        >,
    >,
>;
pub(super) type UnitCapability = Rc<
    RefCell<
        Option<
            Box<
                dyn FnOnce(
                    SanitizedSearchUnitOperation,
                ) -> Result<(), SanitizedSearchCapabilityRejection>,
            >,
        >,
    >,
>;

pub(super) enum SanitizedSearchCapability {
    Text(TextCapability),
    Unit(UnitCapability),
}

pub struct SanitizedSearchTarget {
    pub(super) opaque: Box<[u8]>,
    pub(super) capability: Option<SanitizedSearchCapability>,
}

impl SanitizedSearchTarget {
    #[must_use]
    pub fn from_opaque_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            opaque: bytes.into().into_boxed_slice(),
            capability: None,
        }
    }

    #[must_use]
    pub fn with_text_capability<F, E>(mut self, callback: F) -> Self
    where
        F: FnOnce(SanitizedSearchTextOperation, String) -> Result<(), E> + 'static,
        E: 'static,
    {
        self.capability = Some(SanitizedSearchCapability::Text(Rc::new(RefCell::new(
            Some(Box::new(move |operation, value| {
                callback(operation, value)
                    .map_err(|_| SanitizedSearchCapabilityRejection::CallbackRejected)
            })),
        ))));
        self
    }

    #[must_use]
    pub fn with_unit_capability<F, E>(mut self, callback: F) -> Self
    where
        F: FnOnce(SanitizedSearchUnitOperation) -> Result<(), E> + 'static,
        E: 'static,
    {
        self.capability = Some(SanitizedSearchCapability::Unit(Rc::new(RefCell::new(
            Some(Box::new(move |operation| {
                callback(operation)
                    .map_err(|_| SanitizedSearchCapabilityRejection::CallbackRejected)
            })),
        ))));
        self
    }

    fn stable_signature(&self) -> [u8; SHA256_BYTES] {
        SanitizedProjectionHash::sha256_signature(&self.opaque)
    }
}

impl std::fmt::Debug for SanitizedSearchTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SanitizedSearchTextPresentation {
    pub(super) visible: String,
    pub(super) tooltip: String,
    pub(super) accessibility_label: String,
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

    fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        if self.visible.is_empty() || self.tooltip.is_empty() || self.accessibility_label.is_empty()
        {
            return Err(SanitizedSearchProjectionBuildError::EmptyPresentationText);
        }
        Ok(())
    }

    pub(super) fn hash_into(&self, hasher: &mut Sha256) {
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
    pub(super) empty: String,
    pub(super) zero_results: String,
    pub(super) single_result: String,
    pub(super) indexed_result: String,
    pub(super) count_results: String,
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

    fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
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

    fn hash_into(&self, hasher: &mut Sha256) {
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
    pub(super) strip: SanitizedSearchTextPresentation,
    pub(super) query: SanitizedSearchTextPresentation,
    pub(super) replacement: SanitizedSearchTextPresentation,
    pub(super) match_case: SanitizedSearchTextPresentation,
    pub(super) whole_word: SanitizedSearchTextPresentation,
    pub(super) regex: SanitizedSearchTextPresentation,
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

    fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        self.strip.validate()?;
        self.query.validate()?;
        self.replacement.validate()?;
        self.match_case.validate()?;
        self.whole_word.validate()?;
        self.regex.validate()?;
        Ok(())
    }

    fn hash_into(&self, hasher: &mut Sha256) {
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
    pub(super) previous: SanitizedSearchTextPresentation,
    pub(super) next: SanitizedSearchTextPresentation,
    pub(super) replace: SanitizedSearchTextPresentation,
    pub(super) replace_all: SanitizedSearchTextPresentation,
    pub(super) close: SanitizedSearchTextPresentation,
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

    fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        self.previous.validate()?;
        self.next.validate()?;
        self.replace.validate()?;
        self.replace_all.validate()?;
        self.close.validate()?;
        Ok(())
    }

    fn hash_into(&self, hasher: &mut Sha256) {
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

#[derive(Clone, PartialEq, Eq)]
pub struct SanitizedSearchUnavailablePresentation {
    pub(super) regex: String,
    pub(super) replace: String,
    pub(super) navigation: String,
    pub(super) close: String,
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

    fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        if self.regex.is_empty()
            || self.replace.is_empty()
            || self.navigation.is_empty()
            || self.close.is_empty()
        {
            return Err(SanitizedSearchProjectionBuildError::EmptyPresentationText);
        }
        Ok(())
    }

    fn hash_into(&self, hasher: &mut Sha256) {
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
    pub(super) controls: SanitizedSearchControlPresentation,
    pub(super) operations: SanitizedSearchOperationPresentation,
    pub(super) result_summary: SanitizedSearchResultSummaryPresentation,
    pub(super) unavailable: SanitizedSearchUnavailablePresentation,
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

    fn validate(&self) -> Result<(), SanitizedSearchProjectionBuildError> {
        self.controls.validate()?;
        self.operations.validate()?;
        self.result_summary.validate()?;
        self.unavailable.validate()?;
        Ok(())
    }

    fn hash_into(&self, hasher: &mut Sha256) {
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

pub(super) struct SanitizedSearchOperation {
    pub(super) enabled: bool,
    pub(super) current: bool,
    pub(super) target: Option<SanitizedSearchTarget>,
}

impl SanitizedSearchOperation {
    fn new() -> Self {
        Self {
            enabled: false,
            current: false,
            target: None,
        }
    }

    fn validate(
        &self,
        slot: SanitizedSearchOperationSlot,
    ) -> Result<(), SanitizedSearchProjectionBuildError> {
        if self.enabled && self.target.is_none() {
            return Err(
                SanitizedSearchProjectionBuildError::EnabledOperationWithoutTarget {
                    operation: slot,
                },
            );
        }
        Ok(())
    }

    fn hash_into(&self, hasher: &mut Sha256) {
        hasher.update([u8::from(self.enabled)]);
        hasher.update([u8::from(self.current)]);
        match &self.target {
            Some(target) => {
                hasher.update([1]);
                hasher.update(target.stable_signature());
            }
            None => hasher.update([0]),
        }
    }
}

impl std::fmt::Debug for SanitizedSearchOperation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.target.is_some();
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .field("有効", &self.enabled)
            .finish_non_exhaustive()
    }
}

pub struct SanitizedSearchProjection {
    pub(super) query: SanitizedSearchOperation,
    pub(super) replacement: SanitizedSearchOperation,
    pub(super) match_case: SanitizedSearchOperation,
    pub(super) whole_word: SanitizedSearchOperation,
    pub(super) regex: SanitizedSearchOperation,
    pub(super) close: SanitizedSearchOperation,
    pub(super) next: SanitizedSearchOperation,
    pub(super) previous: SanitizedSearchOperation,
    pub(super) replace: SanitizedSearchOperation,
    pub(super) replace_all: SanitizedSearchOperation,
    pub(super) presentation: SanitizedSearchLocalizedPresentation,
}

impl SanitizedSearchProjection {
    #[cfg(test)]
    pub(super) fn same_as(&self, other: &Self) -> bool {
        self.stable_fingerprint() == other.stable_fingerprint()
    }

    pub(crate) fn stable_fingerprint(&self) -> [u8; SHA256_BYTES] {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanitizedSearchOperationSlot {
    Query,
    Replacement,
    MatchCase,
    WholeWord,
    Regex,
    Close,
    Next,
    Previous,
    Replace,
    ReplaceAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanitizedSearchProjectionBuildError {
    MissingPresentation,
    EmptyPresentationText,
    EnabledOperationWithoutTarget {
        operation: SanitizedSearchOperationSlot,
    },
}

pub struct SanitizedSearchProjectionBuilder {
    query: SanitizedSearchOperation,
    replacement: SanitizedSearchOperation,
    match_case: SanitizedSearchOperation,
    whole_word: SanitizedSearchOperation,
    regex: SanitizedSearchOperation,
    close: SanitizedSearchOperation,
    next: SanitizedSearchOperation,
    previous: SanitizedSearchOperation,
    replace: SanitizedSearchOperation,
    replace_all: SanitizedSearchOperation,
    presentation: Option<SanitizedSearchLocalizedPresentation>,
}

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

fn hash_text(hasher: &mut Sha256, value: &str) {
    SanitizedProjectionHash::hash_len_prefixed_text(hasher, value);
}

#[cfg(test)]
mod tests {
    use super::{
        SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
        SanitizedSearchOperation, SanitizedSearchOperationPresentation, SanitizedSearchOperationSlot,
        SanitizedSearchProjectionBuildError, SanitizedSearchProjectionBuilder,
        SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget,
        SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
    };

    fn text(value: &str) -> SanitizedSearchTextPresentation {
        SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
    }

    fn localized(next: &str) -> SanitizedSearchLocalizedPresentation {
        SanitizedSearchLocalizedPresentation::new(
            SanitizedSearchControlPresentation::new(
                text("検索"),
                text("検索語"),
                text("置換"),
                text("大文字小文字"),
                text("単語"),
                text("正規表現"),
            ),
            SanitizedSearchOperationPresentation::new(
                text("前へ"),
                text(next),
                text("置換"),
                text("すべて置換"),
                text("閉じる"),
            ),
            SanitizedSearchResultSummaryPresentation::new(
                "検索待機 ⭐️",
                "一致なし ⭐️",
                "一件 ⭐️",
                "位置 ⭐️",
                "件数 ⭐️",
            ),
            SanitizedSearchUnavailablePresentation::new(
                "正規表現は利用不可",
                "置換は利用不可",
                "移動は利用不可",
                "閉じる操作は利用不可",
            ),
        )
    }

    #[test]
    fn debug_contracts_remain_opaque_for_every_search_projection_layer() {
        let target = SanitizedSearchTarget::from_opaque_bytes([0xde, 0xad]);
        let presentation = localized("次へ");
        let controls_debug = format!("{:?}", presentation.controls);
        let operations_debug = format!("{:?}", presentation.operations);
        let summary_debug = format!("{:?}", presentation.result_summary);
        let unavailable_debug = format!("{:?}", presentation.unavailable);
        let localized_debug = format!("{presentation:?}");
        let text_debug = format!("{:?}", text("秘密"));
        let target_debug = format!("{target:?}");
        let builder_debug = format!("{:?}", SanitizedSearchProjectionBuilder::default());

        for debug in [
            controls_debug,
            operations_debug,
            summary_debug,
            unavailable_debug,
            localized_debug,
            text_debug,
            target_debug,
            builder_debug,
        ] {
            assert!(!debug.contains("秘密"));
            assert!(!debug.contains("次へ"));
            assert!(!debug.contains("222"));
        }
        let operation_debug = format!("{:?}", SanitizedSearchOperation::new());
        assert!(operation_debug.contains("有効"));
    }

    #[test]
    fn build_requires_localized_presentation() {
        let error = SanitizedSearchProjectionBuilder::new()
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1]))
            .build()
            .expect_err("表示未指定");

        assert_eq!(
            error,
            SanitizedSearchProjectionBuildError::MissingPresentation
        );
    }

    #[test]
    fn build_rejects_empty_presentation_text() {
        let invalid = SanitizedSearchLocalizedPresentation::new(
            SanitizedSearchControlPresentation::new(
                SanitizedSearchTextPresentation::new("", "検索 ⭐️", "検索 ⭐️"),
                text("検索語"),
                text("置換"),
                text("大文字小文字"),
                text("単語"),
                text("正規表現"),
            ),
            SanitizedSearchOperationPresentation::new(
                text("前へ"),
                text("次へ"),
                text("置換"),
                text("すべて置換"),
                text("閉じる"),
            ),
            SanitizedSearchResultSummaryPresentation::new(
                "検索待機 ⭐️",
                "一致なし ⭐️",
                "一件 ⭐️",
                "位置 ⭐️",
                "件数 ⭐️",
            ),
            SanitizedSearchUnavailablePresentation::new(
                "正規表現は利用不可",
                "置換は利用不可",
                "移動は利用不可",
                "閉じる操作は利用不可",
            ),
        );

        let error = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(invalid)
            .build()
            .expect_err("空表示拒否");

        assert_eq!(
            error,
            SanitizedSearchProjectionBuildError::EmptyPresentationText
        );

        let mut invalid_summary = localized("次へ");
        invalid_summary.result_summary.empty.clear();
        assert!(matches!(
            SanitizedSearchProjectionBuilder::new()
                .localized_presentation(invalid_summary)
                .build(),
            Err(SanitizedSearchProjectionBuildError::EmptyPresentationText)
        ));

        let mut invalid_unavailable = localized("次へ");
        invalid_unavailable.unavailable.regex.clear();
        assert!(matches!(
            SanitizedSearchProjectionBuilder::new()
                .localized_presentation(invalid_unavailable)
                .build(),
            Err(SanitizedSearchProjectionBuildError::EmptyPresentationText)
        ));
    }

    #[test]
    fn build_rejects_enabled_operation_without_target() {
        let error = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次へ"))
            .next_enabled(true)
            .build()
            .expect_err("対象未指定");

        assert_eq!(
            error,
            SanitizedSearchProjectionBuildError::EnabledOperationWithoutTarget {
                operation: SanitizedSearchOperationSlot::Next
            }
        );
    }

    #[test]
    fn fingerprint_tracks_target_and_presentation_without_debug_leaks() {
        let base = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次へ"))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1, 2, 3]))
            .build()
            .expect("検証済み");
        let same = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次へ"))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1, 2, 3]))
            .build()
            .expect("検証済み");
        let changed_presentation = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次の一致"))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1, 2, 3]))
            .build()
            .expect("検証済み");
        let changed_target = SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized("次へ"))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([1, 2, 4]))
            .build()
            .expect("検証済み");

        assert!(base.same_as(&same));
        assert!(!base.same_as(&changed_presentation));
        assert!(!base.same_as(&changed_target));

        let debug = format!("{base:?}");
        assert!(!debug.contains("検索"));
        assert!(!debug.contains("置換"));
        assert!(!debug.contains("次へ"));
        assert!(!debug.contains("⭐️"));
        assert!(!debug.contains("1, 2, 3"));
    }
}
