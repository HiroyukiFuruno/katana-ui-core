use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum SanitizedContextMenuCapabilityRejection {
    Missing,
    AlreadyConsumed,
    Reentrant,
    CallbackRejected,
}

pub(crate) type ContextMenuCapability =
    Rc<RefCell<Option<Box<dyn FnOnce() -> Result<(), SanitizedContextMenuCapabilityRejection>>>>>;

/// Opaque target supplied by the host for a context-menu item.
pub struct SanitizedContextMenuTarget {
    opaque: Box<[u8]>,
    capability: Option<ContextMenuCapability>,
}

impl SanitizedContextMenuTarget {
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
            callback().map_err(|_| SanitizedContextMenuCapabilityRejection::CallbackRejected)
        })))));
        self
    }

    #[must_use]
    pub(crate) const fn opaque(&self) -> &[u8] {
        &self.opaque
    }

    pub(crate) fn capability(&self) -> Option<ContextMenuCapability> {
        self.capability.as_ref().map(std::borrow::ToOwned::to_owned)
    }
}

impl std::fmt::Debug for SanitizedContextMenuTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque().len();
        formatter.write_str("SanitizedContextMenuTarget(..)")
    }
}
