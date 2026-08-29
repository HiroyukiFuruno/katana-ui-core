use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanitizedCommandCapabilityRejection {
    Missing,
    AlreadyConsumed,
    Reentrant,
    CallbackRejected,
}

pub(crate) type CommandCapability =
    Rc<RefCell<Option<Box<dyn FnOnce() -> Result<(), SanitizedCommandCapabilityRejection>>>>>;

/// Opaque command target supplied by the host.
pub struct SanitizedCommandTarget {
    opaque: Box<[u8]>,
    capability: Option<CommandCapability>,
}

impl SanitizedCommandTarget {
    /// Creates a target without assigning meaning to the supplied data.
    #[must_use]
    pub fn from_opaque_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            opaque: bytes.into().into_boxed_slice(),
            capability: None,
        }
    }

    #[must_use]
    pub fn with_unit_capability<F, E>(mut self, callback: F) -> Self
    where
        F: FnOnce() -> Result<(), E> + 'static,
        E: 'static,
    {
        self.capability = Some(Rc::new(RefCell::new(Some(Box::new(move || {
            callback().map_err(|_| SanitizedCommandCapabilityRejection::CallbackRejected)
        })))));
        self
    }

    #[must_use]
    pub(crate) fn stable_fingerprint(&self) -> String {
        format!("{:x}", Sha256::digest(&self.opaque))
    }

    pub(crate) fn capability(&self) -> Option<CommandCapability> {
        self.capability.clone()
    }
}

impl std::fmt::Debug for SanitizedCommandTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter.write_str("SanitizedCommandTarget(..)")
    }
}
