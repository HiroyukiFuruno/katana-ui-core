use super::super::sanitized_search_projection::SanitizedSearchProjection;
use super::{
    SanitizedSearchCapability, SanitizedSearchCapabilityRejection, SanitizedSearchEventKind,
    SanitizedSearchEventTransport, SanitizedSearchOneShotText, SanitizedSearchRoutedTarget,
    SanitizedSearchTarget,
};
use katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent;
use katana_ui_core::molecule::structured::{
    SearchControlStripEvent, SearchNavigationDirection, SearchOptionKind, SearchReplaceScope,
};
use sha2::{Digest, Sha256};

pub(super) fn route_search_events(
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
        | SearchControlStripEvent::SearchResultPositionChanged { .. } => Ok(None),
    }
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
