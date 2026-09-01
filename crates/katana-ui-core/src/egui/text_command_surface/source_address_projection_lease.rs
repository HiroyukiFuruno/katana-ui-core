//! Unwired ownership lease for the generic source-address projection.
//!
//! This module intentionally has no public module export yet. The retained
//! root integration will consume the lease when its projection boundary is
//! ready.

use crate::molecule::structured::source_address_strip::SourceAddressStrip;
use crate::molecule::structured::source_address_strip::SourceAddressSubmission;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceAddressSubmissionPortError {
    Rejected,
}

pub trait SourceAddressSubmissionPort {
    fn forward_submission(
        &mut self,
        submission: SourceAddressSubmission,
    ) -> Result<(), SourceAddressSubmissionPortError>;
}

pub(crate) struct SourceAddressSubmissionPortHandle(
    Rc<RefCell<Box<dyn SourceAddressSubmissionPort>>>,
);

impl SourceAddressSubmissionPortHandle {
    pub(crate) fn new(port: impl SourceAddressSubmissionPort + 'static) -> Self {
        Self(Rc::new(RefCell::new(Box::new(port))))
    }

    pub(crate) fn forward_submission(
        &self,
        submission: SourceAddressSubmission,
    ) -> Result<(), SourceAddressSubmissionPortError> {
        self.0.borrow_mut().forward_submission(submission)
    }
}

impl Clone for SourceAddressSubmissionPortHandle {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

/// Owns a generic source-address strip until the retained root consumes it.
pub struct SourceAddressProjectionLease {
    strip: SourceAddressStrip,
    submission_port: Option<SourceAddressSubmissionPortHandle>,
}

impl SourceAddressProjectionLease {
    /// Retains a source-address strip without interpreting its contents.
    #[must_use]
    pub const fn new(strip: SourceAddressStrip) -> Self {
        Self {
            strip,
            submission_port: None,
        }
    }

    #[must_use]
    pub fn with_submission_port<P>(mut self, port: P) -> Self
    where
        P: SourceAddressSubmissionPort + 'static,
    {
        self.submission_port = Some(SourceAddressSubmissionPortHandle::new(port));
        self
    }

    /// Gives the retained strip to the next root-integration stage.
    #[cfg(test)]
    pub(crate) fn into_strip(self) -> SourceAddressStrip {
        self.strip
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SourceAddressStrip,
        Option<SourceAddressSubmissionPortHandle>,
    ) {
        (self.strip, self.submission_port)
    }
}

#[cfg(test)]
mod tests {
    use super::SourceAddressProjectionLease;
    use crate::molecule::structured::source_address_strip::{
        SourceAddressPresentation, SourceAddressStrip,
    };

    #[test]
    fn lease_consumes_and_returns_the_same_strip() {
        let strip = SourceAddressStrip::new(SourceAddressPresentation::new(
            "表示",
            "ツールチップ",
            "アクセシビリティ",
        ));
        let lease = SourceAddressProjectionLease::new(strip);

        let returned = lease.into_strip();

        assert_eq!(returned.presentation().visible(), "表示");
    }

    #[test]
    fn lease_api_stays_non_wire_and_non_readback() {
        let source = include_str!("source_address_projection_lease.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("test module marker is present");

        for forbidden in [
            "impl std::fmt::Debug",
            "impl fmt::Debug",
            "Serialize",
            "Deserialize",
            "fn target",
            "fn draft",
            "fn presentation",
            "pub(crate) fn strip",
            "pub(crate) fn target",
            "pub(crate) fn draft",
        ] {
            assert!(
                !production.contains(forbidden),
                "lease source exposes forbidden API: {forbidden}"
            );
        }
    }
}
