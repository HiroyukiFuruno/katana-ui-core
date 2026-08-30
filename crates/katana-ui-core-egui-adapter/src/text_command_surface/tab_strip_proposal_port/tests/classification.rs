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
