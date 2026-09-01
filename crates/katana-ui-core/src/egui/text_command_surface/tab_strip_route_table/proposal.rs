use super::super::tab_strip_proposal_port::TabStripProposalOperation;
use super::TabStripRoute;

impl TabStripRoute {
    pub(super) fn proposal(&self) -> TabStripProposalOperation {
        match self {
            Self::SelectTab(tab) => TabStripProposalOperation::SelectTab(tab.copy_for_route()),
            Self::Previous => TabStripProposalOperation::SelectPrevious,
            Self::Next => TabStripProposalOperation::SelectNext,
            Self::OpenOverflow => TabStripProposalOperation::OpenOverflow,
            Self::RequestClose(tab) => {
                TabStripProposalOperation::RequestClose(tab.copy_for_route())
            }
            Self::CloseOthers(tab) => TabStripProposalOperation::CloseOthers(tab.copy_for_route()),
            Self::CloseAll => TabStripProposalOperation::CloseAll,
            Self::CloseToLeft(tab) => TabStripProposalOperation::CloseToLeft(tab.copy_for_route()),
            Self::CloseToRight(tab) => {
                TabStripProposalOperation::CloseToRight(tab.copy_for_route())
            }
            Self::RestoreClosed => TabStripProposalOperation::RestoreClosed,
            Self::Unpin(tab) => TabStripProposalOperation::SetPinned {
                tab: tab.copy_for_route(),
                pinned: false,
            },
            Self::Pin(tab) => TabStripProposalOperation::SetPinned {
                tab: tab.copy_for_route(),
                pinned: true,
            },
            Self::CreateGroup(tab) => TabStripProposalOperation::CreateGroup(tab.copy_for_route()),
            Self::MoveTabToGroup { tab, group } => TabStripProposalOperation::MoveTabToGroup {
                tab: tab.copy_for_route(),
                group: group.copy_for_route(),
            },
            Self::RemoveFromGroup(tab) => {
                TabStripProposalOperation::RemoveFromGroup(tab.copy_for_route())
            }
            Self::SetGroupCollapsed { group, collapsed } => {
                TabStripProposalOperation::SetGroupCollapsed {
                    group: group.copy_for_route(),
                    collapsed: *collapsed,
                }
            }
            Self::RenameGroup(_) => unreachable!("rename proposals require their one-shot name"),
            Self::RecolorGroup { group, swatch } => TabStripProposalOperation::RecolorGroup {
                group: group.copy_for_route(),
                swatch: swatch.copy_for_route(),
            },
            Self::Ungroup(group) => TabStripProposalOperation::Ungroup(group.copy_for_route()),
            Self::CloseGroup(group) => {
                TabStripProposalOperation::CloseGroup(group.copy_for_route())
            }
        }
    }
}
