use super::{
    TabStripProposal, TabStripProposalOperation, TabStripProposalOperationClass,
    TabStripProposalPort, TabStripProposalPortError, TabStripProposalPortHandle,
};
use crate::text_command_surface::{
    TabStripCorrelation, TabStripGroupPlacement, TabStripGroupTarget, TabStripSwatchTarget,
    TabStripTabPlacement, TabStripTabTarget, TabStripText,
};
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

#[test]
fn every_operation_and_placement_is_consumed_as_opaque() {
    let tab = || TabStripTabTarget::from_opaque_bytes([1]);
    let group = || TabStripGroupTarget::from_opaque_bytes([2]);
    let swatch = || TabStripSwatchTarget::from_opaque_bytes([3]);
    let correlation = || TabStripCorrelation::from_opaque_bytes([4]);
    let operations = vec![
        TabStripProposalOperation::SelectTab(tab()),
        TabStripProposalOperation::SelectPrevious,
        TabStripProposalOperation::SelectNext,
        TabStripProposalOperation::OpenOverflow,
        TabStripProposalOperation::RequestClose(tab()),
        TabStripProposalOperation::ConfirmClose(tab()),
        TabStripProposalOperation::CloseOthers(tab()),
        TabStripProposalOperation::CloseAll,
        TabStripProposalOperation::CloseToLeft(tab()),
        TabStripProposalOperation::CloseToRight(tab()),
        TabStripProposalOperation::RestoreClosed,
        TabStripProposalOperation::SetPinned {
            tab: tab(),
            pinned: true,
        },
        TabStripProposalOperation::CreateGroup(tab()),
        TabStripProposalOperation::MoveTabToGroup {
            tab: tab(),
            group: group(),
        },
        TabStripProposalOperation::RemoveFromGroup(tab()),
        TabStripProposalOperation::SetGroupCollapsed {
            group: group(),
            collapsed: true,
        },
        TabStripProposalOperation::RenameGroup {
            group: group(),
            name: TabStripText::new("name"),
        },
        TabStripProposalOperation::RecolorGroup {
            group: group(),
            swatch: swatch(),
        },
        TabStripProposalOperation::Ungroup(group()),
        TabStripProposalOperation::CloseGroup(group()),
        TabStripProposalOperation::ReorderTab {
            source: tab(),
            destination: TabStripTabPlacement::Before(tab()),
        },
        TabStripProposalOperation::ReorderTab {
            source: tab(),
            destination: TabStripTabPlacement::After(tab()),
        },
        TabStripProposalOperation::ReorderTab {
            source: tab(),
            destination: TabStripTabPlacement::InGroup(group()),
        },
        TabStripProposalOperation::ReorderTab {
            source: tab(),
            destination: TabStripTabPlacement::EndOfStrip,
        },
        TabStripProposalOperation::ReorderTab {
            source: tab(),
            destination: TabStripTabPlacement::NewGroup,
        },
        TabStripProposalOperation::ReorderGroup {
            source: group(),
            destination: TabStripGroupPlacement::Before(group()),
        },
        TabStripProposalOperation::ReorderGroup {
            source: group(),
            destination: TabStripGroupPlacement::After(group()),
        },
        TabStripProposalOperation::ReorderGroup {
            source: group(),
            destination: TabStripGroupPlacement::EndOfStrip,
        },
        TabStripProposalOperation::StartDrag(tab()),
        TabStripProposalOperation::FinishDrag {
            committed: true,
            destination: Some(TabStripTabPlacement::EndOfStrip),
        },
        TabStripProposalOperation::FinishDrag {
            committed: false,
            destination: None,
        },
        TabStripProposalOperation::CancelDrag,
        TabStripProposalOperation::HoverCollapsedGroup { group: group() },
    ];
    for (nonce, operation) in operations.into_iter().enumerate() {
        TabStripProposal::new(nonce as u64, correlation(), operation).consume_for_port();
    }
}

#[test]
fn operation_classification_covers_all_important_branches() {
    let tab = || TabStripTabTarget::from_opaque_bytes([1]);
    let group = || TabStripGroupTarget::from_opaque_bytes([2]);
    let swatch = || TabStripSwatchTarget::from_opaque_bytes([3]);
    let correlation = || TabStripCorrelation::from_opaque_bytes([4]);

    assert_eq!(
        TabStripProposal::new(
            1,
            correlation(),
            TabStripProposalOperation::RequestClose(tab())
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::RequestClose
    );
    assert_eq!(
        TabStripProposal::new(
            2,
            correlation(),
            TabStripProposalOperation::SetPinned {
                tab: tab(),
                pinned: false
            }
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::Unpin
    );
    assert_eq!(
        TabStripProposal::new(
            3,
            correlation(),
            TabStripProposalOperation::RenameGroup {
                group: group(),
                name: TabStripText::new("name")
            }
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::RenameGroup
    );
    assert_eq!(
        TabStripProposal::new(
            4,
            correlation(),
            TabStripProposalOperation::RecolorGroup {
                group: group(),
                swatch: swatch()
            }
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::RecolorGroup
    );
    assert_eq!(
        TabStripProposal::new(
            5,
            correlation(),
            TabStripProposalOperation::StartDrag(tab())
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::StartDrag
    );
    assert_eq!(
        TabStripProposal::new(
            6,
            correlation(),
            TabStripProposalOperation::FinishDrag {
                committed: true,
                destination: Some(TabStripTabPlacement::Before(tab()))
            }
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::FinishDragBefore
    );
    assert_eq!(
        TabStripProposal::new(
            7,
            correlation(),
            TabStripProposalOperation::FinishDrag {
                committed: true,
                destination: Some(TabStripTabPlacement::After(tab()))
            }
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::FinishDragAfter
    );
    assert_eq!(
        TabStripProposal::new(
            8,
            correlation(),
            TabStripProposalOperation::FinishDrag {
                committed: true,
                destination: Some(TabStripTabPlacement::InGroup(group()))
            }
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::FinishDragInGroup
    );
    assert_eq!(
        TabStripProposal::new(
            9,
            correlation(),
            TabStripProposalOperation::FinishDrag {
                committed: true,
                destination: Some(TabStripTabPlacement::EndOfStrip)
            }
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::FinishDragAtEnd
    );
    assert_eq!(
        TabStripProposal::new(
            10,
            correlation(),
            TabStripProposalOperation::FinishDrag {
                committed: true,
                destination: None
            }
        )
        .operation_class_for_test(),
        TabStripProposalOperationClass::FinishDragWithoutDestination
    );
    assert_eq!(
        TabStripProposal::new(11, correlation(), TabStripProposalOperation::CancelDrag)
            .operation_class_for_test(),
        TabStripProposalOperationClass::CancelDrag
    );
    assert_eq!(
        TabStripProposal::new(12, correlation(), TabStripProposalOperation::SelectPrevious)
            .operation_class_for_test(),
        TabStripProposalOperationClass::Other
    );
}
