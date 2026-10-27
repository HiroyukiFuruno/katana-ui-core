use super::super::tab_strip_projection_lease::{TabStripCorrelation, TabStripGroupTarget};
use super::model::TabStripProposal;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

/// 要求された入力欄が実際に開かれたことを表す通知。
pub struct TabStripInlineRenameStarted {
    group: TabStripGroupTarget,
    correlation: TabStripCorrelation,
}
impl TabStripInlineRenameStarted {
    pub(crate) fn new(group: TabStripGroupTarget, correlation: TabStripCorrelation) -> Self {
        Self { group, correlation }
    }
    #[must_use]
    pub fn group(&self) -> TabStripGroupTarget {
        self.group.clone()
    }
    #[must_use]
    pub fn correlation(&self) -> TabStripCorrelation {
        self.correlation.clone()
    }
}

pub trait TabStripProposalPort {
    fn inline_rename_started(
        &mut self,
        _receipt: TabStripInlineRenameStarted,
    ) -> Result<(), TabStripProposalPortError> {
        Ok(())
    }
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStripProposalPortError {
    Rejected,
    Duplicate,
}

#[derive(Clone)]
pub struct TabStripProposalPortHandle {
    inner: Rc<RefCell<TabStripProposalPortInner>>,
}
struct TabStripProposalPortInner {
    port: Box<dyn TabStripProposalPort>,
    consumed_nonces: BTreeSet<u64>,
}

impl TabStripProposalPortHandle {
    pub fn new(port: impl TabStripProposalPort + 'static) -> Self {
        Self {
            inner: Rc::new(RefCell::new(TabStripProposalPortInner {
                port: Box::new(port),
                consumed_nonces: BTreeSet::new(),
            })),
        }
    }

    pub fn forward_once(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        let mut inner = self.inner.borrow_mut();
        if !inner.consumed_nonces.insert(proposal.nonce()) {
            return Err(TabStripProposalPortError::Duplicate);
        }
        inner.port.forward_proposal(proposal)
    }
    pub(crate) fn forward_inline_rename_started(
        &mut self,
        receipt: TabStripInlineRenameStarted,
    ) -> Result<(), TabStripProposalPortError> {
        self.inner.borrow_mut().port.inline_rename_started(receipt)
    }
}

impl std::fmt::Debug for TabStripProposal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("TabStripProposal(..)")
    }
}
