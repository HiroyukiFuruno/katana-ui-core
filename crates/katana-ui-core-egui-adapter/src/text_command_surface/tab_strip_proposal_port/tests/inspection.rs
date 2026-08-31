#[test]
fn test_inspection_helpers_distinguish_navigation_and_group_operations() {
    let correlation = || TabStripCorrelation::from_opaque_bytes([4]);
    let group = || TabStripGroupTarget::from_opaque_bytes([2]);
    let tab = || TabStripTabTarget::from_opaque_bytes([1]);

    let previous =
        TabStripProposal::new(1, correlation(), TabStripProposalOperation::SelectPrevious);
    let next = TabStripProposal::new(2, correlation(), TabStripProposalOperation::SelectNext);
    let collapsed = TabStripProposal::new(
        3,
        correlation(),
        TabStripProposalOperation::SetGroupCollapsed {
            group: group(),
            collapsed: false,
        },
    );
    let unrelated = TabStripProposal::new(
        4,
        correlation(),
        TabStripProposalOperation::SelectTab(tab()),
    );

    assert_eq!(previous.navigation_direction_for_test(), Some(true));
    assert_eq!(next.navigation_direction_for_test(), Some(false));
    assert_eq!(unrelated.navigation_direction_for_test(), None);
    assert_eq!(collapsed.group_collapsed_for_test(), Some(false));
    assert_eq!(unrelated.group_collapsed_for_test(), None);
}
