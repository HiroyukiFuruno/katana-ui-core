use super::tab_strip_projection_lease::TabStripCorrelation;
use super::{
    TabStripDragState, TabStripProposal, TabStripProposalOperation, TabStripProposalPortHandle,
    TabStripRetainedError, TabStripRetainedState,
};

pub(super) struct TabStripDragOrigin {
    port: Option<TabStripProposalPortHandle>,
    correlation: TabStripCorrelation,
}

impl TabStripRetainedState {
    pub(super) fn bind_drag_origin(&mut self) {
        if let Some(drag) = self.drag.as_mut()
            && drag.origin.is_none()
        {
            /* WHY: 後続leaseの操作配線を更新しても、Start通知を受けたhostへ終端を返す。 */
            drag.origin = Some(TabStripDragOrigin {
                port: self.port.take(),
                correlation: self.routes.correlation_for_proposal(),
            });
        }
    }

    pub(super) fn forward_drag_terminal(
        &mut self,
        drag: &mut TabStripDragState,
        operation: TabStripProposalOperation,
    ) -> Result<(), TabStripRetainedError> {
        let Some(origin) = drag.origin.as_mut() else {
            return self.forward_proposal(self.routes.correlation_for_proposal(), operation);
        };
        self.next_nonce = self.next_nonce.saturating_add(1);
        let proposal =
            TabStripProposal::new(self.next_nonce, origin.correlation.clone(), operation);
        origin
            .port
            .as_mut()
            .ok_or(TabStripRetainedError::MissingPort)?
            .forward_once(proposal)
            .map_err(TabStripRetainedError::Port)
    }
}

#[cfg(test)]
#[path = "drag_origin/tests.rs"]
mod tests;
