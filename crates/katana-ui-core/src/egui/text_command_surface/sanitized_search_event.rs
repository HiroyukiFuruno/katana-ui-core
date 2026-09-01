use super::sanitized_search_projection::{
    SanitizedSearchCapability, SanitizedSearchCapabilityRejection, SanitizedSearchProjection,
    SanitizedSearchTarget, SanitizedSearchTextOperation, SanitizedSearchUnitOperation,
    TextCapability, UnitCapability,
};
use crate::molecule::command_chrome::CommandChromeSearchEvent;
use sha2::{Digest, Sha256};

#[path = "sanitized_search_event/routing.rs"]
mod routing;

type SearchEventRouter =
    fn(
        Option<&SanitizedSearchProjection>,
        &[CommandChromeSearchEvent],
        u64,
        &str,
    ) -> Result<Vec<SanitizedSearchEventTransport>, SanitizedSearchCapabilityRejection>;

#[allow(non_upper_case_globals)]
pub(crate) const route_search_events: SearchEventRouter = routing::route_search_events;

const SHA256_DIGEST_LENGTH: usize = 32;
const KIND_QUERY: u8 = 3;
const KIND_REPLACEMENT: u8 = 4;
const KIND_MATCH_CASE: u8 = 5;
const KIND_WHOLE_WORD: u8 = 6;
const KIND_REGEX: u8 = 7;
const KIND_REPLACE: u8 = 8;
const KIND_REPLACE_ALL: u8 = 9;

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
        let text_operation = match self.kind {
            SanitizedSearchEventKind::Query => Some(SanitizedSearchTextOperation::Query),
            SanitizedSearchEventKind::Replacement => {
                Some(SanitizedSearchTextOperation::Replacement)
            }
            SanitizedSearchEventKind::Replace => Some(SanitizedSearchTextOperation::Replace),
            SanitizedSearchEventKind::ReplaceAll => Some(SanitizedSearchTextOperation::ReplaceAll),
            _ => None,
        };
        match (
            capability,
            self.kind,
            text_operation,
            self.text.take(),
            self.unit_value,
        ) {
            (
                SanitizedSearchCapabilityRef::Text(slot),
                _,
                Some(operation),
                Some(mut text),
                None,
            ) => {
                let callback = slot
                    .try_borrow_mut()
                    .map_err(|_| SanitizedSearchCapabilityRejection::Reentrant)?
                    .take()
                    .ok_or(SanitizedSearchCapabilityRejection::AlreadyConsumed)?;
                let callback = callback;
                callback(
                    operation,
                    text.value
                        .take()
                        .ok_or(SanitizedSearchCapabilityRejection::AlreadyConsumed)?,
                )
            }
            (SanitizedSearchCapabilityRef::Unit(slot), kind, _, None, value) => {
                let operation = match kind {
                    SanitizedSearchEventKind::Close => SanitizedSearchUnitOperation::Close,
                    SanitizedSearchEventKind::Previous => SanitizedSearchUnitOperation::Previous,
                    SanitizedSearchEventKind::Next => SanitizedSearchUnitOperation::Next,
                    SanitizedSearchEventKind::MatchCase => SanitizedSearchUnitOperation::MatchCase(
                        value.ok_or(SanitizedSearchCapabilityRejection::WrongOperation)?,
                    ),
                    SanitizedSearchEventKind::WholeWord => SanitizedSearchUnitOperation::WholeWord(
                        value.ok_or(SanitizedSearchCapabilityRejection::WrongOperation)?,
                    ),
                    SanitizedSearchEventKind::Regex => SanitizedSearchUnitOperation::Regex(
                        value.ok_or(SanitizedSearchCapabilityRejection::WrongOperation)?,
                    ),
                    _ => return Err(SanitizedSearchCapabilityRejection::WrongOperation),
                };
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

    fn signature(&self) -> [u8; SHA256_DIGEST_LENGTH] {
        Sha256::digest(&self.opaque).into()
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
            Self::Query => KIND_QUERY,
            Self::Replacement => KIND_REPLACEMENT,
            Self::MatchCase => KIND_MATCH_CASE,
            Self::WholeWord => KIND_WHOLE_WORD,
            Self::Regex => KIND_REGEX,
            Self::Replace => KIND_REPLACE,
            Self::ReplaceAll => KIND_REPLACE_ALL,
        }
    }
}

#[cfg(test)]
#[path = "sanitized_search_event_inline_tests.rs"]
mod tests;
