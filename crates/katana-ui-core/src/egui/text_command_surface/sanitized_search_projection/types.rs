use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) const SEARCH_SIGNATURE_BYTES: usize = 32;

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

pub(crate) type TextCapability = Rc<
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
pub(crate) type UnitCapability = Rc<
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

pub(crate) enum SanitizedSearchCapability {
    Text(TextCapability),
    Unit(UnitCapability),
}

pub struct SanitizedSearchTarget {
    pub(crate) opaque: Box<[u8]>,
    pub(crate) capability: Option<SanitizedSearchCapability>,
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

    pub(crate) fn stable_signature(&self) -> [u8; SEARCH_SIGNATURE_BYTES] {
        Sha256::digest(&self.opaque).into()
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
