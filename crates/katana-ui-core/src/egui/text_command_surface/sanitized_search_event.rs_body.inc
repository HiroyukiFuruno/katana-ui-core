use super::sanitized_search_projection::{
    SanitizedSearchCapability, SanitizedSearchCapabilityRejection, SanitizedSearchProjection,
    SanitizedSearchTarget, SanitizedSearchTextOperation, SanitizedSearchUnitOperation,
    TextCapability, UnitCapability,
};
use crate::molecule::command_chrome::CommandChromeSearchEvent;
use crate::molecule::structured::{
    SearchControlStripEvent, SearchNavigationDirection, SearchOptionKind, SearchReplaceScope,
};
use sha2::{Digest, Sha256};
const QUERY_DISCRIMINANT: u8 = 3;
const REPLACEMENT_DISCRIMINANT: u8 = 4;
const MATCH_CASE_DISCRIMINANT: u8 = 5;
const WHOLE_WORD_DISCRIMINANT: u8 = 6;
const REGEX_DISCRIMINANT: u8 = 7;
const REPLACE_DISCRIMINANT: u8 = 8;
const REPLACE_ALL_DISCRIMINANT: u8 = 9;

pub(crate) struct SanitizedSearchEventTransport {
    target: SanitizedSearchRoutedTarget,
    kind: SanitizedSearchEventKind,
    text: Option<SanitizedSearchOneShotText>,
    unit_value: Option<bool>,
    revision: u64,
    correlation: String,
}

pub(super) struct SanitizedSearchRoutedTarget {
    opaque: Box<[u8]>,
    capability: Option<SanitizedSearchCapabilityRef>,
}

#[derive(Clone)]
enum SanitizedSearchCapabilityRef {
    Text(TextCapability),
    Unit(UnitCapability),
}

pub(super) struct SanitizedSearchOneShotText {
    value: Option<String>,
}

#[derive(Clone, Copy)]
enum SanitizedSearchEventKind {
    Close,
    Previous,
    Next,
    Query,
    Replacement,
    MatchCase,
    WholeWord,
    Regex,
    Replace,
    ReplaceAll,
}

impl SanitizedSearchEventTransport {
    pub(super) fn read_for_transport(&self) {
        let _ = self.target.opaque.len();
        let _ = self.kind.discriminant();
        let _ = self.revision;
        let _ = self.correlation.as_bytes();
        let _ = self.unit_value;
        if let Some(text) = &self.text {
            let _ = text.value.as_ref().map(String::len);
        }
    }

    pub(super) fn fingerprint_into(&self, hasher: &mut Sha256) {
        hasher.update(b"sanitized-search-event");
        hasher.update(self.target.signature());
        hasher.update([self.kind.discriminant()]);
        hasher.update(self.revision.to_le_bytes());
        hasher.update(self.correlation.as_bytes());
        if let Some(text) = &self.text {
            hasher.update([1]);
            if let Some(value) = &text.value {
                hasher.update(value.len().to_le_bytes());
                hasher.update(value.as_bytes());
            }
        } else {
            hasher.update([0]);
        }
        hasher.update([self.unit_value.is_some() as u8]);
        if let Some(value) = self.unit_value {
            hasher.update([value as u8]);
        }
    }

    pub(super) fn invoke_once(&mut self) -> Result<(), SanitizedSearchCapabilityRejection> {
        let capability = self
            .target
            .capability
            .take()
            .ok_or(SanitizedSearchCapabilityRejection::AlreadyConsumed)?;
        match (capability, self.kind, self.text.take(), self.unit_value) {
            (
                SanitizedSearchCapabilityRef::Text(slot),
                kind @ (SanitizedSearchEventKind::Query
                | SanitizedSearchEventKind::Replacement
                | SanitizedSearchEventKind::Replace
                | SanitizedSearchEventKind::ReplaceAll),
                Some(mut text),
                None,
            ) => {
                let callback = slot
                    .try_borrow_mut()
                    .map_err(|_| SanitizedSearchCapabilityRejection::Reentrant)?
                    .take()
                    .ok_or(SanitizedSearchCapabilityRejection::AlreadyConsumed)?;
                let operation = text_operation(kind)?;
                let callback = callback;
                callback(
                    operation,
                    text.value
                        .take()
                        .ok_or(SanitizedSearchCapabilityRejection::AlreadyConsumed)?,
                )
            }
            (SanitizedSearchCapabilityRef::Unit(slot), kind, None, value) => {
                let operation = unit_operation(kind, value)?;
                let callback = slot
                    .try_borrow_mut()
                    .map_err(|_| SanitizedSearchCapabilityRejection::Reentrant)?
                    .take()
                    .ok_or(SanitizedSearchCapabilityRejection::AlreadyConsumed)?;
                let callback = callback;
                callback(operation)
            }
            _ => Err(SanitizedSearchCapabilityRejection::WrongOperation),
        }
    }
}

fn text_operation(
    kind: SanitizedSearchEventKind,
) -> Result<SanitizedSearchTextOperation, SanitizedSearchCapabilityRejection> {
    match kind {
        SanitizedSearchEventKind::Query => Ok(SanitizedSearchTextOperation::Query),
        SanitizedSearchEventKind::Replacement => Ok(SanitizedSearchTextOperation::Replacement),
        SanitizedSearchEventKind::Replace => Ok(SanitizedSearchTextOperation::Replace),
        SanitizedSearchEventKind::ReplaceAll => Ok(SanitizedSearchTextOperation::ReplaceAll),
        _ => Err(SanitizedSearchCapabilityRejection::WrongOperation),
    }
}

fn unit_operation(
    kind: SanitizedSearchEventKind,
    value: Option<bool>,
) -> Result<SanitizedSearchUnitOperation, SanitizedSearchCapabilityRejection> {
    match kind {
        SanitizedSearchEventKind::Close => Ok(SanitizedSearchUnitOperation::Close),
        SanitizedSearchEventKind::Previous => Ok(SanitizedSearchUnitOperation::Previous),
        SanitizedSearchEventKind::Next => Ok(SanitizedSearchUnitOperation::Next),
        SanitizedSearchEventKind::MatchCase => Ok(SanitizedSearchUnitOperation::MatchCase(
            value.ok_or(SanitizedSearchCapabilityRejection::WrongOperation)?,
        )),
        SanitizedSearchEventKind::WholeWord => Ok(SanitizedSearchUnitOperation::WholeWord(
            value.ok_or(SanitizedSearchCapabilityRejection::WrongOperation)?,
        )),
        SanitizedSearchEventKind::Regex => Ok(SanitizedSearchUnitOperation::Regex(
            value.ok_or(SanitizedSearchCapabilityRejection::WrongOperation)?,
        )),
        _ => Err(SanitizedSearchCapabilityRejection::WrongOperation),
    }
}

impl std::fmt::Debug for SanitizedSearchEventTransport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedSearchEventTransport")
            .field("payload", &"<opaque>")
            .finish()
    }
}

impl SanitizedSearchRoutedTarget {
    fn from_target(target: &SanitizedSearchTarget) -> Self {
        Self {
            opaque: target.opaque.to_vec().into_boxed_slice(),
            capability: target
                .capability
                .as_ref()
                .map(|capability| match capability {
                    SanitizedSearchCapability::Text(slot) => {
                        SanitizedSearchCapabilityRef::Text(slot.clone())
                    }
                    SanitizedSearchCapability::Unit(slot) => {
                        SanitizedSearchCapabilityRef::Unit(slot.clone())
                    }
                }),
        }
    }

    fn signature(&self) -> [u8; SHA256_BYTES] {
        SanitizedProjectionHash::sha256_signature(&self.opaque)
    }
}

impl std::fmt::Debug for SanitizedSearchRoutedTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter.write_str("SanitizedSearchRoutedTarget(..)")
    }
}

impl SanitizedSearchOneShotText {
    fn new(value: String) -> Self {
        Self { value: Some(value) }
    }
}

impl std::fmt::Debug for SanitizedSearchOneShotText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.value.as_ref().map(String::len);
        formatter.write_str("SanitizedSearchOneShotText(..)")
    }
}

impl SanitizedSearchEventKind {
    const fn discriminant(self) -> u8 {
        match self {
            Self::Close => 0,
            Self::Previous => 1,
            Self::Next => 2,
            Self::Query => QUERY_DISCRIMINANT,
            Self::Replacement => REPLACEMENT_DISCRIMINANT,
            Self::MatchCase => MATCH_CASE_DISCRIMINANT,
            Self::WholeWord => WHOLE_WORD_DISCRIMINANT,
            Self::Regex => REGEX_DISCRIMINANT,
            Self::Replace => REPLACE_DISCRIMINANT,
            Self::ReplaceAll => REPLACE_ALL_DISCRIMINANT,
        }
    }
}

pub(crate) struct SanitizedSearchEventRouter;

impl SanitizedSearchEventRouter {
    pub(crate) fn route_events(
        projection: Option<&SanitizedSearchProjection>,
        events: &[CommandChromeSearchEvent],
        revision: u64,
        root_identity_fingerprint: &str,
    ) -> Result<Vec<SanitizedSearchEventTransport>, SanitizedSearchCapabilityRejection> {
        let Some(projection) = projection else {
            return Ok(Vec::new());
        };
        let correlation = event_correlation(root_identity_fingerprint, revision);
        events
            .iter()
            .map(|event| route_event(projection, event, revision, &correlation))
            .filter_map(Result::transpose)
            .collect()
    }
}

fn route_event(
    projection: &SanitizedSearchProjection,
    event: &CommandChromeSearchEvent,
    revision: u64,
    correlation: &str,
) -> Result<Option<SanitizedSearchEventTransport>, SanitizedSearchCapabilityRejection> {
    match event {
        CommandChromeSearchEvent::CloseRequested => operation_event(
            projection.close.enabled,
            projection.close.target.as_ref(),
            SanitizedSearchEventKind::Close,
            None,
            None,
            revision,
            correlation,
        ),
        CommandChromeSearchEvent::Strip { event } => {
            route_strip_event(projection, event, revision, correlation)
        }
    }
}

fn route_strip_event(
    projection: &SanitizedSearchProjection,
    event: &SearchControlStripEvent,
    revision: u64,
    correlation: &str,
) -> Result<Option<SanitizedSearchEventTransport>, SanitizedSearchCapabilityRejection> {
    match event {
        SearchControlStripEvent::SearchQueryChanged(value) => operation_event(
            true,
            projection.query.target.as_ref(),
            SanitizedSearchEventKind::Query,
            Some(SanitizedSearchOneShotText::new(value.clone())),
            None,
            revision,
            correlation,
        ),
        SearchControlStripEvent::ReplaceValueChanged(value) => operation_event(
            true,
            projection.replacement.target.as_ref(),
            SanitizedSearchEventKind::Replacement,
            Some(SanitizedSearchOneShotText::new(value.clone())),
            None,
            revision,
            correlation,
        ),
        SearchControlStripEvent::SearchNavigationRequested { direction } => match direction {
            SearchNavigationDirection::Previous => operation_event(
                projection.previous.enabled,
                projection.previous.target.as_ref(),
                SanitizedSearchEventKind::Previous,
                None,
                None,
                revision,
                correlation,
            ),
            SearchNavigationDirection::Next => operation_event(
                projection.next.enabled,
                projection.next.target.as_ref(),
                SanitizedSearchEventKind::Next,
                None,
                None,
                revision,
                correlation,
            ),
        },
        SearchControlStripEvent::SearchOptionChanged { option, enabled } => match option {
            SearchOptionKind::MatchCase => operation_event(
                projection.match_case.enabled,
                projection.match_case.target.as_ref(),
                SanitizedSearchEventKind::MatchCase,
                None,
                Some(*enabled),
                revision,
                correlation,
            ),
            SearchOptionKind::WholeWord => operation_event(
                projection.whole_word.enabled,
                projection.whole_word.target.as_ref(),
                SanitizedSearchEventKind::WholeWord,
                None,
                Some(*enabled),
                revision,
                correlation,
            ),
            SearchOptionKind::UseRegex => operation_event(
                projection.regex.enabled,
                projection.regex.target.as_ref(),
                SanitizedSearchEventKind::Regex,
                None,
                Some(*enabled),
                revision,
                correlation,
            ),
        },
        SearchControlStripEvent::ReplaceRequested { scope, value } => match scope {
            SearchReplaceScope::One => operation_event(
                projection.replace.enabled,
                projection.replace.target.as_ref(),
                SanitizedSearchEventKind::Replace,
                Some(SanitizedSearchOneShotText::new(value.clone())),
                None,
                revision,
                correlation,
            ),
            SearchReplaceScope::All => operation_event(
                projection.replace_all.enabled,
                projection.replace_all.target.as_ref(),
                SanitizedSearchEventKind::ReplaceAll,
                Some(SanitizedSearchOneShotText::new(value.clone())),
                None,
                revision,
                correlation,
            ),
        },
        SearchControlStripEvent::ReplaceModeChanged(_)
        | SearchControlStripEvent::SearchResultPositionChanged { .. } => ignored_projection_event(),
    }
}

fn ignored_projection_event(
) -> Result<Option<SanitizedSearchEventTransport>, SanitizedSearchCapabilityRejection> {
    Ok(None)
}

fn operation_event(
    enabled: bool,
    target: Option<&SanitizedSearchTarget>,
    kind: SanitizedSearchEventKind,
    text: Option<SanitizedSearchOneShotText>,
    unit_value: Option<bool>,
    revision: u64,
    correlation: &str,
) -> Result<Option<SanitizedSearchEventTransport>, SanitizedSearchCapabilityRejection> {
    if !enabled {
        return Ok(None);
    }
    let target = target.ok_or(SanitizedSearchCapabilityRejection::Missing)?;
    let capability = target
        .capability
        .as_ref()
        .ok_or(SanitizedSearchCapabilityRejection::Missing)?;
    let compatible = matches!(
        (kind, capability),
        (
            SanitizedSearchEventKind::Query
                | SanitizedSearchEventKind::Replacement
                | SanitizedSearchEventKind::Replace
                | SanitizedSearchEventKind::ReplaceAll,
            SanitizedSearchCapability::Text(_),
        ) | (
            SanitizedSearchEventKind::Close
                | SanitizedSearchEventKind::Previous
                | SanitizedSearchEventKind::Next
                | SanitizedSearchEventKind::MatchCase
                | SanitizedSearchEventKind::WholeWord
                | SanitizedSearchEventKind::Regex,
            SanitizedSearchCapability::Unit(_),
        )
    );
    if !compatible {
        return Err(SanitizedSearchCapabilityRejection::WrongOperation);
    }
    Ok(Some(SanitizedSearchEventTransport {
        target: SanitizedSearchRoutedTarget::from_target(target),
        kind,
        text,
        unit_value,
        revision,
        correlation: correlation.to_owned(),
    }))
}

fn event_correlation(root_identity_fingerprint: &str, revision: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"kuc.sanitized-search-correlation/v1");
    hasher.update(root_identity_fingerprint.as_bytes());
    hasher.update(revision.to_le_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::SanitizedSearchUnitOperation;
    use super::super::sanitized_search_projection::{
        SanitizedSearchCapability, SanitizedSearchCapabilityRejection,
        SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
        SanitizedSearchOperationPresentation, SanitizedSearchProjectionBuilder,
        SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget,
        SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
    };
    use super::{
        SanitizedSearchEventKind, SanitizedSearchEventRouter, SanitizedSearchEventTransport,
        SanitizedSearchOneShotText, SanitizedSearchRoutedTarget, ignored_projection_event,
        operation_event, route_strip_event, text_operation, unit_operation,
    };
    use crate::molecule::structured::SearchControlStripEvent;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn successful_unit_capability(_operation: SanitizedSearchUnitOperation) -> Result<(), ()> {
        Ok(())
    }

    fn text(value: &str) -> SanitizedSearchTextPresentation {
        SanitizedSearchTextPresentation::new(value, value, value)
    }

    fn projection() -> super::super::sanitized_search_projection::SanitizedSearchProjection {
        SanitizedSearchProjectionBuilder::new()
            .localized_presentation(SanitizedSearchLocalizedPresentation::new(
                SanitizedSearchControlPresentation::new(
                    text("search"), text("query"), text("replace"), text("case"), text("word"),
                    text("regex"),
                ),
                SanitizedSearchOperationPresentation::new(
                    text("previous"), text("next"), text("one"), text("all"), text("close"),
                ),
                SanitizedSearchResultSummaryPresentation::new(
                    "empty", "zero", "single", "indexed", "count",
                ),
                SanitizedSearchUnavailablePresentation::new(
                    "regex unavailable", "replace unavailable", "navigation unavailable",
                    "close unavailable",
                ),
            ))
            .build()
            .expect("localized projection")
    }

    #[test]
    fn opaque_debug_and_fail_closed_routing_boundaries_are_covered() {
        assert!(ignored_projection_event().expect("ignored event").is_none());
        assert!(
            route_strip_event(
                &projection(),
                &SearchControlStripEvent::ReplaceModeChanged(
                    crate::molecule::structured::ReplaceMode::Hidden,
                ),
                1,
                "correlation",
            )
            .expect("projection-only event is ignored")
            .is_none()
        );
        let target = SanitizedSearchTarget::from_opaque_bytes(b"semantic-secret")
            .with_unit_capability(successful_unit_capability);
        assert!(successful_unit_capability(SanitizedSearchUnitOperation::Close).is_ok());
        let routed = SanitizedSearchRoutedTarget::from_target(&target);
        assert_eq!(format!("{routed:?}"), "SanitizedSearchRoutedTarget(..)");
        let text = SanitizedSearchOneShotText::new("hidden-text".into());
        assert_eq!(format!("{text:?}"), "SanitizedSearchOneShotText(..)");
        assert!(SanitizedSearchEventRouter::route_events(None, &[], 1, "root")
            .expect("absent projection is ignored")
            .is_empty());
        assert!(operation_event(
            false,
            None,
            SanitizedSearchEventKind::Close,
            None,
            None,
            1,
            "correlation",
        )
        .expect("disabled operations are ignored")
        .is_none());
        assert_eq!(
            operation_event(
                true,
                Some(&target),
                SanitizedSearchEventKind::Query,
                Some(SanitizedSearchOneShotText::new("value".into())),
                None,
                1,
                "correlation",
            )
            .expect_err("a unit capability cannot serve text"),
            SanitizedSearchCapabilityRejection::WrongOperation
        );
        assert_eq!(
            text_operation(SanitizedSearchEventKind::Close),
            Err(SanitizedSearchCapabilityRejection::WrongOperation)
        );
        assert_eq!(
            unit_operation(SanitizedSearchEventKind::Query, None),
            Err(SanitizedSearchCapabilityRejection::WrongOperation)
        );
        assert_eq!(
            unit_operation(SanitizedSearchEventKind::Regex, None),
            Err(SanitizedSearchCapabilityRejection::WrongOperation)
        );
    }

    #[test]
    fn wrong_capability_kind_does_not_call_callback_and_consumes_transport_once() {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let callback = {
            let calls = calls.clone();
            move |operation| {
                calls.borrow_mut().push(operation);
                Ok::<(), ()>(())
            }
        };
        callback(SanitizedSearchUnitOperation::Close).expect("callback fixture is valid");
        calls.borrow_mut().clear();
        let target = SanitizedSearchTarget::from_opaque_bytes([0x73, 0x65, 0x63, 0x72, 0x65, 0x74])
            .with_unit_capability(callback);
        let mut transport = SanitizedSearchEventTransport {
            target: SanitizedSearchRoutedTarget::from_target(&target),
            kind: SanitizedSearchEventKind::Query,
            text: Some(SanitizedSearchOneShotText::new("日本語 ⭐️👩‍💻".to_owned())),
            unit_value: None,
            revision: 1,
            correlation: "correlation".to_owned(),
        };

        assert_eq!(
            transport.invoke_once(),
            Err(SanitizedSearchCapabilityRejection::WrongOperation)
        );
        assert!(calls.borrow().is_empty());
        assert_eq!(
            transport.invoke_once(),
            Err(SanitizedSearchCapabilityRejection::AlreadyConsumed)
        );
        assert!(matches!(
            target.capability.as_ref(),
            Some(SanitizedSearchCapability::Unit(_))
        ));
        let debug = format!("{transport:?}");
        for forbidden in ["日本語", "⭐️", "👩‍💻"] {
            assert!(!debug.contains(forbidden));
        }
    }
}
