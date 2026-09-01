use super::model::TabStripProposal;
use std::collections::BTreeSet;

pub trait TabStripProposalPort {
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

pub struct TabStripProposalPortHandle {
    port: Box<dyn TabStripProposalPort>,
    consumed_nonces: BTreeSet<u64>,
}

impl TabStripProposalPortHandle {
    pub fn new(port: impl TabStripProposalPort + 'static) -> Self {
        Self {
            port: Box::new(port),
            consumed_nonces: BTreeSet::new(),
        }
    }

    pub fn forward_once(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        if !self.consumed_nonces.insert(proposal.nonce()) {
            return Err(TabStripProposalPortError::Duplicate);
        }
        self.port.forward_proposal(proposal)
    }
}

impl std::fmt::Debug for TabStripProposal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("TabStripProposal(..)")
    }
}
