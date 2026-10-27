use super::super::tab_strip_projection_lease::{
    TabStripCorrelation, TabStripGroupTarget, TabStripSwatchTarget, TabStripTabTarget, TabStripText,
};

/// 添字を公開せず、opaque targetでタブの移動先を表す。
pub enum TabStripTabPlacement {
    Before(TabStripTabTarget),
    After(TabStripTabTarget),
    InGroup(TabStripGroupTarget),
    EndOfStrip,
    NewGroup,
}

/// 添字を公開せず、opaque targetでグループの移動先を表す。
pub enum TabStripGroupPlacement {
    Before(TabStripGroupTarget),
    After(TabStripGroupTarget),
    EndOfStrip,
}

/// hostが対象と確定値を処理する型付き操作通知。
pub enum TabStripProposalOperation {
    SelectTab(TabStripTabTarget),
    SelectPrevious,
    SelectNext,
    OpenOverflow,
    RequestClose(TabStripTabTarget),
    ConfirmClose(TabStripTabTarget),
    CloseOthers(TabStripTabTarget),
    CloseAll,
    CloseToLeft(TabStripTabTarget),
    CloseToRight(TabStripTabTarget),
    RestoreClosed,
    SetPinned {
        tab: TabStripTabTarget,
        pinned: bool,
    },
    CreateGroup(TabStripTabTarget),
    MoveTabToGroup {
        tab: TabStripTabTarget,
        group: TabStripGroupTarget,
    },
    RemoveFromGroup(TabStripTabTarget),
    SetGroupCollapsed {
        group: TabStripGroupTarget,
        collapsed: bool,
    },
    RenameGroup {
        group: TabStripGroupTarget,
        name: TabStripText,
    },
    RecolorGroup {
        group: TabStripGroupTarget,
        swatch: TabStripSwatchTarget,
    },
    Ungroup(TabStripGroupTarget),
    CloseGroup(TabStripGroupTarget),
    ReorderTab {
        source: TabStripTabTarget,
        destination: TabStripTabPlacement,
    },
    ReorderGroup {
        source: TabStripGroupTarget,
        destination: TabStripGroupPlacement,
    },
    StartDrag(TabStripTabTarget),
    FinishDrag {
        committed: bool,
        destination: Option<TabStripTabPlacement>,
    },
    CancelDrag,
    HoverCollapsedGroup {
        group: TabStripGroupTarget,
    },
}

/// 操作を発行した UI の種類。位置やラベルは公開しない。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStripProposalOrigin {
    TabBody,
    TrailingControl,
    ContextMenu,
    Navigation,
    GroupHeader,
    GroupPopup,
    DragTerminal,
}

/// KUCのroute解決後に発行され、hostへ一度だけ渡される操作。
pub struct TabStripProposal {
    nonce: u64,
    correlation: TabStripCorrelation,
    operation: TabStripProposalOperation,
    origin: Option<TabStripProposalOrigin>,
}

impl TabStripProposal {
    pub const fn new(
        nonce: u64,
        correlation: TabStripCorrelation,
        operation: TabStripProposalOperation,
    ) -> Self {
        Self {
            nonce,
            correlation,
            operation,
            origin: None,
        }
    }

    /// 明示された起点を付ける。従来の constructor は None を返す。
    #[must_use]
    pub const fn with_origin(mut self, origin: TabStripProposalOrigin) -> Self {
        self.origin = Some(origin);
        self
    }

    #[must_use]
    pub const fn origin(&self) -> Option<TabStripProposalOrigin> {
        self.origin
    }

    pub const fn nonce(&self) -> u64 {
        self.nonce
    }

    /// payloadを公開せず、hostが発行したcorrelationと照合できる値を返す。
    pub fn correlation(&self) -> TabStripCorrelation {
        TabStripCorrelation::from_opaque_bytes(self.correlation.payload.to_vec())
    }

    /// proposalを消費してhostへ型付き操作を渡す。KUCは対象のアプリ固有の意味を解釈しない。
    pub fn into_operation(self) -> TabStripProposalOperation {
        self.operation
    }

    pub fn consume_for_port(self) {
        let _ = self.correlation.payload.len();
        match self.operation {
            TabStripProposalOperation::SelectTab(target)
            | TabStripProposalOperation::RequestClose(target)
            | TabStripProposalOperation::ConfirmClose(target)
            | TabStripProposalOperation::CloseOthers(target)
            | TabStripProposalOperation::CloseToLeft(target)
            | TabStripProposalOperation::CloseToRight(target)
            | TabStripProposalOperation::CreateGroup(target)
            | TabStripProposalOperation::RemoveFromGroup(target)
            | TabStripProposalOperation::StartDrag(target) => {
                let _ = target.payload.len();
            }
            TabStripProposalOperation::SelectPrevious
            | TabStripProposalOperation::SelectNext
            | TabStripProposalOperation::OpenOverflow
            | TabStripProposalOperation::CloseAll
            | TabStripProposalOperation::RestoreClosed
            | TabStripProposalOperation::CancelDrag => {}
            TabStripProposalOperation::SetPinned { tab, pinned } => {
                let _ = tab.payload.len();
                let _ = pinned;
            }
            TabStripProposalOperation::MoveTabToGroup { tab, group } => {
                let _ = tab.payload.len();
                let _ = group.payload.len();
            }
            TabStripProposalOperation::SetGroupCollapsed { group, collapsed } => {
                let _ = group.payload.len();
                let _ = collapsed;
            }
            TabStripProposalOperation::RenameGroup { group, name } => {
                let _ = group.payload.len();
                let _ = name.value.len();
            }
            TabStripProposalOperation::RecolorGroup { group, swatch } => {
                let _ = group.payload.len();
                let _ = swatch.payload.len();
            }
            TabStripProposalOperation::Ungroup(group)
            | TabStripProposalOperation::CloseGroup(group) => {
                let _ = group.payload.len();
            }
            TabStripProposalOperation::ReorderTab {
                source,
                destination,
            } => {
                let _ = source.payload.len();
                consume_tab_placement(destination);
            }
            TabStripProposalOperation::ReorderGroup {
                source,
                destination,
            } => {
                let _ = source.payload.len();
                consume_group_placement(destination);
            }
            TabStripProposalOperation::FinishDrag {
                committed,
                destination,
            } => {
                let _ = committed;
                if let Some(destination) = destination {
                    consume_tab_placement(destination);
                }
            }
            TabStripProposalOperation::HoverCollapsedGroup { group } => {
                let _ = group.payload.len();
            }
        }
    }

    #[cfg(test)]
    pub(crate) const fn navigation_direction_for_test(&self) -> Option<bool> {
        match self.operation {
            TabStripProposalOperation::SelectPrevious => Some(true),
            TabStripProposalOperation::SelectNext => Some(false),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) const fn group_collapsed_for_test(&self) -> Option<bool> {
        match self.operation {
            TabStripProposalOperation::SetGroupCollapsed { collapsed, .. } => Some(collapsed),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) const fn operation_class_for_test(&self) -> TabStripProposalOperationClass {
        match self.operation {
            TabStripProposalOperation::RequestClose(_) => {
                TabStripProposalOperationClass::RequestClose
            }
            TabStripProposalOperation::SetPinned { pinned: false, .. } => {
                TabStripProposalOperationClass::Unpin
            }
            TabStripProposalOperation::RenameGroup { .. } => {
                TabStripProposalOperationClass::RenameGroup
            }
            TabStripProposalOperation::RecolorGroup { .. } => {
                TabStripProposalOperationClass::RecolorGroup
            }
            TabStripProposalOperation::StartDrag(_) => TabStripProposalOperationClass::StartDrag,
            TabStripProposalOperation::FinishDrag {
                destination: Some(TabStripTabPlacement::Before(_)),
                ..
            } => TabStripProposalOperationClass::FinishDragBefore,
            TabStripProposalOperation::FinishDrag {
                destination: Some(TabStripTabPlacement::After(_)),
                ..
            } => TabStripProposalOperationClass::FinishDragAfter,
            TabStripProposalOperation::FinishDrag {
                destination: Some(TabStripTabPlacement::InGroup(_)),
                ..
            } => TabStripProposalOperationClass::FinishDragInGroup,
            TabStripProposalOperation::FinishDrag {
                destination: Some(TabStripTabPlacement::EndOfStrip),
                ..
            } => TabStripProposalOperationClass::FinishDragAtEnd,
            TabStripProposalOperation::FinishDrag {
                destination: None, ..
            } => TabStripProposalOperationClass::FinishDragWithoutDestination,
            TabStripProposalOperation::CancelDrag => TabStripProposalOperationClass::CancelDrag,
            _ => TabStripProposalOperationClass::Other,
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TabStripProposalOperationClass {
    RequestClose,
    Unpin,
    RenameGroup,
    RecolorGroup,
    StartDrag,
    FinishDragBefore,
    FinishDragAfter,
    FinishDragInGroup,
    FinishDragAtEnd,
    FinishDragWithoutDestination,
    CancelDrag,
    Other,
}
fn consume_tab_placement(placement: TabStripTabPlacement) {
    match placement {
        TabStripTabPlacement::Before(target) | TabStripTabPlacement::After(target) => {
            let _ = target.payload.len();
        }
        TabStripTabPlacement::InGroup(group) => {
            let _ = group.payload.len();
        }
        TabStripTabPlacement::EndOfStrip | TabStripTabPlacement::NewGroup => {}
    }
}

fn consume_group_placement(placement: TabStripGroupPlacement) {
    match placement {
        TabStripGroupPlacement::Before(target) | TabStripGroupPlacement::After(target) => {
            let _ = target.payload.len();
        }
        TabStripGroupPlacement::EndOfStrip => {}
    }
}
