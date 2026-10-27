use super::super::TabStripInlineRenameStarted;

struct LegacyPort(Rc<Cell<usize>>);

impl TabStripProposalPort for LegacyPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        proposal.consume_for_port();
        self.0.set(self.0.get() + 1);
        Ok(())
    }
}

#[test]
fn legacy_port_accepts_rename_start_receipt_without_forwarding_an_operation() {
    let forwarded = Rc::new(Cell::new(0));
    let mut handle = TabStripProposalPortHandle::new(LegacyPort(Rc::clone(&forwarded)));
    let receipt = TabStripInlineRenameStarted::new(
        TabStripGroupTarget::from_opaque_bytes(b"group"),
        TabStripCorrelation::from_opaque_bytes(b"correlation"),
    );

    assert_eq!(Ok(()), handle.forward_inline_rename_started(receipt));
    assert_eq!(0, forwarded.get());
    assert_eq!(Ok(()), handle.forward_once(select(1)));
    assert_eq!(1, forwarded.get());
}
