use super::{
    TabStripProposal, TabStripProposalOperation, TabStripProposalPort, TabStripProposalPortError,
    TabStripProposalPortHandle,
};
use crate::text_command_surface::{TabStripCorrelation, TabStripTabTarget};
use std::cell::Cell;
use std::rc::Rc;

struct CountingPort(Rc<Cell<usize>>);

impl TabStripProposalPort for CountingPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        proposal.consume_for_port();
        self.0.set(self.0.get().saturating_add(1));
        Ok(())
    }
}

fn select(nonce: u64) -> TabStripProposal {
    TabStripProposal::new(
        nonce,
        TabStripCorrelation::from_opaque_bytes(b"correlation"),
        TabStripProposalOperation::SelectTab(TabStripTabTarget::from_opaque_bytes(b"tab")),
    )
}

#[test]
fn each_nonce_is_consumed_once_without_a_root_global_gate() {
    let count = Rc::new(Cell::new(0));
    let mut port = TabStripProposalPortHandle::new(CountingPort(Rc::clone(&count)));

    assert_eq!(Ok(()), port.forward_once(select(1)));
    assert_eq!(Ok(()), port.forward_once(select(2)));
    assert_eq!(
        Err(TabStripProposalPortError::Duplicate),
        port.forward_once(select(1))
    );
    assert_eq!(2, count.get());
}

#[test]
fn debug_does_not_reveal_proposal_payloads() {
    let proposal = select(9);
    assert_eq!("TabStripProposal(..)", format!("{proposal:?}"));
}
